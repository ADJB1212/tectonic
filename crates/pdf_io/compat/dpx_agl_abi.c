#include "tectonic_bridge_core.h"
#include "dpx-agl.h"
#include <assert.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-pdfparse.h"
#include "dpx-unicode.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_agl_chop_suffix(const char *glyphname, char **suffix, char **_result);
char *agl_chop_suffix(const char *glyphname, char **suffix) {
  char *_result;
  if (__pdf_entry_agl_chop_suffix(glyphname, suffix, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_agl_suffix_to_otltag(const char *suffix, const char **_result);
const char *agl_suffix_to_otltag(const char *suffix) {
  const char *_result;
  if (__pdf_entry_agl_suffix_to_otltag(suffix, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_agl_init_map(void);
void agl_init_map(void) {
  if (__pdf_entry_agl_init_map() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_agl_close_map(void);
void agl_close_map(void) {
  if (__pdf_entry_agl_close_map() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_agl_lookup_list(const char *glyphname, agl_name **_result);
agl_name *agl_lookup_list(const char *glyphname) {
  agl_name *_result;
  if (__pdf_entry_agl_lookup_list(glyphname, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_agl_name_is_unicode(const char *glyphname, bool *_result);
bool agl_name_is_unicode(const char *glyphname) {
  bool _result;
  if (__pdf_entry_agl_name_is_unicode(glyphname, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_agl_name_convert_unicode(const char *glyphname, int32_t *_result);
int32_t agl_name_convert_unicode(const char *glyphname) {
  int32_t _result;
  if (__pdf_entry_agl_name_convert_unicode(glyphname, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_agl_sput_UTF16BE(const char *glyphstr, unsigned char **dstpp, unsigned char *limptr, int *fail_count, int32_t *_result);
int32_t agl_sput_UTF16BE(const char *glyphstr, unsigned char **dstpp, unsigned char *limptr, int *fail_count) {
  int32_t _result;
  if (__pdf_entry_agl_sput_UTF16BE(glyphstr, dstpp, limptr, fail_count, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_agl_get_unicodes(const char *glyphstr, int32_t *unicodes, int max_unicodes, int *_result);
int agl_get_unicodes(const char *glyphstr, int32_t *unicodes, int max_unicodes) {
  int _result;
  if (__pdf_entry_agl_get_unicodes(glyphstr, unicodes, max_unicodes, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
