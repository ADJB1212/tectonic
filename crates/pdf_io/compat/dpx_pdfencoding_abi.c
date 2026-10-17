#include "tectonic_bridge_core.h"
#include "dpx-pdfencoding.h"
#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfparse.h"
#include "dpx-agl.h"
#include "dpx-cid.h"
#include "dpx-cmap.h"
#include "dpx-cmap_read.h"
#include "dpx-cmap_write.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_init_encodings(void);
void pdf_init_encodings(void) {
  if (__pdf_entry_pdf_init_encodings() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_encoding_complete(void);
void pdf_encoding_complete(void) {
  if (__pdf_entry_pdf_encoding_complete() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_close_encodings(void);
void pdf_close_encodings(void) {
  if (__pdf_entry_pdf_close_encodings() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_encoding_findresource(const char *enc_name, int *_result);
int pdf_encoding_findresource(const char *enc_name) {
  int _result;
  if (__pdf_entry_pdf_encoding_findresource(enc_name, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_encoding_get_encoding(int enc_id, char ***_result);
char **pdf_encoding_get_encoding(int enc_id) {
  char **_result;
  if (__pdf_entry_pdf_encoding_get_encoding(enc_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_get_encoding_obj(int enc_id, pdf_obj **_result);
pdf_obj *pdf_get_encoding_obj(int enc_id) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_get_encoding_obj(enc_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_encoding_is_predefined(int enc_id, int *_result);
int pdf_encoding_is_predefined(int enc_id) {
  int _result;
  if (__pdf_entry_pdf_encoding_is_predefined(enc_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_encoding_used_by_type3(int enc_id);
void pdf_encoding_used_by_type3(int enc_id) {
  if (__pdf_entry_pdf_encoding_used_by_type3(enc_id) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_encoding_get_name(int enc_id, char **_result);
char *pdf_encoding_get_name(int enc_id) {
  char *_result;
  if (__pdf_entry_pdf_encoding_get_name(enc_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_encoding_add_usedchars(int encoding_id, const char *is_used);
void pdf_encoding_add_usedchars(int encoding_id, const char *is_used) {
  if (__pdf_entry_pdf_encoding_add_usedchars(encoding_id, is_used) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_encoding_get_tounicode(int encoding_id, pdf_obj **_result);
pdf_obj *pdf_encoding_get_tounicode(int encoding_id) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_encoding_get_tounicode(encoding_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_create_ToUnicode_CMap(const char *enc_name, char **enc_vec, const char *is_used, pdf_obj **_result);
pdf_obj *pdf_create_ToUnicode_CMap(const char *enc_name, char **enc_vec, const char *is_used) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_create_ToUnicode_CMap(enc_name, enc_vec, is_used, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_load_ToUnicode_stream(const char *ident, pdf_obj **_result);
pdf_obj *pdf_load_ToUnicode_stream(const char *ident) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_load_ToUnicode_stream(ident, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
