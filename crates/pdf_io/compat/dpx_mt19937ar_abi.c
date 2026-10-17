#include "tectonic_bridge_core.h"
#include <stdio.h>

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_init_genrand(unsigned long long s);
void init_genrand(unsigned long long s) {
  if (__pdf_entry_init_genrand(s) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_genrand_int31(long *_result);
long genrand_int31(void) {
  long _result;
  if (__pdf_entry_genrand_int31(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
