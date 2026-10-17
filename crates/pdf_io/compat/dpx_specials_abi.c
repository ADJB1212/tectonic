#include "tectonic_bridge_core.h"
#include "dpx-specials.h"
#include <assert.h>
#include <ctype.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-dvi.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfnames.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfparse.h"
#include "dpx-spc_color.h"
#include "dpx-spc_dvipdfmx.h"
#include "dpx-spc_dvips.h"
#include "dpx-spc_html.h"
#include "dpx-spc_misc.h"
#include "dpx-spc_pdfm.h"
#include "dpx-spc_tpic.h"
#include "dpx-spc_xtx.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_spc_begin_annot(struct spc_env *spe, pdf_obj *dict, int *_result);
int spc_begin_annot(struct spc_env *spe, pdf_obj *dict) {
  int _result;
  if (__pdf_entry_spc_begin_annot(spe, dict, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_end_annot(struct spc_env *spe, int *_result);
int spc_end_annot(struct spc_env *spe) {
  int _result;
  if (__pdf_entry_spc_end_annot(spe, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_resume_annot(struct spc_env *spe, int *_result);
int spc_resume_annot(struct spc_env *spe) {
  int _result;
  if (__pdf_entry_spc_resume_annot(spe, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_suspend_annot(struct spc_env *spe, int *_result);
int spc_suspend_annot(struct spc_env *spe) {
  int _result;
  if (__pdf_entry_spc_suspend_annot(spe, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_is_tracking_boxes(struct spc_env *spe, int *_result);
int spc_is_tracking_boxes(struct spc_env *spe) {
  int _result;
  if (__pdf_entry_spc_is_tracking_boxes(spe, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_set_linkmode(struct spc_env *spe, int mode);
void spc_set_linkmode(struct spc_env *spe, int mode) {
  if (__pdf_entry_spc_set_linkmode(spe, mode) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_set_phantom(struct spc_env *spe, double height, double depth);
void spc_set_phantom(struct spc_env *spe, double height, double depth) {
  if (__pdf_entry_spc_set_phantom(spe, height, depth) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_lookup_reference(const char *key, pdf_obj **_result);
pdf_obj *spc_lookup_reference(const char *key) {
  pdf_obj *_result;
  if (__pdf_entry_spc_lookup_reference(key, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_lookup_object(const char *key, pdf_obj **_result);
pdf_obj *spc_lookup_object(const char *key) {
  pdf_obj *_result;
  if (__pdf_entry_spc_lookup_object(key, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_push_object(struct spc_env *spe, const char *key, pdf_obj *value);
void spc_push_object(struct spc_env *spe, const char *key, pdf_obj *value) {
  if (__pdf_entry_spc_push_object(spe, key, value) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_flush_object(struct spc_env *spe, const char *key);
void spc_flush_object(struct spc_env *spe, const char *key) {
  if (__pdf_entry_spc_flush_object(spe, key) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_clear_objects(struct spc_env *spe);
void spc_clear_objects(struct spc_env *spe) {
  if (__pdf_entry_spc_clear_objects(spe) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_get_coord(struct spc_env *spe, double *x, double *y);
void spc_get_coord(struct spc_env *spe, double *x, double *y) {
  if (__pdf_entry_spc_get_coord(spe, x, y) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_push_coord(struct spc_env *spe, double x, double y);
void spc_push_coord(struct spc_env *spe, double x, double y) {
  if (__pdf_entry_spc_push_coord(spe, x, y) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_pop_coord(struct spc_env *spe);
void spc_pop_coord(struct spc_env *spe) {
  if (__pdf_entry_spc_pop_coord(spe) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_set_fixed_point(struct spc_env *spe, double x, double y);
void spc_set_fixed_point(struct spc_env *spe, double x, double y) {
  if (__pdf_entry_spc_set_fixed_point(spe, x, y) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_get_fixed_point(struct spc_env *spe, double *x, double *y);
void spc_get_fixed_point(struct spc_env *spe, double *x, double *y) {
  if (__pdf_entry_spc_get_fixed_point(spe, x, y) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_put_fixed_point(struct spc_env *spe, double x, double y);
void spc_put_fixed_point(struct spc_env *spe, double x, double y) {
  if (__pdf_entry_spc_put_fixed_point(spe, x, y) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_dup_fixed_point(struct spc_env *spe);
void spc_dup_fixed_point(struct spc_env *spe) {
  if (__pdf_entry_spc_dup_fixed_point(spe) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_pop_fixed_point(struct spc_env *spe);
void spc_pop_fixed_point(struct spc_env *spe) {
  if (__pdf_entry_spc_pop_fixed_point(spe) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_clear_fixed_point(struct spc_env *spe);
void spc_clear_fixed_point(struct spc_env *spe) {
  if (__pdf_entry_spc_clear_fixed_point(spe) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_get_current_point(struct spc_env *spe, pdf_coord *cp);
void spc_get_current_point(struct spc_env *spe, pdf_coord *cp) {
  if (__pdf_entry_spc_get_current_point(spe, cp) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_put_image(struct spc_env *spe, int res_id, transform_info *ti, double xpos, double ypos);
void spc_put_image(struct spc_env *spe, int res_id, transform_info *ti, double xpos, double ypos) {
  if (__pdf_entry_spc_put_image(spe, res_id, ti, xpos, ypos) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_spc_begin_form(struct spc_env *spe, const char *ident, pdf_coord cp, pdf_rect *cropbox, int *_result);
int spc_begin_form(struct spc_env *spe, const char *ident, pdf_coord cp, pdf_rect *cropbox) {
  int _result;
  if (__pdf_entry_spc_begin_form(spe, ident, cp, cropbox, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_end_form(struct spc_env *spe, pdf_obj *attr, int *_result);
int spc_end_form(struct spc_env *spe, pdf_obj *attr) {
  int _result;
  if (__pdf_entry_spc_end_form(spe, attr, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_exec_at_begin_page(int *_result);
int spc_exec_at_begin_page(void) {
  int _result;
  if (__pdf_entry_spc_exec_at_begin_page(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_exec_at_end_page(int *_result);
int spc_exec_at_end_page(void) {
  int _result;
  if (__pdf_entry_spc_exec_at_end_page(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_exec_at_begin_document(int *_result);
int spc_exec_at_begin_document(void) {
  int _result;
  if (__pdf_entry_spc_exec_at_begin_document(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_exec_at_end_document(int *_result);
int spc_exec_at_end_document(void) {
  int _result;
  if (__pdf_entry_spc_exec_at_end_document(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_spc_exec_special(const char *buffer, int32_t size, double x_user, double y_user, double mag, int *is_drawable, pdf_rect *rect, int *_result);
int spc_exec_special(const char *buffer, int32_t size, double x_user, double y_user, double mag, int *is_drawable, pdf_rect *rect) {
  int _result;
  if (__pdf_entry_spc_exec_special(buffer, size, x_user, y_user, mag, is_drawable, rect, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
