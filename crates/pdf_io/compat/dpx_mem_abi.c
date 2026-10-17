#include "tectonic_bridge_core.h"
#include "dpx-mem.h"
#include <stdlib.h>
#include "tectonic_bridge_core.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_new(uint32_t size, void **_result);
void *new(uint32_t size) {
  void *_result;
  if (__pdf_entry_new(size, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_renew(void *mem, uint32_t size, void **_result);
void *renew(void *mem, uint32_t size) {
  void *_result;
  if (__pdf_entry_renew(mem, size, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
