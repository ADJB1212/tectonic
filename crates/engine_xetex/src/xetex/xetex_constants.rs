#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports
)]

pub use crate::xetex::xetex_format::*;

pub const MIN_HALFWORD: i32 = -0x0FFFFFFF;
pub const MAX_HALFWORD: i32 = 0x3FFFFFFF;

pub const TEX_NULL: i32 = MIN_HALFWORD;
pub const TEX_INFINITY: i32 = 0x7FFFFFFF;
pub const NULL_FLAG: i32 = -0x40000000;
pub const DEFAULT_CODE: i32 = 0x40000000;

pub const BIGGEST_CHAR: i32 = 0xFFFF;
pub const BIGGEST_USV: i32 = 0x10FFFF;
pub const NUMBER_USVS: i32 = BIGGEST_USV + 1;

pub const NUMBER_MATH_FONTS: i32 = 3 * NUMBER_MATH_FAMILIES;
pub const NUMBER_REGS: i32 = 256;

pub const PAGE_INS_HEAD: i32 = MEM_TOP;
pub const CONTRIB_HEAD: i32 = MEM_TOP - 1;
pub const PAGE_HEAD: i32 = MEM_TOP - 2;
pub const TEMP_HEAD: i32 = MEM_TOP - 3;
pub const HOLD_HEAD: i32 = MEM_TOP - 4;
pub const ADJUST_HEAD: i32 = MEM_TOP - 5;
pub const ACTIVE_LIST: i32 = MEM_TOP - 7;
pub const ALIGN_HEAD: i32 = MEM_TOP - 8;
pub const END_SPAN: i32 = MEM_TOP - 9;
pub const OMIT_TEMPLATE: i32 = MEM_TOP - 10;
pub const NULL_LIST: i32 = MEM_TOP - 11;
pub const LIG_TRICK: i32 = MEM_TOP - 12;
pub const GARBAGE: i32 = MEM_TOP - 12;
pub const BACKUP_HEAD: i32 = MEM_TOP - 13;
pub const PRE_ADJUST_HEAD: i32 = MEM_TOP - 14;

pub const FONT_ID_BASE: i32 = FROZEN_NULL_FONT;

#[macro_export]
macro_rules! GLUEPAR {
    ($p:ident) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::GLUE_BASE + $crate::xetex::xetex_format::$p) as isize))
            .b32
            .s1
    };
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::GLUE_BASE + ($p as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! SKIP_REG {
    ($n:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::SKIP_BASE + ($n as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! MU_SKIP_REG {
    ($n:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::MU_SKIP_BASE + ($n as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! LOCAL {
    ($p:ident) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::LOCAL_BASE + $crate::xetex::xetex_format::$p) as isize))
            .b32
            .s1
    };
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::LOCAL_BASE + ($p as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! TOKS_REG {
    ($n:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::TOKS_BASE + ($n as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! ETEX_PENALTIES_PAR {
    ($p:ident) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::ETEX_PEN_BASE + $crate::xetex::xetex_format::$p) as isize))
            .b32
            .s1
    };
    ($p:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::ETEX_PEN_BASE + ($p as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! BOX_REG {
    ($n:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::BOX_BASE + ($n as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! MATH_FONT {
    ($n:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::MATH_FONT_BASE + ($n as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! CAT_CODE {
    ($n:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::CAT_CODE_BASE + ($n as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! LC_CODE {
    ($n:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::LC_CODE_BASE + ($n as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! UC_CODE {
    ($n:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::UC_CODE_BASE + ($n as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! SF_CODE {
    ($n:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::SF_CODE_BASE + ($n as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! MATH_CODE {
    ($n:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::MATH_CODE_BASE + ($n as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! INTPAR {
    ($n:ident) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::INT_BASE + $crate::xetex::xetex_format::$n) as isize))
            .b32
            .s1
    };
    ($n:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::INT_BASE + ($n as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! COUNT_REG {
    ($n:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::COUNT_BASE + ($n as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! DEL_CODE {
    ($n:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::DEL_CODE_BASE + ($n as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! DIMENPAR {
    ($n:ident) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::DIMEN_BASE + $crate::xetex::xetex_format::$n) as isize))
            .b32
            .s1
    };
    ($n:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::DIMEN_BASE + ($n as i32)) as isize))
            .b32
            .s1
    };
}

#[macro_export]
macro_rules! SCALED_REG {
    ($n:expr) => {
        (*$crate::xetex::xetex_xetexd::eqtb.offset(($crate::xetex::xetex_format::SCALED_BASE + ($n as i32)) as isize))
            .b32
            .s1
    };
}

pub const LEVEL_ZERO: i32 = 0;
pub const LEVEL_ONE: i32 = 1;

pub const UNSPECIFIED_MODE: i32 = 4;

pub const LEFT_TO_RIGHT: i32 = 0;
pub const RIGHT_TO_LEFT: i32 = 1;

pub const SYNCTEX_FIELD_SIZE: i32 = 1;

pub const DELTA_NODE: i32 = RULE_NODE;
pub const EDGE_NODE: i32 = STYLE_NODE;

pub const IF_NODE_SIZE: i32 = 2;
pub const PASSIVE_NODE_SIZE: i32 = 2;
pub const POINTER_NODE_SIZE: i32 = 2;
pub const SMALL_NODE_SIZE: i32 = 2;
pub const SPAN_NODE_SIZE: i32 = 2;
pub const WRITE_NODE_SIZE: i32 = 2;
pub const ACTIVE_NODE_SIZE_NORMAL: i32 = 3;
pub const EDGE_NODE_SIZE: i32 = 3;
pub const MARGIN_KERN_NODE_SIZE: i32 = 3;
pub const MEDIUM_NODE_SIZE: i32 = 3;
pub const MOVEMENT_NODE_SIZE: i32 = 3;
pub const OPEN_NODE_SIZE: i32 = 3;
pub const STYLE_NODE_SIZE: i32 = 3;
pub const WORD_NODE_SIZE: i32 = 3;
pub const EXPR_NODE_SIZE: i32 = 4;
pub const GLUE_SPEC_SIZE: i32 = 4;
pub const MARK_CLASS_NODE_SIZE: i32 = 4;
pub const PAGE_INS_NODE_SIZE: i32 = 4;
pub const ACTIVE_NODE_SIZE_EXTENDED: i32 = 5;
pub const GLYPH_NODE_SIZE: i32 = 5;
pub const INS_NODE_SIZE: i32 = 5;
pub const RULE_NODE_SIZE: i32 = 5;
pub const ALIGN_STACK_NODE_SIZE: i32 = 6;
pub const NATIVE_NODE_SIZE: i32 = 6;
pub const DELTA_NODE_SIZE: i32 = 7;
pub const BOX_NODE_SIZE: i32 = 8;
pub const PIC_NODE_SIZE: i32 = 9;
pub const INDEX_NODE_SIZE: i32 = 33;

pub const NOAD_SIZE: i32 = 4;
pub const ACCENT_NOAD_SIZE: i32 = 5;
pub const RADICAL_NOAD_SIZE: i32 = 5;
pub const FRACTION_NOAD_SIZE: i32 = 6;

pub const L_CODE: i32 = 4;
pub const R_CODE: i32 = 8;

pub const EXPR_NONE: i32 = 0;
pub const EXPR_ADD: i32 = 1;
pub const EXPR_SUB: i32 = 2;
pub const EXPR_MULT: i32 = 3;
pub const EXPR_DIV: i32 = 4;
pub const EXPR_SCALE: i32 = 5;

pub const BOTTOM_LEVEL: i32 = 0;
pub const SIMPLE_GROUP: i32 = 1;
pub const HBOX_GROUP: i32 = 2;
pub const ADJUSTED_HBOX_GROUP: i32 = 3;
pub const VBOX_GROUP: i32 = 4;
pub const VTOP_GROUP: i32 = 5;
pub const ALIGN_GROUP: i32 = 6;
pub const NO_ALIGN_GROUP: i32 = 7;
pub const OUTPUT_GROUP: i32 = 8;
pub const MATH_GROUP: i32 = 9;
pub const DISC_GROUP: i32 = 10;
pub const INSERT_GROUP: i32 = 11;
pub const VCENTER_GROUP: i32 = 12;
pub const MATH_CHOICE_GROUP: i32 = 13;
pub const SEMI_SIMPLE_GROUP: i32 = 14;
pub const MATH_SHIFT_GROUP: i32 = 15;
pub const MATH_LEFT_GROUP: i32 = 16;

pub const SUP_CMD: i32 = 0;
pub const SUB_CMD: i32 = 1;

pub const FIL: i32 = 1;
pub const FILL: i32 = 2;
pub const FILLL: i32 = 3;

pub const LIG_TAG: i32 = 1;
pub const LIST_TAG: i32 = 2;
pub const EXT_TAG: i32 = 3;

pub const SKIPPING: i32 = 1;
pub const DEFINING: i32 = 2;
pub const MATCHING: i32 = 3;
pub const ALIGNING: i32 = 4;
pub const ABSORBING: i32 = 5;

pub const DELIMITED_CODE: i32 = 3;

pub const PARAMETER: i32 = 0;
pub const U_TEMPLATE: i32 = 1;
pub const V_TEMPLATE: i32 = 2;
pub const BACKED_UP: i32 = 3;
pub const BACKED_UP_CHAR: i32 = 4;
pub const INSERTED: i32 = 5;
pub const MACRO: i32 = 6;
pub const OUTPUT_TEXT: i32 = 7;
pub const EVERY_PAR_TEXT: i32 = 8;
pub const EVERY_MATH_TEXT: i32 = 9;
pub const EVERY_DISPLAY_TEXT: i32 = 10;
pub const EVERY_HBOX_TEXT: i32 = 11;
pub const EVERY_VBOX_TEXT: i32 = 12;
pub const EVERY_JOB_TEXT: i32 = 13;
pub const EVERY_CR_TEXT: i32 = 14;
pub const MARK_TEXT: i32 = 15;
pub const EVERY_EOF_TEXT: i32 = 16;
pub const INTER_CHAR_TEXT: i32 = 17;
pub const WRITE_TEXT: i32 = 18;
pub const TECTONIC_CODA_TEXT: i32 = 19;

pub const MID_LINE: i32 = 1;
pub const SKIP_BLANKS: i32 = 17;
pub const NEW_LINE: i32 = 33;

pub const XDV_ID_BYTE: i32 = 7;
pub const SPX_ID_BYTE: i32 = 100;

pub const EMPTY: i32 = 0;
pub const INSERTS_ONLY: i32 = 1;
pub const BOX_THERE: i32 = 2;

pub const SET1: i32 = 128;
pub const SET_RULE: i32 = 132;
pub const PUT_RULE: i32 = 137;
pub const BOP: i32 = 139;
pub const EOP: i32 = 140;
pub const PUSH: i32 = 141;
pub const POP: i32 = 142;
pub const RIGHT1: i32 = 143;
pub const DOWN1: i32 = 157;
pub const FNT1: i32 = 235;
pub const XXX1: i32 = 239;
pub const XXX4: i32 = 242;
pub const FNT_DEF1: i32 = 243;
pub const PRE: i32 = 247;
pub const POST: i32 = 248;
pub const POST_POST: i32 = 249;
pub const DEFINE_NATIVE_FONT: i32 = 252;
pub const SET_GLYPHS: i32 = 253;
pub const SET_TEXT_AND_GLYPHS: i32 = 254;

pub const XETEX_INPUT_MODE_AUTO: i32 = 0;
pub const XETEX_VERSION: i32 = 0;
pub const EXACTLY: i32 = 0;
pub const FONT_BASE: i32 = 0;
pub const INSERTING: i32 = 0;
pub const NON_ADDRESS: i32 = 0;
pub const RESTORE_OLD_VALUE: i32 = 0;
pub const TOKEN_LIST: i32 = 0;
pub const UNDEFINED_PRIMITIVE: i32 = 0;
pub const UNHYPHENATED: i32 = 0;
pub const ADDITIONAL: i32 = 1;
pub const FIXED_ACC: i32 = 1;
pub const HYPHENATED: i32 = 1;
pub const IF_CODE: i32 = 1;
pub const JUST_OPEN: i32 = 1;
pub const MATH_CHAR: i32 = 1;
pub const PRIM_BASE: i32 = 1;
pub const RESTORE_ZERO: i32 = 1;
pub const REVERSED: i32 = 1;
pub const SLANT_CODE: i32 = 1;
pub const SPLIT_UP: i32 = 1;
pub const STRETCHING: i32 = 1;
pub const BOTTOM_ACC: i32 = 2;
pub const CLOSED: i32 = 2;
pub const DLIST: i32 = 2;
pub const ETEX_VERSION: i32 = 2;
pub const INSERT_TOKEN: i32 = 2;
pub const SHRINKING: i32 = 2;
pub const SPACE_CODE: i32 = 2;
pub const SUB_BOX: i32 = 2;
pub const DISPLAYOPERATORMINHEIGHT: i32 = 3;
pub const LEVEL_BOUNDARY: i32 = 3;
pub const SUB_MLIST: i32 = 3;
pub const MU_VAL: i32 = 3;
pub const IDENT_VAL: i32 = 4;
pub const MATH_TEXT_CHAR: i32 = 4;
pub const RESTORE_SA: i32 = 4;
pub const SPACE_SHRINK_CODE: i32 = 4;
pub const TOK_VAL: i32 = 5;
pub const X_HEIGHT_CODE: i32 = 5;
pub const ACCENTBASEHEIGHT: i32 = 6;
pub const INTER_CHAR_VAL: i32 = 6;
pub const QUAD_CODE: i32 = 6;
pub const EXTRA_SPACE_CODE: i32 = 7;
pub const MARK_VAL: i32 = 7;
pub const VAR_FAM_CLASS: i32 = 7;
pub const SUBSCRIPTTOPMAX: i32 = 9;
pub const NATIVE_GLYPH_INFO_SIZE: i32 = 10;
pub const CARRIAGE_RETURN: i32 = 13;
pub const SUPERSCRIPTBOTTOMMIN: i32 = 13;
pub const TOTAL_MATHEX_PARAMS: i32 = 13;
pub const HI_MEM_STAT_USAGE: i32 = 15;
pub const SUBSUPERSCRIPTGAPMIN: i32 = 15;
pub const SUPERSCRIPTBOTTOMMAXWITHSUBSCRIPT: i32 = 16;
pub const TOTAL_MATHSY_PARAMS: i32 = 22;
pub const STACKGAPMIN: i32 = 26;
pub const STACKDISPLAYSTYLEGAPMIN: i32 = 27;
pub const UNLESS_CODE: i32 = 32;
pub const FRACTIONNUMERATORGAPMIN: i32 = 36;
pub const FRACTIONNUMDISPLAYSTYLEGAPMIN: i32 = 37;
pub const FRACTIONDENOMINATORGAPMIN: i32 = 39;
pub const FRACTIONDENOMDISPLAYSTYLEGAPMIN: i32 = 40;
pub const RADICALVERTICALGAP: i32 = 49;
pub const RADICALDISPLAYSTYLEVERTICALGAP: i32 = 50;
pub const RADICALRULETHICKNESS: i32 = 51;
pub const COND_MATH_GLUE: i32 = 98;
pub const DIMEN_VAL_LIMIT: i32 = 128;
pub const BIGGEST_LANG: i32 = 255;
pub const MU_VAL_LIMIT: i32 = 256;
pub const TOO_BIG_LANG: i32 = 256;
pub const BOX_VAL_LIMIT: i32 = 320;
pub const TOK_VAL_LIMIT: i32 = 384;
pub const MAX_HLIST_STACK: i32 = 512;
pub const HYPH_PRIME: i32 = 607;
pub const HYPHENATABLE_LENGTH_LIMIT: i32 = 4095;
pub const CHAR_CLASS_LIMIT: i32 = 4096;
pub const EJECT_PENALTY: i32 = -10000;
pub const INF_BAD: i32 = 10000;
pub const INF_PENALTY: i32 = 10000;
pub const DEFAULT_RULE: i32 = 26214;
pub const TOO_BIG_CHAR: i32 = 65536;
pub const NO_EXPAND_FLAG: i32 = BIGGEST_USV + 2;

pub const ACTIVE_MATH_CHAR: i32 = 0x1FFFFF;

pub const MAX_CHAR_VAL: i32 = 0x200000;
pub const CS_TOKEN_FLAG: i32 = 0x1FFFFFF;
pub const LEFT_BRACE_TOKEN: i32 = 0x200000;
pub const LEFT_BRACE_LIMIT: i32 = 0x400000;
pub const RIGHT_BRACE_TOKEN: i32 = 0x400000;
pub const RIGHT_BRACE_LIMIT: i32 = 0x600000;
pub const MATH_SHIFT_TOKEN: i32 = 0x600000;
pub const TAB_TOKEN: i32 = 0x800000;
pub const OUT_PARAM_TOKEN: i32 = 0xA00000;
pub const SPACE_TOKEN: i32 = 0x1400020;
pub const LETTER_TOKEN: i32 = 0x1600000;
pub const OTHER_TOKEN: i32 = 0x1800000;
pub const MATCH_TOKEN: i32 = 0x1A00000;
pub const END_MATCH_TOKEN: i32 = 0x1C00000;
pub const PROTECTED_TOKEN: i32 = END_MATCH_TOKEN + 1;

pub const A_TOKEN: i32 = LETTER_TOKEN + ('A' as i32);
pub const OTHER_A_TOKEN: i32 = OTHER_TOKEN + ('A' as i32);
pub const HEX_TOKEN: i32 = OTHER_TOKEN + ('"' as i32);
pub const OCTAL_TOKEN: i32 = OTHER_TOKEN + ('\'' as i32);
pub const CONTINENTAL_POINT_TOKEN: i32 = OTHER_TOKEN + (',' as i32);
pub const POINT_TOKEN: i32 = OTHER_TOKEN + ('.' as i32);
pub const ZERO_TOKEN: i32 = OTHER_TOKEN + ('0' as i32);
pub const ALPHA_TOKEN: i32 = OTHER_TOKEN + ('`' as i32);

pub const BOX_FLAG: i32 = 0x40000000;
pub const GLOBAL_BOX_FLAG: i32 = 0x40008000;
pub const SHIP_OUT_FLAG: i32 = 0x40010000;
pub const LEADER_FLAG: i32 = 0x40010001;

pub const LP_CODE_BASE: i32 = 2;
pub const RP_CODE_BASE: i32 = 3;

pub const VSPLIT_INIT: i32 = 0;
pub const FIRE_UP_INIT: i32 = 1;
pub const FIRE_UP_DONE: i32 = 2;
pub const DESTROY_MARKS: i32 = 3;

pub const MARKS_CODE: i32 = 5;

pub const IGNORE_DEPTH: i32 = -65536000;

pub const MIDDLE_NOAD: i32 = 1;

pub const MOV_NONE_SEEN: i32 = 0;
pub const MOV_Y_HERE: i32 = 1;
pub const MOV_Z_HERE: i32 = 2;
pub const MOV_YZ_OK: i32 = 3;
pub const MOV_Y_OK: i32 = 4;
pub const MOV_Z_OK: i32 = 5;
pub const MOV_Y_SEEN: i32 = 6;
pub const MOV_D_FIXED: i32 = 6;
pub const MOV_Z_SEEN: i32 = 12;
