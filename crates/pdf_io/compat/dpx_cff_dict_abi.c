#include "tectonic_bridge_core.h"
#include <assert.h>
#include <errno.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-cff_dict.h"
#include "dpx-cff.h"
#include "dpx-cff_types.h"
#include "dpx-mfileio.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_cff_new_dict(cff_dict **_result);
cff_dict *cff_new_dict(void) {
  cff_dict *_result;
  if (__pdf_entry_cff_new_dict(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_release_dict(cff_dict *dict);
void cff_release_dict(cff_dict *dict) {
  if (__pdf_entry_cff_release_dict(dict) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_cff_dict_unpack(card8 *data, card8 *endptr, cff_dict **_result);
cff_dict *cff_dict_unpack(card8 *data, card8 *endptr) {
  cff_dict *_result;
  if (__pdf_entry_cff_dict_unpack(data, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_dict_pack(cff_dict *dict, card8 *dest, int destlen, int *_result);
int cff_dict_pack(cff_dict *dict, card8 *dest, int destlen) {
  int _result;
  if (__pdf_entry_cff_dict_pack(dict, dest, destlen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_dict_add(cff_dict *dict, const char *key, int count);
void cff_dict_add(cff_dict *dict, const char *key, int count) {
  if (__pdf_entry_cff_dict_add(dict, key, count) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_cff_dict_remove(cff_dict *dict, const char *key);
void cff_dict_remove(cff_dict *dict, const char *key) {
  if (__pdf_entry_cff_dict_remove(dict, key) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_cff_dict_known(cff_dict *dict, const char *key, int *_result);
int cff_dict_known(cff_dict *dict, const char *key) {
  int _result;
  if (__pdf_entry_cff_dict_known(dict, key, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_dict_get(cff_dict *dict, const char *key, int idx, double *_result);
double cff_dict_get(cff_dict *dict, const char *key, int idx) {
  double _result;
  if (__pdf_entry_cff_dict_get(dict, key, idx, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_dict_set(cff_dict *dict, const char *key, int idx, double value);
void cff_dict_set(cff_dict *dict, const char *key, int idx, double value) {
  if (__pdf_entry_cff_dict_set(dict, key, idx, value) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_cff_dict_update(cff_dict *dict, cff_font *cff);
void cff_dict_update(cff_dict *dict, cff_font *cff) {
  if (__pdf_entry_cff_dict_update(dict, cff) != 0) _tt_abort("%s", tt_pdf_last_error());
}
