#include "tectonic_bridge_core.h"
#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdffont.h"
#include "dpx-pdflimits.h"
#include "dpx-pdfobj.h"
#include "dpx-cidtype2.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-cid.h"
#include "dpx-cmap.h"
#include "dpx-sfnt.h"
#include "dpx-tt_aux.h"
#include "dpx-tt_cmap.h"
#include "dpx-tt_glyf.h"
#include "dpx-tt_gsub.h"
#include "dpx-tt_table.h"
#include "dpx-type0.h"
#include "dpx-unicode.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_CIDFont_type2_dofont(pdf_font *font, int *_result);
int CIDFont_type2_dofont(pdf_font *font) {
  int _result;
  if (__pdf_entry_CIDFont_type2_dofont(font, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CIDFont_type2_open(pdf_font *font, const char *name, int index, cid_opt *opt, int *_result);
int CIDFont_type2_open(pdf_font *font, const char *name, int index, cid_opt *opt) {
  int _result;
  if (__pdf_entry_CIDFont_type2_open(font, name, index, opt, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
