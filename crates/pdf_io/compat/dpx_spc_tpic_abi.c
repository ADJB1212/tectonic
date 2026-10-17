#include "tectonic_bridge_core.h"
#include "dpx-spc_tpic.h"
#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfobj.h"
#include "dpx-specials.h"
#include "dpx-pdfparse.h" /* parse_val_ident :( */

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_tpic_set_fill_mode(int mode);
void tpic_set_fill_mode(int mode) {
  if (__pdf_entry_tpic_set_fill_mode(mode) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_tpic_at_begin_page(int *_result);
int spc_tpic_at_begin_page(void) {
  int _result;
  if (__pdf_entry_spc_tpic_at_begin_page(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_tpic_at_end_page(int *_result);
int spc_tpic_at_end_page(void) {
  int _result;
  if (__pdf_entry_spc_tpic_at_end_page(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_tpic_at_begin_document(int *_result);
int spc_tpic_at_begin_document(void) {
  int _result;
  if (__pdf_entry_spc_tpic_at_begin_document(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_tpic_at_end_document(int *_result);
int spc_tpic_at_end_document(void) {
  int _result;
  if (__pdf_entry_spc_tpic_at_end_document(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_tpic_check_special(const char *buf, int len, bool *_result);
bool spc_tpic_check_special(const char *buf, int len) {
  bool _result;
  if (__pdf_entry_spc_tpic_check_special(buf, len, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_tpic_setup_handler(struct spc_handler *sph, struct spc_env *spe, struct spc_arg *ap, int *_result);
int spc_tpic_setup_handler(struct spc_handler *sph, struct spc_env *spe, struct spc_arg *ap) {
  int _result;
  if (__pdf_entry_spc_tpic_setup_handler(sph, spe, ap, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
