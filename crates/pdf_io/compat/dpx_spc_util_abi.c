#include "tectonic_bridge_core.h"
#include "dpx-spc_util.h"
#include <assert.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfparse.h"
#include "dpx-pdfresource.h"
#include "dpx-specials.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_spc_util_read_numbers(double *values, int num_values, struct spc_arg *args, int *_result);
int spc_util_read_numbers(double *values, int num_values, struct spc_arg *args) {
  int _result;
  if (__pdf_entry_spc_util_read_numbers(values, num_values, args, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_util_read_colorspec(struct spc_env *spe, pdf_color *colorspec, struct spc_arg *ap, int syntax, int *_result);
int spc_util_read_colorspec(struct spc_env *spe, pdf_color *colorspec, struct spc_arg *ap, int syntax) {
  int _result;
  if (__pdf_entry_spc_util_read_colorspec(spe, colorspec, ap, syntax, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_util_read_pdfcolor(struct spc_env *spe, pdf_color *colorspec, struct spc_arg *ap, pdf_color *defaultcolor, int *_result);
int spc_util_read_pdfcolor(struct spc_env *spe, pdf_color *colorspec, struct spc_arg *ap, pdf_color *defaultcolor) {
  int _result;
  if (__pdf_entry_spc_util_read_pdfcolor(spe, colorspec, ap, defaultcolor, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_util_read_dimtrns(struct spc_env *spe, transform_info *ti, struct spc_arg *args, int syntax, int *_result);
int spc_util_read_dimtrns(struct spc_env *spe, transform_info *ti, struct spc_arg *args, int syntax) {
  int _result;
  if (__pdf_entry_spc_util_read_dimtrns(spe, ti, args, syntax, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_util_read_blahblah(struct spc_env *spe, transform_info *p, int *page_no, enum pdf_page_boundary *bbox_type, struct spc_arg *ap, int *_result);
int spc_util_read_blahblah(struct spc_env *spe, transform_info *p, int *page_no, enum pdf_page_boundary *bbox_type, struct spc_arg *ap) {
  int _result;
  if (__pdf_entry_spc_util_read_blahblah(spe, p, page_no, bbox_type, ap, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
