#include "tectonic_bridge_core.h"
#include "dpx-cs_type2.h"
#include <math.h>
#include <stdio.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-cff_limits.h"
#include "dpx-cff_types.h"
#include "dpx-error.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_cs_copy_charstring(card8 *dst, int dstlen, card8 *src, int srclen, cff_index *gsubr, cff_index *subr, double default_width, double nominal_width, cs_ginfo *ginfo, int *_result);
int cs_copy_charstring(card8 *dst, int dstlen, card8 *src, int srclen, cff_index *gsubr, cff_index *subr, double default_width, double nominal_width, cs_ginfo *ginfo) {
  int _result;
  if (__pdf_entry_cs_copy_charstring(dst, dstlen, src, srclen, gsubr, subr, default_width, nominal_width, ginfo, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
