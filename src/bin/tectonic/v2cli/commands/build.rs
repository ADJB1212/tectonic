use clap::Args;
use std::num::NonZeroUsize;
use tectonic::{
    config::PersistentConfig,
    config::is_config_test_mode_activated,
    docmodel::{DocumentExt, DocumentSetupOptions},
    tt_error, tt_note,
};
use tectonic_bridge_core::{SecuritySettings, SecurityStance};
use tectonic_docmodel::workspace::Workspace;
use tectonic_errors::Result;
use tectonic_status_base::StatusBackend;

use crate::v2cli::{CommandCustomizations, TectonicCommand};

/// `build`: Build a document
#[derive(Debug, Eq, PartialEq, Args, Clone)]
pub struct BuildCommand {
    /// Resume TeX execution from persistent checkpoints
    #[arg(long)]
    incremental: bool,

    /// Override the incremental checkpoint directory
    #[arg(long, requires = "incremental", value_hint = clap::ValueHint::DirPath)]
    incremental_cache_dir: Option<std::path::PathBuf>,
    /// Number of PDF workers [default: available CPU count]
    #[arg(long)]
    pdf_workers: Option<NonZeroUsize>,

    /// Maximum queued PDF payload in MiB
    #[arg(long, default_value = "64", value_parser = crate::compile::parse_pdf_queue_mib)]
    pdf_queue_mib: NonZeroUsize,
    /// Document is untrusted -- disable all known-insecure features
    #[arg(long)]
    untrusted: bool,

    /// Use only resource files cached locally
    #[arg(short = 'C', long)]
    only_cached: bool,

    /// Keep the intermediate files generated during processing
    #[arg(short, long)]
    keep_intermediates: bool,

    /// Keep the log files generated during processing
    #[arg(long)]
    keep_logs: bool,

    /// Print the engine's chatter during processing
    #[arg(long = "print", short)]
    print_stdout: bool,

    /// Open built document using system handler
    #[arg(long)]
    open: bool,

    /// Specify a target to be used by the build
    #[arg(long, help = "Specify the target of the build.")]
    target: Option<String>,

    /// Use this URL to find resource files instead of the default
    #[arg(long, short)]
    bundle: Option<String>,
}

impl TectonicCommand for BuildCommand {
    fn customize(&self, _cc: &mut CommandCustomizations) {}

    fn execute(self, config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        // `--bundle` is not used for `-X build`,
        // tell the user instead of ignoring silently.
        if let Some(url) = &self.bundle {
            tt_note!(status, "--bundle {} ignored", url);
            tt_note!(status, "using workspace bundle configuration");
        }
        let ws = Workspace::open_from_environment()?;
        let doc = ws.first_document();

        // Default to allowing insecure since it would be super duper annoying
        // to have to pass `--trusted` every time to build a personal document
        // that uses shell-escape! This default can be overridden by setting the
        // environment variable TECTONIC_UNTRUSTED_MODE to a nonempty value.
        let stance = if self.untrusted {
            SecurityStance::DisableInsecures
        } else {
            SecurityStance::MaybeAllowInsecures
        };

        let mut setup_options = DocumentSetupOptions::new_with_security(SecuritySettings::new(stance));
        setup_options.only_cached(self.only_cached);

        for output_name in doc.output_names() {
            if let Some(out) = self.target.as_ref()
                && out != output_name
            {
                continue;
            }

            let mut builder = doc.setup_session(output_name, &setup_options, status)?;
            if self.incremental {
                let default = doc.output_main_file(output_name).parent().unwrap().join(".tectonic-incremental");
                builder.incremental_cache_path(self.incremental_cache_dir.as_ref().unwrap_or(&default));
            }
            builder.pdf_build_options(tectonic::PdfBuildOptions {
                workers: self.pdf_workers,
                queued_bytes: NonZeroUsize::new(self.pdf_queue_mib.get() * 1024 * 1024).unwrap(),
            });

            builder
                .format_cache_path(config.format_cache_path()?)
                .keep_intermediates(self.keep_intermediates)
                .keep_logs(self.keep_logs)
                .print_stdout(self.print_stdout);

            crate::compile::run_and_report(builder, status)?;

            if self.open {
                let out_file = doc.output_main_file(output_name);

                if is_config_test_mode_activated() {
                    tt_note!(status, "not opening `{}` -- test mode", out_file.display());
                } else {
                    tt_note!(status, "opening `{}`", out_file.display());
                    if let Err(e) = open::that(&out_file) {
                        tt_error!(
                            status,
                            "failed to open `{}` with system handler",
                            out_file.display();
                            e.into()
                        )
                    }
                }
            }
        }

        Ok(0)
    }
}
