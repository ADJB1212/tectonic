#include "tectonic_bridge_core.h"
#include "dpx-pdfximage.h"
#include <fcntl.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-bmpimage.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-dpxutil.h"
#include "dpx-epdf.h"
#include "dpx-error.h"
#include "dpx-jpegimage.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfnames.h"
#include "dpx-pdfobj.h"
#include "dpx-pngimage.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_init_images(void);
void pdf_init_images(void) {
  if (__pdf_entry_pdf_init_images() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_close_images(void);
void pdf_close_images(void) {
  if (__pdf_entry_pdf_close_images() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_ximage_load_image(const char *ident, const char *filename, load_options options, int *_result);
int pdf_ximage_load_image(const char *ident, const char *filename, load_options options) {
  int _result;
  if (__pdf_entry_pdf_ximage_load_image(ident, filename, options, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_ximage_findresource(const char *ident, int *_result);
int pdf_ximage_findresource(const char *ident) {
  int _result;
  if (__pdf_entry_pdf_ximage_findresource(ident, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_ximage_init_form_info(xform_info *info);
void pdf_ximage_init_form_info(xform_info *info) {
  if (__pdf_entry_pdf_ximage_init_form_info(info) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_ximage_init_image_info(ximage_info *info);
void pdf_ximage_init_image_info(ximage_info *info) {
  if (__pdf_entry_pdf_ximage_init_image_info(info) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_ximage_set_image(pdf_ximage *I, void *image_info, pdf_obj *resource);
void pdf_ximage_set_image(pdf_ximage *I, void *image_info, pdf_obj *resource) {
  if (__pdf_entry_pdf_ximage_set_image(I, image_info, resource) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_ximage_set_form(pdf_ximage *I, void *form_info, pdf_obj *resource);
void pdf_ximage_set_form(pdf_ximage *I, void *form_info, pdf_obj *resource) {
  if (__pdf_entry_pdf_ximage_set_form(I, form_info, resource) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_ximage_get_page(pdf_ximage *I, int *_result);
int pdf_ximage_get_page(pdf_ximage *I) {
  int _result;
  if (__pdf_entry_pdf_ximage_get_page(I, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_ximage_get_reference(int id, pdf_obj **_result);
pdf_obj *pdf_ximage_get_reference(int id) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_ximage_get_reference(id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_ximage_defineresource(const char *ident, int subtype, void *info, pdf_obj *resource, int *_result);
int pdf_ximage_defineresource(const char *ident, int subtype, void *info, pdf_obj *resource) {
  int _result;
  if (__pdf_entry_pdf_ximage_defineresource(ident, subtype, info, resource, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_ximage_reserve(const char *ident, int *_result);
int pdf_ximage_reserve(const char *ident) {
  int _result;
  if (__pdf_entry_pdf_ximage_reserve(ident, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_ximage_get_resname(int id, char **_result);
char *pdf_ximage_get_resname(int id) {
  char *_result;
  if (__pdf_entry_pdf_ximage_get_resname(id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_ximage_get_subtype(int id, int *_result);
int pdf_ximage_get_subtype(int id) {
  int _result;
  if (__pdf_entry_pdf_ximage_get_subtype(id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_ximage_set_attr(int id, int width, int height, double xdensity, double ydensity, double llx, double lly, double urx, double ury);
void pdf_ximage_set_attr(int id, int width, int height, double xdensity, double ydensity, double llx, double lly, double urx, double ury) {
  if (__pdf_entry_pdf_ximage_set_attr(id, width, height, xdensity, ydensity, llx, lly, urx, ury) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_ximage_scale_image(int id, pdf_tmatrix *M, pdf_rect *r, transform_info *p, int *_result);
int pdf_ximage_scale_image(int id, pdf_tmatrix *M, pdf_rect *r, transform_info *p) {
  int _result;
  if (__pdf_entry_pdf_ximage_scale_image(id, M, r, p, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_set_distiller_template(char *s);
void set_distiller_template(char *s) {
  if (__pdf_entry_set_distiller_template(s) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_get_distiller_template(char **_result);
char *get_distiller_template(void) {
  char *_result;
  if (__pdf_entry_get_distiller_template(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_error_cleanup_cache(void);
void pdf_error_cleanup_cache(void) {
  if (__pdf_entry_pdf_error_cleanup_cache() != 0) _tt_abort("%s", tt_pdf_last_error());
}
