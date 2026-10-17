#include "tectonic_bridge_core.h"
#include "dpx-otl_opt.h"
#include <assert.h>
#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-error.h"
#include "dpx-mem.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_otl_new_opt(otl_opt **_result);
otl_opt *otl_new_opt(void) {
  otl_opt *_result;
  if (__pdf_entry_otl_new_opt(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_otl_release_opt(otl_opt *opt);
void otl_release_opt(otl_opt *opt) {
  if (__pdf_entry_otl_release_opt(opt) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_otl_parse_optstring(otl_opt *opt, const char *optstr, int *_result);
int otl_parse_optstring(otl_opt *opt, const char *optstr) {
  int _result;
  if (__pdf_entry_otl_parse_optstring(opt, optstr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_otl_match_optrule(otl_opt *opt, const char *tag, int *_result);
int otl_match_optrule(otl_opt *opt, const char *tag) {
  int _result;
  if (__pdf_entry_otl_match_optrule(opt, tag, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
