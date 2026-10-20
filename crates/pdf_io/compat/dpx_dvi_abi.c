#include "tectonic_bridge_core.h"
#include "dpx-dvi.h"
#include "tectonic-pipeline.h"
#include <fcntl.h>
#include <inttypes.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include "tectonic_bridge_core.h"
#include "dpx-cff.h"
#include "dpx-cff_dict.h"
#include "dpx-cff_types.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-dpxutil.h"
#include "dpx-dvipdfmx.h"
#include "dpx-error.h"
#include "dpx-fontmap.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfparse.h"
#include "dpx-pdfresource.h"
#include "dpx-sfnt.h"
#include "dpx-specials.h"
#include "dpx-subfont.h"
#include "dpx-t1_char.h"
#include "dpx-t1_load.h"
#include "dpx-tfm.h"
#include "dpx-tt_aux.h"
#include "dpx-tt_table.h"
#include "dpx-vf.h"
#include "dpx-dpxutil.h"
#include "dpx-pdfencrypt.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_get_origin(int x, double *_result);
double get_origin(int x) {
  double _result;
  if (__pdf_entry_get_origin(x, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dvi_npages(unsigned int *_result);
unsigned int dvi_npages(void) {
  unsigned int _result;
  if (__pdf_entry_dvi_npages(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dvi_tell_mag(double *_result);
double dvi_tell_mag(void) {
  double _result;
  if (__pdf_entry_dvi_tell_mag(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dvi_comment(const char **_result);
const char *dvi_comment(void) {
  const char *_result;
  if (__pdf_entry_dvi_comment(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dvi_tag_depth(void);
void dvi_tag_depth(void) {
  if (__pdf_entry_dvi_tag_depth() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_untag_depth(void);
void dvi_untag_depth(void) {
  if (__pdf_entry_dvi_untag_depth() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_compute_boxes(int flag);
void dvi_compute_boxes(int flag) {
  if (__pdf_entry_dvi_compute_boxes(flag) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_link_annot(int flag);
void dvi_link_annot(int flag) {
  if (__pdf_entry_dvi_link_annot(flag) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_is_tracking_boxes(bool *_result);
bool dvi_is_tracking_boxes(void) {
  bool _result;
  if (__pdf_entry_dvi_is_tracking_boxes(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dvi_set_linkmode(int mode);
void dvi_set_linkmode(int mode) {
  if (__pdf_entry_dvi_set_linkmode(mode) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_set_phantom_height(double height, double depth);
void dvi_set_phantom_height(double height, double depth) {
  if (__pdf_entry_dvi_set_phantom_height(height, depth) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_set_compensation(double x, double y);
void dvi_set_compensation(double x, double y) {
  if (__pdf_entry_dvi_set_compensation(x, y) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_do_special(const void *buffer, int32_t size);
void dvi_do_special(const void *buffer, int32_t size) {
  if (__pdf_entry_dvi_do_special(buffer, size) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_unit_size(double *_result);
double dvi_unit_size(void) {
  double _result;
  if (__pdf_entry_dvi_unit_size(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dvi_locate_font(const char *tfm_name, spt_t ptsize, unsigned int *_result);
unsigned int dvi_locate_font(const char *tfm_name, spt_t ptsize) {
  unsigned int _result;
  if (__pdf_entry_dvi_locate_font(tfm_name, ptsize, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dvi_dev_xpos(double *_result);
double dvi_dev_xpos(void) {
  double _result;
  if (__pdf_entry_dvi_dev_xpos(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dvi_dev_ypos(double *_result);
double dvi_dev_ypos(void) {
  double _result;
  if (__pdf_entry_dvi_dev_ypos(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dvi_right(int32_t x);
void dvi_right(int32_t x) {
  if (__pdf_entry_dvi_right(x) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_down(int32_t y);
void dvi_down(int32_t y) {
  if (__pdf_entry_dvi_down(y) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_set(int32_t ch);
void dvi_set(int32_t ch) {
  if (__pdf_entry_dvi_set(ch) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_put(int32_t ch);
void dvi_put(int32_t ch) {
  if (__pdf_entry_dvi_put(ch) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_rule(int32_t width, int32_t height);
void dvi_rule(int32_t width, int32_t height) {
  if (__pdf_entry_dvi_rule(width, height) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_dirchg(unsigned char dir);
void dvi_dirchg(unsigned char dir) {
  if (__pdf_entry_dvi_dirchg(dir) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_push(void);
void dvi_push(void) {
  if (__pdf_entry_dvi_push() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dpx_dvi_pop(void);
void dpx_dvi_pop(void) {
  if (__pdf_entry_dpx_dvi_pop() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_w(int32_t ch);
void dvi_w(int32_t ch) {
  if (__pdf_entry_dvi_w(ch) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_w0(void);
void dvi_w0(void) {
  if (__pdf_entry_dvi_w0() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_x(int32_t ch);
void dvi_x(int32_t ch) {
  if (__pdf_entry_dvi_x(ch) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_x0(void);
void dvi_x0(void) {
  if (__pdf_entry_dvi_x0() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_y(int32_t ch);
void dvi_y(int32_t ch) {
  if (__pdf_entry_dvi_y(ch) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_y0(void);
void dvi_y0(void) {
  if (__pdf_entry_dvi_y0() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_z(int32_t ch);
void dvi_z(int32_t ch) {
  if (__pdf_entry_dvi_z(ch) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_z0(void);
void dvi_z0(void) {
  if (__pdf_entry_dvi_z0() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_set_font(int font_id);
void dvi_set_font(int font_id) {
  if (__pdf_entry_dvi_set_font(font_id) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_do_page(double page_paper_height, double hmargin, double vmargin);
void dvi_do_page(double page_paper_height, double hmargin, double vmargin) {
  if (__pdf_entry_dvi_do_page(page_paper_height, hmargin, vmargin) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_init(const char *dvi_filename, double mag, double *_result);
double dvi_init(const char *dvi_filename, double mag) {
  double _result;
  if (__pdf_entry_dvi_init(dvi_filename, mag, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dvi_close(void);
void dvi_close(void) {
  if (__pdf_entry_dvi_close() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_vf_init(int dev_font_id);
void dvi_vf_init(int dev_font_id) {
  if (__pdf_entry_dvi_vf_init(dev_font_id) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_vf_finish(void);
void dvi_vf_finish(void) {
  if (__pdf_entry_dvi_vf_finish() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_scan_specials(int page_no, double *page_width, double *page_height, double *x_offset, double *y_offset, int *landscape, int *majorversion, int *minorversion, int *compression_level, double *annot_grow_x, double *annot_grow_y, int *do_enc, int *key_bits, int32_t *permission, char *owner_pw, char *user_pw, int *has_id, unsigned char *id1, unsigned char *id2, int *opt_flags);
void dvi_scan_specials(int page_no, double *page_width, double *page_height, double *x_offset, double *y_offset, int *landscape, int *majorversion, int *minorversion, int *compression_level, double *annot_grow_x, double *annot_grow_y, int *do_enc, int *key_bits, int32_t *permission, char *owner_pw, char *user_pw, int *has_id, unsigned char *id1, unsigned char *id2, int *opt_flags) {
  if (__pdf_entry_dvi_scan_specials(page_no, page_width, page_height, x_offset, y_offset, landscape, majorversion, minorversion, compression_level, annot_grow_x, annot_grow_y, do_enc, key_bits, permission, owner_pw, user_pw, has_id, id1, id2, opt_flags) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_dvi_parallel_eligible(int *_result);
int dvi_parallel_eligible(void) {
  int _result;
  if (__pdf_entry_dvi_parallel_eligible(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_dvi_reset_global_state(void);
void dvi_reset_global_state(void) {
  if (__pdf_entry_dvi_reset_global_state() != 0) _tt_abort("%s", tt_pdf_last_error());
}
