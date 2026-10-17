#include "tectonic_bridge_core.h"
#include "dpx-cidtype0.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-agl.h"
#include "dpx-cff.h"
#include "dpx-cff_dict.h"
#include "dpx-cff_limits.h"
#include "dpx-cff_types.h"
#include "dpx-cid.h"
#include "dpx-cmap.h"
#include "dpx-cmap_write.h"
#include "dpx-cs_type2.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-numbers.h"
#include "dpx-pdffont.h"
#include "dpx-pdfobj.h"
#include "dpx-sfnt.h"
#include "dpx-t1_char.h"
#include "dpx-t1_load.h"
#include "dpx-tt_aux.h"
#include "dpx-tt_table.h"
#include "dpx-type0.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_CIDFont_type0_dofont(pdf_font *font, int *_result);
int CIDFont_type0_dofont(pdf_font *font) {
  int _result;
  if (__pdf_entry_CIDFont_type0_dofont(font, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CIDFont_type0_open_from_t1(pdf_font *font, const char *name, int index, cid_opt *opt, int *_result);
int CIDFont_type0_open_from_t1(pdf_font *font, const char *name, int index, cid_opt *opt) {
  int _result;
  if (__pdf_entry_CIDFont_type0_open_from_t1(font, name, index, opt, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CIDFont_type0_open(pdf_font *font, const char *name, int index, cid_opt *opt, int *_result);
int CIDFont_type0_open(pdf_font *font, const char *name, int index, cid_opt *opt) {
  int _result;
  if (__pdf_entry_CIDFont_type0_open(font, name, index, opt, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CIDFont_type0_open_from_t1c(pdf_font *font, const char *name, int index, cid_opt *opt, int *_result);
int CIDFont_type0_open_from_t1c(pdf_font *font, const char *name, int index, cid_opt *opt) {
  int _result;
  if (__pdf_entry_CIDFont_type0_open_from_t1c(font, name, index, opt, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CIDFont_type0_t1cdofont(pdf_font *font, int *_result);
int CIDFont_type0_t1cdofont(pdf_font *font) {
  int _result;
  if (__pdf_entry_CIDFont_type0_t1cdofont(font, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_t1_load_UnicodeCMap(const char *font_name, const char *otl_tags, int wmode, int *_result);
int t1_load_UnicodeCMap(const char *font_name, const char *otl_tags, int wmode) {
  int _result;
  if (__pdf_entry_t1_load_UnicodeCMap(font_name, otl_tags, wmode, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CIDFont_type0_t1create_ToUnicode_stream(const char *filename, const char *fontname, const char *used_chars, pdf_obj **_result);
pdf_obj *CIDFont_type0_t1create_ToUnicode_stream(const char *filename, const char *fontname, const char *used_chars) {
  pdf_obj *_result;
  if (__pdf_entry_CIDFont_type0_t1create_ToUnicode_stream(filename, fontname, used_chars, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CIDFont_type0_t1dofont(pdf_font *font, int *_result);
int CIDFont_type0_t1dofont(pdf_font *font) {
  int _result;
  if (__pdf_entry_CIDFont_type0_t1dofont(font, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
