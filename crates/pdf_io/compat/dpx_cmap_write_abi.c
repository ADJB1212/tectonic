#include "tectonic_bridge_core.h"
#include "dpx-cmap_write.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-cid.h"
#include "dpx-cmap.h"
#include "dpx-cmap_p.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfresource.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_CMap_create_stream(CMap *cmap, pdf_obj **_result);
pdf_obj *CMap_create_stream(CMap *cmap) {
  pdf_obj *_result;
  if (__pdf_entry_CMap_create_stream(cmap, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
