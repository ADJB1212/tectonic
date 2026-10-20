#include "tectonic_bridge_core.h"
#include "dpx-t1_char.h"
#include <assert.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-cff_types.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_t1char_get_metrics(card8 *src, int srclen, cff_index *subrs, t1_ginfo *ginfo, int *_result);
int t1char_get_metrics(card8 *src, int srclen, cff_index *subrs, t1_ginfo *ginfo) {
  int _result;
  if (__pdf_entry_t1char_get_metrics(src, srclen, subrs, ginfo, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_t1char_convert_charstring(card8 *dst, int dstlen, card8 *src, int srclen, cff_index *subrs, double default_width, double nominal_width, t1_ginfo *ginfo, int *_result);
int t1char_convert_charstring(card8 *dst, int dstlen, card8 *src, int srclen, cff_index *subrs, double default_width, double nominal_width, t1_ginfo *ginfo) {
  int _result;
  if (__pdf_entry_t1char_convert_charstring(dst, dstlen, src, srclen, subrs, default_width, nominal_width, ginfo, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
