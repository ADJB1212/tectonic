#include "tectonic_bridge_core.h"
#include "dpx-pdfparse.h"
#include <assert.h>
#include <ctype.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-pdfobj.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_dump(const char *start, const char *end);
void dump(const char *start, const char *end) {
  if (__pdf_entry_dump(start, end) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdfparse_skip_line(const char **start, const char *end);
void pdfparse_skip_line(const char **start, const char *end) {
  if (__pdf_entry_pdfparse_skip_line(start, end) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_skip_white(const char **start, const char *end);
void skip_white(const char **start, const char *end) {
  if (__pdf_entry_skip_white(start, end) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_parse_number(const char **start, const char *end, char **_result);
char *parse_number(const char **start, const char *end) {
  char *_result;
  if (__pdf_entry_parse_number(start, end, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_unsigned(const char **start, const char *end, char **_result);
char *parse_unsigned(const char **start, const char *end) {
  char *_result;
  if (__pdf_entry_parse_unsigned(start, end, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_ident(const char **start, const char *end, char **_result);
char *parse_ident(const char **start, const char *end) {
  char *_result;
  if (__pdf_entry_parse_ident(start, end, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_val_ident(const char **start, const char *end, char **_result);
char *parse_val_ident(const char **start, const char *end) {
  char *_result;
  if (__pdf_entry_parse_val_ident(start, end, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_opt_ident(const char **start, const char *end, char **_result);
char *parse_opt_ident(const char **start, const char *end) {
  char *_result;
  if (__pdf_entry_parse_opt_ident(start, end, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_pdf_number(const char **pp, const char *endptr, pdf_obj **_result);
pdf_obj *parse_pdf_number(const char **pp, const char *endptr) {
  pdf_obj *_result;
  if (__pdf_entry_parse_pdf_number(pp, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_pdf_name(const char **pp, const char *endptr, pdf_obj **_result);
pdf_obj *parse_pdf_name(const char **pp, const char *endptr) {
  pdf_obj *_result;
  if (__pdf_entry_parse_pdf_name(pp, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_pdf_boolean(const char **pp, const char *endptr, pdf_obj **_result);
pdf_obj *parse_pdf_boolean(const char **pp, const char *endptr) {
  pdf_obj *_result;
  if (__pdf_entry_parse_pdf_boolean(pp, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_pdf_null(const char **pp, const char *endptr, pdf_obj **_result);
pdf_obj *parse_pdf_null(const char **pp, const char *endptr) {
  pdf_obj *_result;
  if (__pdf_entry_parse_pdf_null(pp, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_pdf_string(const char **pp, const char *endptr, pdf_obj **_result);
pdf_obj *parse_pdf_string(const char **pp, const char *endptr) {
  pdf_obj *_result;
  if (__pdf_entry_parse_pdf_string(pp, endptr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_pdf_object_extended(const char **pp, const char *endptr, pdf_file *pf, pdf_obj *(*unknown_handler)(const char **, const char *, void *), void *user_data, pdf_obj **_result);
pdf_obj *parse_pdf_object_extended(const char **pp, const char *endptr, pdf_file *pf, pdf_obj *(*unknown_handler)(const char **, const char *, void *), void *user_data) {
  pdf_obj *_result;
  if (__pdf_entry_parse_pdf_object_extended(pp, endptr, pf, unknown_handler, user_data, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_pdf_dict(const char **pp, const char *endptr, pdf_file *pf, pdf_obj **_result);
pdf_obj *parse_pdf_dict(const char **pp, const char *endptr, pdf_file *pf) {
  pdf_obj *_result;
  if (__pdf_entry_parse_pdf_dict(pp, endptr, pf, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_pdf_array(const char **pp, const char *endptr, pdf_file *pf, pdf_obj **_result);
pdf_obj *parse_pdf_array(const char **pp, const char *endptr, pdf_file *pf) {
  pdf_obj *_result;
  if (__pdf_entry_parse_pdf_array(pp, endptr, pf, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_pdf_object(const char **pp, const char *endptr, pdf_file *pf, pdf_obj **_result);
pdf_obj *parse_pdf_object(const char **pp, const char *endptr, pdf_file *pf) {
  pdf_obj *_result;
  if (__pdf_entry_parse_pdf_object(pp, endptr, pf, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_parse_pdf_tainted_dict(const char **pp, const char *endptr, pdf_obj *(*unknown_handler)(const char **, const char *, void *), void *user_data, pdf_obj **_result);
pdf_obj *parse_pdf_tainted_dict(const char **pp, const char *endptr, pdf_obj *(*unknown_handler)(const char **, const char *, void *), void *user_data) {
  pdf_obj *_result;
  if (__pdf_entry_parse_pdf_tainted_dict(pp, endptr, unknown_handler, user_data, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
