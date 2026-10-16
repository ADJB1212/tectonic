use flate2::{Compression, Crc, read::ZlibDecoder, write::ZlibEncoder};
use std::{
    collections::HashSet,
    io::{Read, Write},
};
use tectonic::{
    TexEngine, XdvipdfmxEngine,
    io::{FilesystemIo, IoStack, MemoryIo, testing::SingleInputFileIo},
};
use tectonic_bridge_core::{CoreBridgeLauncher, MinimalDriver};
use tectonic_io_base::stdstreams::BufferedPrimaryIo;
use tectonic_status_base::NoopStatusBackend;

mod util;

fn stream_data<'a>(pdf: &'a [u8], marker: &[u8]) -> (&'a str, &'a [u8]) {
    let offset = pdf
        .windows(marker.len())
        .position(|bytes| bytes == marker)
        .unwrap();
    let object_start = pdf[..offset]
        .windows(5)
        .rposition(|bytes| bytes == b" obj\n")
        .unwrap()
        + 5;
    let stream = &pdf[object_start..];
    let body_start = stream
        .windows(8)
        .position(|bytes| bytes == b"\nstream\n")
        .unwrap()
        + 8;
    let dictionary = std::str::from_utf8(&stream[..body_start]).unwrap();
    let length = dictionary
        .split("/Length")
        .nth(1)
        .unwrap()
        .trim_start()
        .split(|c: char| !c.is_ascii_digit())
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    assert_eq!(&stream[body_start + length..][..10], b"\nendstream");
    (dictionary, &stream[body_start..body_start + length])
}

fn bmp_image(width: usize, height: usize) -> Vec<u8> {
    let image_len = width * height * 3;
    let mut bmp = vec![0; 54 + image_len];
    let file_len = bmp.len() as u32;
    bmp[..2].copy_from_slice(b"BM");
    bmp[2..6].copy_from_slice(&file_len.to_le_bytes());
    bmp[10..14].copy_from_slice(&54u32.to_le_bytes());
    bmp[14..18].copy_from_slice(&40u32.to_le_bytes());
    bmp[18..22].copy_from_slice(&(width as u32).to_le_bytes());
    bmp[22..26].copy_from_slice(&(height as u32).to_le_bytes());
    bmp[26..28].copy_from_slice(&1u16.to_le_bytes());
    bmp[28..30].copy_from_slice(&24u16.to_le_bytes());
    bmp[34..38].copy_from_slice(&(image_len as u32).to_le_bytes());
    for (index, pixel) in bmp[54..].as_chunks_mut::<3>().0.iter_mut().enumerate() {
        let x = index % width;
        let y = height - 1 - index / width;
        pixel.copy_from_slice(&[((x + y) % 251) as u8, (x % 251) as u8, (y % 251) as u8]);
    }
    bmp
}

fn png_alpha_image(width: usize, height: usize) -> Vec<u8> {
    fn chunk(png: &mut Vec<u8>, kind: [u8; 4], data: &[u8]) {
        png.extend_from_slice(&(data.len() as u32).to_be_bytes());
        png.extend_from_slice(&kind);
        png.extend_from_slice(data);
        let mut crc = Crc::new();
        crc.update(&kind);
        crc.update(data);
        png.extend_from_slice(&crc.sum().to_be_bytes());
    }

    let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut header = Vec::new();
    header.extend_from_slice(&(width as u32).to_be_bytes());
    header.extend_from_slice(&(height as u32).to_be_bytes());
    header.extend_from_slice(&[16, 6, 0, 0, 0]);
    chunk(&mut png, *b"IHDR", &header);
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
    for y in 0..height {
        encoder.write_all(&[0]).unwrap();
        for x in 0..width {
            for sample in [x as u16, y as u16, 0, (x * 4093 + y * 8191) as u16] {
                encoder.write_all(&sample.to_be_bytes()).unwrap();
            }
        }
    }
    chunk(&mut png, *b"IDAT", &encoder.finish().unwrap());
    chunk(&mut png, *b"IEND", &[]);
    png
}

fn decode_png_predictor(filtered: &[u8], width: usize, height: usize) -> Vec<u8> {
    let row_bytes = width * 3;
    assert_eq!(filtered.len(), (row_bytes + 1) * height);
    let mut pixels = vec![0u8; row_bytes * height];
    for (row, filtered) in filtered.chunks_exact(row_bytes + 1).enumerate() {
        for (column, &byte) in filtered[1..].iter().enumerate() {
            let index = row * row_bytes + column;
            let left = if column >= 3 { pixels[index - 3] } else { 0 };
            let up = if row > 0 {
                pixels[index - row_bytes]
            } else {
                0
            };
            let corner = if row > 0 && column >= 3 {
                pixels[index - row_bytes - 3]
            } else {
                0
            };
            let prediction = match filtered[0] {
                0 => 0,
                1 => left,
                2 => up,
                3 => ((u16::from(left) + u16::from(up)) / 2) as u8,
                4 => {
                    let p = i32::from(left) + i32::from(up) - i32::from(corner);
                    let a = (p - i32::from(left)).abs();
                    let b = (p - i32::from(up)).abs();
                    let c = (p - i32::from(corner)).abs();
                    if a <= b && a <= c {
                        left
                    } else if b <= c {
                        up
                    } else {
                        corner
                    }
                }
                filter => panic!("invalid PNG predictor: {filter}"),
            };
            pixels[index] = byte.wrapping_add(prediction);
        }
    }
    pixels
}

#[test]
fn compiled_pdf_preserves_large_compressed_attachments_images_and_transparency() {
    util::set_test_root();
    let mut format = SingleInputFileIo::new(&util::ensure_plain_format().unwrap());
    let mut assets = FilesystemIo::new(&util::test_path(&["assets"]), false, false, HashSet::new());
    let mut memory = MemoryIo::new(true);
    let payload: Vec<_> = (0..2 * 1024 * 1024 + 71).map(|i| (i % 251) as u8).collect();
    memory.create_entry("payload.bin", payload.clone());
    let (width, height) = (512, 1024);
    memory.create_entry("large.bmp", bmp_image(width, height));
    let (alpha_width, alpha_height) = (257, 1025);
    memory.create_entry("alpha.png", png_alpha_image(alpha_width, alpha_height));
    let mut primary = BufferedPrimaryIo::from_text(
        r"\nopagenumbers
\special{pdf:fstream @payload (payload.bin) << /TectonicParallelTest true >>}
\special{pdf:put @catalog << /Payload @payload >>}
\XeTeXpicfile large.bmp width 10pt
\XeTeXpicfile alpha.png width 10pt
\bye
",
    );

    {
        let io = IoStack::new(vec![&mut memory, &mut primary, &mut format, &mut assets]);
        let mut hooks = MinimalDriver::new(io);
        let mut status = NoopStatusBackend {};
        let mut launcher = CoreBridgeLauncher::new(&mut hooks, &mut status);
        TexEngine::default()
            .process(&mut launcher, "plain.fmt", "parallel.tex")
            .unwrap();
        XdvipdfmxEngine::default()
            .enable_deterministic_tags(true)
            .process(&mut launcher, "parallel.xdv", "parallel.pdf")
            .unwrap();
    }

    let files = memory.files.borrow();
    let pdf = &files["parallel.pdf"].data;
    assert!(pdf.starts_with(b"%PDF-"));
    assert!(pdf.ends_with(b"%%EOF\n"));
    let (dictionary, stream) = stream_data(pdf, b"/TectonicParallelTest true");
    assert!(dictionary.contains("/FlateDecode"));
    let mut decoded = Vec::new();
    ZlibDecoder::new(stream).read_to_end(&mut decoded).unwrap();
    assert_eq!(decoded, payload);

    let (dictionary, stream) = stream_data(pdf, b"/Subtype/Image");
    assert!(
        dictionary
            .split("/Predictor")
            .nth(1)
            .unwrap()
            .trim_start()
            .starts_with("15")
    );
    decoded.clear();
    ZlibDecoder::new(stream).read_to_end(&mut decoded).unwrap();
    let pixels = decode_png_predictor(&decoded, width, height);
    for (index, pixel) in pixels.as_chunks::<3>().0.iter().enumerate() {
        let (x, y) = (index % width, index / width);
        assert_eq!(
            pixel,
            &[(y % 251) as u8, (x % 251) as u8, ((x + y) % 251) as u8]
        );
    }

    let (dictionary, stream) = stream_data(pdf, b"/Predictor 2");
    assert!(dictionary.contains("/BitsPerComponent 16"));
    assert!(dictionary.contains("/ColorSpace/DeviceGray"));
    decoded.clear();
    ZlibDecoder::new(stream).read_to_end(&mut decoded).unwrap();
    assert_eq!(decoded.len(), alpha_width * alpha_height * 2);
    for (y, row) in decoded.chunks_exact(alpha_width * 2).enumerate() {
        let mut previous = 0u16;
        for (x, sample) in row.as_chunks::<2>().0.iter().enumerate() {
            previous = previous.wrapping_add(u16::from_be_bytes(*sample));
            assert_eq!(previous, (x * 4093 + y * 8191) as u16);
        }
    }
}
