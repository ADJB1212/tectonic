#include "tectonic_bridge_core.h"
#include "dpx-pdfnames.h"
#include <assert.h>
#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-pdfobj.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_new_name_tree(struct ht_table **_result);
struct ht_table *pdf_new_name_tree(void) {
  struct ht_table *_result;
  if (__pdf_entry_pdf_new_name_tree(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_delete_name_tree(struct ht_table **names);
void pdf_delete_name_tree(struct ht_table **names) {
  if (__pdf_entry_pdf_delete_name_tree(names) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_names_add_object(struct ht_table *names, const void *key, int keylen, pdf_obj *object, int *_result);
int pdf_names_add_object(struct ht_table *names, const void *key, int keylen, pdf_obj *object) {
  int _result;
  if (__pdf_entry_pdf_names_add_object(names, key, keylen, object, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_names_reserve(struct ht_table *names, const void *key, int keylen, pdf_obj **_result);
pdf_obj *pdf_names_reserve(struct ht_table *names, const void *key, int keylen) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_names_reserve(names, key, keylen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_names_lookup_reference(struct ht_table *names, const void *key, int keylen, pdf_obj **_result);
pdf_obj *pdf_names_lookup_reference(struct ht_table *names, const void *key, int keylen) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_names_lookup_reference(names, key, keylen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_names_lookup_object(struct ht_table *names, const void *key, int keylen, pdf_obj **_result);
pdf_obj *pdf_names_lookup_object(struct ht_table *names, const void *key, int keylen) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_names_lookup_object(names, key, keylen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_names_close_object(struct ht_table *names, const void *key, int keylen, int *_result);
int pdf_names_close_object(struct ht_table *names, const void *key, int keylen) {
  int _result;
  if (__pdf_entry_pdf_names_close_object(names, key, keylen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_names_create_tree(struct ht_table *names, int *count, struct ht_table *filter, pdf_obj **_result);
pdf_obj *pdf_names_create_tree(struct ht_table *names, int *count, struct ht_table *filter) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_names_create_tree(names, count, filter, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
