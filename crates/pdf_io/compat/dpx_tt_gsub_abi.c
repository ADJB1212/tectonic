#include "tectonic_bridge_core.h"
#include "dpx-tt_gsub.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-otl_opt.h"
#include "dpx-sfnt.h"
#include "dpx-unicode.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_otl_gsub_new(otl_gsub **_result);
otl_gsub *otl_gsub_new(void) {
  otl_gsub *_result;
  if (__pdf_entry_otl_gsub_new(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_otl_gsub_add_feat(otl_gsub *gsub_list, const char *script, const char *language, const char *feature, sfnt *sfont, int *_result);
int otl_gsub_add_feat(otl_gsub *gsub_list, const char *script, const char *language, const char *feature, sfnt *sfont) {
  int _result;
  if (__pdf_entry_otl_gsub_add_feat(gsub_list, script, language, feature, sfont, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_otl_gsub_release(otl_gsub *gsub_list);
void otl_gsub_release(otl_gsub *gsub_list) {
  if (__pdf_entry_otl_gsub_release(gsub_list) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_otl_gsub_apply(otl_gsub *gsub_list, USHORT *gid, int *_result);
int otl_gsub_apply(otl_gsub *gsub_list, USHORT *gid) {
  int _result;
  if (__pdf_entry_otl_gsub_apply(gsub_list, gid, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_otl_gsub_apply_alt(otl_gsub *gsub_list, USHORT alt_idx, USHORT *gid, int *_result);
int otl_gsub_apply_alt(otl_gsub *gsub_list, USHORT alt_idx, USHORT *gid) {
  int _result;
  if (__pdf_entry_otl_gsub_apply_alt(gsub_list, alt_idx, gid, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_otl_gsub_apply_lig(otl_gsub *gsub_list, USHORT *gid_in, USHORT num_gids, USHORT *gid_out, int *_result);
int otl_gsub_apply_lig(otl_gsub *gsub_list, USHORT *gid_in, USHORT num_gids, USHORT *gid_out) {
  int _result;
  if (__pdf_entry_otl_gsub_apply_lig(gsub_list, gid_in, num_gids, gid_out, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_otl_gsub_select(otl_gsub *gsub_list, const char *script, const char *language, const char *feature, int *_result);
int otl_gsub_select(otl_gsub *gsub_list, const char *script, const char *language, const char *feature) {
  int _result;
  if (__pdf_entry_otl_gsub_select(gsub_list, script, language, feature, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_otl_gsub_set_chain(otl_gsub *gsub_list, const char *otl_tags, int *_result);
int otl_gsub_set_chain(otl_gsub *gsub_list, const char *otl_tags) {
  int _result;
  if (__pdf_entry_otl_gsub_set_chain(gsub_list, otl_tags, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_otl_gsub_add_feat_list(otl_gsub *gsub_list, const char *otl_tags, sfnt *sfont, int *_result);
int otl_gsub_add_feat_list(otl_gsub *gsub_list, const char *otl_tags, sfnt *sfont) {
  int _result;
  if (__pdf_entry_otl_gsub_add_feat_list(gsub_list, otl_tags, sfont, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_otl_gsub_apply_chain(otl_gsub *gsub_list, USHORT *gid, int *_result);
int otl_gsub_apply_chain(otl_gsub *gsub_list, USHORT *gid) {
  int _result;
  if (__pdf_entry_otl_gsub_apply_chain(gsub_list, gid, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_otl_gsub_add_ToUnicode(CMap *cmap, char *used_chars, int32_t *map_base, int32_t *map_sub, USHORT num_glyphs, uint16_t *GIDToCIDMap, sfnt *sfont, int *_result);
int otl_gsub_add_ToUnicode(CMap *cmap, char *used_chars, int32_t *map_base, int32_t *map_sub, USHORT num_glyphs, uint16_t *GIDToCIDMap, sfnt *sfont) {
  int _result;
  if (__pdf_entry_otl_gsub_add_ToUnicode(cmap, used_chars, map_base, map_sub, num_glyphs, GIDToCIDMap, sfont, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
