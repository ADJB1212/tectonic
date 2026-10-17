#include "tectonic_bridge_core.h"
#include "dpx-sfnt.h"
#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-mem.h"
#include "dpx-pdfobj.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_sfnt_open(rust_input_handle_t handle, sfnt **_result);
sfnt *sfnt_open(rust_input_handle_t handle) {
  sfnt *_result;
  if (__pdf_entry_sfnt_open(handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dfont_open(rust_input_handle_t handle, int index, sfnt **_result);
sfnt *dfont_open(rust_input_handle_t handle, int index) {
  sfnt *_result;
  if (__pdf_entry_dfont_open(handle, index, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_sfnt_close(sfnt *sfont);
void sfnt_close(sfnt *sfont) {
  if (__pdf_entry_sfnt_close(sfont) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_put_big_endian(void *s, SFNT_LONG q, int n, int *_result);
int put_big_endian(void *s, SFNT_LONG q, int n) {
  int _result;
  if (__pdf_entry_put_big_endian(s, q, n, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_sfnt_set_table(sfnt *sfont, const char *tag, void *data, SFNT_ULONG length);
void sfnt_set_table(sfnt *sfont, const char *tag, void *data, SFNT_ULONG length) {
  if (__pdf_entry_sfnt_set_table(sfont, tag, data, length) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_sfnt_find_table_len(sfnt *sfont, const char *tag, SFNT_ULONG *_result);
SFNT_ULONG sfnt_find_table_len(sfnt *sfont, const char *tag) {
  SFNT_ULONG _result;
  if (__pdf_entry_sfnt_find_table_len(sfont, tag, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_sfnt_find_table_pos(sfnt *sfont, const char *tag, SFNT_ULONG *_result);
SFNT_ULONG sfnt_find_table_pos(sfnt *sfont, const char *tag) {
  SFNT_ULONG _result;
  if (__pdf_entry_sfnt_find_table_pos(sfont, tag, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_sfnt_locate_table(sfnt *sfont, const char *tag, SFNT_ULONG *_result);
SFNT_ULONG sfnt_locate_table(sfnt *sfont, const char *tag) {
  SFNT_ULONG _result;
  if (__pdf_entry_sfnt_locate_table(sfont, tag, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_sfnt_read_table_directory(sfnt *sfont, SFNT_ULONG offset, int *_result);
int sfnt_read_table_directory(sfnt *sfont, SFNT_ULONG offset) {
  int _result;
  if (__pdf_entry_sfnt_read_table_directory(sfont, offset, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_sfnt_require_table(sfnt *sfont, const char *tag, int must_exist, int *_result);
int sfnt_require_table(sfnt *sfont, const char *tag, int must_exist) {
  int _result;
  if (__pdf_entry_sfnt_require_table(sfont, tag, must_exist, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_sfnt_create_FontFile_stream(sfnt *sfont, pdf_obj **_result);
pdf_obj *sfnt_create_FontFile_stream(sfnt *sfont) {
  pdf_obj *_result;
  if (__pdf_entry_sfnt_create_FontFile_stream(sfont, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
