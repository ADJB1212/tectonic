// Copyright 2018 the Tectonic Project
// Licensed under the MIT License.

//! This test rig is a total hack to quickly exercise `src/driver.rs`.
//!
//! I should make it real, but I just want Codecov to stop complaining about
//! my test coverage. To re-use the `tex-outputs` test artifacts, the
//! ProcessingSessionBuilder will need to learn how to tell `xdvipdfmx` to
//! enable the reproducibility options used in the `tex-outputs` test rig.

use tectonic::{
    config::PersistentConfig, driver::ProcessingSessionBuilder,
    status::termcolor::TermcolorStatusBackend, status::ChatterLevel, test_util::TestBundle,
};

mod util;

// Keep these alphabetized.

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
    ).unwrap();
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
    builder.create(&mut status).unwrap().run(&mut status).unwrap();

    assert_eq!(
        std::fs::read_to_string(tempdir.path().join("nested/chapter.html")).unwrap(),
        "<title>First &amp; &lt;page&gt;</title><main>Hello &amp; &lt;world&gt;</main><a href=\"../index.html\">Home</a>",
    );
    assert_eq!(
        std::fs::read_to_string(tempdir.path().join("index.html")).unwrap(),
        "<title>Second</title><main>Next page</main><a href=\"index.html\">Home</a>",
    );
}
