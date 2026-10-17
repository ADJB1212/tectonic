#include "tectonic_bridge_core.h"
#include "dpx-tt_aux.h"
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-numbers.h"
#include "dpx-pdfobj.h"
#include "dpx-sfnt.h"
#include "dpx-tt_post.h"
#include "dpx-tt_table.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_ttc_read_offset(sfnt *sfont, SFNT_ULONG ttc_idx, SFNT_ULONG *_result);
SFNT_ULONG ttc_read_offset(sfnt *sfont, SFNT_ULONG ttc_idx) {
  SFNT_ULONG _result;
  if (__pdf_entry_ttc_read_offset(sfont, ttc_idx, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_get_fontdesc(sfnt *sfont, int *embed, int stemv, int type_0, const char *fontname, pdf_obj **_result);
pdf_obj *tt_get_fontdesc(sfnt *sfont, int *embed, int stemv, int type_0, const char *fontname) {
  pdf_obj *_result;
  if (__pdf_entry_tt_get_fontdesc(sfont, embed, stemv, type_0, fontname, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
