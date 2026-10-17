#![allow(unused_mut)]

pub(crate) unsafe fn reset_abi_state() {
    unsafe {
        dpx_dpxconf::dpx_conf = dpx_dpxconf::_dpx_conf {
            is_xbb: 0,
            verbose_level: 0,
            compat_mode: 0,
            ignore_font_license: 0,
            file: dpx_dpxconf::C2RustUnnamed { keep_cache: 0 },
        };
        dpx_dvipdfmx::landscape_mode = 0;
        dpx_dvipdfmx::paper_width = 595.0;
        dpx_dvipdfmx::paper_height = 842.0;
        dpx_dvipdfmx::ttpi_source_date_epoch = -1;
        dpx_cid::opt_flags_cidfont = 0;
        dpx_pdfdoc::global_names = core::ptr::null_mut();
        dpx_mfileio::work_buffer = [0; 1024];
    }
}

pub(crate) struct StateOwner<T> {
    pub(crate) pointer: *mut T,
    _storage: Box<T>,
}

impl<T> StateOwner<T> {
    fn new(value: T) -> Self {
        let mut storage = Box::new(value);
        Self {
            pointer: &mut *storage,
            _storage: storage,
        }
    }
}

pub(crate) struct DocumentStates {
    pub(crate) dpx_agl: StateOwner<dpx_agl::DocumentState>,
    pub(crate) dpx_cff_dict: StateOwner<dpx_cff_dict::DocumentState>,
    pub(crate) dpx_cmap: StateOwner<dpx_cmap::DocumentState>,
    pub(crate) dpx_cs_type2: StateOwner<dpx_cs_type2::DocumentState>,
    pub(crate) dpx_dvi: StateOwner<dpx_dvi::DocumentState>,
    pub(crate) dpx_mpost: StateOwner<dpx_mpost::DocumentState>,
    pub(crate) dpx_mt19937ar: StateOwner<dpx_mt19937ar::DocumentState>,
    pub(crate) dpx_pdfcolor: StateOwner<dpx_pdfcolor::DocumentState>,
    pub(crate) dpx_pdfdev: StateOwner<dpx_pdfdev::DocumentState>,
    pub(crate) dpx_pdfdoc: StateOwner<dpx_pdfdoc::DocumentState>,
    pub(crate) dpx_pdfdraw: StateOwner<dpx_pdfdraw::DocumentState>,
    pub(crate) dpx_pdfencoding: StateOwner<dpx_pdfencoding::DocumentState>,
    pub(crate) dpx_pdffont: StateOwner<dpx_pdffont::DocumentState>,
    pub(crate) dpx_pdfobj: StateOwner<dpx_pdfobj::DocumentState>,
    pub(crate) dpx_pdfparse: StateOwner<dpx_pdfparse::DocumentState>,
    pub(crate) dpx_pdfresource: StateOwner<dpx_pdfresource::DocumentState>,
    pub(crate) dpx_pdfximage: StateOwner<dpx_pdfximage::DocumentState>,
    pub(crate) dpx_pkfont: StateOwner<dpx_pkfont::DocumentState>,
    pub(crate) dpx_sfnt: StateOwner<dpx_sfnt::DocumentState>,
    pub(crate) dpx_spc_dvips: StateOwner<dpx_spc_dvips::DocumentState>,
    pub(crate) dpx_spc_html: StateOwner<dpx_spc_html::DocumentState>,
    pub(crate) dpx_spc_misc: StateOwner<dpx_spc_misc::DocumentState>,
    pub(crate) dpx_spc_pdfm: StateOwner<dpx_spc_pdfm::DocumentState>,
    pub(crate) dpx_spc_tpic: StateOwner<dpx_spc_tpic::DocumentState>,
    pub(crate) dpx_spc_xtx: StateOwner<dpx_spc_xtx::DocumentState>,
    pub(crate) dpx_specials: StateOwner<dpx_specials::DocumentState>,
    pub(crate) dpx_subfont: StateOwner<dpx_subfont::DocumentState>,
    pub(crate) dpx_t1_char: StateOwner<dpx_t1_char::DocumentState>,
    pub(crate) dpx_vf: StateOwner<dpx_vf::DocumentState>,
}

impl DocumentStates {
    pub(crate) fn new() -> Self {
        Self {
            dpx_agl: StateOwner::new(dpx_agl::DocumentState::new()),
            dpx_cff_dict: StateOwner::new(dpx_cff_dict::DocumentState::new()),
            dpx_cmap: StateOwner::new(dpx_cmap::DocumentState::new()),
            dpx_cs_type2: StateOwner::new(dpx_cs_type2::DocumentState::new()),
            dpx_dvi: StateOwner::new(dpx_dvi::DocumentState::new()),
            dpx_mpost: StateOwner::new(dpx_mpost::DocumentState::new()),
            dpx_mt19937ar: StateOwner::new(dpx_mt19937ar::DocumentState::new()),
            dpx_pdfcolor: StateOwner::new(dpx_pdfcolor::DocumentState::new()),
            dpx_pdfdev: StateOwner::new(dpx_pdfdev::DocumentState::new()),
            dpx_pdfdoc: StateOwner::new(dpx_pdfdoc::DocumentState::new()),
            dpx_pdfdraw: StateOwner::new(dpx_pdfdraw::DocumentState::new()),
            dpx_pdfencoding: StateOwner::new(dpx_pdfencoding::DocumentState::new()),
            dpx_pdffont: StateOwner::new(dpx_pdffont::DocumentState::new()),
            dpx_pdfobj: StateOwner::new(dpx_pdfobj::DocumentState::new()),
            dpx_pdfparse: StateOwner::new(dpx_pdfparse::DocumentState::new()),
            dpx_pdfresource: StateOwner::new(dpx_pdfresource::DocumentState::new()),
            dpx_pdfximage: StateOwner::new(dpx_pdfximage::DocumentState::new()),
            dpx_pkfont: StateOwner::new(dpx_pkfont::DocumentState::new()),
            dpx_sfnt: StateOwner::new(dpx_sfnt::DocumentState::new()),
            dpx_spc_dvips: StateOwner::new(dpx_spc_dvips::DocumentState::new()),
            dpx_spc_html: StateOwner::new(dpx_spc_html::DocumentState::new()),
            dpx_spc_misc: StateOwner::new(dpx_spc_misc::DocumentState::new()),
            dpx_spc_pdfm: StateOwner::new(dpx_spc_pdfm::DocumentState::new()),
            dpx_spc_tpic: StateOwner::new(dpx_spc_tpic::DocumentState::new()),
            dpx_spc_xtx: StateOwner::new(dpx_spc_xtx::DocumentState::new()),
            dpx_specials: StateOwner::new(dpx_specials::DocumentState::new()),
            dpx_subfont: StateOwner::new(dpx_subfont::DocumentState::new()),
            dpx_t1_char: StateOwner::new(dpx_t1_char::DocumentState::new()),
            dpx_vf: StateOwner::new(dpx_vf::DocumentState::new()),
        }
    }
}

#[macro_use]
mod macros;
mod dpx_agl;
mod dpx_bmpimage;
mod dpx_cff;
mod dpx_cff_dict;
mod dpx_cid;
mod dpx_cidtype0;
mod dpx_cidtype2;
mod dpx_cmap;
mod dpx_cmap_read;
mod dpx_cmap_write;
mod dpx_cs_type2;
mod dpx_dpxconf;
mod dpx_dpxcrypt;
mod dpx_dpxfile;
mod dpx_dpxutil;
mod dpx_dvi;
mod dpx_dvipdfmx;
mod dpx_epdf;
mod dpx_fontmap;
mod dpx_jp2image;
mod dpx_jpegimage;
mod dpx_mem;
mod dpx_mfileio;
mod dpx_mpost;
mod dpx_mt19937ar;
mod dpx_numbers;
mod dpx_otl_opt;
mod dpx_pdfcolor;
pub(crate) mod dpx_pdfdev;
mod dpx_pdfdoc;
pub(crate) mod dpx_pdfdraw;
mod dpx_pdfencoding;
mod dpx_pdfencrypt;
mod dpx_pdffont;
mod dpx_pdfnames;
mod dpx_pdfobj;
mod dpx_pdfparse;
mod dpx_pdfresource;
mod dpx_pdfximage;
mod dpx_pkfont;
mod dpx_pngimage;
mod dpx_pst;
mod dpx_pst_obj;
mod dpx_sfnt;
mod dpx_spc_color;
mod dpx_spc_dvipdfmx;
mod dpx_spc_dvips;
mod dpx_spc_html;
mod dpx_spc_misc;
mod dpx_spc_pdfm;
mod dpx_spc_tpic;
mod dpx_spc_util;
mod dpx_spc_xtx;
mod dpx_specials;
mod dpx_subfont;
mod dpx_t1_char;
mod dpx_t1_load;
mod dpx_tfm;
mod dpx_truetype;
mod dpx_tt_aux;
mod dpx_tt_cmap;
mod dpx_tt_glyf;
mod dpx_tt_gsub;
mod dpx_tt_post;
mod dpx_tt_table;
mod dpx_type0;
mod dpx_type1;
mod dpx_type1c;
mod dpx_unicode;
mod dpx_vf;
pub mod runtime;

#[cfg(test)]
mod tests;
