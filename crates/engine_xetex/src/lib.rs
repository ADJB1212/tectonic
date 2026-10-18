// Copyright 2021-2022 the Tectonic Project
// Licensed under the MIT License.

//! The [XeTeX] program as a reusable crate.
//!
//! [XeTeX]: http://www.xetex.org/
//!
//! This crate provides the core TeX engine implementation used by [Tectonic].
//! However, in order to obtain the full Tectonic user experience, it must be
//! combined with a variety of other utilities: the `xdvipdfmx` engine, code to
//! fetch support files, and so on. Rather than using this crate directly you
//! should probably use the main [`tectonic`] crate, which combines all of these
//! pieces into a (semi) coherent whole.
//!
//! [Tectonic]: https://tectonic-typesetting.github.io/
//! [`tectonic`]: https://docs.rs/tectonic/

// TODO: the internal interface we're using here is pretty janky. The bibtex
// engine has a nicer approach that we should probably start using.

use std::{ffi::CString, time::SystemTime};
use tectonic_bridge_core::{CoreBridgeLauncher, CoreBridgeState, EngineAbortedError};
use tectonic_errors::prelude::*;

/// A serial number describing the detailed binary layout of the TeX "format
/// files" used by this crate. This number will occasionally increment,
/// indicating that the format file structure has changed. There is no provision
/// for partial forwards or backwards compatibility: if the number changes, you
/// need to regenerate your format files. If you’re generating format files, you
/// should munge this serial number in the filename, or something along those
/// lines, to make sure that when the engine is updated you don’t attempt to
/// reuse old files.
//
// DEVELOPER NOTE: if you change this, rerun cbindgen! This value is exported
// into the C/C++ code as a #define.
pub const FORMAT_SERIAL: u32 = 33;

/// An engine checkpoint and the I/O operations that precede it.
#[derive(Clone, Debug)]
pub struct TexCheckpoint {
    pub native: Vec<u8>,
    pub io: Vec<u8>,
}

/// A saved prefix could not be reused. Reset driver state before retrying.
#[derive(Debug)]
pub struct CheckpointRejected(pub String);

impl std::fmt::Display for CheckpointRejected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for CheckpointRejected {}

/// A possible outcome from a (Xe)TeX engine invocation.
///
/// The classic TeX implementation provides a fourth outcome: "fatal error". In
/// Tectonic, this outcome is represented as an `Err` result rather than a
/// [`TexOutcome`].
///
/// The `Errors` possibility will only occur if the `halt_on_error` engine
/// option is false: if it’s true, errors get upgraded to fatals.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TexOutcome {
    /// Nothing bad happened.
    Spotless,

    /// Warnings were issued by the TeX engine. Note that, due to the ways that
    /// people are used to using TeX, warnings are *extremely* common in the
    /// wild. It’s rare to find a real document that *doesn’t* compile with
    /// warnings.
    Warnings,

    /// Errors were issued by the TeX engine. Note that, in TeX terminology,
    /// errors are not necessarily *fatal* errors: the engine will try extremely
    /// hard to proceed when it encounters them. It is not uncommon to find TeX
    /// documents in the wild that produce errors.
    Errors,
}

/// A struct for invoking the (Xe)TeX engine.
///
/// This struct has a fairly straightforward "builder" interface: you create it,
/// apply any settings that you wish, and eventually run the
/// [`process()`](Self::process) method.
///
/// Due to constraints of the gnarly C/C++ code underlying the engine
/// implementation, only one engine may run at once in one process. The engine
/// execution framework uses a global mutex to ensure that this is the case.
/// This restriction applies not only to the [`TexEngine`] type but to *all*
/// Tectonic engines. I.e., you can't run this engine and the BibTeX engine at
/// the same time.
#[derive(Debug)]
pub struct TexEngine {
    checkpointing: bool,
    checkpoint: Option<TexCheckpoint>,
    resumed_dispatches: u64,
    // One day, the engine will hold its own state. For the time being,
    // though, it's just a proxy for the global constants in the C code.
    halt_on_error: bool,
    initex_mode: bool,
    synctex_enabled: bool,
    semantic_pagination_enabled: bool,
    shell_escape_enabled: bool,
    build_date: SystemTime,
}

impl Default for TexEngine {
    fn default() -> Self {
        TexEngine {
            checkpointing: false,
            checkpoint: None,
            resumed_dispatches: 0,
            halt_on_error: true,
            initex_mode: false,
            synctex_enabled: false,
            semantic_pagination_enabled: false,
            shell_escape_enabled: false,
            build_date: SystemTime::UNIX_EPOCH,
        }
    }
}

impl TexEngine {
    /// Number of prefix dispatches skipped by the last successful restoration.
    pub fn resumed_dispatches(&self) -> u64 {
        self.resumed_dispatches
    }
    /// Record resumable checkpoints through the driver's checkpoint hook.
    pub fn checkpointing(&mut self, enabled: bool) -> &mut Self {
        self.checkpointing = enabled;
        self
    }

    /// Resume a checkpoint in a fresh bridge and driver state.
    pub fn resume_checkpoint(&mut self, checkpoint: TexCheckpoint) -> &mut Self {
        self.checkpoint = Some(checkpoint);
        self
    }
    /// Configure whether the engine will halt on errors.
    ///
    /// The default setting is true. If false, the engine will plunge on ahead
    /// in the face of all but the most catastrophic problems. It’s really quite
    /// impressive!
    pub fn halt_on_error_mode(&mut self, halt_on_error: bool) -> &mut Self {
        self.halt_on_error = halt_on_error;
        self
    }

    /// Configure the engine to run in "initex" mode, in which it generates a
    /// "format" file that serializes the engine state rather than a PDF
    /// document. The default is false.
    pub fn initex_mode(&mut self, initex: bool) -> &mut Self {
        self.initex_mode = initex;
        self
    }

    /// Configure the engine to produce SyncTeX data.
    ///
    /// The default is false.
    pub fn synctex(&mut self, synctex_enabled: bool) -> &mut Self {
        self.synctex_enabled = synctex_enabled;
        self
    }

    /// Configure the engine to use "semantic pagination".
    ///
    /// **Important:** this mode is essentially unimplemented.
    ///
    /// The goal of this mode is to set up the engine to create HTML-friendly
    /// output by altering how paragraphs and pages are constructed. When this
    /// mode is activated, the engine output type changes from XDV to SPX
    /// (although the two formats are quite similar).
    ///
    /// The default is false.
    pub fn semantic_pagination(&mut self, enabled: bool) -> &mut Self {
        self.semantic_pagination_enabled = enabled;
        self
    }

    /// Configure whether the "shell escape" TeX feature is enabled.
    ///
    /// The default is false.
    pub fn shell_escape(&mut self, shell_escape_enabled: bool) -> &mut Self {
        self.shell_escape_enabled = shell_escape_enabled;
        self
    }

    /// Sets the date and time used by the TeX engine. This affects things like
    /// LaTeX's \today command.
    ///
    /// The default vaue is the Unix epoch, so you should almost always override
    /// this setting. If you are aiming to achieve reproducible builds, you will
    /// need a way to fix this parameter from one engine invocation to the next.
    pub fn build_date(&mut self, date: SystemTime) -> &mut Self {
        self.build_date = date;
        self
    }

    /// Process a document using the current engine configuration.
    ///
    /// The *launcher* parameter gives overarching environmental context in
    /// which the engine will be run.
    ///
    /// The *format_file_name* is the name for the TeX "format file" giving
    /// preloaded engine state. It must be findable in the I/O stack, using the
    /// special hooks that are provided for handing format files, which allow
    /// updates to the file format to be handed (see [`FORMAT_SERIAL`]). If in
    /// "initex" mode, this parameter will be ignored.
    ///
    /// The *input_file_name* is used to name the "primary input file". The I/O
    /// system has special hooks for opening this primary input, so be aware
    /// that this filename is *not* opened using the usual mechanisms. This
    /// setting affects some of the names used by the engine internally,
    /// including the name it uses to create its main output files. The
    /// traditional default value is `"texput"`.
    pub fn process(
        &mut self,
        launcher: &mut CoreBridgeLauncher,
        format_file_name: &str,
        input_file_name: &str,
    ) -> Result<TexOutcome> {
        let cformat = CString::new(format_file_name)?;
        let cinput = CString::new(input_file_name)?;

        launcher.with_global_lock(|state| {
            if let Some(checkpoint) = &self.checkpoint {
                state
                    .replay_checkpoint_io(&checkpoint.io)
                    .map_err(|error| CheckpointRejected(error.to_string()))?;
            } else if self.checkpointing {
                state.record_checkpoint_io();
            }
            // Note that we have to do all of this setup while holding the
            // lock, because we're modifying static state variables.

            // SAFETY: All methods are called with valid C-strings and while the global lock is held.
            let r = unsafe {
                use c_api::*;
                let (bytes, length) = self
                    .checkpoint
                    .as_ref()
                    .map_or((std::ptr::null(), 0), |checkpoint| {
                        (checkpoint.native.as_ptr(), checkpoint.native.len())
                    });
                tt_checkpoint_set(bytes, length, self.checkpointing);
                tt_xetex_set_int_variable(
                    c"shell_escape_enabled".as_ptr(),
                    self.shell_escape_enabled.into(),
                );
                tt_xetex_set_int_variable(c"halt_on_error_p".as_ptr(), self.halt_on_error.into());
                tt_xetex_set_int_variable(c"in_initex_mode".as_ptr(), self.initex_mode.into());
                tt_xetex_set_int_variable(c"synctex_enabled".as_ptr(), self.synctex_enabled.into());
                tt_xetex_set_int_variable(
                    c"semantic_pagination_enabled".as_ptr(),
                    self.semantic_pagination_enabled.into(),
                );

                tt_engine_xetex_main(
                    state,
                    cformat.as_ptr(),
                    cinput.as_ptr(),
                    self.build_date
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("invalid build date")
                        .as_secs(),
                )
            };
            self.resumed_dispatches = unsafe { c_api::tt_checkpoint_skipped() };

            match r {
                0 => Ok(TexOutcome::Spotless),
                1 => Ok(TexOutcome::Warnings),
                2 => Ok(TexOutcome::Errors),
                3 if self.checkpoint.is_some() && !unsafe { c_api::tt_checkpoint_restored() } => {
                    Err(CheckpointRejected("engine checkpoint could not be restored".into()).into())
                }
                3 => Err(EngineAbortedError::new_abort_indicator().into()),
                x => Err(anyhow!("internal error: unexpected 'history' value {}", x)),
            }
        })
    }
}

/// # Safety
/// `bytes` must reference the native snapshot for the duration of this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tt_xetex_checkpoint_emit(bytes: *const u8, length: usize) {
    if length > 512 * 1024 * 1024 {
        return;
    }
    let bytes = unsafe { std::slice::from_raw_parts(bytes, length) };
    CoreBridgeState::with_global_state(|state| state.capture_engine_checkpoint(bytes));
}

#[unsafe(no_mangle)]
pub extern "C" fn tt_xetex_checkpoint_ready() -> bool {
    CoreBridgeState::with_global_state(|state| state.checkpoint_io_ready())
}

/// # Safety
/// `checkpoint` must reference the active native checkpoint transaction.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn tt_xetex_checkpoint_layout(
    checkpoint: *mut std::ffi::c_void,
    restoring: bool,
) -> bool {
    unsafe extern "C" {
        fn tt_checkpoint_field(
            checkpoint: *mut std::ffi::c_void,
            name: *const std::ffi::c_char,
            value: *mut std::ffi::c_void,
            length: usize,
        );
    }
    use tectonic_xetex_layout::{ProtrusionCode, protrusion_codes, restore_protrusion_codes};
    let mut codes = if restoring {
        Vec::new()
    } else {
        protrusion_codes()
    };
    let mut count = codes.len() as u64;
    unsafe {
        tt_checkpoint_field(
            checkpoint,
            c"protrusion_count".as_ptr(),
            (&mut count as *mut u64).cast(),
            8,
        );
    }
    if count > 1_000_000 {
        return false;
    }
    if restoring {
        codes.resize(
            count as usize,
            ProtrusionCode {
                font: 0,
                character: 0,
                side: 0,
                value: 0,
            },
        );
    }
    unsafe {
        tt_checkpoint_field(
            checkpoint,
            c"protrusion_codes".as_ptr(),
            codes.as_mut_ptr().cast(),
            codes.len() * std::mem::size_of::<ProtrusionCode>(),
        );
    }
    if restoring {
        if codes
            .iter()
            .any(|code| code.font < 0 || code.font > 9000 || !(0..=1).contains(&code.side))
        {
            return false;
        }
        restore_protrusion_codes(&codes);
    }
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn tt_xetex_checkpoint_handle_valid(handle: usize, output: bool) -> bool {
    CoreBridgeState::with_global_state(|state| state.checkpoint_handle_valid(handle, output))
}

#[doc(hidden)]
pub mod c_api {
    // If you change the interfaces here, rerun cbindgen as described in the README!

    use tectonic_bridge_core::CoreBridgeState;

    #[allow(improper_ctypes)] // for CoreBridgeState
    unsafe extern "C" {
        pub fn tt_checkpoint_set(bytes: *const u8, length: usize, enabled: bool);
        pub fn tt_checkpoint_skipped() -> u64;
        pub fn tt_checkpoint_restored() -> bool;
        pub fn tt_xetex_set_int_variable(
            var_name: *const libc::c_char,
            value: libc::c_int,
        ) -> libc::c_int;

        pub fn tt_engine_xetex_main(
            api: &mut CoreBridgeState,
            dump_name: *const libc::c_char,
            input_file_name: *const libc::c_char,
            build_date: u64,
        ) -> libc::c_int;
    }
}

/// Import things from our bridge crates to ensure that we actually link with
/// them.
mod linkage {
    #[allow(unused_imports)]
    use tectonic_pdf_io as clipyrenamehack1;

    #[allow(unused_imports)]
    use tectonic_xetex_layout as clipyrenamehack2;

    #[allow(unused_imports)]
    use tectonic_bridge_icu as clipyrenamehack3;
}

/// Does our resulting executable link correctly?
#[test]
fn linkage() {}
