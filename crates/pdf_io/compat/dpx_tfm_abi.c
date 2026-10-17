#include "tectonic_bridge_core.h"
#include "dpx-tfm.h"
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "tectonic-tfm.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_tfm_reset_global_state(void);
void tfm_reset_global_state(void) {
  if (__pdf_entry_tfm_reset_global_state() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_tfm_open(const char *tfm_name, int must_exist, int *_result);
int tfm_open(const char *tfm_name, int must_exist) {
  int _result;
  if (__pdf_entry_tfm_open(tfm_name, must_exist, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tfm_close_all(void);
void tfm_close_all(void) {
  if (__pdf_entry_tfm_close_all() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_tfm_get_fw_width(int font_id, int32_t ch, fixword *_result);
fixword tfm_get_fw_width(int font_id, int32_t ch) {
  fixword _result;
  if (__pdf_entry_tfm_get_fw_width(font_id, ch, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tfm_get_fw_height(int font_id, int32_t ch, fixword *_result);
fixword tfm_get_fw_height(int font_id, int32_t ch) {
  fixword _result;
  if (__pdf_entry_tfm_get_fw_height(font_id, ch, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tfm_get_fw_depth(int font_id, int32_t ch, fixword *_result);
fixword tfm_get_fw_depth(int font_id, int32_t ch) {
  fixword _result;
  if (__pdf_entry_tfm_get_fw_depth(font_id, ch, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tfm_get_width(int font_id, int32_t ch, double *_result);
double tfm_get_width(int font_id, int32_t ch) {
  double _result;
  if (__pdf_entry_tfm_get_width(font_id, ch, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tfm_string_width(int font_id, const unsigned char *s, unsigned int len, fixword *_result);
fixword tfm_string_width(int font_id, const unsigned char *s, unsigned int len) {
  fixword _result;
  if (__pdf_entry_tfm_string_width(font_id, s, len, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tfm_get_design_size(int font_id, double *_result);
double tfm_get_design_size(int font_id) {
  double _result;
  if (__pdf_entry_tfm_get_design_size(font_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tfm_is_jfm(int font_id, int *_result);
int tfm_is_jfm(int font_id) {
  int _result;
  if (__pdf_entry_tfm_is_jfm(font_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tfm_exists(const char *tfm_name, bool *_result);
bool tfm_exists(const char *tfm_name) {
  bool _result;
  if (__pdf_entry_tfm_exists(tfm_name, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
