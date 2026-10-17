#include "tectonic_bridge_core.h"
#include "dpx-numbers.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_get_unsigned_byte(FILE *file, unsigned char *_result);
unsigned char get_unsigned_byte(FILE *file) {
  unsigned char _result;
  if (__pdf_entry_get_unsigned_byte(file, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_skip_bytes(unsigned int n, FILE *file);
void skip_bytes(unsigned int n, FILE *file) {
  if (__pdf_entry_skip_bytes(n, file) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_get_signed_byte(FILE *file, signed char *_result);
signed char get_signed_byte(FILE *file) {
  signed char _result;
  if (__pdf_entry_get_signed_byte(file, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_get_unsigned_pair(FILE *file, unsigned short *_result);
unsigned short get_unsigned_pair(FILE *file) {
  unsigned short _result;
  if (__pdf_entry_get_unsigned_pair(file, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_sget_unsigned_pair(unsigned char *s, unsigned short *_result);
unsigned short sget_unsigned_pair(unsigned char *s) {
  unsigned short _result;
  if (__pdf_entry_sget_unsigned_pair(s, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_get_signed_pair(FILE *file, short *_result);
short get_signed_pair(FILE *file) {
  short _result;
  if (__pdf_entry_get_signed_pair(file, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_get_unsigned_triple(FILE *file, unsigned int *_result);
unsigned int get_unsigned_triple(FILE *file) {
  unsigned int _result;
  if (__pdf_entry_get_unsigned_triple(file, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_get_signed_triple(FILE *file, int *_result);
int get_signed_triple(FILE *file) {
  int _result;
  if (__pdf_entry_get_signed_triple(file, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_get_signed_quad(FILE *file, int32_t *_result);
int32_t get_signed_quad(FILE *file) {
  int32_t _result;
  if (__pdf_entry_get_signed_quad(file, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_get_unsigned_quad(FILE *file, uint32_t *_result);
uint32_t get_unsigned_quad(FILE *file) {
  uint32_t _result;
  if (__pdf_entry_get_unsigned_quad(file, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_get_unsigned_num(FILE *file, unsigned char num, uint32_t *_result);
uint32_t get_unsigned_num(FILE *file, unsigned char num) {
  uint32_t _result;
  if (__pdf_entry_get_unsigned_num(file, num, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_get_positive_quad(FILE *file, const char *type_0, const char *name, uint32_t *_result);
uint32_t get_positive_quad(FILE *file, const char *type_0, const char *name) {
  uint32_t _result;
  if (__pdf_entry_get_positive_quad(file, type_0, name, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_sqxfw(int32_t sq, fixword fw, int32_t *_result);
int32_t sqxfw(int32_t sq, fixword fw) {
  int32_t _result;
  if (__pdf_entry_sqxfw(sq, fw, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_skip_bytes(unsigned int n, rust_input_handle_t handle);
void tt_skip_bytes(unsigned int n, rust_input_handle_t handle) {
  if (__pdf_entry_tt_skip_bytes(n, handle) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_tt_get_unsigned_byte(rust_input_handle_t handle, unsigned char *_result);
unsigned char tt_get_unsigned_byte(rust_input_handle_t handle) {
  unsigned char _result;
  if (__pdf_entry_tt_get_unsigned_byte(handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_get_signed_byte(rust_input_handle_t handle, signed char *_result);
signed char tt_get_signed_byte(rust_input_handle_t handle) {
  signed char _result;
  if (__pdf_entry_tt_get_signed_byte(handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_get_unsigned_pair(rust_input_handle_t handle, unsigned short *_result);
unsigned short tt_get_unsigned_pair(rust_input_handle_t handle) {
  unsigned short _result;
  if (__pdf_entry_tt_get_unsigned_pair(handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_get_signed_pair(rust_input_handle_t handle, short *_result);
short tt_get_signed_pair(rust_input_handle_t handle) {
  short _result;
  if (__pdf_entry_tt_get_signed_pair(handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_get_unsigned_triple(rust_input_handle_t handle, unsigned int *_result);
unsigned int tt_get_unsigned_triple(rust_input_handle_t handle) {
  unsigned int _result;
  if (__pdf_entry_tt_get_unsigned_triple(handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_get_unsigned_quad(rust_input_handle_t handle, uint32_t *_result);
uint32_t tt_get_unsigned_quad(rust_input_handle_t handle) {
  uint32_t _result;
  if (__pdf_entry_tt_get_unsigned_quad(handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_get_signed_quad(rust_input_handle_t handle, int32_t *_result);
int32_t tt_get_signed_quad(rust_input_handle_t handle) {
  int32_t _result;
  if (__pdf_entry_tt_get_signed_quad(handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_get_unsigned_num(rust_input_handle_t handle, unsigned char num, uint32_t *_result);
uint32_t tt_get_unsigned_num(rust_input_handle_t handle, unsigned char num) {
  uint32_t _result;
  if (__pdf_entry_tt_get_unsigned_num(handle, num, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_get_positive_quad(rust_input_handle_t handle, const char *type_0, const char *name, uint32_t *_result);
uint32_t tt_get_positive_quad(rust_input_handle_t handle, const char *type_0, const char *name) {
  uint32_t _result;
  if (__pdf_entry_tt_get_positive_quad(handle, type_0, name, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
