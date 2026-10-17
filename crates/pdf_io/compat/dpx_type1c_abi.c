#include "tectonic_bridge_core.h"
#include "dpx-type1c.h"
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-cff.h"
#include "dpx-cff_dict.h"
#include "dpx-cff_limits.h"
#include "dpx-cff_types.h"
#include "dpx-cs_type2.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-numbers.h"
#include "dpx-pdfencoding.h"
#include "dpx-pdffont.h"
#include "dpx-pdfobj.h"
#include "dpx-sfnt.h"
#include "dpx-tt_aux.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_font_open_type1c(pdf_font *font, const char *ident, int index, int encoding_id, int embedding, int *_result);
int pdf_font_open_type1c(pdf_font *font, const char *ident, int index, int encoding_id, int embedding) {
  int _result;
  if (__pdf_entry_pdf_font_open_type1c(font, ident, index, encoding_id, embedding, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_font_load_type1c(pdf_font *font, int *_result);
int pdf_font_load_type1c(pdf_font *font) {
  int _result;
  if (__pdf_entry_pdf_font_load_type1c(font, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
