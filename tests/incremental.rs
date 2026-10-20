use std::{
    fmt::Arguments,
    io::Read,
    path::Path,
    time::{Duration, SystemTime},
};
use tectonic::{driver::ProcessingSessionBuilder, io::memory::MemoryFileCollection, test_util::TestBundle};
use tectonic_errors::Error;
use tectonic_status_base::{MessageKind, StatusBackend};

mod util;

#[derive(Default)]
struct Status(Vec<String>);

impl StatusBackend for Status {
    fn dump_error_logs(&mut self, output: &[u8]) {
        self.0.push(String::from_utf8_lossy(output).into_owned());
    }
    fn report(&mut self, _: MessageKind, args: Arguments, error: Option<&Error>) {
        self.0.push(format!("{args} {}", error.map_or_else(String::new, ToString::to_string)));
    }
}

fn compile(root: &Path, incremental: bool, synctex: bool) -> (MemoryFileCollection, Vec<String>) {
    let mut builder = ProcessingSessionBuilder::default();
    builder
        .primary_input_path(root.join("main.tex"))
        .tex_input_name("main.tex")
        .format_name("plain")
        .format_cache_path(util::test_path(&[]))
        .do_not_write_output_files()
        .keep_intermediates(true)
        .bundle(Box::<TestBundle>::default())
        .build_date(SystemTime::UNIX_EPOCH + Duration::from_secs(1_600_000_000))
        .synctex(synctex);
    if incremental {
        builder.incremental_cache_path(root.join("cache"));
    }
    let mut status = Status::default();
    let mut session = builder.create(&mut status).unwrap();
    session
        .run(&mut status)
        .unwrap_or_else(|error| panic!("{error:?}\n{}", status.0.join("\n")));
    (session.into_file_data(), status.0)
}

fn assert_outputs_match(actual: &MemoryFileCollection, expected: &MemoryFileCollection) {
    assert_eq!(actual["main.pdf"].data, expected["main.pdf"].data);
    if let Some(actual) = actual.get("main.synctex.gz") {
        let mut a = Vec::new();
        let mut b = Vec::new();
        flate2::read::GzDecoder::new(actual.data.as_slice()).read_to_end(&mut a).unwrap();
        flate2::read::GzDecoder::new(expected["main.synctex.gz"].data.as_slice())
            .read_to_end(&mut b)
            .unwrap();
        assert_eq!(a, b);
    }
}

#[test]
fn restores_a_single_file_after_editing_its_suffix() {
    util::set_test_root();
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let prefix = format!(
        "\\XeTeXprotrudechars=2\\lpcode\\tenrm`E=500\\rpcode\\tenrm`.=500\n\\newcount\\iteration\n\\loop\\advance\\iteration by1\\ifnum\\iteration<180000\\repeat\nPrefix.\\par\n{}",
        "% Padding keeps the edited suffix outside input lookahead.\n".repeat(300)
    );
    std::fs::write(root.join("main.tex"), format!("{prefix}Original suffix.\\par\\bye\n")).unwrap();
    compile(root, true, true);
    std::fs::write(root.join("main.tex"), format!("{prefix}Edited suffix.\\par\\bye\n")).unwrap();
    let (actual, status) = compile(root, true, true);
    assert!(
        status.iter().any(|message| message.contains("resumed TeX execution")),
        "{}",
        status.join("\n")
    );
    let (clean, _) = compile(root, false, true);
    assert_outputs_match(&actual, &clean);
}

#[test]
fn restores_tex_prefix_after_editing_a_later_input() {
    util::set_test_root();
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    std::fs::write(root.join("main.tex"), b"\\input prefix.tex\n\\input suffix.tex\n\\bye\n").unwrap();
    std::fs::write(
        root.join("prefix.tex"),
        format!("\\def\\saved{{Restored}}\n{}\\par\n", "A long prefix with text. ".repeat(150)),
    )
    .unwrap();
    std::fs::write(root.join("suffix.tex"), b"\\saved\nOriginal suffix.\\par\n").unwrap();
    compile(root, true, false);
    std::fs::write(root.join("suffix.tex"), b"\\saved\nEdited suffix.\\par\n").unwrap();
    let (resumed, status) = compile(root, true, false);
    assert!(
        status.iter().any(|message| message.contains("resumed TeX execution")),
        "{}",
        status.join("\n")
    );
    let (clean, _) = compile(root, false, false);
    assert_outputs_match(&resumed, &clean);
}

#[test]
fn restores_native_fonts_pending_glyphs_and_synctex() {
    util::set_test_root();
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let font = util::test_path(&["assets", "lmroman12-regular.otf"]);
    std::fs::write(
        root.join("main.tex"),
        format!(
            "\\font\\native=\"[{}]\" at 12pt\n\\native\n\\input prefix.tex\n\\input suffix.tex\n\\bye\n",
            font.display()
        ),
    )
    .unwrap();
    std::fs::write(root.join("prefix.tex"), "Unicode text: caf\u{e9}.\\par\n").unwrap();
    std::fs::write(root.join("suffix.tex"), b"Original suffix.\\par\n").unwrap();
    compile(root, true, true);
    std::fs::write(root.join("suffix.tex"), b"Edited native-font suffix.\\par\n").unwrap();
    let (resumed, status) = compile(root, true, true);
    assert!(
        status.iter().any(|message| message.contains("resumed TeX execution")),
        "{}",
        status.join("\n")
    );
    let (clean, _) = compile(root, false, true);
    assert_outputs_match(&resumed, &clean);
}

#[test]
fn resumes_across_shipped_pages_and_repeated_edits() {
    util::set_test_root();
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let font = util::test_path(&["assets", "lmroman12-regular.otf"]);
    std::fs::write(
        root.join("main.tex"),
        format!(
            "\\font\\native=\"[{}]\" at 12pt\n\\native\n\\input prefix.tex\n\\input suffix.tex\n\\bye\n",
            font.display()
        ),
    )
    .unwrap();
    std::fs::write(
        root.join("prefix.tex"),
        b"First shipped page.\\par\\vfill\\eject\nPending second page.\\par\n",
    )
    .unwrap();
    std::fs::write(root.join("suffix.tex"), b"Original.\\par\n").unwrap();
    compile(root, true, true);
    for suffix in ["Replacement.\\par\n", "Inserted page.\\par\\vfill\\eject\nFinal page.\\par\n", ""] {
        std::fs::write(root.join("suffix.tex"), suffix).unwrap();
        let (resumed, status) = compile(root, true, true);
        assert!(
            status.iter().any(|message| message.contains("resumed TeX execution")),
            "{}",
            status.join("\n")
        );
        let (clean, _) = compile(root, false, true);
        assert_outputs_match(&resumed, &clean);
    }
}

#[test]
fn invalidates_earlier_dependencies_and_recovers_from_corrupt_cache() {
    util::set_test_root();
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    std::fs::write(root.join("main.tex"), b"\\input prefix.tex\n\\input suffix.tex\n\\bye\n").unwrap();
    std::fs::write(root.join("prefix.tex"), b"Original prefix.\\par\n").unwrap();
    std::fs::write(root.join("suffix.tex"), b"Original suffix.\\par\n").unwrap();
    compile(root, true, false);
    std::fs::write(root.join("prefix.tex"), b"Changed prefix and definitions.\\def\\saved{New}\\par\n").unwrap();
    let (actual, status) = compile(root, true, false);
    assert!(
        status.iter().any(|message| message.contains("checkpoint invalidated")),
        "{}",
        status.join("\n")
    );
    assert_outputs_match(&actual, &compile(root, false, false).0);
    for directory in std::fs::read_dir(root.join("cache")).unwrap() {
        let file = directory.unwrap().path().join("checkpoints");
        if file.is_file() {
            std::fs::write(file, b"truncated cache").unwrap();
        }
    }
    let (actual, status) = compile(root, true, false);
    assert!(status.iter().any(|message| message.contains("cache discarded")), "{}", status.join("\n"));
    assert_outputs_match(&actual, &compile(root, false, false).0);
}

#[test]
fn cli_reuses_engine_state_in_a_fresh_process() {
    util::set_test_root();
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    std::fs::write(root.join("main.tex"), b"\\input prefix.tex\n\\input suffix.tex\n\\bye\n").unwrap();
    std::fs::write(
        root.join("prefix.tex"),
        format!("{}\\par\n", "Substantial unchanged prefix. ".repeat(200)),
    )
    .unwrap();
    std::fs::write(root.join("suffix.tex"), b"Original suffix.\\par\n").unwrap();
    let run = |incremental: bool| {
        let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_tectonic"));
        command
            .current_dir(root)
            .args(["-X", "compile", "--format", "plain", "--synctex"])
            .env(tectonic::test_util::TEST_ROOT_ENV_VAR, util::test_path(&[]))
            .env("SOURCE_DATE_EPOCH", "1600000000");
        if incremental {
            command.arg("--incremental");
        }
        let output = command.arg("main.tex").output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        format!("{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr))
    };
    run(true);
    std::fs::write(root.join("suffix.tex"), b"Edited in a separate compiler process.\\par\n").unwrap();
    let status = run(true);
    let skipped = status
        .split("skipped ")
        .filter_map(|text| text.split_whitespace().next()?.parse::<u64>().ok())
        .max()
        .unwrap_or(0);
    assert!(skipped > 100, "{status}");
    let resumed_pdf = std::fs::read(root.join("main.pdf")).unwrap();
    run(false);
    assert_eq!(resumed_pdf, std::fs::read(root.join("main.pdf")).unwrap());
}

#[test]
fn failed_suffix_build_preserves_the_last_successful_cache() {
    util::set_test_root();
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    std::fs::write(root.join("main.tex"), b"\\input prefix.tex\n\\input suffix.tex\n\\bye\n").unwrap();
    std::fs::write(root.join("prefix.tex"), b"Unchanged prefix.\\par\n").unwrap();
    std::fs::write(root.join("suffix.tex"), b"Successful suffix.\\par\n").unwrap();
    let run = || {
        std::process::Command::new(env!("CARGO_BIN_EXE_tectonic"))
            .current_dir(root)
            .args(["-X", "compile", "--format", "plain", "--incremental", "main.tex"])
            .env(tectonic::test_util::TEST_ROOT_ENV_VAR, util::test_path(&[]))
            .env("SOURCE_DATE_EPOCH", "1600000000")
            .output()
            .unwrap()
    };
    assert!(run().status.success());
    let cache = std::fs::read_dir(root.join(".tectonic-incremental"))
        .unwrap()
        .map(|entry| entry.unwrap().path().join("checkpoints"))
        .find(|path| path.is_file())
        .unwrap();
    let successful_cache = std::fs::read(&cache).unwrap();
    std::fs::write(root.join("suffix.tex"), b"\\undefinedcommand\n").unwrap();
    let failed = run();
    assert!(!failed.status.success());
    assert!(!String::from_utf8_lossy(&failed.stderr).contains("engine checkpoint could not be restored"));
    assert_eq!(successful_cache, std::fs::read(&cache).unwrap());
    std::fs::write(root.join("suffix.tex"), b"Repaired suffix.\\par\n").unwrap();
    let repaired = run();
    assert!(repaired.status.success(), "{}", String::from_utf8_lossy(&repaired.stderr));
    assert!(
        format!(
            "{}{}",
            String::from_utf8_lossy(&repaired.stdout),
            String::from_utf8_lossy(&repaired.stderr)
        )
        .contains("resumed TeX execution")
    );
}

#[test]
fn restores_document_group_and_converges_changed_auxiliary_inputs() {
    util::set_test_root();
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    std::fs::write(
        root.join("main.tex"),
        br"\newread\auxin
\newwrite\auxout
\def\previous{Unknown}
\openin\auxin=main.aux
\ifeof\auxin\closein\auxin\else\closein\auxin\input main.aux\fi
\immediate\openout\auxout=main.aux
\begingroup
\input prefix.tex
\input suffix.tex
\immediate\write\auxout{\string\def\string\previous{\current}}
\endgroup
\immediate\closeout\auxout
\bye
",
    )
    .unwrap();
    std::fs::write(root.join("main.aux"), b"\\def\\previous{Initial}\n").unwrap();
    std::fs::write(root.join("prefix.tex"), b"Previous result: \\previous.\\par\n").unwrap();
    std::fs::write(root.join("suffix.tex"), b"\\def\\current{Original}Original suffix.\\par\n").unwrap();
    compile(root, true, false);
    std::fs::write(root.join("suffix.tex"), b"\\def\\current{Updated}Edited suffix.\\par\n").unwrap();
    let (actual, status) = compile(root, true, false);
    assert!(
        status.iter().any(|message| message.contains("resumed TeX execution")),
        "{}",
        status.join("\n")
    );
    assert!(
        status.iter().any(|message| message.contains("checkpoint invalidated")),
        "{}",
        status.join("\n")
    );
    let (clean, _) = compile(root, false, false);
    assert_outputs_match(&actual, &clean);
    assert_eq!(actual["main.aux"].data, clean["main.aux"].data);
}

#[test]
#[ignore = "requires a cached real LaTeX bundle and fontspec"]
fn real_latex_fontspec_and_cross_references_match_clean_builds() {
    util::set_test_root();
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    std::fs::write(
        root.join("main.tex"),
        br"\documentclass{article}
\usepackage{fontspec}
\usepackage{hyperref}
\setmainfont{lmroman12-regular.otf}
\begin{document}
\input{prefix}
\input{suffix}
\end{document}
",
    )
    .unwrap();
    std::fs::write(
        root.join("prefix.tex"),
        b"\\section{Prefix}\\label{prefix}\nSee section \\ref{suffix}.\n\n",
    )
    .unwrap();
    std::fs::write(root.join("suffix.tex"), b"\\section{Suffix}\\label{suffix}\nInitial content.\n\n").unwrap();
    let run = |incremental: bool| {
        let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_tectonic"));
        command
            .current_dir(root)
            .args(["-X", "compile", "--only-cached", "--synctex"])
            .env_remove(tectonic::test_util::TEST_ROOT_ENV_VAR)
            .env("SOURCE_DATE_EPOCH", "1600000000");
        if incremental {
            command.arg("--incremental");
        }
        let output = command.arg("main.tex").output().unwrap();
        let status = format!("{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        assert!(output.status.success(), "{status}");
        status
    };
    run(true);
    std::fs::write(
        root.join("suffix.tex"),
        b"\\section{Suffix}\\label{suffix}\nEdited content referring to section \\ref{prefix}.\n\n",
    )
    .unwrap();
    let status = run(true);
    assert!(status.contains("resumed TeX execution"), "{status}");
    let pdf = std::fs::read(root.join("main.pdf")).unwrap();
    let mut synctex = Vec::new();
    flate2::read::GzDecoder::new(std::fs::File::open(root.join("main.synctex.gz")).unwrap())
        .read_to_end(&mut synctex)
        .unwrap();
    run(false);
    assert_eq!(pdf, std::fs::read(root.join("main.pdf")).unwrap());
    let mut clean_synctex = Vec::new();
    flate2::read::GzDecoder::new(std::fs::File::open(root.join("main.synctex.gz")).unwrap())
        .read_to_end(&mut clean_synctex)
        .unwrap();
    assert_eq!(synctex, clean_synctex);
}

#[test]
#[ignore = "manual incremental compilation benchmark"]
fn benchmark_multi_input_document() {
    util::set_test_root();
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    std::fs::write(
        root.join("main.tex"),
        b"\\input prefix.tex\n\\input middle.tex\n\\input suffix.tex\n\\bye\n",
    )
    .unwrap();
    std::fs::write(
        root.join("prefix.tex"),
        b"\\newcount\\work\\work=0\n\\loop\\advance\\work by1 \\ifnum\\work<5000000 \\repeat\nPrefix work: \\the\\work.\\par\n",
    )
    .unwrap();
    std::fs::write(root.join("middle.tex"), format!("{}\\par\n", "Unchanged middle chapter. ".repeat(500))).unwrap();
    std::fs::write(root.join("suffix.tex"), b"Initial suffix.\\par\n").unwrap();
    compile(root, true, false);
    std::fs::write(root.join("suffix.tex"), b"Edited suffix.\\par\n").unwrap();
    let start = std::time::Instant::now();
    let (resumed, status) = compile(root, true, false);
    let incremental_time = start.elapsed();
    let start = std::time::Instant::now();
    let (clean, _) = compile(root, false, false);
    let clean_time = start.elapsed();
    assert_outputs_match(&resumed, &clean);
    assert!(status.iter().any(|line| line.contains("resumed TeX execution")));
    println!("clean: {clean_time:?}; incremental: {incremental_time:?}");
}
