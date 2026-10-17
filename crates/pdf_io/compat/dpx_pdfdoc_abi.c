#include "tectonic_bridge_core.h"
#include "dpx-pdfdoc.h"
#include <assert.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxutil.h"
#include "dpx-dvipdfmx.h"
#include "dpx-error.h"
#include "dpx-jpegimage.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfencrypt.h"
#include "dpx-pdffont.h"
#include "dpx-pdfnames.h"
#include "dpx-pdfobj.h"
#include "tectonic-pipeline.h"
#include "dpx-pdfresource.h"
#include "dpx-pdfximage.h"
#include "dpx-pngimage.h"
#include "dpx-system.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_doc_set_bop_content(const char *content, unsigned int length);
void pdf_doc_set_bop_content(const char *content, unsigned int length) {
  if (__pdf_entry_pdf_doc_set_bop_content(content, length) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_set_eop_content(const char *content, unsigned int length);
void pdf_doc_set_eop_content(const char *content, unsigned int length) {
  if (__pdf_entry_pdf_doc_set_eop_content(content, length) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_add_page_resource(const char *category, const char *resource_name, pdf_obj *resource_ref);
void pdf_doc_add_page_resource(const char *category, const char *resource_name, pdf_obj *resource_ref) {
  if (__pdf_entry_pdf_doc_add_page_resource(category, resource_name, resource_ref) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_get_page_count(pdf_file *pf, int *_result);
int pdf_doc_get_page_count(pdf_file *pf) {
  int _result;
  if (__pdf_entry_pdf_doc_get_page_count(pf, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_doc_get_page(pdf_file *pf, int page_no, enum pdf_page_boundary opt_bbox, pdf_rect *bbox, pdf_tmatrix *matrix, pdf_obj **resources_p, pdf_obj **_result);
pdf_obj *pdf_doc_get_page(pdf_file *pf, int page_no, enum pdf_page_boundary opt_bbox, pdf_rect *bbox, pdf_tmatrix *matrix, pdf_obj **resources_p) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_doc_get_page(pf, page_no, opt_bbox, bbox, matrix, resources_p, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_doc_bookmarks_up(int *_result);
int pdf_doc_bookmarks_up(void) {
  int _result;
  if (__pdf_entry_pdf_doc_bookmarks_up(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_doc_bookmarks_down(int *_result);
int pdf_doc_bookmarks_down(void) {
  int _result;
  if (__pdf_entry_pdf_doc_bookmarks_down(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_doc_bookmarks_depth(int *_result);
int pdf_doc_bookmarks_depth(void) {
  int _result;
  if (__pdf_entry_pdf_doc_bookmarks_depth(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_doc_bookmarks_add(pdf_obj *dict, int is_open);
void pdf_doc_bookmarks_add(pdf_obj *dict, int is_open) {
  if (__pdf_entry_pdf_doc_bookmarks_add(dict, is_open) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_add_names(const char *category, const void *key, int keylen, pdf_obj *value, int *_result);
int pdf_doc_add_names(const char *category, const void *key, int keylen, pdf_obj *value) {
  int _result;
  if (__pdf_entry_pdf_doc_add_names(category, key, keylen, value, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_doc_add_annot(unsigned int page_no, const pdf_rect *rect, pdf_obj *annot_dict, int new_annot);
void pdf_doc_add_annot(unsigned int page_no, const pdf_rect *rect, pdf_obj *annot_dict, int new_annot) {
  if (__pdf_entry_pdf_doc_add_annot(page_no, rect, annot_dict, new_annot) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_begin_article(const char *article_id, pdf_obj *article_info);
void pdf_doc_begin_article(const char *article_id, pdf_obj *article_info) {
  if (__pdf_entry_pdf_doc_begin_article(article_id, article_info) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_add_bead(const char *article_id, const char *bead_id, int page_no, const pdf_rect *rect);
void pdf_doc_add_bead(const char *article_id, const char *bead_id, int page_no, const pdf_rect *rect) {
  if (__pdf_entry_pdf_doc_add_bead(article_id, bead_id, page_no, rect) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_set_mediabox(unsigned int page_no, const pdf_rect *mediabox);
void pdf_doc_set_mediabox(unsigned int page_no, const pdf_rect *mediabox) {
  if (__pdf_entry_pdf_doc_set_mediabox(page_no, mediabox) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_current_page_resources(pdf_obj **_result);
pdf_obj *pdf_doc_current_page_resources(void) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_doc_current_page_resources(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_doc_get_dictionary(const char *category, pdf_obj **_result);
pdf_obj *pdf_doc_get_dictionary(const char *category) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_doc_get_dictionary(category, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_doc_current_page_number(int *_result);
int pdf_doc_current_page_number(void) {
  int _result;
  if (__pdf_entry_pdf_doc_current_page_number(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_doc_ref_page(unsigned int page_no, pdf_obj **_result);
pdf_obj *pdf_doc_ref_page(unsigned int page_no) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_doc_ref_page(page_no, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_doc_get_reference(const char *category, pdf_obj **_result);
pdf_obj *pdf_doc_get_reference(const char *category) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_doc_get_reference(category, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_doc_set_bgcolor(const pdf_color *color);
void pdf_doc_set_bgcolor(const pdf_color *color) {
  if (__pdf_entry_pdf_doc_set_bgcolor(color) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_begin_page(double scale, double x_origin, double y_origin);
void pdf_doc_begin_page(double scale, double x_origin, double y_origin) {
  if (__pdf_entry_pdf_doc_begin_page(scale, x_origin, y_origin) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_end_page(void);
void pdf_doc_end_page(void) {
  if (__pdf_entry_pdf_doc_end_page() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_add_page_content(const char *buffer, unsigned int length);
void pdf_doc_add_page_content(const char *buffer, unsigned int length) {
  if (__pdf_entry_pdf_doc_add_page_content(buffer, length) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_open_document(const char *filename, const char *creator, const unsigned char *id1, const unsigned char *id2, struct pdf_setting settings);
void pdf_open_document(const char *filename, const char *creator, const unsigned char *id1, const unsigned char *id2, struct pdf_setting settings) {
  if (__pdf_entry_pdf_open_document(filename, creator, id1, id2, settings) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_close_document(void);
void pdf_close_document(void) {
  if (__pdf_entry_pdf_close_document() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_begin_grabbing(const char *ident, double ref_x, double ref_y, const pdf_rect *cropbox, int *_result);
int pdf_doc_begin_grabbing(const char *ident, double ref_x, double ref_y, const pdf_rect *cropbox) {
  int _result;
  if (__pdf_entry_pdf_doc_begin_grabbing(ident, ref_x, ref_y, cropbox, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_doc_end_grabbing(pdf_obj *attrib);
void pdf_doc_end_grabbing(pdf_obj *attrib) {
  if (__pdf_entry_pdf_doc_end_grabbing(attrib) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_begin_annot(pdf_obj *dict);
void pdf_doc_begin_annot(pdf_obj *dict) {
  if (__pdf_entry_pdf_doc_begin_annot(dict) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_end_annot(void);
void pdf_doc_end_annot(void) {
  if (__pdf_entry_pdf_doc_end_annot() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_break_annot(void);
void pdf_doc_break_annot(void) {
  if (__pdf_entry_pdf_doc_break_annot() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_doc_expand_box(const pdf_rect *rect);
void pdf_doc_expand_box(const pdf_rect *rect) {
  if (__pdf_entry_pdf_doc_expand_box(rect) != 0) _tt_abort("%s", tt_pdf_last_error());
}
