#include "tectonic_bridge_core.h"
#include "dpx-spc_color.h"
#include <assert.h>
#include <stdlib.h>
#include "dpx-dpxutil.h"
#include "dpx-pdfcolor.h"
#include "dpx-spc_util.h"
#include "dpx-specials.h"
#include "dpx-pdfdoc.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_spc_color_check_special(const char *buf, int len, bool *_result);
bool spc_color_check_special(const char *buf, int len) {
  bool _result;
  if (__pdf_entry_spc_color_check_special(buf, len, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_color_setup_handler(struct spc_handler *sph, struct spc_env *spe, struct spc_arg *ap, int *_result);
int spc_color_setup_handler(struct spc_handler *sph, struct spc_env *spe, struct spc_arg *ap) {
  int _result;
  if (__pdf_entry_spc_color_setup_handler(sph, spe, ap, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
