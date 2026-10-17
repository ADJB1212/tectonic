#include "tectonic_bridge_core.h"
#include "dpx-type1.h"
#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-cff.h"
#include "dpx-cff_dict.h"
#include "dpx-cff_limits.h"
#include "dpx-cff_types.h"
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfencoding.h"
#include "dpx-pdffont.h"
#include "dpx-pdflimits.h"
#include "dpx-pdfobj.h"
#include "dpx-t1_char.h"
#include "dpx-t1_load.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_font_open_type1(pdf_font *font, const char *ident, int index, int encoding_id, int embedding, int *_result);
int pdf_font_open_type1(pdf_font *font, const char *ident, int index, int encoding_id, int embedding) {
  int _result;
  if (__pdf_entry_pdf_font_open_type1(font, ident, index, encoding_id, embedding, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_font_load_type1(pdf_font *font, int *_result);
int pdf_font_load_type1(pdf_font *font) {
  int _result;
  if (__pdf_entry_pdf_font_load_type1(font, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
