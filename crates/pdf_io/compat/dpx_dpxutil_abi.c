#include "tectonic_bridge_core.h"
#include "dpx-dpxutil.h"
#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-dvipdfmx.h"
#include "dpx-error.h"
#include "dpx-mem.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_xtoi(char c, int *_result);
int xtoi(char c) {
  int _result;
  if (__pdf_entry_xtoi(c, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_min4(double x1, double x2, double x3, double x4, double *_result);
double min4(double x1, double x2, double x3, double x4) {
  double _result;
  if (__pdf_entry_min4(x1, x2, x3, x4, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_max4(double x1, double x2, double x3, double x4, double *_result);
double max4(double x1, double x2, double x3, double x4) {
  double _result;
  if (__pdf_entry_max4(x1, x2, x3, x4, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dpx_util_read_length(double *vp, double mag, const char **pp, const char *endptr, int *_result);
int dpx_util_read_length(double *vp, double mag, const char **pp, const char *endptr) {
  int _result;
  if (__pdf_entry_dpx_util_read_length(vp, mag, pp, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dpx_util_get_unique_time_if_given(time_t *_result);
time_t dpx_util_get_unique_time_if_given(void) {
  time_t _result;
  if (__pdf_entry_dpx_util_get_unique_time_if_given(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dpx_util_format_asn_date(char *date_string, int need_timezone, int *_result);
int dpx_util_format_asn_date(char *date_string, int need_timezone) {
  int _result;
  if (__pdf_entry_dpx_util_format_asn_date(date_string, need_timezone, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_skip_white_spaces(unsigned char **s, unsigned char *endptr);
void skip_white_spaces(unsigned char **s, unsigned char *endptr) {
  if (__pdf_entry_skip_white_spaces(s, endptr) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dpx_stack_init(dpx_stack *stack);
void dpx_stack_init(dpx_stack *stack) {
  if (__pdf_entry_dpx_stack_init(stack) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dpx_stack_push(dpx_stack *stack, void *data);
void dpx_stack_push(dpx_stack *stack, void *data) {
  if (__pdf_entry_dpx_stack_push(stack, data) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dpx_stack_pop(dpx_stack *stack, void **_result);
void *dpx_stack_pop(dpx_stack *stack) {
  void *_result;
  if (__pdf_entry_dpx_stack_pop(stack, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dpx_stack_top(dpx_stack *stack, void **_result);
void *dpx_stack_top(dpx_stack *stack) {
  void *_result;
  if (__pdf_entry_dpx_stack_top(stack, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dpx_stack_at(dpx_stack *stack, int pos, void **_result);
void *dpx_stack_at(dpx_stack *stack, int pos) {
  void *_result;
  if (__pdf_entry_dpx_stack_at(stack, pos, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dpx_stack_roll(dpx_stack *stack, int n, int j);
void dpx_stack_roll(dpx_stack *stack, int n, int j) {
  if (__pdf_entry_dpx_stack_roll(stack, n, j) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dpx_stack_depth(dpx_stack *stack, int *_result);
int dpx_stack_depth(dpx_stack *stack) {
  int _result;
  if (__pdf_entry_dpx_stack_depth(stack, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_ht_init_table(struct ht_table *ht, hval_free_func hval_free_fn);
void ht_init_table(struct ht_table *ht, hval_free_func hval_free_fn) {
  if (__pdf_entry_ht_init_table(ht, hval_free_fn) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_ht_clear_table(struct ht_table *ht);
void ht_clear_table(struct ht_table *ht) {
  if (__pdf_entry_ht_clear_table(ht) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_ht_table_size(struct ht_table *ht, int *_result);
int ht_table_size(struct ht_table *ht) {
  int _result;
  if (__pdf_entry_ht_table_size(ht, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_ht_lookup_table(struct ht_table *ht, const void *key, int keylen, void **_result);
void *ht_lookup_table(struct ht_table *ht, const void *key, int keylen) {
  void *_result;
  if (__pdf_entry_ht_lookup_table(ht, key, keylen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_ht_remove_table(struct ht_table *ht, const void *key, int keylen, int *_result);
int ht_remove_table(struct ht_table *ht, const void *key, int keylen) {
  int _result;
  if (__pdf_entry_ht_remove_table(ht, key, keylen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_ht_insert_table(struct ht_table *ht, const void *key, int keylen, void *value);
void ht_insert_table(struct ht_table *ht, const void *key, int keylen, void *value) {
  if (__pdf_entry_ht_insert_table(ht, key, keylen, value) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_ht_append_table(struct ht_table *ht, const void *key, int keylen, void *value);
void ht_append_table(struct ht_table *ht, const void *key, int keylen, void *value) {
  if (__pdf_entry_ht_append_table(ht, key, keylen, value) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_ht_set_iter(struct ht_table *ht, struct ht_iter *iter, int *_result);
int ht_set_iter(struct ht_table *ht, struct ht_iter *iter) {
  int _result;
  if (__pdf_entry_ht_set_iter(ht, iter, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_ht_clear_iter(struct ht_iter *iter);
void ht_clear_iter(struct ht_iter *iter) {
  if (__pdf_entry_ht_clear_iter(iter) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_ht_iter_getkey(struct ht_iter *iter, int *keylen, char **_result);
char *ht_iter_getkey(struct ht_iter *iter, int *keylen) {
  char *_result;
  if (__pdf_entry_ht_iter_getkey(iter, keylen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_ht_iter_getval(struct ht_iter *iter, void **_result);
void *ht_iter_getval(struct ht_iter *iter) {
  void *_result;
  if (__pdf_entry_ht_iter_getval(iter, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_ht_iter_next(struct ht_iter *iter, int *_result);
int ht_iter_next(struct ht_iter *iter) {
  int _result;
  if (__pdf_entry_ht_iter_next(iter, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_c_string(const char **pp, const char *endptr, char **_result);
char *parse_c_string(const char **pp, const char *endptr) {
  char *_result;
  if (__pdf_entry_parse_c_string(pp, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_c_ident(const char **pp, const char *endptr, char **_result);
char *parse_c_ident(const char **pp, const char *endptr) {
  char *_result;
  if (__pdf_entry_parse_c_ident(pp, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_float_decimal(const char **pp, const char *endptr, char **_result);
char *parse_float_decimal(const char **pp, const char *endptr) {
  char *_result;
  if (__pdf_entry_parse_float_decimal(pp, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
