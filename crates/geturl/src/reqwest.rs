// Copyright 2017-2020 the Tectonic Project
// Licensed under the MIT License.

//! A URL-get backend based on the `reqwest` crate.

use reqwest::{
    StatusCode, Url,
    blocking::{Client, Response},
    header::RANGE,
    redirect::Policy,
};
use std::sync::OnceLock;
use tectonic_errors::{Result, anyhow::bail};

use crate::{GetUrlBackend, RangeReader};

const MAX_HTTP_REDIRECTS_ALLOWED: usize = 10;

/// URL-get backend implemented using the `reqwest` crate.
#[derive(Debug, Default)]
pub struct ReqwestBackend {
    client: OnceLock<Client>,
}

impl GetUrlBackend for ReqwestBackend {
    type Response = Response;
    type RangeReader = ReqwestRangeReader;

    fn get_url(&mut self, url: &str) -> Result<Response> {
        let res = self.client.get_or_init(Client::new).get(url).send()?;
        if !res.status().is_success() {
            bail!(
                "unexpected HTTP response code {} for URL {}",
                res.status(),
                url
            );
        }
        Ok(res)
    }

    fn resolve_url(&mut self, url: &str) -> Result<String> {
        let parsed = Url::parse(url)?;
        let original_filename = parsed
            .path_segments()
            .and_then(|mut s| s.next_back())
            .unwrap_or(".") // if the filename is this, the `contains('.')` will already match
            .to_owned();

        // First, we actually do a HEAD request on the URL for the data file.
        // If it's redirected, we update our URL to follow the redirects. If
        // we didn't do this separately, the index file would have to be the
        // one with the redirect setup, which would be confusing and annoying.
        let redirect_policy = Policy::custom(move |attempt| {
            // In the process of resolving the file URL it might be necessary to
            // stop at a certain level of redirection. This might be required
            // because some hosts might redirect to a version of the URL where
            // it isn't possible to select the index file by appending
            // `.index.gz`. (This mostly happens because CDNs redirect to a file
            // hash.) However, the original default Tectonic URL doesn't contain
            // a `.` in the filename, and an October 2021 change to archive.org
            // added a redirect through a secondary URL with a similar path
            // structure. So we also keep resolving if the new filename seems to
            // match the original filename.
            if attempt.previous().len() >= MAX_HTTP_REDIRECTS_ALLOWED {
                attempt.error("too many redirections")
            } else if let Some(mut segments) = attempt.url().path_segments() {
                let follow = segments
                    .next_back()
                    .map(|file| file.contains('.') || file == original_filename)
                    .unwrap_or(true);
                if follow {
                    attempt.follow()
                } else {
                    attempt.stop()
                }
            } else {
                attempt.follow()
            }
        });

        let res = Client::builder()
            .redirect(redirect_policy)
            .build()?
            .head(url)
            .send()?;

        if !(res.status().is_success()
            || res.status() == StatusCode::FOUND
            || res.status() == StatusCode::TEMPORARY_REDIRECT)
        {
            bail!(
                "unexpected HTTP response code {} for URL {}",
                res.status(),
                url
            );
        }

        let final_url: String = res.url().clone().into();
        Ok(final_url)
    }

    fn open_range_reader(&self, url: &str) -> Self::RangeReader {
        ReqwestRangeReader::new(url, self.client.get_or_init(Client::new).clone())
    }
}

/// A simple way to read chunks out of a big seekable byte stream. You could
/// implement this for io::File pretty trivially but that's not currently
/// needed.
#[derive(Debug)]
pub struct ReqwestRangeReader {
    url: String,
    client: Client,
}

impl ReqwestRangeReader {
    fn new(url: &str, client: Client) -> ReqwestRangeReader {
        ReqwestRangeReader {
            url: url.to_owned(),
            client,
        }
    }
}

impl RangeReader for ReqwestRangeReader {
    type Response = Response;

    fn read_range(&mut self, offset: u64, length: usize) -> Result<Response> {
        let end_inclusive = offset + length as u64 - 1;
        let res = self
            .client
            .get(&self.url)
            .header(RANGE, format!("bytes={offset}-{end_inclusive}"))
            .send()?;

        if res.status() != StatusCode::PARTIAL_CONTENT {
            bail!(
                "unexpected HTTP response code {} for URL {}",
                res.status(),
                self.url
            );
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{BufRead, BufReader, Read, Write},
        net::TcpListener,
        thread,
        time::Duration,
    };

    #[test]
    fn get_and_range_requests_share_a_connection() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/file", listener.local_addr().unwrap());
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());

            for ranged in [false, false, true] {
                let mut request = String::new();
                loop {
                    let mut line = String::new();
                    assert_ne!(reader.read_line(&mut line).unwrap(), 0);
                    request.push_str(&line);
                    if line == "\r\n" {
                        break;
                    }
                }
                assert!(request.starts_with("GET /file HTTP/1.1\r\n"));
                assert_eq!(
                    request
                        .to_ascii_lowercase()
                        .contains("range: bytes=2-5\r\n"),
                    ranged
                );
                let status = if ranged {
                    "206 Partial Content"
                } else {
                    "200 OK"
                };
                write!(stream, "HTTP/1.1 {status}\r\nContent-Length: 4\r\n\r\ndata").unwrap();
            }
        });

        let mut backend = ReqwestBackend {
            client: Client::builder()
                .no_proxy()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap()
                .into(),
        };
        let mut contents = String::new();
        for _ in 0..2 {
            backend
                .get_url(&url)
                .unwrap()
                .read_to_string(&mut contents)
                .unwrap();
        }
        backend
            .open_range_reader(&url)
            .read_range(2, 4)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        assert_eq!(contents, "datadatadata");
        server.join().unwrap();
    }
}
