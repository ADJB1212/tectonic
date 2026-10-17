#include "tectonic_bridge_core.h"
#include "dpx-spc_misc.h"
#include <assert.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-mpost.h"
#include "dpx-numbers.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfparse.h"
#include "dpx-pdfximage.h"
#include "dpx-specials.h"
#include "dpx-pdffont.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_spc_misc_at_begin_document(int *_result);
int spc_misc_at_begin_document(void) {
  int _result;
  if (__pdf_entry_spc_misc_at_begin_document(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_misc_at_end_document(int *_result);
int spc_misc_at_end_document(void) {
  int _result;
  if (__pdf_entry_spc_misc_at_end_document(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_misc_at_begin_page(int *_result);
int spc_misc_at_begin_page(void) {
  int _result;
  if (__pdf_entry_spc_misc_at_begin_page(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_misc_at_begin_form(int *_result);
int spc_misc_at_begin_form(void) {
  int _result;
  if (__pdf_entry_spc_misc_at_begin_form(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_misc_at_end_form(int *_result);
int spc_misc_at_end_form(void) {
  int _result;
  if (__pdf_entry_spc_misc_at_end_form(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_misc_check_special(const char *buffer, int size, bool *_result);
bool spc_misc_check_special(const char *buffer, int size) {
  bool _result;
  if (__pdf_entry_spc_misc_check_special(buffer, size, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_misc_setup_handler(struct spc_handler *handle, struct spc_env *spe, struct spc_arg *args, int *_result);
int spc_misc_setup_handler(struct spc_handler *handle, struct spc_env *spe, struct spc_arg *args) {
  int _result;
  if (__pdf_entry_spc_misc_setup_handler(handle, spe, args, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
