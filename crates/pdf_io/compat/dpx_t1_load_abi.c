#include "tectonic_bridge_core.h"
#include "dpx-t1_load.h"
#include <fcntl.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-cff.h"
#include "dpx-cff_dict.h"
#include "dpx-cff_limits.h"
#include "dpx-cff_types.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-pst.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_is_pfb(rust_input_handle_t handle, bool *_result);
bool is_pfb(rust_input_handle_t handle) {
  bool _result;
  if (__pdf_entry_is_pfb(handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_t1_get_standard_glyph(int code, const char **_result);
const char *t1_get_standard_glyph(int code) {
  const char *_result;
  if (__pdf_entry_t1_get_standard_glyph(code, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_t1_get_fontname(rust_input_handle_t handle, char *fontname, int *_result);
int t1_get_fontname(rust_input_handle_t handle, char *fontname) {
  int _result;
  if (__pdf_entry_t1_get_fontname(handle, fontname, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_t1_load_font(char **enc_vec, int mode, rust_input_handle_t handle, cff_font **_result);
cff_font *t1_load_font(char **enc_vec, int mode, rust_input_handle_t handle) {
  cff_font *_result;
  if (__pdf_entry_t1_load_font(enc_vec, mode, handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
