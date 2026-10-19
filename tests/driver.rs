// Copyright 2018 the Tectonic Project
// Licensed under the MIT License.

//! This test rig is a total hack to quickly exercise `src/driver.rs`.
//!
//! I should make it real, but I just want Codecov to stop complaining about
//! my test coverage. To re-use the `tex-outputs` test artifacts, the
//! ProcessingSessionBuilder will need to learn how to tell `xdvipdfmx` to
//! enable the reproducibility options used in the `tex-outputs` test rig.

use tectonic::{
    config::PersistentConfig, driver::ProcessingSessionBuilder, status::ChatterLevel,
    status::termcolor::TermcolorStatusBackend, test_util::TestBundle,
};

mod util;

// Keep these alphabetized.

#[test]
fn buffered_input_preserves_unicode_normalization_and_line_endings() {
    util::set_test_root();
    let directory = tempfile::tempdir().unwrap();
    for ending in ["\n", "\r", "\r\n"] {
        for (normalization, expected) in [(0, "a\u{301}"), (1, "\u{e1}"), (2, "a\u{301}")] {
            let source = [
                format!("\\XeTeXinputnormalization={normalization}"),
                format!("%{}", "x".repeat(8190)),
                "\\message{INPUT:[a\u{301}]}".to_owned(),
                "% trailing spaces   ".to_owned(),
                String::new(),
                "\\message{END-OF-INPUT}\\shipout\\hbox{}\\end".to_owned(),
            ]
            .join(ending);
            let mut status = tectonic_status_base::NoopStatusBackend::default();
            let mut builder = ProcessingSessionBuilder::default();
            builder
                .primary_input_buffer(source.as_bytes())
                .tex_input_name("buffered.tex")
                .format_name("plain")
                .format_cache_path(util::test_path(&[]))
                .output_dir(directory.path())
                .output_format(tectonic::driver::OutputFormat::Xdv)
                .keep_logs(true)
                .bundle(Box::<TestBundle>::default());
            builder
                .create(&mut status)
                .unwrap()
                .run(&mut status)
                .unwrap();
            let log = std::fs::read_to_string(directory.path().join("buffered.log")).unwrap();
            assert!(
                log.contains(&format!("INPUT:[{expected}]")),
                "{normalization}: {log}"
            );
            assert!(log.contains("END-OF-INPUT"), "{log}");
        }
    }
}

#[test]
fn control_sequence_lookup_preserves_bmp_supplementary_and_redefined_names() {
    util::set_test_root();
    let directory = tempfile::tempdir().unwrap();
    let source = "\\catcode\"03B1=11 \\catcode\"1F600=11\n\
        \\def\\\u{3b1}unicode{BMP}\\def\\\u{1f600}unicode{ASTRAL}\n\
        \\def\\abcdefgh{first}\n\
        \\message{LOOKUP:[\\\u{3b1}unicode/\\\u{1f600}unicode/\\abcdefgh]}\n\
        \\def\\abcdefgh{second}\n\
        \\message{REDEFINED:[\\csname abcdefgh\\endcsname]}\n\
        \\def\\identity#1{#1}\\edef\\saved{\\noexpand\\abcdefgh}\n\
        \\message{PARAMETER:[\\identity{\\saved}]}\n\
        \\outer\\def\\outertest{\\message{OUTER:[ok]}}\\outertest\n\
        \\halign{#\\hfil&#\\hfil\\cr a&b\\cr}\n\
        \\shipout\\hbox{}\\end";
    let mut status = tectonic_status_base::NoopStatusBackend::default();
    let mut builder = ProcessingSessionBuilder::default();
    builder
        .primary_input_buffer(source.as_bytes())
        .tex_input_name("lookup.tex")
        .format_name("plain")
        .format_cache_path(util::test_path(&[]))
        .output_dir(directory.path())
        .output_format(tectonic::driver::OutputFormat::Xdv)
        .keep_logs(true)
        .bundle(Box::<TestBundle>::default());
    builder
        .create(&mut status)
        .unwrap()
        .run(&mut status)
        .unwrap();
    let log = std::fs::read_to_string(directory.path().join("lookup.log")).unwrap();
    assert!(log.contains("LOOKUP:[BMP/ASTRAL/first]"), "{log}");
    assert!(log.contains("REDEFINED:[second]"), "{log}");
    assert!(log.contains("PARAMETER:[second]"), "{log}");
    assert!(log.contains("OUTER:[ok]"), "{log}");
}

#[test]
fn cleanup_reports_unclosed_groups_and_conditionals_without_leaking_state() {
    util::set_test_root();
    let directory = tempfile::tempdir().unwrap();
    for (source, unclosed) in [
        (br"\begingroup\iftrue\shipout\hbox{}\end".as_slice(), true),
        (br"\shipout\hbox{}\end".as_slice(), false),
    ] {
        let mut status = tectonic_status_base::NoopStatusBackend::default();
        let mut builder = ProcessingSessionBuilder::default();
        builder
            .primary_input_buffer(source)
            .tex_input_name("cleanup.tex")
            .format_name("plain")
            .format_cache_path(util::test_path(&[]))
            .output_dir(directory.path())
            .output_format(tectonic::driver::OutputFormat::Xdv)
            .keep_logs(true)
            .bundle(Box::<TestBundle>::default());
        builder
            .create(&mut status)
            .unwrap()
            .run(&mut status)
            .unwrap();

        let log = std::fs::read_to_string(directory.path().join("cleanup.log")).unwrap();
        assert_eq!(log.contains("inside a group at level 1"), unclosed, "{log}");
        assert_eq!(log.contains("was incomplete"), unclosed, "{log}");
    }
}

#[test]
fn the_letter_a() {
    util::set_test_root();

    let _config = PersistentConfig::default();

    // The "Normal" chatter escapes the test rig's attempts to eat stdout ...
    let mut status = TermcolorStatusBackend::new(ChatterLevel::Minimal);

    let bundle = TestBundle::default();

    let tempdir = tempfile::Builder::new()
        .prefix("tectonic_driver_test")
        .tempdir()
        .unwrap();

    let mut pbuilder = ProcessingSessionBuilder::default();
    pbuilder
        .primary_input_path(util::test_path(&["tex-outputs", "the_letter_a.tex"]))
        .tex_input_name("the_letter_a.tex")
        .format_name("plain")
        .format_cache_path(util::test_path(&[]))
        .output_dir(tempdir.path())
        .bundle(Box::new(bundle));

    let mut session = pbuilder
        .create(&mut status)
        .expect("couldn't create processing session");

    session
        .run(&mut status)
        .expect("failed to execute processing session");
}

#[test]
fn html_templates_preserve_content_and_update_variables_between_outputs() {
    util::set_test_root();
    let tempdir = tempfile::tempdir().unwrap();
    std::fs::write(
        tempdir.path().join("base.txt"),
        "<title>{{ title | escape_html }}</title>{% block content %}{% endblock content %}",
    )
    .unwrap();
    std::fs::write(
        tempdir.path().join("page.html"),
        "{% extends \"base.txt\" %}{% block content %}<main>{{ tduxContent }}</main><a href=\"{{ tduxRelTop }}index.html\">Home</a>{% endblock content %}",
    ).unwrap();

    let source = br"\shipout\vbox{
        \special{tdux:addTemplate base.txt}
        \special{tdux:setTemplate page.html}
        \special{tdux:setTemplateVariable title First & <page>}
        \special{tdux:setOutputPath nested/chapter.html}
        \special{tdux:dt Hello & <world>}
        \special{tdux:emit}
        \special{tdux:setTemplateVariable title Second}
        \special{tdux:setOutputPath index.html}
        \special{tdux:dt Next page}
        \special{tdux:emit}
    }\end";
    let mut status = tectonic_status_base::NoopStatusBackend::default();
    let mut builder = ProcessingSessionBuilder::default();
    builder
        .primary_input_buffer(source)
        .tex_input_name("html.tex")
        .filesystem_root(tempdir.path())
        .format_name("plain")
        .format_cache_path(util::test_path(&[]))
        .output_dir(tempdir.path())
        .output_format(tectonic::driver::OutputFormat::Html)
        .bundle(Box::<TestBundle>::default());
    builder
        .create(&mut status)
        .unwrap()
        .run(&mut status)
        .unwrap();

    assert_eq!(
        std::fs::read_to_string(tempdir.path().join("nested/chapter.html")).unwrap(),
        "<title>First &amp; &lt;page&gt;</title><main>Hello &amp; &lt;world&gt;</main><a href=\"../index.html\">Home</a>",
    );
    assert_eq!(
        std::fs::read_to_string(tempdir.path().join("index.html")).unwrap(),
        "<title>Second</title><main>Next page</main><a href=\"index.html\">Home</a>",
    );
}
