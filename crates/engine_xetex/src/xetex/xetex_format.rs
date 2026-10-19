#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

pub const TECTONIC_FORMAT_VERSION: i32 = 33;

pub const HASH_SIZE: i32 = 15000;
pub const HASH_EXTRA: i32 = 600000;
pub const HASH_OFFSET: i32 = 514;
pub const HASH_PRIME: i32 = 8501;

pub const TOO_BIG_USV: i32 = 1114112;
pub const SPECIAL_CHAR: i32 = 1114113;

pub const PRIM_SIZE: i32 = 2100;
pub const PRIM_PRIME: i32 = 1777;
pub const MAX_FONT_MAX: i32 = 9000;
pub const MEM_TOP: i32 = 4999999;
pub const NUMBER_MATH_FAMILIES: i32 = 256;

pub const INT_PAR__pretolerance: i32 = 0;
pub const INT_PAR__tolerance: i32 = 1;
pub const INT_PAR__line_penalty: i32 = 2;
pub const INT_PAR__hyphen_penalty: i32 = 3;
pub const INT_PAR__ex_hyphen_penalty: i32 = 4;
pub const INT_PAR__club_penalty: i32 = 5;
pub const INT_PAR__widow_penalty: i32 = 6;
pub const INT_PAR__display_widow_penalty: i32 = 7;
pub const INT_PAR__broken_penalty: i32 = 8;
pub const INT_PAR__bin_op_penalty: i32 = 9;
pub const INT_PAR__rel_penalty: i32 = 10;
pub const INT_PAR__pre_display_penalty: i32 = 11;
pub const INT_PAR__post_display_penalty: i32 = 12;
pub const INT_PAR__inter_line_penalty: i32 = 13;
pub const INT_PAR__double_hyphen_demerits: i32 = 14;
pub const INT_PAR__final_hyphen_demerits: i32 = 15;
pub const INT_PAR__adj_demerits: i32 = 16;
pub const INT_PAR__mag: i32 = 17;
pub const INT_PAR__delimiter_factor: i32 = 18;
pub const INT_PAR__looseness: i32 = 19;
pub const INT_PAR__time: i32 = 20;
pub const INT_PAR__day: i32 = 21;
pub const INT_PAR__month: i32 = 22;
pub const INT_PAR__year: i32 = 23;
pub const INT_PAR__show_box_breadth: i32 = 24;
pub const INT_PAR__show_box_depth: i32 = 25;
pub const INT_PAR__hbadness: i32 = 26;
pub const INT_PAR__vbadness: i32 = 27;
pub const INT_PAR__pausing: i32 = 28;
pub const INT_PAR__tracing_online: i32 = 29;
pub const INT_PAR__tracing_macros: i32 = 30;
pub const INT_PAR__tracing_stats: i32 = 31;
pub const INT_PAR__tracing_paragraphs: i32 = 32;
pub const INT_PAR__tracing_pages: i32 = 33;
pub const INT_PAR__tracing_output: i32 = 34;
pub const INT_PAR__tracing_lost_chars: i32 = 35;
pub const INT_PAR__tracing_commands: i32 = 36;
pub const INT_PAR__tracing_restores: i32 = 37;
pub const INT_PAR__uc_hyph: i32 = 38;
pub const INT_PAR__output_penalty: i32 = 39;
pub const INT_PAR__max_dead_cycles: i32 = 40;
pub const INT_PAR__hang_after: i32 = 41;
pub const INT_PAR__floating_penalty: i32 = 42;
pub const INT_PAR__global_defs: i32 = 43;
pub const INT_PAR__cur_fam: i32 = 44;
pub const INT_PAR__escape_char: i32 = 45;
pub const INT_PAR__default_hyphen_char: i32 = 46;
pub const INT_PAR__default_skew_char: i32 = 47;
pub const INT_PAR__end_line_char: i32 = 48;
pub const INT_PAR__new_line_char: i32 = 49;
pub const INT_PAR__language: i32 = 50;
pub const INT_PAR__left_hyphen_min: i32 = 51;
pub const INT_PAR__right_hyphen_min: i32 = 52;
pub const INT_PAR__holding_inserts: i32 = 53;
pub const INT_PAR__error_context_lines: i32 = 54;
pub const INT_PAR__tracing_stack_levels: i32 = 55;
pub const INT_PAR__tracing_assigns: i32 = 56;
pub const INT_PAR__tracing_groups: i32 = 57;
pub const INT_PAR__tracing_ifs: i32 = 58;
pub const INT_PAR__tracing_scan_tokens: i32 = 59;
pub const INT_PAR__tracing_nesting: i32 = 60;
pub const INT_PAR__pre_display_direction: i32 = 61;
pub const INT_PAR__last_line_fit: i32 = 62;
pub const INT_PAR__saving_vdiscards: i32 = 63;
pub const INT_PAR__saving_hyph_codes: i32 = 64;
pub const INT_PAR__suppress_fontnotfound_error: i32 = 65;
pub const INT_PAR__xetex_linebreak_locale: i32 = 66;
pub const INT_PAR__xetex_linebreak_penalty: i32 = 67;
pub const INT_PAR__xetex_protrude_chars: i32 = 68;
pub const INT_PAR__texxet: i32 = 69;
pub const INT_PAR__xetex_dash_break: i32 = 70;
pub const INT_PAR__xetex_upwards: i32 = 71;
pub const INT_PAR__xetex_use_glyph_metrics: i32 = 72;
pub const INT_PAR__xetex_inter_char_tokens: i32 = 73;
pub const INT_PAR__xetex_input_normalization: i32 = 74;
pub const INT_PAR__xetex_default_input_mode: i32 = 75;
pub const INT_PAR__xetex_default_input_encoding: i32 = 76;
pub const INT_PAR__xetex_tracing_fonts: i32 = 77;
pub const INT_PAR__xetex_interword_space_shaping: i32 = 78;
pub const INT_PAR__xetex_generate_actual_text: i32 = 79;
pub const INT_PAR__xetex_hyphenatable_length: i32 = 80;
pub const INT_PAR__synctex: i32 = 81;
pub const INT_PAR__pdfoutput: i32 = 82;
pub const INT_PARS: i32 = 83;

pub const DIMEN_PAR__par_indent: i32 = 0;
pub const DIMEN_PAR__math_surround: i32 = 1;
pub const DIMEN_PAR__line_skip_limit: i32 = 2;
pub const DIMEN_PAR__hsize: i32 = 3;
pub const DIMEN_PAR__vsize: i32 = 4;
pub const DIMEN_PAR__max_depth: i32 = 5;
pub const DIMEN_PAR__split_max_depth: i32 = 6;
pub const DIMEN_PAR__box_max_depth: i32 = 7;
pub const DIMEN_PAR__hfuzz: i32 = 8;
pub const DIMEN_PAR__vfuzz: i32 = 9;
pub const DIMEN_PAR__delimiter_shortfall: i32 = 10;
pub const DIMEN_PAR__null_delimiter_space: i32 = 11;
pub const DIMEN_PAR__script_space: i32 = 12;
pub const DIMEN_PAR__pre_display_size: i32 = 13;
pub const DIMEN_PAR__display_width: i32 = 14;
pub const DIMEN_PAR__display_indent: i32 = 15;
pub const DIMEN_PAR__overfull_rule: i32 = 16;
pub const DIMEN_PAR__hang_indent: i32 = 17;
pub const DIMEN_PAR__h_offset: i32 = 18;
pub const DIMEN_PAR__v_offset: i32 = 19;
pub const DIMEN_PAR__emergency_stretch: i32 = 20;
pub const DIMEN_PAR__pdf_page_width: i32 = 21;
pub const DIMEN_PAR__pdf_page_height: i32 = 22;
pub const DIMEN_PARS: i32 = 23;

pub const GLUE_PAR__line_skip: i32 = 0;
pub const GLUE_PAR__baseline_skip: i32 = 1;
pub const GLUE_PAR__par_skip: i32 = 2;
pub const GLUE_PAR__above_display_skip: i32 = 3;
pub const GLUE_PAR__below_display_skip: i32 = 4;
pub const GLUE_PAR__above_display_short_skip: i32 = 5;
pub const GLUE_PAR__below_display_short_skip: i32 = 6;
pub const GLUE_PAR__left_skip: i32 = 7;
pub const GLUE_PAR__right_skip: i32 = 8;
pub const GLUE_PAR__top_skip: i32 = 9;
pub const GLUE_PAR__split_top_skip: i32 = 10;
pub const GLUE_PAR__tab_skip: i32 = 11;
pub const GLUE_PAR__space_skip: i32 = 12;
pub const GLUE_PAR__xspace_skip: i32 = 13;
pub const GLUE_PAR__par_fill_skip: i32 = 14;
pub const GLUE_PAR__xetex_linebreak_skip: i32 = 15;
pub const GLUE_PAR__thin_mu_skip: i32 = 16;
pub const GLUE_PAR__med_mu_skip: i32 = 17;
pub const GLUE_PAR__thick_mu_skip: i32 = 18;
pub const GLUE_PARS: i32 = 19;

pub const LOCAL__par_shape: i32 = 0;
pub const LOCAL__output_routine: i32 = 1;
pub const LOCAL__every_par: i32 = 2;
pub const LOCAL__every_math: i32 = 3;
pub const LOCAL__every_display: i32 = 4;
pub const LOCAL__every_hbox: i32 = 5;
pub const LOCAL__every_vbox: i32 = 6;
pub const LOCAL__every_job: i32 = 7;
pub const LOCAL__every_cr: i32 = 8;
pub const LOCAL__err_help: i32 = 9;
pub const LOCAL__every_eof: i32 = 10;
pub const LOCAL__xetex_inter_char_toks: i32 = 11;
pub const LOCAL__tectonic_coda_tokens: i32 = 12;
pub const NUM_LOCALS: i32 = 13;

pub const ETEX_PENALTIES_PAR__inter_line_penalties: i32 = 0;
pub const ETEX_PENALTIES_PAR__club_penalties: i32 = 1;
pub const ETEX_PENALTIES_PAR__widow_penalties: i32 = 2;
pub const ETEX_PENALTIES_PAR__display_widow_penalties: i32 = 3;
pub const NUM_ETEX_PENALTIES: i32 = 4;

pub const ACTIVE_BASE: i32 = 1;
pub const SINGLE_BASE: i32 = 1114113;
pub const NULL_CS: i32 = 2228225;
pub const HASH_BASE: i32 = 2228226;
pub const FROZEN_CONTROL_SEQUENCE: i32 = 2243226;
pub const PRIM_EQTB_BASE: i32 = 2243238;
pub const FROZEN_NULL_FONT: i32 = 2245338;
pub const UNDEFINED_CONTROL_SEQUENCE: i32 = 2254339;
pub const GLUE_BASE: i32 = 2254340;
pub const SKIP_BASE: i32 = 2254359;
pub const MU_SKIP_BASE: i32 = 2254615;
pub const LOCAL_BASE: i32 = 2254871;
pub const TOKS_BASE: i32 = 2254884;
pub const ETEX_PEN_BASE: i32 = 2255140;
pub const BOX_BASE: i32 = 2255144;
pub const CUR_FONT_LOC: i32 = 2255400;
pub const MATH_FONT_BASE: i32 = 2255401;
pub const CAT_CODE_BASE: i32 = 2256169;
pub const LC_CODE_BASE: i32 = 3370281;
pub const UC_CODE_BASE: i32 = 4484393;
pub const SF_CODE_BASE: i32 = 5598505;
pub const MATH_CODE_BASE: i32 = 6712617;
pub const INT_BASE: i32 = 7826729;
pub const COUNT_BASE: i32 = 7826812;
pub const DEL_CODE_BASE: i32 = 7827068;
pub const DIMEN_BASE: i32 = 8941180;
pub const SCALED_BASE: i32 = 8941203;
pub const EQTB_SIZE: i32 = 8941458;
pub const EQTB_TOP: i32 = 9541458;

pub const FROZEN_PROTECTION: i32 = FROZEN_CONTROL_SEQUENCE + 0;
pub const FROZEN_CR: i32 = FROZEN_CONTROL_SEQUENCE + 1;
pub const FROZEN_END_GROUP: i32 = FROZEN_CONTROL_SEQUENCE + 2;
pub const FROZEN_RIGHT: i32 = FROZEN_CONTROL_SEQUENCE + 3;
pub const FROZEN_FI: i32 = FROZEN_CONTROL_SEQUENCE + 4;
pub const FROZEN_END_TEMPLATE: i32 = FROZEN_CONTROL_SEQUENCE + 5;
pub const FROZEN_ENDV: i32 = FROZEN_CONTROL_SEQUENCE + 6;
pub const FROZEN_RELAX: i32 = FROZEN_CONTROL_SEQUENCE + 7;
pub const END_WRITE: i32 = FROZEN_CONTROL_SEQUENCE + 8;
pub const FROZEN_DONT_EXPAND: i32 = FROZEN_CONTROL_SEQUENCE + 9;
pub const FROZEN_SPECIAL: i32 = FROZEN_CONTROL_SEQUENCE + 10;
pub const FROZEN_PRIMITIVE: i32 = FROZEN_CONTROL_SEQUENCE + 11;

pub const RELAX: i32 = 0;
pub const ESCAPE: i32 = 0;
pub const LEFT_BRACE: i32 = 1;
pub const RIGHT_BRACE: i32 = 2;
pub const MATH_SHIFT: i32 = 3;
pub const TAB_MARK: i32 = 4;
pub const CAR_RET: i32 = 5;
pub const OUT_PARAM: i32 = 5;
pub const MAC_PARAM: i32 = 6;
pub const SUP_MARK: i32 = 7;
pub const SUB_MARK: i32 = 8;
pub const ENDV: i32 = 9;
pub const IGNORE: i32 = 9;
pub const SPACER: i32 = 10;
pub const LETTER: i32 = 11;
pub const OTHER_CHAR: i32 = 12;
pub const PAR_END: i32 = 13;
pub const ACTIVE_CHAR: i32 = 13;
pub const MATCH: i32 = 13;
pub const STOP: i32 = 14;
pub const COMMENT: i32 = 14;
pub const END_MATCH: i32 = 14;
pub const DELIM_NUM: i32 = 15;
pub const INVALID_CHAR: i32 = 15;
pub const CHAR_NUM: i32 = 16;
pub const MAX_CHAR_CODE: i32 = 15;
pub const MATH_CHAR_NUM: i32 = 17;
pub const MARK: i32 = 18;
pub const XRAY: i32 = 19;
pub const MAKE_BOX: i32 = 20;
pub const HMOVE: i32 = 21;
pub const VMOVE: i32 = 22;
pub const UN_HBOX: i32 = 23;
pub const UN_VBOX: i32 = 24;
pub const REMOVE_ITEM: i32 = 25;
pub const HSKIP: i32 = 26;
pub const VSKIP: i32 = 27;
pub const MSKIP: i32 = 28;
pub const KERN: i32 = 29;
pub const MKERN: i32 = 30;
pub const LEADER_SHIP: i32 = 31;
pub const HALIGN: i32 = 32;
pub const VALIGN: i32 = 33;
pub const NO_ALIGN: i32 = 34;
pub const VRULE: i32 = 35;
pub const HRULE: i32 = 36;
pub const INSERT: i32 = 37;
pub const VADJUST: i32 = 38;
pub const IGNORE_SPACES: i32 = 39;
pub const AFTER_ASSIGNMENT: i32 = 40;
pub const AFTER_GROUP: i32 = 41;
pub const BREAK_PENALTY: i32 = 42;
pub const START_PAR: i32 = 43;
pub const ITAL_CORR: i32 = 44;
pub const ACCENT: i32 = 45;
pub const MATH_ACCENT: i32 = 46;
pub const DISCRETIONARY: i32 = 47;
pub const EQ_NO: i32 = 48;
pub const LEFT_RIGHT: i32 = 49;
pub const MATH_COMP: i32 = 50;
pub const LIMIT_SWITCH: i32 = 51;
pub const ABOVE: i32 = 52;
pub const MATH_STYLE: i32 = 53;
pub const MATH_CHOICE: i32 = 54;
pub const NON_SCRIPT: i32 = 55;
pub const VCENTER: i32 = 56;
pub const CASE_SHIFT: i32 = 57;
pub const MESSAGE: i32 = 58;
pub const EXTENSION: i32 = 59;
pub const IN_STREAM: i32 = 60;
pub const BEGIN_GROUP: i32 = 61;
pub const END_GROUP: i32 = 62;
pub const OMIT: i32 = 63;
pub const EX_SPACE: i32 = 64;
pub const NO_BOUNDARY: i32 = 65;
pub const RADICAL: i32 = 66;
pub const END_CS_NAME: i32 = 67;
pub const CHAR_GIVEN: i32 = 68;
pub const MIN_INTERNAL: i32 = 68;
pub const MATH_GIVEN: i32 = 69;
pub const XETEX_MATH_GIVEN: i32 = 70;
pub const LAST_ITEM: i32 = 71;
pub const TOKS_REGISTER: i32 = 72;
pub const MAX_NON_PREFIXED_COMMAND: i32 = 71;
pub const ASSIGN_TOKS: i32 = 73;
pub const ASSIGN_INT: i32 = 74;
pub const ASSIGN_DIMEN: i32 = 75;
pub const ASSIGN_GLUE: i32 = 76;
pub const ASSIGN_MU_GLUE: i32 = 77;
pub const ASSIGN_FONT_DIMEN: i32 = 78;
pub const ASSIGN_FONT_INT: i32 = 79;
pub const SET_AUX: i32 = 80;
pub const SET_PREV_GRAF: i32 = 81;
pub const SET_PAGE_DIMEN: i32 = 82;
pub const SET_PAGE_INT: i32 = 83;
pub const SET_BOX_DIMEN: i32 = 84;
pub const SET_SHAPE: i32 = 85;
pub const DEF_CODE: i32 = 86;
pub const XETEX_DEF_CODE: i32 = 87;
pub const DEF_FAMILY: i32 = 88;
pub const SET_FONT: i32 = 89;
pub const DEF_FONT: i32 = 90;
pub const REGISTER: i32 = 91;
pub const ADVANCE: i32 = 92;
pub const MAX_INTERNAL: i32 = 91;
pub const MULTIPLY: i32 = 93;
pub const DIVIDE: i32 = 94;
pub const PREFIX: i32 = 95;
pub const LET: i32 = 96;
pub const SHORTHAND_DEF: i32 = 97;
pub const READ_TO_CS: i32 = 98;
pub const DEF: i32 = 99;
pub const SET_BOX: i32 = 100;
pub const HYPH_DATA: i32 = 101;
pub const SET_INTERACTION: i32 = 102;
pub const UNDEFINED_CS: i32 = 103;
pub const MAX_COMMAND: i32 = 102;
pub const EXPAND_AFTER: i32 = 104;
pub const NO_EXPAND: i32 = 105;
pub const INPUT: i32 = 106;
pub const IF_TEST: i32 = 107;
pub const FI_OR_ELSE: i32 = 108;
pub const CS_NAME: i32 = 109;
pub const CONVERT: i32 = 110;
pub const THE: i32 = 111;
pub const TOP_BOT_MARK: i32 = 112;
pub const CALL: i32 = 113;
pub const LONG_CALL: i32 = 114;
pub const OUTER_CALL: i32 = 115;
pub const LONG_OUTER_CALL: i32 = 116;
pub const END_TEMPLATE: i32 = 117;
pub const DONT_EXPAND: i32 = 118;
pub const GLUE_REF: i32 = 119;
pub const SHAPE_REF: i32 = 120;
pub const BOX_REF: i32 = 121;
pub const DATA: i32 = 122;

pub const VMODE: i32 = 1;
pub const HMODE: i32 = 104;
pub const MMODE: i32 = 207;

pub const TEXT_SIZE: i32 = 0;
pub const SCRIPT_SIZE: i32 = 256;
pub const SCRIPT_SCRIPT_SIZE: i32 = 512;

pub const HLIST_NODE: i32 = 0;
pub const VLIST_NODE: i32 = 1;
pub const RULE_NODE: i32 = 2;
pub const INS_NODE: i32 = 3;
pub const MARK_NODE: i32 = 4;
pub const ADJUST_NODE: i32 = 5;
pub const LIGATURE_NODE: i32 = 6;
pub const DISC_NODE: i32 = 7;
pub const WHATSIT_NODE: i32 = 8;
pub const MATH_NODE: i32 = 9;
pub const GLUE_NODE: i32 = 10;
pub const KERN_NODE: i32 = 11;
pub const PENALTY_NODE: i32 = 12;
pub const UNSET_NODE: i32 = 13;
pub const STYLE_NODE: i32 = 14;
pub const CHOICE_NODE: i32 = 15;
pub const MARGIN_KERN_NODE: i32 = 40;

pub const TT_LEFT_RIGHT_MIDDLE_MODE: i32 = 1;
pub const ORD_NOAD: i32 = 16;
pub const OP_NOAD: i32 = 17;
pub const BIN_NOAD: i32 = 18;
pub const REL_NOAD: i32 = 19;
pub const OPEN_NOAD: i32 = 20;
pub const CLOSE_NOAD: i32 = 21;
pub const PUNCT_NOAD: i32 = 22;
pub const INNER_NOAD: i32 = 23;
pub const RADICAL_NOAD: i32 = 24;
pub const FRACTION_NOAD: i32 = 25;
pub const UNDER_NOAD: i32 = 26;
pub const OVER_NOAD: i32 = 27;
pub const ACCENT_NOAD: i32 = 28;
pub const VCENTER_NOAD: i32 = 29;
pub const LEFT_NOAD: i32 = 30;
pub const RIGHT_NOAD: i32 = 31;

pub const NORMAL: i32 = 0;
pub const MU_GLUE: i32 = 99;
pub const A_LEADERS: i32 = 100;
pub const C_LEADERS: i32 = 101;
pub const X_LEADERS: i32 = 102;

pub const EXPLICIT: i32 = 1;
pub const ACC_KERN: i32 = 2;
pub const SPACE_ADJUSTMENT: i32 = 3;

pub const BEFORE: i32 = 0;
pub const AFTER: i32 = 1;
pub const BEGIN_M_CODE: i32 = 2;
pub const END_M_CODE: i32 = 3;
pub const BEGIN_L_CODE: i32 = 6;
pub const END_L_CODE: i32 = 7;
pub const BEGIN_R_CODE: i32 = 10;
pub const END_R_CODE: i32 = 11;

pub const DISPLAY_STYLE: i32 = 0;
pub const TEXT_STYLE: i32 = 2;
pub const SCRIPT_STYLE: i32 = 4;
pub const SCRIPT_SCRIPT_STYLE: i32 = 6;

pub const LIMITS: i32 = 1;
pub const NO_LIMITS: i32 = 2;

pub const OPEN_NODE: i32 = 0;
pub const WRITE_NODE: i32 = 1;
pub const CLOSE_NODE: i32 = 2;
pub const SPECIAL_NODE: i32 = 3;
pub const LANGUAGE_NODE: i32 = 4;
pub const PDF_SAVE_POS_NODE: i32 = 21;
pub const NATIVE_WORD_NODE: i32 = 40;
pub const NATIVE_WORD_NODE_AT: i32 = 41;
pub const GLYPH_NODE: i32 = 42;
pub const PIC_NODE: i32 = 43;
pub const PDF_NODE: i32 = 44;

pub const ABOVE_CODE: i32 = 0;
pub const OVER_CODE: i32 = 1;
pub const ATOP_CODE: i32 = 2;
pub const TT_ABOVE_WITH_DELIMS: i32 = 3;
pub const TT_OVER_WITH_DELIMS: i32 = 4;
pub const TT_ATOP_WITH_DELIMS: i32 = 5;

pub const BOX_CODE: i32 = 0;
pub const COPY_CODE: i32 = 1;
pub const LAST_BOX_CODE: i32 = 2;
pub const VSPLIT_CODE: i32 = 3;
pub const VTOP_CODE: i32 = 4;
pub const TT_VBOX_CODE: i32 = 5;
pub const TT_HBOX_CODE: i32 = 108;

pub const NUMBER_CODE: i32 = 0;
pub const ROMAN_NUMERAL_CODE: i32 = 1;
pub const STRING_CODE: i32 = 2;
pub const MEANING_CODE: i32 = 3;
pub const FONT_NAME_CODE: i32 = 4;
pub const ETEX_CONVERT_BASE: i32 = 5;
pub const ETEX_REVISION_CODE: i32 = 5;
pub const ETEX_CONVERT_CODES: i32 = 6;
pub const EXPANDED_CODE: i32 = 6;
pub const PDFTEX_FIRST_EXPAND_CODE: i32 = 7;
pub const LEFT_MARGIN_KERN_CODE: i32 = 16;
pub const RIGHT_MARGIN_KERN_CODE: i32 = 17;
pub const PDF_STRCMP_CODE: i32 = 18;
pub const PDF_CREATION_DATE_CODE: i32 = 22;
pub const PDF_FILE_MOD_DATE_CODE: i32 = 23;
pub const PDF_FILE_SIZE_CODE: i32 = 24;
pub const PDF_MDFIVE_SUM_CODE: i32 = 25;
pub const PDF_FILE_DUMP_CODE: i32 = 26;
pub const UNIFORM_DEVIATE_CODE: i32 = 29;
pub const NORMAL_DEVIATE_CODE: i32 = 30;
pub const PDFTEX_CONVERT_CODES: i32 = 33;
pub const XETEX_FIRST_EXPAND_CODE: i32 = 33;
pub const XETEX_REVISION_CODE: i32 = 33;
pub const XETEX_VARIATION_NAME_CODE: i32 = 34;
pub const XETEX_FEATURE_NAME_CODE: i32 = 35;
pub const XETEX_SELECTOR_NAME_CODE: i32 = 36;
pub const XETEX_GLYPH_NAME_CODE: i32 = 37;
pub const XETEX_UCHAR_CODE: i32 = 38;
pub const XETEX_UCHARCAT_CODE: i32 = 39;
pub const JOB_NAME_CODE: i32 = 40;
pub const XETEX_CONVERT_CODES: i32 = 40;

pub const IMMEDIATE_CODE: i32 = 4;
pub const SET_LANGUAGE_CODE: i32 = 5;
pub const RESET_TIMER_CODE: i32 = 31;
pub const SET_RANDOM_SEED_CODE: i32 = 33;
pub const PIC_FILE_CODE: i32 = 41;
pub const PDF_FILE_CODE: i32 = 42;
pub const GLYPH_CODE: i32 = 43;
pub const XETEX_INPUT_ENCODING_EXTENSION_CODE: i32 = 44;
pub const XETEX_DEFAULT_ENCODING_EXTENSION_CODE: i32 = 45;
pub const XETEX_LINEBREAK_LOCALE_EXTENSION_CODE: i32 = 46;

pub const FI_CODE: i32 = 2;
pub const ELSE_CODE: i32 = 3;
pub const OR_CODE: i32 = 4;

pub const IF_CHAR_CODE: i32 = 0;
pub const IF_CAT_CODE: i32 = 1;
pub const IF_INT_CODE: i32 = 2;
pub const IF_DIM_CODE: i32 = 3;
pub const IF_ODD_CODE: i32 = 4;
pub const IF_VMODE_CODE: i32 = 5;
pub const IF_HMODE_CODE: i32 = 6;
pub const IF_MMODE_CODE: i32 = 7;
pub const IF_INNER_CODE: i32 = 8;
pub const IF_VOID_CODE: i32 = 9;
pub const IF_HBOX_CODE: i32 = 10;
pub const IF_VBOX_CODE: i32 = 11;
pub const IFX_CODE: i32 = 12;
pub const IF_EOF_CODE: i32 = 13;
pub const IF_TRUE_CODE: i32 = 14;
pub const IF_FALSE_CODE: i32 = 15;
pub const IF_CASE_CODE: i32 = 16;
pub const IF_DEF_CODE: i32 = 17;
pub const IF_CS_CODE: i32 = 18;
pub const IF_FONT_CHAR_CODE: i32 = 19;
pub const IF_IN_CSNAME_CODE: i32 = 20;
pub const IF_PRIMITIVE_CODE: i32 = 21;

pub const INT_VAL: i32 = 0;
pub const DIMEN_VAL: i32 = 1;
pub const GLUE_VAL: i32 = 2;
pub const LAST_NODE_TYPE_CODE: i32 = 3;
pub const INPUT_LINE_NO_CODE: i32 = 4;
pub const BADNESS_CODE: i32 = 5;
pub const PDFTEX_FIRST_RINT_CODE: i32 = 6;
pub const PDF_LAST_X_POS_CODE: i32 = 12;
pub const PDF_LAST_Y_POS_CODE: i32 = 13;
pub const ELAPSED_TIME_CODE: i32 = 16;
pub const PDF_SHELL_ESCAPE_CODE: i32 = 17;
pub const RANDOM_SEED_CODE: i32 = 18;
pub const ETEX_INT: i32 = 19;
pub const ETEX_VERSION_CODE: i32 = 19;
pub const CURRENT_GROUP_LEVEL_CODE: i32 = 20;
pub const CURRENT_GROUP_TYPE_CODE: i32 = 21;
pub const CURRENT_IF_LEVEL_CODE: i32 = 22;
pub const CURRENT_IF_TYPE_CODE: i32 = 23;
pub const CURRENT_IF_BRANCH_CODE: i32 = 24;
pub const GLUE_STRETCH_ORDER_CODE: i32 = 25;
pub const GLUE_SHRINK_ORDER_CODE: i32 = 26;
pub const XETEX_INT: i32 = 27;
pub const XETEX_VERSION_CODE: i32 = 27;
pub const XETEX_COUNT_GLYPHS_CODE: i32 = 28;
pub const XETEX_COUNT_VARIATIONS_CODE: i32 = 29;
pub const XETEX_VARIATION_CODE: i32 = 30;
pub const XETEX_FIND_VARIATION_BY_NAME_CODE: i32 = 31;
pub const XETEX_VARIATION_MIN_CODE: i32 = 32;
pub const XETEX_VARIATION_MAX_CODE: i32 = 33;
pub const XETEX_VARIATION_DEFAULT_CODE: i32 = 34;
pub const XETEX_COUNT_FEATURES_CODE: i32 = 35;
pub const XETEX_FEATURE_CODE_CODE: i32 = 36;
pub const XETEX_FIND_FEATURE_BY_NAME_CODE: i32 = 37;
pub const XETEX_IS_EXCLUSIVE_FEATURE_CODE: i32 = 38;
pub const XETEX_COUNT_SELECTORS_CODE: i32 = 39;
pub const XETEX_SELECTOR_CODE_CODE: i32 = 40;
pub const XETEX_FIND_SELECTOR_BY_NAME_CODE: i32 = 41;
pub const XETEX_IS_DEFAULT_SELECTOR_CODE: i32 = 42;
pub const XETEX_OT_COUNT_SCRIPTS_CODE: i32 = 43;
pub const XETEX_OT_COUNT_LANGUAGES_CODE: i32 = 44;
pub const XETEX_OT_COUNT_FEATURES_CODE: i32 = 45;
pub const XETEX_OT_SCRIPT_CODE: i32 = 46;
pub const XETEX_OT_LANGUAGE_CODE: i32 = 47;
pub const XETEX_OT_FEATURE_CODE: i32 = 48;
pub const XETEX_MAP_CHAR_TO_GLYPH_CODE: i32 = 49;
pub const XETEX_GLYPH_INDEX_CODE: i32 = 50;
pub const XETEX_FONT_TYPE_CODE: i32 = 51;
pub const XETEX_FIRST_CHAR_CODE: i32 = 52;
pub const XETEX_LAST_CHAR_CODE: i32 = 53;
pub const XETEX_PDF_PAGE_COUNT_CODE: i32 = 54;
pub const XETEX_LAST_ITEM_CODES: i32 = 54;
pub const XETEX_DIM: i32 = 55;
pub const XETEX_GLYPH_BOUNDS_CODE: i32 = 55;
pub const XETEX_LAST_DIM_CODES: i32 = 55;
pub const ETEX_DIM: i32 = 56;
pub const FONT_CHAR_WD_CODE: i32 = 56;
pub const FONT_CHAR_HT_CODE: i32 = 57;
pub const FONT_CHAR_DP_CODE: i32 = 58;
pub const FONT_CHAR_IC_CODE: i32 = 59;
pub const PAR_SHAPE_LENGTH_CODE: i32 = 60;
pub const PAR_SHAPE_INDENT_CODE: i32 = 61;
pub const PAR_SHAPE_DIMEN_CODE: i32 = 62;
pub const GLUE_STRETCH_CODE: i32 = 63;
pub const GLUE_SHRINK_CODE: i32 = 64;
pub const ETEX_GLUE: i32 = 65;
pub const MU_TO_GLUE_CODE: i32 = 65;
pub const ETEX_MU: i32 = 66;
pub const GLUE_TO_MU_CODE: i32 = 66;
pub const ETEX_EXPR: i32 = 67;
pub const TT_ETEX_NUM_EXPR_CODE: i32 = 67;
pub const TT_ETEX_DIM_EXPR_CODE: i32 = 68;
pub const TT_ETEX_GLUE_EXPR_CODE: i32 = 69;
pub const TT_ETEX_MU_EXPR_CODE: i32 = 70;

pub const BATCH_MODE: i32 = 0;
pub const NONSTOP_MODE: i32 = 1;
pub const SCROLL_MODE: i32 = 2;
pub const ERROR_STOP_MODE: i32 = 3;

pub const WIDTH_OFFSET: i32 = 1;
pub const DEPTH_OFFSET: i32 = 2;
pub const HEIGHT_OFFSET: i32 = 3;

pub const CHAR_DEF_CODE: i32 = 0;
pub const MATH_CHAR_DEF_CODE: i32 = 1;
pub const COUNT_DEF_CODE: i32 = 2;
pub const DIMEN_DEF_CODE: i32 = 3;
pub const SKIP_DEF_CODE: i32 = 4;
pub const MU_SKIP_DEF_CODE: i32 = 5;
pub const TOKS_DEF_CODE: i32 = 6;
pub const CHAR_SUB_DEF_CODE: i32 = 7;
pub const XETEX_MATH_CHAR_NUM_DEF_CODE: i32 = 8;
pub const XETEX_MATH_CHAR_DEF_CODE: i32 = 9;

pub const FIL_CODE: i32 = 0;
pub const FILL_CODE: i32 = 1;
pub const SS_CODE: i32 = 2;
pub const FIL_NEG_CODE: i32 = 3;
pub const SKIP_CODE: i32 = 4;
pub const MSKIP_CODE: i32 = 5;

pub const SPAN_CODE: i32 = 1114113;
pub const CR_CODE: i32 = 1114114;
pub const CR_CR_CODE: i32 = 1114115;

pub const TOP_MARK_CODE: i32 = 0;
pub const FIRST_MARK_CODE: i32 = 1;
pub const BOT_MARK_CODE: i32 = 2;
pub const SPLIT_FIRST_MARK_CODE: i32 = 3;
pub const SPLIT_BOT_MARK_CODE: i32 = 4;
pub const TT_TOP_MARKS_CODE: i32 = 5;
pub const TT_FIRST_MARKS_CODE: i32 = 6;
pub const TT_BOT_MARKS_CODE: i32 = 7;
pub const TT_SPLIT_FIRST_MARKS_CODE: i32 = 8;
pub const TT_SPLIT_BOT_MARKS_CODE: i32 = 9;

pub const SHOW_CODE: i32 = 0;
pub const SHOW_BOX_CODE: i32 = 1;
pub const SHOW_THE_CODE: i32 = 2;
pub const SHOW_LISTS: i32 = 3;
pub const SHOW_GROUPS: i32 = 4;
pub const SHOW_TOKENS: i32 = 5;
pub const SHOW_IFS: i32 = 6;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum xetex_format_primitive_extra_init_t {
    xf_prim_init_none = 0,
    xf_prim_init_par = 1,
    xf_prim_init_write = 2,
}

pub use xetex_format_primitive_extra_init_t::*;

pub const xf_prim_init_none_i32: i32 = 0;
pub const xf_prim_init_par_i32: i32 = 1;
pub const xf_prim_init_write_i32: i32 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct xetex_format_primitive_def_t {
    pub name: Option<&'static std::ffi::CStr>,
    pub cmd: u8,
    pub chr: i32,
    pub extra_init: i32,
}

pub static XETEX_FORMAT_PRIMITIVE_INITIALIZERS: &[xetex_format_primitive_def_t] = &[
    xetex_format_primitive_def_t {
        name: Some(c"relax"),
        cmd: RELAX as u8,
        chr: TOO_BIG_USV,
        extra_init: FROZEN_RELAX,
    },
    xetex_format_primitive_def_t {
        name: Some(c"span"),
        cmd: TAB_MARK as u8,
        chr: SPAN_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"cr"),
        cmd: CAR_RET as u8,
        chr: CR_CODE,
        extra_init: FROZEN_CR,
    },
    xetex_format_primitive_def_t {
        name: Some(c"crcr"),
        cmd: CAR_RET as u8,
        chr: CR_CR_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"par"),
        cmd: PAR_END as u8,
        chr: TOO_BIG_USV,
        extra_init: xf_prim_init_par_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"end"),
        cmd: STOP as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"dump"),
        cmd: STOP as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"delimiter"),
        cmd: DELIM_NUM as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"Udelimiter"),
        cmd: DELIM_NUM as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXdelimiter"),
        cmd: DELIM_NUM as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"char"),
        cmd: CHAR_NUM as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mathchar"),
        cmd: MATH_CHAR_NUM as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"Umathcharnum"),
        cmd: MATH_CHAR_NUM as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXmathcharnum"),
        cmd: MATH_CHAR_NUM as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"Umathchar"),
        cmd: MATH_CHAR_NUM as u8,
        chr: 2,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXmathchar"),
        cmd: MATH_CHAR_NUM as u8,
        chr: 2,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mark"),
        cmd: MARK as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"marks"),
        cmd: MARK as u8,
        chr: 5,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"show"),
        cmd: XRAY as u8,
        chr: SHOW_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"showbox"),
        cmd: XRAY as u8,
        chr: SHOW_BOX_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"showthe"),
        cmd: XRAY as u8,
        chr: SHOW_THE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"showlists"),
        cmd: XRAY as u8,
        chr: SHOW_LISTS,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"showgroups"),
        cmd: XRAY as u8,
        chr: SHOW_GROUPS,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"showtokens"),
        cmd: XRAY as u8,
        chr: SHOW_TOKENS,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"showifs"),
        cmd: XRAY as u8,
        chr: SHOW_IFS,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"box"),
        cmd: MAKE_BOX as u8,
        chr: BOX_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"copy"),
        cmd: MAKE_BOX as u8,
        chr: COPY_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"lastbox"),
        cmd: MAKE_BOX as u8,
        chr: LAST_BOX_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"vsplit"),
        cmd: MAKE_BOX as u8,
        chr: VSPLIT_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"vtop"),
        cmd: MAKE_BOX as u8,
        chr: VTOP_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"vbox"),
        cmd: MAKE_BOX as u8,
        chr: TT_VBOX_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hbox"),
        cmd: MAKE_BOX as u8,
        chr: TT_HBOX_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"moveright"),
        cmd: HMOVE as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"moveleft"),
        cmd: HMOVE as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"lower"),
        cmd: VMOVE as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"raise"),
        cmd: VMOVE as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"unhbox"),
        cmd: UN_HBOX as u8,
        chr: BOX_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"unhcopy"),
        cmd: UN_HBOX as u8,
        chr: COPY_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"unvbox"),
        cmd: UN_VBOX as u8,
        chr: BOX_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"unvcopy"),
        cmd: UN_VBOX as u8,
        chr: COPY_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pagediscards"),
        cmd: UN_VBOX as u8,
        chr: LAST_BOX_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"splitdiscards"),
        cmd: UN_VBOX as u8,
        chr: VSPLIT_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"unskip"),
        cmd: REMOVE_ITEM as u8,
        chr: GLUE_NODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"unkern"),
        cmd: REMOVE_ITEM as u8,
        chr: KERN_NODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"unpenalty"),
        cmd: REMOVE_ITEM as u8,
        chr: PENALTY_NODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hfil"),
        cmd: HSKIP as u8,
        chr: FIL_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hfill"),
        cmd: HSKIP as u8,
        chr: FILL_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hss"),
        cmd: HSKIP as u8,
        chr: SS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hfilneg"),
        cmd: HSKIP as u8,
        chr: FIL_NEG_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hskip"),
        cmd: HSKIP as u8,
        chr: SKIP_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"vfil"),
        cmd: VSKIP as u8,
        chr: FIL_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"vfill"),
        cmd: VSKIP as u8,
        chr: FILL_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"vss"),
        cmd: VSKIP as u8,
        chr: SS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"vfilneg"),
        cmd: VSKIP as u8,
        chr: FIL_NEG_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"vskip"),
        cmd: VSKIP as u8,
        chr: SKIP_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mskip"),
        cmd: MSKIP as u8,
        chr: MSKIP_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"kern"),
        cmd: KERN as u8,
        chr: EXPLICIT,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mkern"),
        cmd: MKERN as u8,
        chr: MU_GLUE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"shipout"),
        cmd: LEADER_SHIP as u8,
        chr: MU_GLUE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"leaders"),
        cmd: LEADER_SHIP as u8,
        chr: A_LEADERS,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"cleaders"),
        cmd: LEADER_SHIP as u8,
        chr: C_LEADERS,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"xleaders"),
        cmd: LEADER_SHIP as u8,
        chr: X_LEADERS,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"halign"),
        cmd: HALIGN as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"valign"),
        cmd: VALIGN as u8,
        chr: BEFORE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"beginL"),
        cmd: VALIGN as u8,
        chr: BEGIN_L_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"endL"),
        cmd: VALIGN as u8,
        chr: END_L_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"beginR"),
        cmd: VALIGN as u8,
        chr: BEGIN_R_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"endR"),
        cmd: VALIGN as u8,
        chr: END_R_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"noalign"),
        cmd: NO_ALIGN as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"vrule"),
        cmd: VRULE as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hrule"),
        cmd: HRULE as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"insert"),
        cmd: INSERT as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"vadjust"),
        cmd: VADJUST as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ignorespaces"),
        cmd: IGNORE_SPACES as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"afterassignment"),
        cmd: AFTER_ASSIGNMENT as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"aftergroup"),
        cmd: AFTER_GROUP as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"penalty"),
        cmd: BREAK_PENALTY as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"noindent"),
        cmd: START_PAR as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"indent"),
        cmd: START_PAR as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"/"),
        cmd: ITAL_CORR as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"accent"),
        cmd: ACCENT as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mathaccent"),
        cmd: MATH_ACCENT as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"Umathaccent"),
        cmd: MATH_ACCENT as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXmathaccent"),
        cmd: MATH_ACCENT as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"discretionary"),
        cmd: DISCRETIONARY as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"-"),
        cmd: DISCRETIONARY as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"eqno"),
        cmd: EQ_NO as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"leqno"),
        cmd: EQ_NO as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"middle"),
        cmd: LEFT_RIGHT as u8,
        chr: TT_LEFT_RIGHT_MIDDLE_MODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"left"),
        cmd: LEFT_RIGHT as u8,
        chr: LEFT_NOAD,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"right"),
        cmd: LEFT_RIGHT as u8,
        chr: RIGHT_NOAD,
        extra_init: FROZEN_RIGHT,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mathord"),
        cmd: MATH_COMP as u8,
        chr: ORD_NOAD,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mathop"),
        cmd: MATH_COMP as u8,
        chr: OP_NOAD,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mathbin"),
        cmd: MATH_COMP as u8,
        chr: BIN_NOAD,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mathrel"),
        cmd: MATH_COMP as u8,
        chr: REL_NOAD,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mathopen"),
        cmd: MATH_COMP as u8,
        chr: OPEN_NOAD,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mathclose"),
        cmd: MATH_COMP as u8,
        chr: CLOSE_NOAD,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mathpunct"),
        cmd: MATH_COMP as u8,
        chr: PUNCT_NOAD,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mathinner"),
        cmd: MATH_COMP as u8,
        chr: INNER_NOAD,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"underline"),
        cmd: MATH_COMP as u8,
        chr: UNDER_NOAD,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"overline"),
        cmd: MATH_COMP as u8,
        chr: OVER_NOAD,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"displaylimits"),
        cmd: LIMIT_SWITCH as u8,
        chr: NORMAL,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"limits"),
        cmd: LIMIT_SWITCH as u8,
        chr: LIMITS,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"nolimits"),
        cmd: LIMIT_SWITCH as u8,
        chr: NO_LIMITS,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"above"),
        cmd: ABOVE as u8,
        chr: ABOVE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"over"),
        cmd: ABOVE as u8,
        chr: OVER_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"atop"),
        cmd: ABOVE as u8,
        chr: ATOP_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"abovewithdelims"),
        cmd: ABOVE as u8,
        chr: TT_ABOVE_WITH_DELIMS,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"overwithdelims"),
        cmd: ABOVE as u8,
        chr: TT_OVER_WITH_DELIMS,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"atopwithdelims"),
        cmd: ABOVE as u8,
        chr: TT_ATOP_WITH_DELIMS,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"displaystyle"),
        cmd: MATH_STYLE as u8,
        chr: DISPLAY_STYLE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"textstyle"),
        cmd: MATH_STYLE as u8,
        chr: TEXT_STYLE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"scriptstyle"),
        cmd: MATH_STYLE as u8,
        chr: SCRIPT_STYLE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"scriptscriptstyle"),
        cmd: MATH_STYLE as u8,
        chr: SCRIPT_SCRIPT_STYLE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mathchoice"),
        cmd: MATH_CHOICE as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"nonscript"),
        cmd: NON_SCRIPT as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"vcenter"),
        cmd: VCENTER as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"lowercase"),
        cmd: CASE_SHIFT as u8,
        chr: LC_CODE_BASE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"uppercase"),
        cmd: CASE_SHIFT as u8,
        chr: UC_CODE_BASE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"message"),
        cmd: MESSAGE as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"errmessage"),
        cmd: MESSAGE as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"openout"),
        cmd: EXTENSION as u8,
        chr: OPEN_NODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"write"),
        cmd: EXTENSION as u8,
        chr: WRITE_NODE,
        extra_init: xf_prim_init_write_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"closeout"),
        cmd: EXTENSION as u8,
        chr: CLOSE_NODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"special"),
        cmd: EXTENSION as u8,
        chr: SPECIAL_NODE,
        extra_init: FROZEN_SPECIAL,
    },
    xetex_format_primitive_def_t {
        name: Some(c"immediate"),
        cmd: EXTENSION as u8,
        chr: IMMEDIATE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"setlanguage"),
        cmd: EXTENSION as u8,
        chr: SET_LANGUAGE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pdfsavepos"),
        cmd: EXTENSION as u8,
        chr: PDF_SAVE_POS_NODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"resettimer"),
        cmd: EXTENSION as u8,
        chr: RESET_TIMER_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"setrandomseed"),
        cmd: EXTENSION as u8,
        chr: SET_RANDOM_SEED_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXpicfile"),
        cmd: EXTENSION as u8,
        chr: PIC_FILE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXpdffile"),
        cmd: EXTENSION as u8,
        chr: PDF_FILE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXglyph"),
        cmd: EXTENSION as u8,
        chr: GLYPH_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXinputencoding"),
        cmd: EXTENSION as u8,
        chr: XETEX_INPUT_ENCODING_EXTENSION_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXdefaultencoding"),
        cmd: EXTENSION as u8,
        chr: XETEX_DEFAULT_ENCODING_EXTENSION_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXlinebreaklocale"),
        cmd: EXTENSION as u8,
        chr: XETEX_LINEBREAK_LOCALE_EXTENSION_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"closein"),
        cmd: IN_STREAM as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"openin"),
        cmd: IN_STREAM as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"begingroup"),
        cmd: BEGIN_GROUP as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"endgroup"),
        cmd: END_GROUP as u8,
        chr: 0,
        extra_init: FROZEN_END_GROUP,
    },
    xetex_format_primitive_def_t {
        name: Some(c"omit"),
        cmd: OMIT as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c" "),
        cmd: EX_SPACE as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"noboundary"),
        cmd: NO_BOUNDARY as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"radical"),
        cmd: RADICAL as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"Uradical"),
        cmd: RADICAL as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXradical"),
        cmd: RADICAL as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"endcsname"),
        cmd: END_CS_NAME as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"lastpenalty"),
        cmd: LAST_ITEM as u8,
        chr: INT_VAL,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"lastkern"),
        cmd: LAST_ITEM as u8,
        chr: DIMEN_VAL,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"lastskip"),
        cmd: LAST_ITEM as u8,
        chr: GLUE_VAL,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"lastnodetype"),
        cmd: LAST_ITEM as u8,
        chr: LAST_NODE_TYPE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"inputlineno"),
        cmd: LAST_ITEM as u8,
        chr: INPUT_LINE_NO_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"badness"),
        cmd: LAST_ITEM as u8,
        chr: BADNESS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pdflastxpos"),
        cmd: LAST_ITEM as u8,
        chr: PDF_LAST_X_POS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pdflastypos"),
        cmd: LAST_ITEM as u8,
        chr: PDF_LAST_Y_POS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"elapsedtime"),
        cmd: LAST_ITEM as u8,
        chr: ELAPSED_TIME_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"shellescape"),
        cmd: LAST_ITEM as u8,
        chr: PDF_SHELL_ESCAPE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"randomseed"),
        cmd: LAST_ITEM as u8,
        chr: RANDOM_SEED_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"eTeXversion"),
        cmd: LAST_ITEM as u8,
        chr: ETEX_VERSION_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"currentgrouplevel"),
        cmd: LAST_ITEM as u8,
        chr: CURRENT_GROUP_LEVEL_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"currentgrouptype"),
        cmd: LAST_ITEM as u8,
        chr: CURRENT_GROUP_TYPE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"currentiflevel"),
        cmd: LAST_ITEM as u8,
        chr: CURRENT_IF_LEVEL_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"currentiftype"),
        cmd: LAST_ITEM as u8,
        chr: CURRENT_IF_TYPE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"currentifbranch"),
        cmd: LAST_ITEM as u8,
        chr: CURRENT_IF_BRANCH_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"gluestretchorder"),
        cmd: LAST_ITEM as u8,
        chr: GLUE_STRETCH_ORDER_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"glueshrinkorder"),
        cmd: LAST_ITEM as u8,
        chr: GLUE_SHRINK_ORDER_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXversion"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_VERSION_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXcountglyphs"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_COUNT_GLYPHS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXcountvariations"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_COUNT_VARIATIONS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXvariation"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_VARIATION_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXfindvariationbyname"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_FIND_VARIATION_BY_NAME_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXvariationmin"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_VARIATION_MIN_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXvariationmax"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_VARIATION_MAX_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXvariationdefault"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_VARIATION_DEFAULT_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXcountfeatures"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_COUNT_FEATURES_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXfeaturecode"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_FEATURE_CODE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXfindfeaturebyname"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_FIND_FEATURE_BY_NAME_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXisexclusivefeature"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_IS_EXCLUSIVE_FEATURE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXcountselectors"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_COUNT_SELECTORS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXselectorcode"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_SELECTOR_CODE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXfindselectorbyname"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_FIND_SELECTOR_BY_NAME_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXisdefaultselector"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_IS_DEFAULT_SELECTOR_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXOTcountscripts"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_OT_COUNT_SCRIPTS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXOTcountlanguages"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_OT_COUNT_LANGUAGES_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXOTcountfeatures"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_OT_COUNT_FEATURES_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXOTscripttag"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_OT_SCRIPT_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXOTlanguagetag"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_OT_LANGUAGE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXOTfeaturetag"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_OT_FEATURE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXcharglyph"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_MAP_CHAR_TO_GLYPH_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXglyphindex"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_GLYPH_INDEX_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXfonttype"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_FONT_TYPE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXfirstfontchar"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_FIRST_CHAR_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXlastfontchar"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_LAST_CHAR_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXpdfpagecount"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_PDF_PAGE_COUNT_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXglyphbounds"),
        cmd: LAST_ITEM as u8,
        chr: XETEX_GLYPH_BOUNDS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"fontcharwd"),
        cmd: LAST_ITEM as u8,
        chr: FONT_CHAR_WD_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"fontcharht"),
        cmd: LAST_ITEM as u8,
        chr: FONT_CHAR_HT_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"fontchardp"),
        cmd: LAST_ITEM as u8,
        chr: FONT_CHAR_DP_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"fontcharic"),
        cmd: LAST_ITEM as u8,
        chr: FONT_CHAR_IC_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"parshapelength"),
        cmd: LAST_ITEM as u8,
        chr: PAR_SHAPE_LENGTH_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"parshapeindent"),
        cmd: LAST_ITEM as u8,
        chr: PAR_SHAPE_INDENT_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"parshapedimen"),
        cmd: LAST_ITEM as u8,
        chr: PAR_SHAPE_DIMEN_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"gluestretch"),
        cmd: LAST_ITEM as u8,
        chr: GLUE_STRETCH_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"glueshrink"),
        cmd: LAST_ITEM as u8,
        chr: GLUE_SHRINK_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mutoglue"),
        cmd: LAST_ITEM as u8,
        chr: MU_TO_GLUE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"gluetomu"),
        cmd: LAST_ITEM as u8,
        chr: GLUE_TO_MU_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"numexpr"),
        cmd: LAST_ITEM as u8,
        chr: TT_ETEX_NUM_EXPR_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"dimexpr"),
        cmd: LAST_ITEM as u8,
        chr: TT_ETEX_DIM_EXPR_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"glueexpr"),
        cmd: LAST_ITEM as u8,
        chr: TT_ETEX_GLUE_EXPR_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"muexpr"),
        cmd: LAST_ITEM as u8,
        chr: TT_ETEX_MU_EXPR_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"toks"),
        cmd: TOKS_REGISTER as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"fontdimen"),
        cmd: ASSIGN_FONT_DIMEN as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hyphenchar"),
        cmd: ASSIGN_FONT_INT as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"skewchar"),
        cmd: ASSIGN_FONT_INT as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"lpcode"),
        cmd: ASSIGN_FONT_INT as u8,
        chr: 2,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"rpcode"),
        cmd: ASSIGN_FONT_INT as u8,
        chr: 3,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"prevdepth"),
        cmd: SET_AUX as u8,
        chr: VMODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"spacefactor"),
        cmd: SET_AUX as u8,
        chr: HMODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"prevgraf"),
        cmd: SET_PREV_GRAF as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pagegoal"),
        cmd: SET_PAGE_DIMEN as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pagetotal"),
        cmd: SET_PAGE_DIMEN as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pagestretch"),
        cmd: SET_PAGE_DIMEN as u8,
        chr: 2,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pagefilstretch"),
        cmd: SET_PAGE_DIMEN as u8,
        chr: 3,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pagefillstretch"),
        cmd: SET_PAGE_DIMEN as u8,
        chr: 4,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pagefilllstretch"),
        cmd: SET_PAGE_DIMEN as u8,
        chr: 5,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pageshrink"),
        cmd: SET_PAGE_DIMEN as u8,
        chr: 6,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pagedepth"),
        cmd: SET_PAGE_DIMEN as u8,
        chr: 7,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"deadcycles"),
        cmd: SET_PAGE_INT as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"insertpenalties"),
        cmd: SET_PAGE_INT as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"interactionmode"),
        cmd: SET_PAGE_INT as u8,
        chr: 2,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"wd"),
        cmd: SET_BOX_DIMEN as u8,
        chr: WIDTH_OFFSET,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"dp"),
        cmd: SET_BOX_DIMEN as u8,
        chr: DEPTH_OFFSET,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ht"),
        cmd: SET_BOX_DIMEN as u8,
        chr: HEIGHT_OFFSET,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"catcode"),
        cmd: DEF_CODE as u8,
        chr: CAT_CODE_BASE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"lccode"),
        cmd: DEF_CODE as u8,
        chr: LC_CODE_BASE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"uccode"),
        cmd: DEF_CODE as u8,
        chr: UC_CODE_BASE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"sfcode"),
        cmd: DEF_CODE as u8,
        chr: SF_CODE_BASE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mathcode"),
        cmd: DEF_CODE as u8,
        chr: MATH_CODE_BASE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"delcode"),
        cmd: DEF_CODE as u8,
        chr: DEL_CODE_BASE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXcharclass"),
        cmd: XETEX_DEF_CODE as u8,
        chr: SF_CODE_BASE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"Umathcodenum"),
        cmd: XETEX_DEF_CODE as u8,
        chr: MATH_CODE_BASE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXmathcodenum"),
        cmd: XETEX_DEF_CODE as u8,
        chr: MATH_CODE_BASE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"Umathcode"),
        cmd: XETEX_DEF_CODE as u8,
        chr: 6712618,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXmathcode"),
        cmd: XETEX_DEF_CODE as u8,
        chr: 6712618,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"Udelcodenum"),
        cmd: XETEX_DEF_CODE as u8,
        chr: DEL_CODE_BASE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXdelcodenum"),
        cmd: XETEX_DEF_CODE as u8,
        chr: DEL_CODE_BASE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"Udelcode"),
        cmd: XETEX_DEF_CODE as u8,
        chr: 7827069,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXdelcode"),
        cmd: XETEX_DEF_CODE as u8,
        chr: 7827069,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"textfont"),
        cmd: DEF_FAMILY as u8,
        chr: 2255401,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"scriptfont"),
        cmd: DEF_FAMILY as u8,
        chr: 2255657,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"scriptscriptfont"),
        cmd: DEF_FAMILY as u8,
        chr: 2255913,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"nullfont"),
        cmd: SET_FONT as u8,
        chr: 0,
        extra_init: FROZEN_NULL_FONT,
    },
    xetex_format_primitive_def_t {
        name: Some(c"font"),
        cmd: DEF_FONT as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"count"),
        cmd: REGISTER as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"dimen"),
        cmd: REGISTER as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"skip"),
        cmd: REGISTER as u8,
        chr: 2,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"muskip"),
        cmd: REGISTER as u8,
        chr: 3,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"advance"),
        cmd: ADVANCE as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"multiply"),
        cmd: MULTIPLY as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"divide"),
        cmd: DIVIDE as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"long"),
        cmd: PREFIX as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"outer"),
        cmd: PREFIX as u8,
        chr: 2,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"global"),
        cmd: PREFIX as u8,
        chr: 4,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"protected"),
        cmd: PREFIX as u8,
        chr: 8,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"let"),
        cmd: LET as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"futurelet"),
        cmd: LET as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"chardef"),
        cmd: SHORTHAND_DEF as u8,
        chr: CHAR_DEF_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mathchardef"),
        cmd: SHORTHAND_DEF as u8,
        chr: MATH_CHAR_DEF_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"countdef"),
        cmd: SHORTHAND_DEF as u8,
        chr: COUNT_DEF_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"dimendef"),
        cmd: SHORTHAND_DEF as u8,
        chr: DIMEN_DEF_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"skipdef"),
        cmd: SHORTHAND_DEF as u8,
        chr: SKIP_DEF_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"muskipdef"),
        cmd: SHORTHAND_DEF as u8,
        chr: MU_SKIP_DEF_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"toksdef"),
        cmd: SHORTHAND_DEF as u8,
        chr: TOKS_DEF_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"Umathcharnumdef"),
        cmd: SHORTHAND_DEF as u8,
        chr: XETEX_MATH_CHAR_NUM_DEF_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXmathcharnumdef"),
        cmd: SHORTHAND_DEF as u8,
        chr: XETEX_MATH_CHAR_NUM_DEF_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"Umathchardef"),
        cmd: SHORTHAND_DEF as u8,
        chr: XETEX_MATH_CHAR_DEF_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXmathchardef"),
        cmd: SHORTHAND_DEF as u8,
        chr: XETEX_MATH_CHAR_DEF_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"read"),
        cmd: READ_TO_CS as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"readline"),
        cmd: READ_TO_CS as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"def"),
        cmd: DEF as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"gdef"),
        cmd: DEF as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"edef"),
        cmd: DEF as u8,
        chr: 2,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"xdef"),
        cmd: DEF as u8,
        chr: 3,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"setbox"),
        cmd: SET_BOX as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hyphenation"),
        cmd: HYPH_DATA as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"patterns"),
        cmd: HYPH_DATA as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"batchmode"),
        cmd: SET_INTERACTION as u8,
        chr: BATCH_MODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"nonstopmode"),
        cmd: SET_INTERACTION as u8,
        chr: NONSTOP_MODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"scrollmode"),
        cmd: SET_INTERACTION as u8,
        chr: SCROLL_MODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"errorstopmode"),
        cmd: SET_INTERACTION as u8,
        chr: ERROR_STOP_MODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"expandafter"),
        cmd: EXPAND_AFTER as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"unless"),
        cmd: EXPAND_AFTER as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"noexpand"),
        cmd: NO_EXPAND as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"primitive"),
        cmd: NO_EXPAND as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"input"),
        cmd: INPUT as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"endinput"),
        cmd: INPUT as u8,
        chr: 1,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"scantokens"),
        cmd: INPUT as u8,
        chr: 2,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"if"),
        cmd: IF_TEST as u8,
        chr: IF_CHAR_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifcat"),
        cmd: IF_TEST as u8,
        chr: IF_CAT_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifnum"),
        cmd: IF_TEST as u8,
        chr: IF_INT_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifdim"),
        cmd: IF_TEST as u8,
        chr: IF_DIM_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifodd"),
        cmd: IF_TEST as u8,
        chr: IF_ODD_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifvmode"),
        cmd: IF_TEST as u8,
        chr: IF_VMODE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifhmode"),
        cmd: IF_TEST as u8,
        chr: IF_HMODE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifmmode"),
        cmd: IF_TEST as u8,
        chr: IF_MMODE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifinner"),
        cmd: IF_TEST as u8,
        chr: IF_INNER_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifvoid"),
        cmd: IF_TEST as u8,
        chr: IF_VOID_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifhbox"),
        cmd: IF_TEST as u8,
        chr: IF_HBOX_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifvbox"),
        cmd: IF_TEST as u8,
        chr: IF_VBOX_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifx"),
        cmd: IF_TEST as u8,
        chr: IFX_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifeof"),
        cmd: IF_TEST as u8,
        chr: IF_EOF_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"iftrue"),
        cmd: IF_TEST as u8,
        chr: IF_TRUE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"iffalse"),
        cmd: IF_TEST as u8,
        chr: IF_FALSE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifcase"),
        cmd: IF_TEST as u8,
        chr: IF_CASE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifdefined"),
        cmd: IF_TEST as u8,
        chr: IF_DEF_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifcsname"),
        cmd: IF_TEST as u8,
        chr: IF_CS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"iffontchar"),
        cmd: IF_TEST as u8,
        chr: IF_FONT_CHAR_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifincsname"),
        cmd: IF_TEST as u8,
        chr: IF_IN_CSNAME_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"ifprimitive"),
        cmd: IF_TEST as u8,
        chr: IF_PRIMITIVE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"fi"),
        cmd: FI_OR_ELSE as u8,
        chr: FI_CODE,
        extra_init: FROZEN_FI,
    },
    xetex_format_primitive_def_t {
        name: Some(c"else"),
        cmd: FI_OR_ELSE as u8,
        chr: ELSE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"or"),
        cmd: FI_OR_ELSE as u8,
        chr: OR_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"csname"),
        cmd: CS_NAME as u8,
        chr: 0,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"number"),
        cmd: CONVERT as u8,
        chr: NUMBER_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"romannumeral"),
        cmd: CONVERT as u8,
        chr: ROMAN_NUMERAL_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"string"),
        cmd: CONVERT as u8,
        chr: STRING_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"meaning"),
        cmd: CONVERT as u8,
        chr: MEANING_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"fontname"),
        cmd: CONVERT as u8,
        chr: FONT_NAME_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"eTeXrevision"),
        cmd: CONVERT as u8,
        chr: ETEX_REVISION_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"expanded"),
        cmd: CONVERT as u8,
        chr: EXPANDED_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"leftmarginkern"),
        cmd: CONVERT as u8,
        chr: LEFT_MARGIN_KERN_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"rightmarginkern"),
        cmd: CONVERT as u8,
        chr: RIGHT_MARGIN_KERN_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"strcmp"),
        cmd: CONVERT as u8,
        chr: PDF_STRCMP_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"creationdate"),
        cmd: CONVERT as u8,
        chr: PDF_CREATION_DATE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"filemoddate"),
        cmd: CONVERT as u8,
        chr: PDF_FILE_MOD_DATE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"filesize"),
        cmd: CONVERT as u8,
        chr: PDF_FILE_SIZE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mdfivesum"),
        cmd: CONVERT as u8,
        chr: PDF_MDFIVE_SUM_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"filedump"),
        cmd: CONVERT as u8,
        chr: PDF_FILE_DUMP_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"uniformdeviate"),
        cmd: CONVERT as u8,
        chr: UNIFORM_DEVIATE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"normaldeviate"),
        cmd: CONVERT as u8,
        chr: NORMAL_DEVIATE_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXrevision"),
        cmd: CONVERT as u8,
        chr: XETEX_REVISION_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXvariationname"),
        cmd: CONVERT as u8,
        chr: XETEX_VARIATION_NAME_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXfeaturename"),
        cmd: CONVERT as u8,
        chr: XETEX_FEATURE_NAME_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXselectorname"),
        cmd: CONVERT as u8,
        chr: XETEX_SELECTOR_NAME_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXglyphname"),
        cmd: CONVERT as u8,
        chr: XETEX_GLYPH_NAME_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"Uchar"),
        cmd: CONVERT as u8,
        chr: XETEX_UCHAR_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"Ucharcat"),
        cmd: CONVERT as u8,
        chr: XETEX_UCHARCAT_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"jobname"),
        cmd: CONVERT as u8,
        chr: JOB_NAME_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"the"),
        cmd: THE as u8,
        chr: SHOW_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"unexpanded"),
        cmd: THE as u8,
        chr: SHOW_BOX_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"detokenize"),
        cmd: THE as u8,
        chr: SHOW_TOKENS,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"topmark"),
        cmd: TOP_BOT_MARK as u8,
        chr: TOP_MARK_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"firstmark"),
        cmd: TOP_BOT_MARK as u8,
        chr: FIRST_MARK_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"botmark"),
        cmd: TOP_BOT_MARK as u8,
        chr: BOT_MARK_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"splitfirstmark"),
        cmd: TOP_BOT_MARK as u8,
        chr: SPLIT_FIRST_MARK_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"splitbotmark"),
        cmd: TOP_BOT_MARK as u8,
        chr: SPLIT_BOT_MARK_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"topmarks"),
        cmd: TOP_BOT_MARK as u8,
        chr: TT_TOP_MARKS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"firstmarks"),
        cmd: TOP_BOT_MARK as u8,
        chr: TT_FIRST_MARKS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"botmarks"),
        cmd: TOP_BOT_MARK as u8,
        chr: TT_BOT_MARKS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"splitfirstmarks"),
        cmd: TOP_BOT_MARK as u8,
        chr: TT_SPLIT_FIRST_MARKS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"splitbotmarks"),
        cmd: TOP_BOT_MARK as u8,
        chr: TT_SPLIT_BOT_MARKS_CODE,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pretolerance"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__pretolerance,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tolerance"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tolerance,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"linepenalty"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__line_penalty,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hyphenpenalty"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__hyphen_penalty,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"exhyphenpenalty"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__ex_hyphen_penalty,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"clubpenalty"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__club_penalty,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"widowpenalty"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__widow_penalty,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"displaywidowpenalty"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__display_widow_penalty,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"brokenpenalty"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__broken_penalty,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"binoppenalty"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__bin_op_penalty,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"relpenalty"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__rel_penalty,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"predisplaypenalty"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__pre_display_penalty,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"postdisplaypenalty"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__post_display_penalty,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"interlinepenalty"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__inter_line_penalty,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"doublehyphendemerits"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__double_hyphen_demerits,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"finalhyphendemerits"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__final_hyphen_demerits,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"adjdemerits"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__adj_demerits,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mag"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__mag,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"delimiterfactor"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__delimiter_factor,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"looseness"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__looseness,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"time"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__time,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"day"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__day,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"month"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__month,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"year"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__year,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"showboxbreadth"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__show_box_breadth,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"showboxdepth"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__show_box_depth,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hbadness"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__hbadness,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"vbadness"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__vbadness,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pausing"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__pausing,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tracingonline"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tracing_online,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tracingmacros"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tracing_macros,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tracingstats"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tracing_stats,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tracingparagraphs"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tracing_paragraphs,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tracingpages"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tracing_pages,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tracingoutput"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tracing_output,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tracinglostchars"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tracing_lost_chars,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tracingcommands"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tracing_commands,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tracingrestores"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tracing_restores,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"uchyph"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__uc_hyph,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"outputpenalty"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__output_penalty,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"maxdeadcycles"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__max_dead_cycles,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hangafter"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__hang_after,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"floatingpenalty"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__floating_penalty,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"globaldefs"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__global_defs,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"fam"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__cur_fam,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"escapechar"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__escape_char,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"defaulthyphenchar"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__default_hyphen_char,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"defaultskewchar"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__default_skew_char,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"endlinechar"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__end_line_char,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"newlinechar"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__new_line_char,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"language"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__language,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"lefthyphenmin"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__left_hyphen_min,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"righthyphenmin"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__right_hyphen_min,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"holdinginserts"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__holding_inserts,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"errorcontextlines"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__error_context_lines,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tracingstacklevels"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tracing_stack_levels,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tracingassigns"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tracing_assigns,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tracinggroups"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tracing_groups,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tracingifs"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tracing_ifs,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tracingscantokens"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tracing_scan_tokens,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tracingnesting"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__tracing_nesting,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"predisplaydirection"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__pre_display_direction,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"lastlinefit"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__last_line_fit,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"savingvdiscards"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__saving_vdiscards,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"savinghyphcodes"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__saving_hyph_codes,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"suppressfontnotfounderror"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__suppress_fontnotfound_error,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXlinebreakpenalty"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__xetex_linebreak_penalty,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXprotrudechars"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__xetex_protrude_chars,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"TeXXeTstate"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__texxet,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXdashbreakstate"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__xetex_dash_break,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXupwardsmode"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__xetex_upwards,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXuseglyphmetrics"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__xetex_use_glyph_metrics,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXinterchartokenstate"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__xetex_inter_char_tokens,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXinputnormalization"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__xetex_input_normalization,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXtracingfonts"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__xetex_tracing_fonts,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXinterwordspaceshaping"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__xetex_interword_space_shaping,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXgenerateactualtext"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__xetex_generate_actual_text,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXhyphenatablelength"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__xetex_hyphenatable_length,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"synctex"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__synctex,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pdfoutput"),
        cmd: ASSIGN_INT as u8,
        chr: INT_BASE + INT_PAR__pdfoutput,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"parindent"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__par_indent,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"mathsurround"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__math_surround,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"lineskiplimit"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__line_skip_limit,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hsize"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__hsize,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"vsize"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__vsize,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"maxdepth"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__max_depth,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"splitmaxdepth"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__split_max_depth,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"boxmaxdepth"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__box_max_depth,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hfuzz"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__hfuzz,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"vfuzz"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__vfuzz,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"delimitershortfall"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__delimiter_shortfall,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"nulldelimiterspace"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__null_delimiter_space,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"scriptspace"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__script_space,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"predisplaysize"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__pre_display_size,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"displaywidth"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__display_width,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"displayindent"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__display_indent,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"overfullrule"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__overfull_rule,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hangindent"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__hang_indent,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"hoffset"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__h_offset,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"voffset"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__v_offset,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"emergencystretch"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__emergency_stretch,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pdfpagewidth"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__pdf_page_width,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"pdfpageheight"),
        cmd: ASSIGN_DIMEN as u8,
        chr: DIMEN_BASE + DIMEN_PAR__pdf_page_height,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"lineskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__line_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"baselineskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__baseline_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"parskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__par_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"abovedisplayskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__above_display_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"belowdisplayskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__below_display_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"abovedisplayshortskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__above_display_short_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"belowdisplayshortskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__below_display_short_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"leftskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__left_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"rightskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__right_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"topskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__top_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"splittopskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__split_top_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"tabskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__tab_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"spaceskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__space_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"xspaceskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__xspace_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"parfillskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__par_fill_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXlinebreakskip"),
        cmd: ASSIGN_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__xetex_linebreak_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"thinmuskip"),
        cmd: ASSIGN_MU_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__thin_mu_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"medmuskip"),
        cmd: ASSIGN_MU_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__med_mu_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"thickmuskip"),
        cmd: ASSIGN_MU_GLUE as u8,
        chr: GLUE_BASE + GLUE_PAR__thick_mu_skip,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"parshape"),
        cmd: SET_SHAPE as u8,
        chr: LOCAL_BASE + LOCAL__par_shape,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"output"),
        cmd: ASSIGN_TOKS as u8,
        chr: LOCAL_BASE + LOCAL__output_routine,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"everypar"),
        cmd: ASSIGN_TOKS as u8,
        chr: LOCAL_BASE + LOCAL__every_par,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"everymath"),
        cmd: ASSIGN_TOKS as u8,
        chr: LOCAL_BASE + LOCAL__every_math,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"everydisplay"),
        cmd: ASSIGN_TOKS as u8,
        chr: LOCAL_BASE + LOCAL__every_display,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"everyhbox"),
        cmd: ASSIGN_TOKS as u8,
        chr: LOCAL_BASE + LOCAL__every_hbox,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"everyvbox"),
        cmd: ASSIGN_TOKS as u8,
        chr: LOCAL_BASE + LOCAL__every_vbox,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"everyjob"),
        cmd: ASSIGN_TOKS as u8,
        chr: LOCAL_BASE + LOCAL__every_job,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"everycr"),
        cmd: ASSIGN_TOKS as u8,
        chr: LOCAL_BASE + LOCAL__every_cr,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"errhelp"),
        cmd: ASSIGN_TOKS as u8,
        chr: LOCAL_BASE + LOCAL__err_help,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"everyeof"),
        cmd: ASSIGN_TOKS as u8,
        chr: LOCAL_BASE + LOCAL__every_eof,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"XeTeXinterchartoks"),
        cmd: ASSIGN_TOKS as u8,
        chr: LOCAL_BASE + LOCAL__xetex_inter_char_toks,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"TectonicCodaTokens"),
        cmd: ASSIGN_TOKS as u8,
        chr: LOCAL_BASE + LOCAL__tectonic_coda_tokens,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"interlinepenalties"),
        cmd: SET_SHAPE as u8,
        chr: ETEX_PEN_BASE + ETEX_PENALTIES_PAR__inter_line_penalties,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"clubpenalties"),
        cmd: SET_SHAPE as u8,
        chr: ETEX_PEN_BASE + ETEX_PENALTIES_PAR__club_penalties,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"widowpenalties"),
        cmd: SET_SHAPE as u8,
        chr: ETEX_PEN_BASE + ETEX_PENALTIES_PAR__widow_penalties,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: Some(c"displaywidowpenalties"),
        cmd: SET_SHAPE as u8,
        chr: ETEX_PEN_BASE + ETEX_PENALTIES_PAR__display_widow_penalties,
        extra_init: xf_prim_init_none_i32,
    },
    xetex_format_primitive_def_t {
        name: None,
        cmd: 0,
        chr: 0,
        extra_init: 0,
    },
];
