#include "tectonic_bridge_core.h"
#include "dpx-mpost.h"
#include <ctype.h>
#include <math.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-dvipdfmx.h"
#include "dpx-error.h"
#include "dpx-fontmap.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfparse.h"
#include "dpx-subfont.h"
#include "dpx-tfm.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_mps_set_translate_origin(int v);
void mps_set_translate_origin(int v) {
  if (__pdf_entry_mps_set_translate_origin(v) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_mps_scan_bbox(const char **pp, const char *endptr, pdf_rect *bbox, int *_result);
int mps_scan_bbox(const char **pp, const char *endptr, pdf_rect *bbox) {
  int _result;
  if (__pdf_entry_mps_scan_bbox(pp, endptr, bbox, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_mps_eop_cleanup(void);
void mps_eop_cleanup(void) {
  if (__pdf_entry_mps_eop_cleanup() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_mps_stack_depth(int *_result);
int mps_stack_depth(void) {
  int _result;
  if (__pdf_entry_mps_stack_depth(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_mps_exec_inline(const char **p, const char *endptr, double x_user, double y_user, int *_result);
int mps_exec_inline(const char **p, const char *endptr, double x_user, double y_user) {
  int _result;
  if (__pdf_entry_mps_exec_inline(p, endptr, x_user, y_user, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_mps_do_page(FILE *image_file, int *_result);
int mps_do_page(FILE *image_file) {
  int _result;
  if (__pdf_entry_mps_do_page(image_file, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_mps_reset_global_state(void);
void mps_reset_global_state(void) {
  if (__pdf_entry_mps_reset_global_state() != 0) _tt_abort("%s", tt_pdf_last_error());
}
