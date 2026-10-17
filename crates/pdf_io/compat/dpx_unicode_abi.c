#include "tectonic_bridge_core.h"
#include "dpx-unicode.h"
#include <assert.h>
#include <stdbool.h>

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_UC_is_valid(int32_t ucv, bool *_result);
bool UC_is_valid(int32_t ucv) {
  bool _result;
  if (__pdf_entry_UC_is_valid(ucv, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_UC_UTF16BE_is_valid_string(const unsigned char *p, const unsigned char *endptr, bool *_result);
bool UC_UTF16BE_is_valid_string(const unsigned char *p, const unsigned char *endptr) {
  bool _result;
  if (__pdf_entry_UC_UTF16BE_is_valid_string(p, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_UC_UTF8_is_valid_string(const unsigned char *p, const unsigned char *endptr, bool *_result);
bool UC_UTF8_is_valid_string(const unsigned char *p, const unsigned char *endptr) {
  bool _result;
  if (__pdf_entry_UC_UTF8_is_valid_string(p, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_UC_UTF16BE_decode_char(const unsigned char **pp, const unsigned char *endptr, int32_t *_result);
int32_t UC_UTF16BE_decode_char(const unsigned char **pp, const unsigned char *endptr) {
  int32_t _result;
  if (__pdf_entry_UC_UTF16BE_decode_char(pp, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_UC_UTF16BE_encode_char(int32_t ucv, unsigned char **pp, unsigned char *endptr, size_t *_result);
size_t UC_UTF16BE_encode_char(int32_t ucv, unsigned char **pp, unsigned char *endptr) {
  size_t _result;
  if (__pdf_entry_UC_UTF16BE_encode_char(ucv, pp, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_UC_UTF8_decode_char(const unsigned char **pp, const unsigned char *endptr, int32_t *_result);
int32_t UC_UTF8_decode_char(const unsigned char **pp, const unsigned char *endptr) {
  int32_t _result;
  if (__pdf_entry_UC_UTF8_decode_char(pp, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_UC_UTF8_encode_char(int32_t ucv, unsigned char **pp, unsigned char *endptr, size_t *_result);
size_t UC_UTF8_encode_char(int32_t ucv, unsigned char **pp, unsigned char *endptr) {
  size_t _result;
  if (__pdf_entry_UC_UTF8_encode_char(ucv, pp, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
