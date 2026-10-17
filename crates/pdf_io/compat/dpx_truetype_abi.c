#include "tectonic_bridge_core.h"
#include "dpx-truetype.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-agl.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfencoding.h"
#include "dpx-pdffont.h"
#include "dpx-pdfobj.h"
#include "dpx-sfnt.h"
#include "dpx-tt_aux.h"
#include "dpx-tt_cmap.h"
#include "dpx-tt_glyf.h"
#include "dpx-tt_gsub.h"
#include "dpx-tt_post.h"
#include "dpx-tt_table.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_font_open_truetype(pdf_font *font, const char *ident, int index, int encoding_id, int embedding, int *_result);
int pdf_font_open_truetype(pdf_font *font, const char *ident, int index, int encoding_id, int embedding) {
  int _result;
  if (__pdf_entry_pdf_font_open_truetype(font, ident, index, encoding_id, embedding, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_font_load_truetype(pdf_font *font, int *_result);
int pdf_font_load_truetype(pdf_font *font) {
  int _result;
  if (__pdf_entry_pdf_font_load_truetype(font, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
