use flate2::{Compression, Crc, read::ZlibDecoder, write::ZlibEncoder};
use std::{
    collections::HashSet,
    io::{Read, Write},
    time::{Duration, Instant},
};
use tectonic::{
    TexEngine, XdvipdfmxEngine,
    io::{FilesystemIo, IoStack, MemoryIo, testing::SingleInputFileIo},
};
use tectonic_bridge_core::{CoreBridgeLauncher, MinimalDriver};
use tectonic_io_base::stdstreams::BufferedPrimaryIo;
use tectonic_status_base::NoopStatusBackend;

mod util;

fn compile_pdf(
    memory: &mut MemoryIo,
    source: &str,
    compression: bool,
) -> (Vec<u8>, Duration, Duration) {
    let (pdf, tex, conversion, _) =
        compile_pdf_with_backend(memory, source, compression, None, false, 64 * 1024 * 1024);
    (pdf, tex, conversion)
}

fn compile_pdf_with_backend(
    memory: &mut MemoryIo,
    source: &str,
    compression: bool,
    workers: Option<usize>,
    legacy: bool,
    queued_bytes: usize,
) -> (
    Vec<u8>,
    Duration,
    Duration,
    Option<tectonic::PdfBuildStatistics>,
) {
    util::set_test_root();
    let mut format = SingleInputFileIo::new(&util::ensure_plain_format().unwrap());
    let mut assets = FilesystemIo::new(&util::test_path(&["assets"]), false, false, HashSet::new());
    let mut primary = BufferedPrimaryIo::from_text(source);
    let (tex_time, pdf_time, statistics) = {
        let io = IoStack::new(vec![memory, &mut primary, &mut format, &mut assets]);
        let mut hooks = MinimalDriver::new(io);
        let mut status = NoopStatusBackend {};
        let mut launcher = CoreBridgeLauncher::new(&mut hooks, &mut status);
        let start = Instant::now();
        TexEngine::default()
            .process(&mut launcher, "plain.fmt", "batch.tex")
            .unwrap();
        let tex_time = start.elapsed();
        let start = Instant::now();
        let mut engine = XdvipdfmxEngine::default();
        engine
            .legacy_backend(legacy)
            .pdf_build_options(tectonic::PdfBuildOptions {
                workers: workers.and_then(std::num::NonZeroUsize::new),
                queued_bytes: std::num::NonZeroUsize::new(queued_bytes).unwrap(),
            })
            .enable_compression(compression)
            .enable_deterministic_tags(true)
            .process(&mut launcher, "batch.xdv", "batch.pdf")
            .unwrap();
        (tex_time, start.elapsed(), engine.statistics().cloned())
    };
    let pdf = memory.files.borrow()["batch.pdf"].data.clone();
    (pdf, tex_time, pdf_time, statistics)
}

fn decoded_streams(pdf: &[u8]) -> Vec<Vec<u8>> {
    let mut streams = Vec::new();
    let mut cursor = 0;
    while let Some(start) = pdf[cursor..]
        .windows(8)
        .position(|bytes| bytes == b"\nstream\n")
    {
        let start = cursor + start;
        let object_start = pdf[..start]
            .windows(5)
            .rposition(|bytes| bytes == b" obj\n")
            .unwrap()
            + 5;
        let (dictionary, data) = read_stream(&pdf[object_start..]);
        cursor = start + 8 + data.len();
        if dictionary.contains("/Type/ObjStm") || dictionary.contains("/Type/XRef") {
            continue;
        }
        let mut decoded = Vec::new();
        if dictionary.contains("/FlateDecode") {
            ZlibDecoder::new(data).read_to_end(&mut decoded).unwrap();
        } else {
            decoded.extend_from_slice(data);
        }
        streams.push(decoded);
    }
    streams.sort();
    streams
}

fn page_document() -> String {
    let mut source =
        String::from("\\nopagenumbers\\font\\native=\"[lmroman12-regular]\" at 12pt\n");
    for page in 0..24 {
        source.push_str(&format!("Page {page}. {{\\native Native office ffi AVATAR}} \\vrule width 8pt height 12pt\\par\n"));
        source.push_str(
            &"Text with (parentheses), \\char92 and relative positioning.\\par\n".repeat(32),
        );
        source.push_str("\\vfill\\eject\n");
    }
    source.push_str("\\bye\n");
    source
}

#[test]
fn buffered_text_input_preserves_encodings_and_line_endings_across_refills() {
    let source = "\\nopagenumbers\\XeTeXgenerateactualtext=1\\font\\native=\"[lmroman12-regular]\"\\native\\input encoded.tex\\bye";
    let text = "\u{1d11e}\u{20ac} office\\par\nMore text.\\par\n";
    let mut memory = MemoryIo::new(true);
    memory.create_entry("encoded.tex", text.as_bytes().to_vec());
    let (expected, _, _) = compile_pdf(&mut memory, source, true);
    let expected = decoded_streams(&expected);
    for actual_text in [
        b"/ActualText (\xfe\xff\xd8\x34\xdd\\036 \xac)".as_slice(),
        b"/ActualText (office)",
    ] {
        assert!(expected.iter().any(|stream| {
            stream
                .windows(actual_text.len())
                .any(|bytes| bytes == actual_text)
        }));
    }

    let padded = format!("%{}\n{text}", " ".repeat(8189));
    let mut utf8_bom = vec![0xef, 0xbb, 0xbf];
    utf8_bom.extend_from_slice(padded.as_bytes());
    let crlf = format!("%{}\r\n{text}", " ".repeat(8190));
    let cr = format!("%{}\r{text}", " ".repeat(8190));
    let mut inputs = vec![
        padded.into_bytes(),
        utf8_bom,
        crlf.into_bytes(),
        cr.into_bytes(),
    ];
    let utf16 = format!("%{}\n{text}", " ".repeat(4093));
    for little_endian in [false, true] {
        let mut bytes = if little_endian {
            vec![0xff, 0xfe]
        } else {
            vec![0xfe, 0xff]
        };
        for unit in utf16.encode_utf16() {
            bytes.extend_from_slice(&if little_endian {
                unit.to_le_bytes()
            } else {
                unit.to_be_bytes()
            });
        }
        inputs.push(bytes);
    }
    for (index, bytes) in inputs.into_iter().enumerate() {
        memory.create_entry("encoded.tex", bytes);
        let (pdf, _, _) = compile_pdf(&mut memory, source, true);
        assert_eq!(decoded_streams(&pdf), expected, "encoding variant {index}");
    }

    memory.create_entry("encoded.tex", "\u{fffd}A office\\par\n".as_bytes().to_vec());
    let (expected, _, _) = compile_pdf(&mut memory, source, true);
    let mut malformed = format!("%{}\n", " ".repeat(8189)).into_bytes();
    malformed.extend_from_slice(b"\xe2A office\\par\n");
    memory.create_entry("encoded.tex", malformed);
    let (pdf, _, _) = compile_pdf(&mut memory, source, true);
    assert_eq!(decoded_streams(&pdf), decoded_streams(&expected));

    memory.create_entry("encoded.tex", "\u{e9} office\\par\n".as_bytes().to_vec());
    let (expected, _, _) = compile_pdf(&mut memory, source, true);
    let mut switched = b"\\XeTeXinputencoding \"ISO-8859-1\"\n".to_vec();
    switched.extend_from_slice(b"\xe9 office\\par\n");
    memory.create_entry("encoded.tex", switched);
    let (pdf, _, _) = compile_pdf(&mut memory, source, true);
    assert_eq!(decoded_streams(&pdf), decoded_streams(&expected));
}

#[test]
fn buffered_font_maps_preserve_duplicate_replace_remove_and_reload_behavior() {
    let valid = "cmr10 CMR10 <cmr10.pfb";
    let invalid = "cmr10 MissingFont <missing-font.pfb";
    let text = "\\nopagenumbers Font map test.\\bye";
    let mut memory = MemoryIo::new(true);
    let (expected, _, _) = compile_pdf(&mut memory, text, true);
    let expected = decoded_streams(&expected);
    for map in [
        format!("%{}\r\n{valid}\r{invalid}", " ".repeat(8190)),
        format!("%{}\r{valid}\n{invalid}", " ".repeat(8190)),
        valid.to_owned(),
    ] {
        memory.create_entry("pdftex.map", map.into_bytes());
        let (pdf, _, _) = compile_pdf(&mut memory, text, true);
        assert_eq!(decoded_streams(&pdf), expected);
    }
    memory.create_entry("pdftex.map", invalid.as_bytes().to_vec());
    memory.create_entry("valid.map", valid.as_bytes().to_vec());
    memory.create_entry("invalid.map", invalid.as_bytes().to_vec());
    for specials in [
        "\\special{pdf:mapfile valid.map}",
        "\\special{pdf:mapfile -invalid.map}\\special{pdf:mapfile +valid.map}",
        "\\special{pdf:mapfile valid.map}\\special{pdf:mapfile +invalid.map}",
    ] {
        let source = format!("\\nopagenumbers{specials}Font map test.\\bye");
        let (pdf, _, _) = compile_pdf(&mut memory, &source, true);
        assert_eq!(decoded_streams(&pdf), expected, "{specials}");
    }
    memory.create_entry("pdftex.map", valid.as_bytes().to_vec());
    let (pdf, _, _) = compile_pdf(&mut memory, text, true);
    assert_eq!(decoded_streams(&pdf), expected);
}

#[test]
fn rust_font_map_loading_matches_mapline_options_and_legacy_syntax() {
    let mut memory = MemoryIo::new(true);
    for record in [
        "cmr10 CMR10 <cmr10.pfb",
        "cmr10 CMR10 \".2 SlantFont 1.1 ExtendFont\" <cmr10.pfb",
        "cmr10 <cmr10.pfb",
        "\"cmr10\" \"CMR10\" <<\"cmr10.pfb\"",
        "cmr10 default cmr10.pfb -s .2 -e 1.1",
    ] {
        memory.create_entry("pdftex.map", Vec::new());
        let source = format!("\\nopagenumbers\\special{{pdf:mapline {record}}}Font map test.\\bye");
        let (expected, _, _) = compile_pdf(&mut memory, &source, true);
        memory.create_entry("pdftex.map", record.as_bytes().to_vec());
        let (pdf, _, _) = compile_pdf(&mut memory, "\\nopagenumbers Font map test.\\bye", true);
        assert_eq!(
            decoded_streams(&pdf),
            decoded_streams(&expected),
            "{record}"
        );
    }
    memory.create_entry("pdftex.map", b"cmr10 CMR10 <\"cmr10.pf\\142\"".to_vec());
    let (pdf, _, _) = compile_pdf(&mut memory, "\\nopagenumbers Font map test.\\bye", true);
    memory.create_entry("pdftex.map", b"cmr10 CMR10 <cmr10.pfb".to_vec());
    let (expected, _, _) = compile_pdf(&mut memory, "\\nopagenumbers Font map test.\\bye", true);
    assert_eq!(decoded_streams(&pdf), decoded_streams(&expected));
}

#[test]
fn page_workers_preserve_content_and_deterministic_output_under_backpressure() {
    let source = page_document();
    for compression in [false, true] {
        let mut memory = MemoryIo::new(true);
        let (legacy, _, _, _) =
            compile_pdf_with_backend(&mut memory, &source, compression, Some(1), true, 1);
        let expected = decoded_streams(&legacy);
        let mut deterministic = None;
        for (workers, queued_bytes) in
            [(1, 1), (3, 1), (3, 64 * 1024 * 1024), (1, 64 * 1024 * 1024)]
        {
            let (pdf, _, _, statistics) = compile_pdf_with_backend(
                &mut memory,
                &source,
                compression,
                Some(workers),
                false,
                queued_bytes,
            );
            let statistics = statistics.unwrap();
            assert_eq!(statistics.rendered_pages, 24);
            assert!(!statistics.serial_page_fallback);
            check_cross_references(&pdf);
            assert_eq!(
                decoded_streams(&pdf),
                expected,
                "workers={workers}, compression={compression}"
            );
            if let Some(expected) = &deterministic {
                assert!(
                    pdf == *expected,
                    "worker or queue limits changed deterministic PDF bytes"
                );
            } else {
                deterministic = Some(pdf);
            }
        }
    }
}

#[test]
fn page_workers_preserve_images_colors_backgrounds_links_and_destinations() {
    let mut memory = MemoryIo::new(true);
    memory.create_entry("small.bmp", bmp_image(64, 64));
    let mut source = String::from("\\nopagenumbers\n");
    for page in 0..8 {
        source.push_str(&format!("\\special{{background rgb 0.9 0.8 0.7}}\\special{{color push rgb 0 0 1}}\\special{{pdf:dest (page{page}) [ @thispage /Fit ]}}\n"));
        source.push_str(
            "Colored text.\\special{color pop}\\par\\special{pdf:image width 20pt (small.bmp)}\n",
        );
        source.push_str("\\special{pdf:bann << /Type /Annot /Subtype /Link /A << /S /URI /URI (https://example.com) >> >>}Link text\\special{pdf:eann}\\vfill\\eject\n");
    }
    source.push_str("\\bye\n");
    let (legacy, _, _, _) =
        compile_pdf_with_backend(&mut memory, &source, true, Some(1), true, 1024);
    let (parallel, _, _, statistics) =
        compile_pdf_with_backend(&mut memory, &source, true, Some(3), false, 1024);
    let statistics = statistics.unwrap();
    assert_eq!(statistics.rendered_pages, 8);
    assert!(!statistics.serial_page_fallback);
    check_cross_references(&parallel);
    assert_eq!(decoded_streams(&parallel), decoded_streams(&legacy));
    if let Some(root) = std::env::var_os("TECTONIC_PDF_BENCH_DIR") {
        std::fs::create_dir_all(&root).unwrap();
        let root = std::path::Path::new(&root);
        std::fs::write(root.join("features-legacy.pdf"), legacy).unwrap();
        std::fs::write(root.join("features-auto.pdf"), parallel).unwrap();
        let (one, _, _, _) =
            compile_pdf_with_backend(&mut memory, &source, true, Some(1), false, 1024);
        std::fs::write(root.join("features-one.pdf"), one).unwrap();
    }
}

#[test]
fn unknown_specials_use_legacy_page_rendering_without_replaying_side_effects() {
    let source = "\\nopagenumbers\\special{pdf:literal direct /FallbackTest BMC}First\\special{pdf:literal direct EMC}\\eject Second\\bye";
    let mut memory = MemoryIo::new(true);
    let (legacy, _, _, _) =
        compile_pdf_with_backend(&mut memory, source, true, Some(1), true, 1024);
    let (parallel, _, _, statistics) =
        compile_pdf_with_backend(&mut memory, source, true, Some(3), false, 1024);
    let statistics = statistics.unwrap();
    assert_eq!(statistics.rendered_pages, 0);
    assert!(statistics.serial_page_fallback);
    assert_eq!(decoded_streams(&parallel), decoded_streams(&legacy));
    check_cross_references(&parallel);
}

#[test]
fn native_abort_with_queued_pages_allows_another_pdf_build() {
    util::set_test_root();
    let mut memory = MemoryIo::new(true);
    let mut format = SingleInputFileIo::new(&util::ensure_plain_format().unwrap());
    let mut assets = FilesystemIo::new(&util::test_path(&["assets"]), false, false, HashSet::new());
    let mut primary = BufferedPrimaryIo::from_text(
        "\\nopagenumbers First\\eject Second\\eject {\\bf Missing font outline}\\bye",
    );
    {
        let io = IoStack::new(vec![&mut memory, &mut primary, &mut format, &mut assets]);
        let mut hooks = MinimalDriver::new(io);
        let mut status = NoopStatusBackend {};
        let mut launcher = CoreBridgeLauncher::new(&mut hooks, &mut status);
        TexEngine::default()
            .process(&mut launcher, "plain.fmt", "batch.tex")
            .unwrap();
        let mut engine = XdvipdfmxEngine::default();
        engine.pdf_build_options(tectonic::PdfBuildOptions {
            workers: std::num::NonZeroUsize::new(3),
            queued_bytes: std::num::NonZeroUsize::new(1024 * 1024).unwrap(),
        });
        let error = engine
            .process(&mut launcher, "batch.xdv", "batch.pdf")
            .unwrap_err();
        assert!(error.to_string().contains("physical"), "{error}");
        assert_eq!(engine.statistics().unwrap().rendered_pages, 2);
    }
    let (pdf, _, _) = compile_pdf(&mut memory, "Successful subsequent build.\\bye", true);
    check_cross_references(&pdf);
}

#[test]
fn malformed_dvi_preflight_keeps_native_diagnostics_and_allows_another_build() {
    let mut memory = MemoryIo::new(true);
    compile_pdf(&mut memory, "Original document.\\bye", true);
    let mut xdv = memory.files.borrow()["batch.xdv"].data.clone();
    let page_start = 15 + usize::from(xdv[14]);
    assert_eq!(xdv[page_start], 139);
    xdv[page_start + 45] = 247;
    memory.create_entry("batch.xdv", xdv);
    let mut assets = FilesystemIo::new(&util::test_path(&["assets"]), false, false, HashSet::new());
    for legacy in [false, true] {
        let io = IoStack::new(vec![&mut memory, &mut assets]);
        let mut hooks = MinimalDriver::new(io);
        let mut status = NoopStatusBackend {};
        let mut launcher = CoreBridgeLauncher::new(&mut hooks, &mut status);
        let error = XdvipdfmxEngine::default()
            .legacy_backend(legacy)
            .process(&mut launcher, "batch.xdv", "batch.pdf")
            .unwrap_err();
        assert!(
            error.to_string().contains("Unexpected opcode 247"),
            "{error}"
        );
    }
    let (pdf, _, _) = compile_pdf(&mut memory, "Successful subsequent build.\\bye", true);
    check_cross_references(&pdf);
}

#[test]
fn resource_workers_preserve_metadata_and_large_stream_bytes_across_limits() {
    let mut memory = MemoryIo::new(true);
    let source = attachment_document(
        &mut memory,
        &[16 * 1024, 2 * 1024 * 1024 + 17, 600 * 1024, 32 * 1024],
    );
    let (legacy, _, _, _) = compile_pdf_with_backend(&mut memory, &source, true, Some(1), true, 1);
    let expected = decoded_streams(&legacy);
    let mut deterministic = None;
    for (workers, queued_bytes) in [(1, 1), (3, 1), (3, 64 * 1024 * 1024)] {
        let (pdf, _, _, statistics) = compile_pdf_with_backend(
            &mut memory,
            &source,
            true,
            Some(workers),
            false,
            queued_bytes,
        );
        let statistics = statistics.unwrap();
        assert!(statistics.serial_page_fallback);
        assert!(statistics.compressed_streams >= 3);
        check_cross_references(&pdf);
        assert_eq!(decoded_streams(&pdf), expected);
        if let Some(expected) = &deterministic {
            assert!(pdf == *expected);
        } else {
            deterministic = Some(pdf);
        }
    }
}

fn attachment_document(memory: &mut MemoryIo, lengths: &[usize]) -> String {
    let mut source = String::from("\\nopagenumbers\nBatch compression test.\n");
    for (index, &length) in lengths.iter().enumerate() {
        let data: Vec<_> = (0..length)
            .map(|offset| ((offset * 31 + index) % 251) as u8)
            .collect();
        memory.create_entry(&format!("payload{index}.bin"), data);
        let metadata = if index == 0 { " /Type /Metadata" } else { "" };
        source.push_str(&format!(
            "\\special{{pdf:fstream @payload{index} (payload{index}.bin) << /TectonicBatchTest {index}{metadata} >>}}\n\\special{{pdf:put @catalog << /Payload{index} @payload{index} >>}}\n"
        ));
    }
    source.push_str("\\bye\n");
    source
}

#[test]
fn batched_streams_preserve_data_across_batch_limits_and_engine_runs() {
    let lengths: Vec<_> = (0..140)
        .map(|index| match index {
            0..64 => 16 * 1024 + index,
            64..82 => 600 * 1024 + index,
            _ => 32 * 1024 + index,
        })
        .collect();
    for compression in [true, false, true] {
        let mut memory = MemoryIo::new(true);
        let source = attachment_document(&mut memory, &lengths);
        let (pdf, _, _) = compile_pdf(&mut memory, &source, compression);
        check_cross_references(&pdf);
        for (index, &length) in lengths.iter().enumerate() {
            let marker = format!("/TectonicBatchTest {index}/");
            let (dictionary, stream) = stream_data(&pdf, marker.as_bytes());
            let data = if compression && index != 0 {
                assert!(dictionary.contains("/FlateDecode"));
                let mut decoded = Vec::new();
                ZlibDecoder::new(stream).read_to_end(&mut decoded).unwrap();
                decoded
            } else {
                assert!(!dictionary.contains("/FlateDecode"));
                stream.to_vec()
            };
            assert_eq!(data.len(), length);
            assert!(
                data.iter()
                    .enumerate()
                    .all(|(offset, &byte)| byte == ((offset * 31 + index) % 251) as u8)
            );
        }
    }
}

fn check_cross_references(pdf: &[u8]) {
    let startxref = pdf
        .windows(10)
        .rposition(|bytes| bytes == b"startxref\n")
        .unwrap()
        + 10;
    let offset = std::str::from_utf8(&pdf[startxref..])
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let xref = &pdf[offset..];
    let header_end = xref
        .windows(5)
        .position(|bytes| bytes == b" obj\n")
        .unwrap();
    assert!(
        xref[..header_end]
            .iter()
            .all(|byte| byte.is_ascii_digit() || *byte == b' ')
    );
    let (dictionary, stream) = stream_data(xref, b"/Type/XRef");
    let widths: Vec<usize> = dictionary
        .split("/W[")
        .nth(1)
        .unwrap()
        .split(']')
        .next()
        .unwrap()
        .split_whitespace()
        .map(|width| width.parse().unwrap())
        .collect();
    assert_eq!(widths.len(), 3);
    let entries = if dictionary.contains("/FlateDecode") {
        let mut decoded = Vec::new();
        ZlibDecoder::new(stream).read_to_end(&mut decoded).unwrap();
        decoded
    } else {
        stream.to_vec()
    };
    let row_bytes: usize = widths.iter().sum();
    assert_eq!(entries.len() % row_bytes, 0);
    let integer = |bytes: &[u8]| {
        bytes
            .iter()
            .fold(0usize, |value, &byte| value * 256 + usize::from(byte))
    };
    for (label, entry) in entries.chunks_exact(row_bytes).enumerate() {
        if integer(&entry[..widths[0]]) == 1 {
            let offset = integer(&entry[widths[0]..widths[0] + widths[1]]);
            let generation = integer(&entry[widths[0] + widths[1]..]);
            assert!(pdf[offset..].starts_with(format!("{label} {generation} obj\n").as_bytes()));
        }
    }
}

#[test]
fn batched_page_streams_preserve_graphics_and_cross_references() {
    let commands = "0 0 1 1 re S ".repeat(128);
    let mut source = String::from("\\nopagenumbers\n");
    for page in 0..70 {
        source.push_str(&format!("Page {page}.\\special{{pdf:literal direct /PageTest{page} BMC {commands}EMC}}\\vfill\\eject\n"));
    }
    source.push_str("\\bye\n");
    for compression in [true, false] {
        let mut memory = MemoryIo::new(true);
        let (pdf, _, _) = compile_pdf(&mut memory, &source, compression);
        check_cross_references(&pdf);
        let mut found = HashSet::new();
        let mut cursor = 0;
        while let Some(start) = pdf[cursor..]
            .windows(8)
            .position(|bytes| bytes == b"\nstream\n")
        {
            let start = cursor + start;
            let object_start = pdf[..start]
                .windows(5)
                .rposition(|bytes| bytes == b" obj\n")
                .unwrap()
                + 5;
            let (dictionary, stream) = read_stream(&pdf[object_start..]);
            cursor = start + 8 + stream.len();
            let data = if dictionary.contains("/FlateDecode") {
                let mut decoded = Vec::new();
                ZlibDecoder::new(stream).read_to_end(&mut decoded).unwrap();
                decoded
            } else {
                stream.to_vec()
            };
            if let Some(start) = data.windows(9).position(|bytes| bytes == b"/PageTest") {
                let content = std::str::from_utf8(&data[start..]).unwrap();
                let page: usize = content[9..]
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                assert!(content.starts_with(&format!("/PageTest{page} BMC {commands}EMC")));
                assert!((1024..16 * 1024).contains(&data.len()));
                assert_eq!(dictionary.contains("/FlateDecode"), compression);
                assert!(found.insert(page));
            }
        }
        assert_eq!(found, (0..70).collect());
    }
}

#[test]
#[ignore = "manual full compilation benchmark"]
fn benchmark_single_pdf_compilation() {
    for workload in ["text", "native", "graphics", "images", "attachments"] {
        if std::env::var("TECTONIC_PDF_BENCH_WORKLOAD").is_ok_and(|selected| selected != workload) {
            continue;
        }
        let mut memory = MemoryIo::new(true);
        let source = if workload == "attachments" {
            attachment_document(&mut memory, &vec![128 * 1024; 128])
        } else if workload == "native" {
            format!(
                "\\font\\native=\"[lmroman12-regular]\"\\native\n{}",
                page_document()
            )
        } else if workload == "images" {
            let mut source = String::from("\\nopagenumbers\n");
            for image in 0..16 {
                let name = format!("benchmark{image}.bmp");
                memory.create_entry(&name, bmp_image(1024, 1024));
                source.push_str(&format!(
                    "Image {image}.\\special{{pdf:image width 200pt ({name})}}\\vfill\\eject\n"
                ));
            }
            source.push_str("\\bye\n");
            source
        } else if workload == "graphics" {
            format!(
                "\\nopagenumbers\n{}\\bye\n",
                format!(
                    "{}\\vfill\\eject\n",
                    "\\vrule width 8pt height 12pt ".repeat(500)
                )
                .repeat(100)
            )
        } else {
            let paragraph = "The quick brown fox jumps over the lazy dog. This paragraph exercises line breaking, font output, and page layout in a single document.\\par\n";
            let page = format!("{}\\vfill\\eject\n", paragraph.repeat(20));
            format!("\\nopagenumbers\n{}\\bye\n", page.repeat(200))
        };
        for (label, workers, legacy) in [
            ("legacy", None, true),
            ("one", Some(1), false),
            ("auto", None, false),
        ] {
            if std::env::var("TECTONIC_PDF_BENCH_BACKEND").is_ok_and(|selected| selected != label) {
                continue;
            }
            let mut tex_times = Vec::new();
            let mut pdf_times = Vec::new();
            for iteration in 0..6 {
                let (bytes, tex, pdf, statistics) = compile_pdf_with_backend(
                    &mut memory,
                    &source,
                    true,
                    workers,
                    legacy,
                    64 * 1024 * 1024,
                );
                if iteration == 5 {
                    if let Some(statistics) = statistics {
                        eprintln!("{statistics:?}");
                    }
                    if let Some(root) = std::env::var_os("TECTONIC_PDF_BENCH_DIR") {
                        std::fs::create_dir_all(&root).unwrap();
                        std::fs::write(
                            std::path::Path::new(&root).join(format!("{workload}-{label}.pdf")),
                            bytes,
                        )
                        .unwrap();
                    }
                }
                if iteration > 0 {
                    tex_times.push(tex);
                    pdf_times.push(pdf);
                }
            }
            tex_times.sort();
            pdf_times.sort();
            eprintln!(
                "workload={workload}, backend={label}: TeX={:?}, PDF={:?}, total={:?}",
                tex_times[2],
                pdf_times[2],
                tex_times[2] + pdf_times[2]
            );
        }
    }
}

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
    read_stream(stream)
}

fn read_stream(stream: &[u8]) -> (&str, &[u8]) {
    let body_start = stream
        .windows(8)
        .position(|bytes| bytes == b"\nstream\n")
        .unwrap()
        + 8;
    let dictionary = std::str::from_utf8(&stream[..body_start]).unwrap();
    let length = dictionary
        .split("/Length")
        .skip(1)
        .find(|suffix| suffix.starts_with(|c: char| c.is_ascii_whitespace()))
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
