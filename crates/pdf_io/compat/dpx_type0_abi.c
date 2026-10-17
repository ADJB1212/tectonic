#include "tectonic_bridge_core.h"
#include "dpx-type0.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-cid.h"
#include "dpx-cidtype0.h"
#include "dpx-cmap.h"
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-fontmap.h"
#include "dpx-mem.h"
#include "dpx-pdfobj.h"
#include "dpx-tt_cmap.h"
#include "dpx-pdfencoding.h"
#include "dpx-pdfresource.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_font_load_type0(pdf_font *font);
void pdf_font_load_type0(pdf_font *font) {
  if (__pdf_entry_pdf_font_load_type0(font) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_font_open_type0(pdf_font *font, int cid_id, int wmode, int *_result);
int pdf_font_open_type0(pdf_font *font, int cid_id, int wmode) {
  int _result;
  if (__pdf_entry_pdf_font_open_type0(font, cid_id, wmode, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
