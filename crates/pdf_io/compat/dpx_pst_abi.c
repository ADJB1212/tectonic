#include "tectonic_bridge_core.h"
#include "dpx-pst.h"
#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-dpxutil.h"
#include "dpx-mem.h"
#include "dpx-pst_obj.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pst_get_token(unsigned char **inbuf, unsigned char *inbufend, pst_obj **_result);
pst_obj *pst_get_token(unsigned char **inbuf, unsigned char *inbufend) {
  pst_obj *_result;
  if (__pdf_entry_pst_get_token(inbuf, inbufend, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
