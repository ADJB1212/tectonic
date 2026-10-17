#include "tectonic_bridge_core.h"
#include "dpx-pdffont.h"
#include <assert.h>
#include <errno.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-agl.h"
#include "dpx-cid.h"
#include "dpx-cidtype0.h"
#include "dpx-cmap.h"
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-pdfencoding.h"
#include "dpx-pdflimits.h"
#include "dpx-pdfobj.h"
#include "dpx-pkfont.h"
#include "dpx-truetype.h"
#include "dpx-tt_cmap.h"
#include "dpx-type0.h"
#include "dpx-type1.h"
#include "dpx-type1c.h"
#include "dpx-tfm.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_font_set_dpi(int font_dpi);
void pdf_font_set_dpi(int font_dpi) {
  if (__pdf_entry_pdf_font_set_dpi(font_dpi) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_font_reset_unique_tag_state(void);
void pdf_font_reset_unique_tag_state(void) {
  if (__pdf_entry_pdf_font_reset_unique_tag_state() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_font_set_deterministic_unique_tags(int value);
void pdf_font_set_deterministic_unique_tags(int value) {
  if (__pdf_entry_pdf_font_set_deterministic_unique_tags(value) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_font_make_uniqueTag(char *tag);
void pdf_font_make_uniqueTag(char *tag) {
  if (__pdf_entry_pdf_font_make_uniqueTag(tag) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_init_fonts(void);
void pdf_init_fonts(void) {
  if (__pdf_entry_pdf_init_fonts() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_abort_fonts(void);
void pdf_abort_fonts(void) {
  if (__pdf_entry_pdf_abort_fonts() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_get_font_data(int font_id, pdf_font **_result);
pdf_font *pdf_get_font_data(int font_id) {
  pdf_font *_result;
  if (__pdf_entry_pdf_get_font_data(font_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_get_font_ident(int font_id, char **_result);
char *pdf_get_font_ident(int font_id) {
  char *_result;
  if (__pdf_entry_pdf_get_font_ident(font_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_get_font_reference(int font_id, pdf_obj **_result);
pdf_obj *pdf_get_font_reference(int font_id) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_get_font_reference(font_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_get_font_resource(int font_id, pdf_obj **_result);
pdf_obj *pdf_get_font_resource(int font_id) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_get_font_resource(font_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_get_font_usedchars(int font_id, char **_result);
char *pdf_get_font_usedchars(int font_id) {
  char *_result;
  if (__pdf_entry_pdf_get_font_usedchars(font_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_get_font_wmode(int font_id, int *_result);
int pdf_get_font_wmode(int font_id) {
  int _result;
  if (__pdf_entry_pdf_get_font_wmode(font_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_get_font_subtype(int font_id, int *_result);
int pdf_get_font_subtype(int font_id) {
  int _result;
  if (__pdf_entry_pdf_get_font_subtype(font_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_get_font_encoding(int font_id, int *_result);
int pdf_get_font_encoding(int font_id) {
  int _result;
  if (__pdf_entry_pdf_get_font_encoding(font_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_font_resource_name(int font_id, char *buff, int *_result);
int pdf_font_resource_name(int font_id, char *buff) {
  int _result;
  if (__pdf_entry_pdf_font_resource_name(font_id, buff, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_close_fonts(void);
void pdf_close_fonts(void) {
  if (__pdf_entry_pdf_close_fonts() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_font_findresource(const char *ident, double scale, int *_result);
int pdf_font_findresource(const char *ident, double scale) {
  int _result;
  if (__pdf_entry_pdf_font_findresource(ident, scale, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_font_load_font(const char *ident, double font_scale, const fontmap_rec *mrec, int *_result);
int pdf_font_load_font(const char *ident, double font_scale, const fontmap_rec *mrec) {
  int _result;
  if (__pdf_entry_pdf_font_load_font(ident, font_scale, mrec, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_font_get_resource(pdf_font *font, pdf_obj **_result);
pdf_obj *pdf_font_get_resource(pdf_font *font) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_font_get_resource(font, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_font_get_descriptor(pdf_font *font, pdf_obj **_result);
pdf_obj *pdf_font_get_descriptor(pdf_font *font) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_font_get_descriptor(font, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_font_get_uniqueTag(pdf_font *font, char **_result);
char *pdf_font_get_uniqueTag(pdf_font *font) {
  char *_result;
  if (__pdf_entry_pdf_font_get_uniqueTag(font, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_check_tfm_widths(const char *ident, double *widths, int firstchar, int lastchar, const char *usedchars, int *_result);
int pdf_check_tfm_widths(const char *ident, double *widths, int firstchar, int lastchar, const char *usedchars) {
  int _result;
  if (__pdf_entry_pdf_check_tfm_widths(ident, widths, firstchar, lastchar, usedchars, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
