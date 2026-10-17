#include "tectonic_bridge_core.h"
#include "dpx-dpxfile.h"
#include <fcntl.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <time.h>
#include <dirent.h>
#include "dpx-dpxconf.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-system.h"
#include <stdlib.h>
#include <string.h>

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_dpx_tt_open(const char *filename, const char *suffix, ttbc_file_format format, rust_input_handle_t *_result);
rust_input_handle_t dpx_tt_open(const char *filename, const char *suffix, ttbc_file_format format) {
  rust_input_handle_t _result;
  if (__pdf_entry_dpx_tt_open(filename, suffix, format, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dpx_open_type1_file(const char *filename, rust_input_handle_t *_result);
rust_input_handle_t dpx_open_type1_file(const char *filename) {
  rust_input_handle_t _result;
  if (__pdf_entry_dpx_open_type1_file(filename, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dpx_open_truetype_file(const char *filename, rust_input_handle_t *_result);
rust_input_handle_t dpx_open_truetype_file(const char *filename) {
  rust_input_handle_t _result;
  if (__pdf_entry_dpx_open_truetype_file(filename, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dpx_open_opentype_file(const char *filename, rust_input_handle_t *_result);
rust_input_handle_t dpx_open_opentype_file(const char *filename) {
  rust_input_handle_t _result;
  if (__pdf_entry_dpx_open_opentype_file(filename, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dpx_open_dfont_file(const char *filename, rust_input_handle_t *_result);
rust_input_handle_t dpx_open_dfont_file(const char *filename) {
  rust_input_handle_t _result;
  if (__pdf_entry_dpx_open_dfont_file(filename, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dpx_create_temp_file(char **_result);
char *dpx_create_temp_file(void) {
  char *_result;
  if (__pdf_entry_dpx_create_temp_file(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dpx_delete_old_cache(int life);
void dpx_delete_old_cache(int life) {
  if (__pdf_entry_dpx_delete_old_cache(life) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dpx_delete_temp_file(char *tmp, int force);
void dpx_delete_temp_file(char *tmp, int force) {
  if (__pdf_entry_dpx_delete_temp_file(tmp, force) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dpx_file_apply_filter(const char *cmdtmpl, const char *input, const char *output, int version, int *_result);
int dpx_file_apply_filter(const char *cmdtmpl, const char *input, const char *output, int version) {
  int _result;
  if (__pdf_entry_dpx_file_apply_filter(cmdtmpl, input, output, version, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
