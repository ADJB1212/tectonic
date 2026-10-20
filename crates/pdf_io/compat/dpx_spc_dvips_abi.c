#include "tectonic_bridge_core.h"
#include "dpx-spc_dvips.h"
#include <assert.h>
#include <ctype.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-dpxfile.h"
#include "dpx-dvi.h"
#include "dpx-epdf.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mpost.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfparse.h"
#include "dpx-pdfximage.h"
#include "dpx-spc_util.h"
#include "dpx-spc_xtx.h"
#include "dpx-specials.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_spc_dvips_at_begin_document(int *_result);
int spc_dvips_at_begin_document(void) {
  int _result;
  if (__pdf_entry_spc_dvips_at_begin_document(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_dvips_at_end_document(int *_result);
int spc_dvips_at_end_document(void) {
  int _result;
  if (__pdf_entry_spc_dvips_at_end_document(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_dvips_at_begin_page(int *_result);
int spc_dvips_at_begin_page(void) {
  int _result;
  if (__pdf_entry_spc_dvips_at_begin_page(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_dvips_at_end_page(int *_result);
int spc_dvips_at_end_page(void) {
  int _result;
  if (__pdf_entry_spc_dvips_at_end_page(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_dvips_check_special(const char *buf, int len, bool *_result);
bool spc_dvips_check_special(const char *buf, int len) {
  bool _result;
  if (__pdf_entry_spc_dvips_check_special(buf, len, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_dvips_setup_handler(struct spc_handler *handle, struct spc_env *spe, struct spc_arg *args, int *_result);
int spc_dvips_setup_handler(struct spc_handler *handle, struct spc_env *spe, struct spc_arg *args) {
  int _result;
  if (__pdf_entry_spc_dvips_setup_handler(handle, spe, args, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
