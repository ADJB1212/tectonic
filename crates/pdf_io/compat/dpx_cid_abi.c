#include "tectonic_bridge_core.h"
#include "dpx-cid.h"
#include <assert.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-cff.h"
#include "dpx-cff_types.h"
#include "dpx-cidtype0.h"
#include "dpx-cidtype2.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-pdfobj.h"
#include "dpx-cid_basefont.h"
#include "dpx-pdfparse.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_CIDFont_is_ACCFont(pdf_font *font, int *_result);
int CIDFont_is_ACCFont(pdf_font *font) {
  int _result;
  if (__pdf_entry_CIDFont_is_ACCFont(font, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CIDFont_is_UCSFont(pdf_font *font, int *_result);
int CIDFont_is_UCSFont(pdf_font *font) {
  int _result;
  if (__pdf_entry_CIDFont_is_UCSFont(font, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CIDFont_get_usedchars(pdf_font *font, char **_result);
char *CIDFont_get_usedchars(pdf_font *font) {
  char *_result;
  if (__pdf_entry_CIDFont_get_usedchars(font, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CIDFont_get_usedchars_v(pdf_font *font, char **_result);
char *CIDFont_get_usedchars_v(pdf_font *font) {
  char *_result;
  if (__pdf_entry_CIDFont_get_usedchars_v(font, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_font_load_cidfont(pdf_font *font);
void pdf_font_load_cidfont(pdf_font *font) {
  if (__pdf_entry_pdf_font_load_cidfont(font) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_font_cidfont_lookup_cache(pdf_font *fonts, int count, const char *map_name, CIDSysInfo *cmap_csi, const fontmap_opt *fmap_opt, int *_result);
int pdf_font_cidfont_lookup_cache(pdf_font *fonts, int count, const char *map_name, CIDSysInfo *cmap_csi, const fontmap_opt *fmap_opt) {
  int _result;
  if (__pdf_entry_pdf_font_cidfont_lookup_cache(fonts, count, map_name, cmap_csi, fmap_opt, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_font_open_cidfont(pdf_font *font, const char *map_name, CIDSysInfo *cmap_csi, const fontmap_opt *fmap_opt, int *_result);
int pdf_font_open_cidfont(pdf_font *font, const char *map_name, CIDSysInfo *cmap_csi, const fontmap_opt *fmap_opt) {
  int _result;
  if (__pdf_entry_pdf_font_open_cidfont(font, map_name, cmap_csi, fmap_opt, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CIDFont_set_flags(int flags);
void CIDFont_set_flags(int flags) {
  if (__pdf_entry_CIDFont_set_flags(flags) != 0) _tt_abort("%s", tt_pdf_last_error());
}
