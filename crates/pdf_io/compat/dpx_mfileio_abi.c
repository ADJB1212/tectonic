#include "tectonic_bridge_core.h"
#include "dpx-mfileio.h"
#include <fcntl.h>
#include <limits.h>
#include <stdio.h>

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_seek_relative(FILE *file, int32_t pos);
void seek_relative(FILE *file, int32_t pos) {
  if (__pdf_entry_seek_relative(file, pos) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_file_size(FILE *file, int32_t *_result);
int32_t file_size(FILE *file) {
  int32_t _result;
  if (__pdf_entry_file_size(file, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_mfgets(char *buffer, int length, FILE *file, char **_result);
char *mfgets(char *buffer, int length, FILE *file) {
  char *_result;
  if (__pdf_entry_mfgets(buffer, length, file, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_mfgets(char *buffer, int length, rust_input_handle_t file, char **_result);
char *tt_mfgets(char *buffer, int length, rust_input_handle_t file) {
  char *_result;
  if (__pdf_entry_tt_mfgets(buffer, length, file, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
