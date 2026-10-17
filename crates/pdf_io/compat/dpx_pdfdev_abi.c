#include "tectonic_bridge_core.h"
#include "dpx-pdfdev.h"
#include <assert.h>
#include <limits.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-cff.h"
#include "dpx-cff_types.h"
#include "dpx-cmap.h"
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-fontmap.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-numbers.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdffont.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfximage.h"
#include "dpx-type0.h"
#include "tectonic-pipeline.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_dev_set_string(spt_t xpos, spt_t ypos, const void *input, size_t length, spt_t width, int font_id);
void pdf_dev_set_string(spt_t xpos, spt_t ypos, const void *input, size_t length, spt_t width, int font_id) {
  if (__pdf_entry_pdf_dev_set_string(xpos, ypos, input, length, width, font_id) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_init_device(double dvi2pts, int precision, int black_and_white);
void pdf_init_device(double dvi2pts, int precision, int black_and_white) {
  if (__pdf_entry_pdf_init_device(dvi2pts, precision, black_and_white) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_close_device(void);
void pdf_close_device(void) {
  if (__pdf_entry_pdf_close_device() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_reset_fonts(int newpage);
void pdf_dev_reset_fonts(int newpage) {
  if (__pdf_entry_pdf_dev_reset_fonts(newpage) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_reset_color(int force);
void pdf_dev_reset_color(int force) {
  if (__pdf_entry_pdf_dev_reset_color(force) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_bop(const pdf_tmatrix *M);
void pdf_dev_bop(const pdf_tmatrix *M) {
  if (__pdf_entry_pdf_dev_bop(M) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_eop(void);
void pdf_dev_eop(void) {
  if (__pdf_entry_pdf_dev_eop() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_locate_font(const char *font_name, spt_t ptsize, int *_result);
int pdf_dev_locate_font(const char *font_name, spt_t ptsize) {
  int _result;
  if (__pdf_entry_pdf_dev_locate_font(font_name, ptsize, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_set_rule(spt_t xpos, spt_t ypos, spt_t width, spt_t height);
void pdf_dev_set_rule(spt_t xpos, spt_t ypos, spt_t width, spt_t height) {
  if (__pdf_entry_pdf_dev_set_rule(xpos, ypos, width, height) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_set_rect(pdf_rect *rect, spt_t x_user, spt_t y_user, spt_t width, spt_t height, spt_t depth);
void pdf_dev_set_rect(pdf_rect *rect, spt_t x_user, spt_t y_user, spt_t width, spt_t height, spt_t depth) {
  if (__pdf_entry_pdf_dev_set_rect(rect, x_user, y_user, width, height, depth) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_get_dirmode(int *_result);
int pdf_dev_get_dirmode(void) {
  int _result;
  if (__pdf_entry_pdf_dev_get_dirmode(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_set_dirmode(int text_dir);
void pdf_dev_set_dirmode(int text_dir) {
  if (__pdf_entry_pdf_dev_set_dirmode(text_dir) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_get_param(int param_type, int *_result);
int pdf_dev_get_param(int param_type) {
  int _result;
  if (__pdf_entry_pdf_dev_get_param(param_type, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_set_param(int param_type, int value);
void pdf_dev_set_param(int param_type, int value) {
  if (__pdf_entry_pdf_dev_set_param(param_type, value) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_put_image(int id, transform_info *ti, double ref_x, double ref_y, pdf_rect *rect, int *_result);
int pdf_dev_put_image(int id, transform_info *ti, double ref_x, double ref_y, pdf_rect *rect) {
  int _result;
  if (__pdf_entry_pdf_dev_put_image(id, ti, ref_x, ref_y, rect, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_transform_info_clear(transform_info *info);
void transform_info_clear(transform_info *info) {
  if (__pdf_entry_transform_info_clear(info) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_begin_actualtext(uint16_t *unicodes, int count);
void pdf_dev_begin_actualtext(uint16_t *unicodes, int count) {
  if (__pdf_entry_pdf_dev_begin_actualtext(unicodes, count) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_end_actualtext(void);
void pdf_dev_end_actualtext(void) {
  if (__pdf_entry_pdf_dev_end_actualtext() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_graphics_mode(void);
void graphics_mode(void) {
  if (__pdf_entry_graphics_mode() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dev_unit_dviunit(double *_result);
double dev_unit_dviunit(void) {
  double _result;
  if (__pdf_entry_dev_unit_dviunit(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_sprint_matrix(char *buf, const pdf_tmatrix *M, int *_result);
int pdf_sprint_matrix(char *buf, const pdf_tmatrix *M) {
  int _result;
  if (__pdf_entry_pdf_sprint_matrix(buf, M, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_sprint_rect(char *buf, const pdf_rect *rect, int *_result);
int pdf_sprint_rect(char *buf, const pdf_rect *rect) {
  int _result;
  if (__pdf_entry_pdf_sprint_rect(buf, rect, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_sprint_coord(char *buf, const pdf_coord *c, int *_result);
int pdf_sprint_coord(char *buf, const pdf_coord *c) {
  int _result;
  if (__pdf_entry_pdf_sprint_coord(buf, c, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_sprint_length(char *buf, double value, int *_result);
int pdf_sprint_length(char *buf, double value) {
  int _result;
  if (__pdf_entry_pdf_sprint_length(buf, value, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_sprint_number(char *buf, double value, int *_result);
int pdf_sprint_number(char *buf, double value) {
  int _result;
  if (__pdf_entry_pdf_sprint_number(buf, value, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_get_font_wmode(int font_id, int *_result);
int pdf_dev_get_font_wmode(int font_id) {
  int _result;
  if (__pdf_entry_pdf_dev_get_font_wmode(font_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_font_minbytes(int font_id, int *_result);
int pdf_dev_font_minbytes(int font_id) {
  int _result;
  if (__pdf_entry_pdf_dev_font_minbytes(font_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_reset_global_state(void);
void pdf_dev_reset_global_state(void) {
  if (__pdf_entry_pdf_dev_reset_global_state() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_begin_page_plan(void);
void pdf_dev_begin_page_plan(void) {
  if (__pdf_entry_pdf_dev_begin_page_plan() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_end_page_plan(void);
void pdf_dev_end_page_plan(void) {
  if (__pdf_entry_pdf_dev_end_page_plan() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_tt_pdf_render_page(const tt_pdf_render_command *commands, size_t count, const tt_pdf_render_font *fonts, size_t font_count, double unit, int precision, int autorotate, int direction, void *output, int *_result);
int tt_pdf_render_page(const tt_pdf_render_command *commands, size_t count, const tt_pdf_render_font *fonts, size_t font_count, double unit, int precision, int autorotate, int direction, void *output) {
  int _result;
  if (__pdf_entry_tt_pdf_render_page(commands, count, fonts, font_count, unit, precision, autorotate, direction, output, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
