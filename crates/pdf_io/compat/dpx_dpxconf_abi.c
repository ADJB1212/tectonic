#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include <stdio.h>

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_paperinfo(const char *ppformat, const struct paper **_result);
const struct paper *paperinfo(const char *ppformat) {
  const struct paper *_result;
  if (__pdf_entry_paperinfo(ppformat, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dumppaperinfo(void);
void dumppaperinfo(void) {
  if (__pdf_entry_dumppaperinfo() != 0) _tt_abort("%s", tt_pdf_last_error());
}
