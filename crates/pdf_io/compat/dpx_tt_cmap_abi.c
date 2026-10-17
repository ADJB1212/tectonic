#include "tectonic_bridge_core.h"
#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-tt_cmap.h"
#include "tectonic_bridge_core.h"
#include "dpx-agl.h"
#include "dpx-cmap.h"
#include "dpx-cmap_write.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-pdfresource.h"
#include "dpx-sfnt.h"
#include "dpx-tt_aux.h"
#include "dpx-tt_gsub.h"
#include "dpx-tt_post.h"
#include "dpx-type0.h"
#include "dpx-unicode.h"
#include "dpx-cff.h"
#include "dpx-cff_dict.h"
#include "dpx-cff_types.h"
#include "dpx-cid.h"
#include "dpx-tt_table.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_tt_cmap_read(sfnt *sfont, USHORT platform, USHORT encoding, tt_cmap **_result);
tt_cmap *tt_cmap_read(sfnt *sfont, USHORT platform, USHORT encoding) {
  tt_cmap *_result;
  if (__pdf_entry_tt_cmap_read(sfont, platform, encoding, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_cmap_release(tt_cmap *cmap);
void tt_cmap_release(tt_cmap *cmap) {
  if (__pdf_entry_tt_cmap_release(cmap) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_tt_cmap_lookup(tt_cmap *cmap, SFNT_ULONG cc, USHORT *_result);
USHORT tt_cmap_lookup(tt_cmap *cmap, SFNT_ULONG cc) {
  USHORT _result;
  if (__pdf_entry_tt_cmap_lookup(cmap, cc, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_otf_create_ToUnicode_stream(const char *font_name, uint32_t ttc_index, const char *basefont, const char *used_chars, pdf_obj **_result);
pdf_obj *otf_create_ToUnicode_stream(const char *font_name, uint32_t ttc_index, const char *basefont, const char *used_chars) {
  pdf_obj *_result;
  if (__pdf_entry_otf_create_ToUnicode_stream(font_name, ttc_index, basefont, used_chars, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_otf_load_Unicode_CMap(const char *map_name, uint32_t ttc_index, const char *otl_tags, int wmode, int *_result);
int otf_load_Unicode_CMap(const char *map_name, uint32_t ttc_index, const char *otl_tags, int wmode) {
  int _result;
  if (__pdf_entry_otf_load_Unicode_CMap(map_name, ttc_index, otl_tags, wmode, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_otf_try_load_GID_to_CID_map(const char *map_name, uint32_t ttc_index, int wmode, int *_result);
int otf_try_load_GID_to_CID_map(const char *map_name, uint32_t ttc_index, int wmode) {
  int _result;
  if (__pdf_entry_otf_try_load_GID_to_CID_map(map_name, ttc_index, wmode, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
