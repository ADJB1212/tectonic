#include "tectonic_bridge_core.h"
#include "dpx-cmap_read.h"
#include <assert.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-cid.h"
#include "dpx-cmap.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-pst.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_CMap_parse_check_sig(rust_input_handle_t handle, int *_result);
int CMap_parse_check_sig(rust_input_handle_t handle) {
  int _result;
  if (__pdf_entry_CMap_parse_check_sig(handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_parse(CMap *cmap, rust_input_handle_t handle, int *_result);
int CMap_parse(CMap *cmap, rust_input_handle_t handle) {
  int _result;
  if (__pdf_entry_CMap_parse(cmap, handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
