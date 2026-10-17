#include "tectonic_bridge_core.h"
#include "dpx-pdfcolor.h"
#include <assert.h>
#include <ctype.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfresource.h"
#include "dpx-dpxcrypt.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_color_type(const pdf_color *color, int *_result);
int pdf_color_type(const pdf_color *color) {
  int _result;
  if (__pdf_entry_pdf_color_type(color, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_color_rgbcolor(pdf_color *color, double r, double g, double b, int *_result);
int pdf_color_rgbcolor(pdf_color *color, double r, double g, double b) {
  int _result;
  if (__pdf_entry_pdf_color_rgbcolor(color, r, g, b, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_color_cmykcolor(pdf_color *color, double c, double m, double y, double k, int *_result);
int pdf_color_cmykcolor(pdf_color *color, double c, double m, double y, double k) {
  int _result;
  if (__pdf_entry_pdf_color_cmykcolor(color, c, m, y, k, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_color_graycolor(pdf_color *color, double g, int *_result);
int pdf_color_graycolor(pdf_color *color, double g) {
  int _result;
  if (__pdf_entry_pdf_color_graycolor(color, g, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_color_spotcolor(pdf_color *color, char *name, double c, int *_result);
int pdf_color_spotcolor(pdf_color *color, char *name, double c) {
  int _result;
  if (__pdf_entry_pdf_color_spotcolor(color, name, c, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_color_copycolor(pdf_color *color1, const pdf_color *color2);
void pdf_color_copycolor(pdf_color *color1, const pdf_color *color2) {
  if (__pdf_entry_pdf_color_copycolor(color1, color2) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_color_brighten_color(pdf_color *dst, const pdf_color *src, double f);
void pdf_color_brighten_color(pdf_color *dst, const pdf_color *src, double f) {
  if (__pdf_entry_pdf_color_brighten_color(dst, src, f) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_color_is_white(const pdf_color *color, bool *_result);
bool pdf_color_is_white(const pdf_color *color) {
  bool _result;
  if (__pdf_entry_pdf_color_is_white(color, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_color_set_color(const pdf_color *color, char *buffer, size_t buffer_len, char mask, int *_result);
int pdf_color_set_color(const pdf_color *color, char *buffer, size_t buffer_len, char mask) {
  int _result;
  if (__pdf_entry_pdf_color_set_color(color, buffer, buffer_len, mask, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_color_compare(const pdf_color *color1, const pdf_color *color2, int *_result);
int pdf_color_compare(const pdf_color *color1, const pdf_color *color2) {
  int _result;
  if (__pdf_entry_pdf_color_compare(color1, color2, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_color_clear_stack(void);
void pdf_color_clear_stack(void) {
  if (__pdf_entry_pdf_color_clear_stack() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_color_set(pdf_color *sc, pdf_color *fc);
void pdf_color_set(pdf_color *sc, pdf_color *fc) {
  if (__pdf_entry_pdf_color_set(sc, fc) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_color_push(pdf_color *sc, pdf_color *fc);
void pdf_color_push(pdf_color *sc, pdf_color *fc) {
  if (__pdf_entry_pdf_color_push(sc, fc) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_color_pop(void);
void pdf_color_pop(void) {
  if (__pdf_entry_pdf_color_pop() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_color_get_current(pdf_color **sc, pdf_color **fc);
void pdf_color_get_current(pdf_color **sc, pdf_color **fc) {
  if (__pdf_entry_pdf_color_get_current(sc, fc) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_iccp_check_colorspace(int colortype, const void *profile, int proflen, int *_result);
int iccp_check_colorspace(int colortype, const void *profile, int proflen) {
  int _result;
  if (__pdf_entry_iccp_check_colorspace(colortype, profile, proflen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_iccp_get_rendering_intent(const void *profile, int proflen, pdf_obj **_result);
pdf_obj *iccp_get_rendering_intent(const void *profile, int proflen) {
  pdf_obj *_result;
  if (__pdf_entry_iccp_get_rendering_intent(profile, proflen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_iccp_load_profile(const char *ident, const void *profile, int proflen, int *_result);
int iccp_load_profile(const char *ident, const void *profile, int proflen) {
  int _result;
  if (__pdf_entry_iccp_load_profile(ident, profile, proflen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_get_colorspace_reference(int cspc_id, pdf_obj **_result);
pdf_obj *pdf_get_colorspace_reference(int cspc_id) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_get_colorspace_reference(cspc_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_init_colors(void);
void pdf_init_colors(void) {
  if (__pdf_entry_pdf_init_colors() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_close_colors(void);
void pdf_close_colors(void) {
  if (__pdf_entry_pdf_close_colors() != 0) _tt_abort("%s", tt_pdf_last_error());
}
