#include "tectonic_bridge_core.h"
#include "dpx-spc_xtx.h"
#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-dpxutil.h"
#include "dpx-fontmap.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfparse.h"
#include "dpx-spc_util.h"
#include "dpx-specials.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_spc_handler_xtx_do_transform(struct spc_env *spe, double x_user, double y_user, double a, double b, double c, double d, double e, double f, int *_result);
int spc_handler_xtx_do_transform(struct spc_env *spe, double x_user, double y_user, double a, double b, double c, double d, double e, double f) {
  int _result;
  if (__pdf_entry_spc_handler_xtx_do_transform(spe, x_user, y_user, a, b, c, d, e, f, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_handler_xtx_gsave(struct spc_env *spe, struct spc_arg *args, int *_result);
int spc_handler_xtx_gsave(struct spc_env *spe, struct spc_arg *args) {
  int _result;
  if (__pdf_entry_spc_handler_xtx_gsave(spe, args, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_handler_xtx_grestore(struct spc_env *spe, struct spc_arg *args, int *_result);
int spc_handler_xtx_grestore(struct spc_env *spe, struct spc_arg *args) {
  int _result;
  if (__pdf_entry_spc_handler_xtx_grestore(spe, args, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_xtx_check_special(const char *buf, int len, bool *_result);
bool spc_xtx_check_special(const char *buf, int len) {
  bool _result;
  if (__pdf_entry_spc_xtx_check_special(buf, len, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_xtx_setup_handler(struct spc_handler *sph, struct spc_env *spe, struct spc_arg *ap, int *_result);
int spc_xtx_setup_handler(struct spc_handler *sph, struct spc_env *spe, struct spc_arg *ap) {
  int _result;
  if (__pdf_entry_spc_xtx_setup_handler(sph, spe, ap, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
