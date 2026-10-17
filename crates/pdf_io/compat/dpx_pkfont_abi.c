#include "tectonic_bridge_core.h"
#include "dpx-pkfont.h"
#include <assert.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-numbers.h"
#include "dpx-pdfdev.h" /* pdf_rect */
#include "dpx-pdfencoding.h"
#include "dpx-pdffont.h"
#include "dpx-pdfobj.h"
#include "dpx-system.h"
#include "dpx-tfm.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_PKFont_set_dpi(int dpi);
void PKFont_set_dpi(int dpi) {
  if (__pdf_entry_PKFont_set_dpi(dpi) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_font_open_pkfont(pdf_font *font, const char *ident, int index, int encoding_id, int embedding, double point_size, int *_result);
int pdf_font_open_pkfont(pdf_font *font, const char *ident, int index, int encoding_id, int embedding, double point_size) {
  int _result;
  if (__pdf_entry_pdf_font_open_pkfont(font, ident, index, encoding_id, embedding, point_size, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_font_load_pkfont(pdf_font *font, int *_result);
int pdf_font_load_pkfont(pdf_font *font) {
  int _result;
  if (__pdf_entry_pdf_font_load_pkfont(font, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
