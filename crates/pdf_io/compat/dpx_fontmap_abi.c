#include "tectonic_bridge_core.h"
#include "dpx-fontmap.h"
#include "tectonic_bridge_core.h"
#include "tectonic-fontmap.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-subfont.h"
#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_init_fontmap_record(fontmap_rec *mrec);
void pdf_init_fontmap_record(fontmap_rec *mrec) {
  if (__pdf_entry_pdf_init_fontmap_record(mrec) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_clear_fontmap_record(fontmap_rec *mrec);
void pdf_clear_fontmap_record(fontmap_rec *mrec) {
  if (__pdf_entry_pdf_clear_fontmap_record(mrec) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_append_fontmap_record(const char *kp, const fontmap_rec *vp, int *_result);
int pdf_append_fontmap_record(const char *kp, const fontmap_rec *vp) {
  int _result;
  if (__pdf_entry_pdf_append_fontmap_record(kp, vp, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_remove_fontmap_record(const char *kp, int *_result);
int pdf_remove_fontmap_record(const char *kp) {
  int _result;
  if (__pdf_entry_pdf_remove_fontmap_record(kp, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_insert_fontmap_record(const char *kp, const fontmap_rec *vp, fontmap_rec **_result);
fontmap_rec *pdf_insert_fontmap_record(const char *kp, const fontmap_rec *vp) {
  fontmap_rec *_result;
  if (__pdf_entry_pdf_insert_fontmap_record(kp, vp, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_read_fontmap_line(fontmap_rec *mrec, const char *mline, int mline_len, int format, int *_result);
int pdf_read_fontmap_line(fontmap_rec *mrec, const char *mline, int mline_len, int format) {
  int _result;
  if (__pdf_entry_pdf_read_fontmap_line(mrec, mline, mline_len, format, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_is_pdfm_mapline(const char *mline, int *_result);
int is_pdfm_mapline(const char *mline) {
  int _result;
  if (__pdf_entry_is_pdfm_mapline(mline, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_load_fontmap_file(const char *filename, int mode, int *_result);
int pdf_load_fontmap_file(const char *filename, int mode) {
  int _result;
  if (__pdf_entry_pdf_load_fontmap_file(filename, mode, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_insert_native_fontmap_record(const char *path, uint32_t index, int layout_dir, int extend, int slant, int embolden, fontmap_rec **_result);
fontmap_rec *pdf_insert_native_fontmap_record(const char *path, uint32_t index, int layout_dir, int extend, int slant, int embolden) {
  fontmap_rec *_result;
  if (__pdf_entry_pdf_insert_native_fontmap_record(path, index, layout_dir, extend, slant, embolden, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_lookup_fontmap_record(const char *tfm_name, fontmap_rec **_result);
fontmap_rec *pdf_lookup_fontmap_record(const char *tfm_name) {
  fontmap_rec *_result;
  if (__pdf_entry_pdf_lookup_fontmap_record(tfm_name, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_init_fontmaps(void);
void pdf_init_fontmaps(void) {
  if (__pdf_entry_pdf_init_fontmaps() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_close_fontmaps(void);
void pdf_close_fontmaps(void) {
  if (__pdf_entry_pdf_close_fontmaps() != 0) _tt_abort("%s", tt_pdf_last_error());
}
