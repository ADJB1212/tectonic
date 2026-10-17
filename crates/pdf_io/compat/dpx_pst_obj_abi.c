#include "tectonic_bridge_core.h"
#include "dpx-pst_obj.h"
#include <assert.h>
#include <ctype.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-pst.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pst_new_obj(pst_type type_0, void *data, pst_obj **_result);
pst_obj *pst_new_obj(pst_type type_0, void *data) {
  pst_obj *_result;
  if (__pdf_entry_pst_new_obj(type_0, data, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pst_new_mark(pst_obj **_result);
pst_obj *pst_new_mark(void) {
  pst_obj *_result;
  if (__pdf_entry_pst_new_mark(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pst_release_obj(pst_obj *obj);
void pst_release_obj(pst_obj *obj) {
  if (__pdf_entry_pst_release_obj(obj) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pst_type_of(pst_obj *obj, pst_type *_result);
pst_type pst_type_of(pst_obj *obj) {
  pst_type _result;
  if (__pdf_entry_pst_type_of(obj, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pst_length_of(pst_obj *obj, int *_result);
int pst_length_of(pst_obj *obj) {
  int _result;
  if (__pdf_entry_pst_length_of(obj, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pst_getIV(pst_obj *obj, int *_result);
int pst_getIV(pst_obj *obj) {
  int _result;
  if (__pdf_entry_pst_getIV(obj, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pst_getRV(pst_obj *obj, double *_result);
double pst_getRV(pst_obj *obj) {
  double _result;
  if (__pdf_entry_pst_getRV(obj, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pst_getSV(pst_obj *obj, unsigned char **_result);
unsigned char *pst_getSV(pst_obj *obj) {
  unsigned char *_result;
  if (__pdf_entry_pst_getSV(obj, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pst_data_ptr(pst_obj *obj, void **_result);
void *pst_data_ptr(pst_obj *obj) {
  void *_result;
  if (__pdf_entry_pst_data_ptr(obj, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pst_parse_boolean(unsigned char **inbuf, unsigned char *inbufend, pst_obj **_result);
pst_obj *pst_parse_boolean(unsigned char **inbuf, unsigned char *inbufend) {
  pst_obj *_result;
  if (__pdf_entry_pst_parse_boolean(inbuf, inbufend, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pst_parse_null(unsigned char **inbuf, unsigned char *inbufend, pst_obj **_result);
pst_obj *pst_parse_null(unsigned char **inbuf, unsigned char *inbufend) {
  pst_obj *_result;
  if (__pdf_entry_pst_parse_null(inbuf, inbufend, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pst_parse_number(unsigned char **inbuf, unsigned char *inbufend, pst_obj **_result);
pst_obj *pst_parse_number(unsigned char **inbuf, unsigned char *inbufend) {
  pst_obj *_result;
  if (__pdf_entry_pst_parse_number(inbuf, inbufend, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pst_parse_name(unsigned char **inbuf, unsigned char *inbufend, pst_obj **_result);
pst_obj *pst_parse_name(unsigned char **inbuf, unsigned char *inbufend) {
  pst_obj *_result;
  if (__pdf_entry_pst_parse_name(inbuf, inbufend, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pst_parse_string(unsigned char **inbuf, unsigned char *inbufend, pst_obj **_result);
pst_obj *pst_parse_string(unsigned char **inbuf, unsigned char *inbufend) {
  pst_obj *_result;
  if (__pdf_entry_pst_parse_string(inbuf, inbufend, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
