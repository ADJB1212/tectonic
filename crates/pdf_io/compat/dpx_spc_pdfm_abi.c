#include "tectonic_bridge_core.h"
#include "dpx-spc_pdfm.h"
#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-dpxutil.h"
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
#include "dpx-pdfximage.h"
#include "dpx-spc_util.h"
#include "dpx-specials.h"
#include "dpx-unicode.h"
#include "dpx-cmap.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_spc_pdfm_at_begin_document(int *_result);
int spc_pdfm_at_begin_document(void) {
  int _result;
  if (__pdf_entry_spc_pdfm_at_begin_document(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_pdfm_at_end_document(int *_result);
int spc_pdfm_at_end_document(void) {
  int _result;
  if (__pdf_entry_spc_pdfm_at_end_document(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_pdfm_at_end_page(int *_result);
int spc_pdfm_at_end_page(void) {
  int _result;
  if (__pdf_entry_spc_pdfm_at_end_page(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_pdfm_check_special(const char *buf, int len, bool *_result);
bool spc_pdfm_check_special(const char *buf, int len) {
  bool _result;
  if (__pdf_entry_spc_pdfm_check_special(buf, len, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_pdfm_setup_handler(struct spc_handler *sph, struct spc_env *spe, struct spc_arg *ap, int *_result);
int spc_pdfm_setup_handler(struct spc_handler *sph, struct spc_env *spe, struct spc_arg *ap) {
  int _result;
  if (__pdf_entry_spc_pdfm_setup_handler(sph, spe, ap, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
