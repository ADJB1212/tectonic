#include "tectonic_bridge_core.h"
#include <assert.h>
#include <ctype.h>
#include <math.h>
#include <limits.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-dpxconf.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-pdfencrypt.h"
#include "dpx-pdflimits.h"
#include "dpx-pdfparse.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfdev.h"
#include "tectonic_bridge_flate.h"
#include "tectonic-predictor.h"
#include "tectonic-compress.h"
#include "tectonic-pipeline.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_get_output_file(rust_output_handle_t *_result);
rust_output_handle_t pdf_get_output_file(void) {
  rust_output_handle_t _result;
  if (__pdf_entry_pdf_get_output_file(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_get_version(int *_result);
int pdf_get_version(void) {
  int _result;
  if (__pdf_entry_pdf_get_version(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_get_version_major(int *_result);
int pdf_get_version_major(void) {
  int _result;
  if (__pdf_entry_pdf_get_version_major(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_get_version_minor(int *_result);
int pdf_get_version_minor(void) {
  int _result;
  if (__pdf_entry_pdf_get_version_minor(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_check_version(int major, int minor, int *_result);
int pdf_check_version(int major, int minor) {
  int _result;
  if (__pdf_entry_pdf_check_version(major, minor, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_out_init(const char *filename, const unsigned char *id1, const unsigned char *id2, int ver_major, int ver_minor, int compression_level, int enable_encrypt, int enable_objstm, int enable_predictor, pdf_out **_result);
pdf_out *pdf_out_init(const char *filename, const unsigned char *id1, const unsigned char *id2, int ver_major, int ver_minor, int compression_level, int enable_encrypt, int enable_objstm, int enable_predictor) {
  pdf_out *_result;
  if (__pdf_entry_pdf_out_init(filename, id1, id2, ver_major, ver_minor, compression_level, enable_encrypt, enable_objstm, enable_predictor, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_out_set_encrypt(int keybits, int32_t permission, const char *opasswd, const char *upasswd, int use_aes, int encrypt_metadata);
void pdf_out_set_encrypt(int keybits, int32_t permission, const char *opasswd, const char *upasswd, int use_aes, int encrypt_metadata) {
  if (__pdf_entry_pdf_out_set_encrypt(keybits, permission, opasswd, upasswd, use_aes, encrypt_metadata) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_out_flush(void);
void pdf_out_flush(void) {
  if (__pdf_entry_pdf_out_flush() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_error_cleanup(void);
void pdf_error_cleanup(void) {
  if (__pdf_entry_pdf_error_cleanup() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_set_root(pdf_obj *object);
void pdf_set_root(pdf_obj *object) {
  if (__pdf_entry_pdf_set_root(object) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_set_info(pdf_obj *object);
void pdf_set_info(pdf_obj *object) {
  if (__pdf_entry_pdf_set_info(object) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_obj_typeof(pdf_obj *object, int *_result);
int pdf_obj_typeof(pdf_obj *object) {
  int _result;
  if (__pdf_entry_pdf_obj_typeof(object, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_transfer_label(pdf_obj *dst, pdf_obj *src);
void pdf_transfer_label(pdf_obj *dst, pdf_obj *src) {
  if (__pdf_entry_pdf_transfer_label(dst, src) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_link_obj(pdf_obj *object, pdf_obj **_result);
pdf_obj *pdf_link_obj(pdf_obj *object) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_link_obj(object, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_ref_obj(pdf_obj *object, pdf_obj **_result);
pdf_obj *pdf_ref_obj(pdf_obj *object) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_ref_obj(object, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_new_undefined(pdf_obj **_result);
pdf_obj *pdf_new_undefined(void) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_new_undefined(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_new_null(pdf_obj **_result);
pdf_obj *pdf_new_null(void) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_new_null(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_new_boolean(char value, pdf_obj **_result);
pdf_obj *pdf_new_boolean(char value) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_new_boolean(value, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_boolean_value(pdf_obj *object, char *_result);
char pdf_boolean_value(pdf_obj *object) {
  char _result;
  if (__pdf_entry_pdf_boolean_value(object, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_new_number(double value, pdf_obj **_result);
pdf_obj *pdf_new_number(double value) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_new_number(value, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_set_number(pdf_obj *object, double value);
void pdf_set_number(pdf_obj *object, double value) {
  if (__pdf_entry_pdf_set_number(object, value) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_number_value(pdf_obj *object, double *_result);
double pdf_number_value(pdf_obj *object) {
  double _result;
  if (__pdf_entry_pdf_number_value(object, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_new_string(const void *str, size_t length, pdf_obj **_result);
pdf_obj *pdf_new_string(const void *str, size_t length) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_new_string(str, length, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_string_value(pdf_obj *object, void **_result);
void *pdf_string_value(pdf_obj *object) {
  void *_result;
  if (__pdf_entry_pdf_string_value(object, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_string_length(pdf_obj *object, unsigned int *_result);
unsigned int pdf_string_length(pdf_obj *object) {
  unsigned int _result;
  if (__pdf_entry_pdf_string_length(object, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdfobj_escape_str(char *buffer, size_t bufsize, const unsigned char *s, size_t len, size_t *_result);
size_t pdfobj_escape_str(char *buffer, size_t bufsize, const unsigned char *s, size_t len) {
  size_t _result;
  if (__pdf_entry_pdfobj_escape_str(buffer, bufsize, s, len, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_set_string(pdf_obj *object, unsigned char *str, size_t length);
void pdf_set_string(pdf_obj *object, unsigned char *str, size_t length) {
  if (__pdf_entry_pdf_set_string(object, str, length) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_new_name(const char *name, pdf_obj **_result);
pdf_obj *pdf_new_name(const char *name) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_new_name(name, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_name_value(pdf_obj *object, char **_result);
char *pdf_name_value(pdf_obj *object) {
  char *_result;
  if (__pdf_entry_pdf_name_value(object, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_new_array(pdf_obj **_result);
pdf_obj *pdf_new_array(void) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_new_array(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_get_array(pdf_obj *array, int idx, pdf_obj **_result);
pdf_obj *pdf_get_array(pdf_obj *array, int idx) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_get_array(array, idx, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_array_length(pdf_obj *array, unsigned int *_result);
unsigned int pdf_array_length(pdf_obj *array) {
  unsigned int _result;
  if (__pdf_entry_pdf_array_length(array, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_add_array(pdf_obj *array, pdf_obj *object);
void pdf_add_array(pdf_obj *array, pdf_obj *object) {
  if (__pdf_entry_pdf_add_array(array, object) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_new_dict(pdf_obj **_result);
pdf_obj *pdf_new_dict(void) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_new_dict(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_add_dict(pdf_obj *dict, pdf_obj *key, pdf_obj *value, int *_result);
int pdf_add_dict(pdf_obj *dict, pdf_obj *key, pdf_obj *value) {
  int _result;
  if (__pdf_entry_pdf_add_dict(dict, key, value, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_merge_dict(pdf_obj *dict1, pdf_obj *dict2);
void pdf_merge_dict(pdf_obj *dict1, pdf_obj *dict2) {
  if (__pdf_entry_pdf_merge_dict(dict1, dict2) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_foreach_dict(pdf_obj *dict, int (*proc_0)(pdf_obj *, pdf_obj *, void *), void *pdata, int *_result);
int pdf_foreach_dict(pdf_obj *dict, int (*proc_0)(pdf_obj *, pdf_obj *, void *), void *pdata) {
  int _result;
  if (__pdf_entry_pdf_foreach_dict(dict, proc_0, pdata, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_lookup_dict(pdf_obj *dict, const char *name, pdf_obj **_result);
pdf_obj *pdf_lookup_dict(pdf_obj *dict, const char *name) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_lookup_dict(dict, name, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dict_keys(pdf_obj *dict, pdf_obj **_result);
pdf_obj *pdf_dict_keys(pdf_obj *dict) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_dict_keys(dict, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_remove_dict(pdf_obj *dict, const char *name);
void pdf_remove_dict(pdf_obj *dict, const char *name) {
  if (__pdf_entry_pdf_remove_dict(dict, name) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_new_stream(int flags, pdf_obj **_result);
pdf_obj *pdf_new_stream(int flags) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_new_stream(flags, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_stream_set_predictor(pdf_obj *stream, int predictor, int32_t columns, int bpc, int colors);
void pdf_stream_set_predictor(pdf_obj *stream, int predictor, int32_t columns, int bpc, int colors) {
  if (__pdf_entry_pdf_stream_set_predictor(stream, predictor, columns, bpc, colors) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_tt_pdf_filter_png_rows(const unsigned char *raster, unsigned char *dst, int32_t rowbytes, int bytes_per_pixel, int32_t first_row, int32_t last_row);
void tt_pdf_filter_png_rows(const unsigned char *raster, unsigned char *dst, int32_t rowbytes, int bytes_per_pixel, int32_t first_row, int32_t last_row) {
  if (__pdf_entry_tt_pdf_filter_png_rows(raster, dst, rowbytes, bytes_per_pixel, first_row, last_row) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_stream_dict(pdf_obj *stream, pdf_obj **_result);
pdf_obj *pdf_stream_dict(pdf_obj *stream) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_stream_dict(stream, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_stream_dataptr(pdf_obj *stream, const void **_result);
const void *pdf_stream_dataptr(pdf_obj *stream) {
  const void *_result;
  if (__pdf_entry_pdf_stream_dataptr(stream, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_stream_length(pdf_obj *stream, int *_result);
int pdf_stream_length(pdf_obj *stream) {
  int _result;
  if (__pdf_entry_pdf_stream_length(stream, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_add_stream(pdf_obj *stream, const void *stream_data, int length);
void pdf_add_stream(pdf_obj *stream, const void *stream_data, int length) {
  if (__pdf_entry_pdf_add_stream(stream, stream_data, length) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_concat_stream(pdf_obj *dst, pdf_obj *src, int *_result);
int pdf_concat_stream(pdf_obj *dst, pdf_obj *src) {
  int _result;
  if (__pdf_entry_pdf_concat_stream(dst, src, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_pipeline_drain(int all);
void pdf_pipeline_drain(int all) {
  if (__pdf_entry_pdf_pipeline_drain(all) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_submit_page(pdf_obj *contents, int *_result);
int pdf_submit_page(pdf_obj *contents) {
  int _result;
  if (__pdf_entry_pdf_submit_page(contents, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_release_obj(pdf_obj *object);
void pdf_release_obj(pdf_obj *object) {
  if (__pdf_entry_pdf_release_obj(object) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_new_indirect(pdf_file *pf, uint32_t obj_num, uint16_t obj_gen, pdf_obj **_result);
pdf_obj *pdf_new_indirect(pdf_file *pf, uint32_t obj_num, uint16_t obj_gen) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_new_indirect(pf, obj_num, obj_gen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_deref_obj(pdf_obj *obj, pdf_obj **_result);
pdf_obj *pdf_deref_obj(pdf_obj *obj) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_deref_obj(obj, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_files_init(void);
void pdf_files_init(void) {
  if (__pdf_entry_pdf_files_init() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_file_get_version(pdf_file *pf, int *_result);
int pdf_file_get_version(pdf_file *pf) {
  int _result;
  if (__pdf_entry_pdf_file_get_version(pf, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_file_get_trailer(pdf_file *pf, pdf_obj **_result);
pdf_obj *pdf_file_get_trailer(pdf_file *pf) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_file_get_trailer(pf, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_file_get_catalog(pdf_file *pf, pdf_obj **_result);
pdf_obj *pdf_file_get_catalog(pdf_file *pf) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_file_get_catalog(pf, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_open(const char *ident, rust_input_handle_t handle, pdf_file **_result);
pdf_file *pdf_open(const char *ident, rust_input_handle_t handle) {
  pdf_file *_result;
  if (__pdf_entry_pdf_open(ident, handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_close(pdf_file *pf);
void pdf_close(pdf_file *pf) {
  if (__pdf_entry_pdf_close(pf) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_files_close(void);
void pdf_files_close(void) {
  if (__pdf_entry_pdf_files_close() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_check_for_pdf(rust_input_handle_t handle, int *_result);
int check_for_pdf(rust_input_handle_t handle) {
  int _result;
  if (__pdf_entry_check_for_pdf(handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_import_object(pdf_obj *object, pdf_obj **_result);
pdf_obj *pdf_import_object(pdf_obj *object) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_import_object(object, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_compare_reference(pdf_obj *ref1, pdf_obj *ref2, int *_result);
int pdf_compare_reference(pdf_obj *ref1, pdf_obj *ref2) {
  int _result;
  if (__pdf_entry_pdf_compare_reference(ref1, ref2, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_compare_object(pdf_obj *obj1, pdf_obj *obj2, int *_result);
int pdf_compare_object(pdf_obj *obj1, pdf_obj *obj2) {
  int _result;
  if (__pdf_entry_pdf_compare_object(obj1, obj2, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_obj_reset_global_state(void);
void pdf_obj_reset_global_state(void) {
  if (__pdf_entry_pdf_obj_reset_global_state() != 0) _tt_abort("%s", tt_pdf_last_error());
}
