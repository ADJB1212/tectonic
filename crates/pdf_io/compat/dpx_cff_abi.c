#include "tectonic_bridge_core.h"
#include "dpx-cff.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-cff_dict.h"
#include "dpx-cff_types.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_cff_open(rust_input_handle_t handle, int offset, int n, cff_font **_result);
cff_font *cff_open(rust_input_handle_t handle, int offset, int n) {
  cff_font *_result;
  if (__pdf_entry_cff_open(handle, offset, n, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_close(cff_font *cff);
void cff_close(cff_font *cff) {
  if (__pdf_entry_cff_close(cff) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_cff_get_name(cff_font *cff, char **_result);
char *cff_get_name(cff_font *cff) {
  char *_result;
  if (__pdf_entry_cff_get_name(cff, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_set_name(cff_font *cff, char *name, int *_result);
int cff_set_name(cff_font *cff, char *name) {
  int _result;
  if (__pdf_entry_cff_set_name(cff, name, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_put_header(cff_font *cff, card8 *dest, int destlen, int *_result);
int cff_put_header(cff_font *cff, card8 *dest, int destlen) {
  int _result;
  if (__pdf_entry_cff_put_header(cff, dest, destlen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_get_index_header(cff_font *cff, cff_index **_result);
cff_index *cff_get_index_header(cff_font *cff) {
  cff_index *_result;
  if (__pdf_entry_cff_get_index_header(cff, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_get_index(cff_font *cff, cff_index **_result);
cff_index *cff_get_index(cff_font *cff) {
  cff_index *_result;
  if (__pdf_entry_cff_get_index(cff, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_pack_index(cff_index *idx, card8 *dest, int destlen, int *_result);
int cff_pack_index(cff_index *idx, card8 *dest, int destlen) {
  int _result;
  if (__pdf_entry_cff_pack_index(idx, dest, destlen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_index_size(cff_index *idx, int *_result);
int cff_index_size(cff_index *idx) {
  int _result;
  if (__pdf_entry_cff_index_size(idx, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_new_index(card16 count, cff_index **_result);
cff_index *cff_new_index(card16 count) {
  cff_index *_result;
  if (__pdf_entry_cff_new_index(count, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_release_index(cff_index *idx);
void cff_release_index(cff_index *idx) {
  if (__pdf_entry_cff_release_index(idx) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_cff_get_string(cff_font *cff, s_SID id, char **_result);
char *cff_get_string(cff_font *cff, s_SID id) {
  char *_result;
  if (__pdf_entry_cff_get_string(cff, id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_get_sid(cff_font *cff, const char *str, int *_result);
int cff_get_sid(cff_font *cff, const char *str) {
  int _result;
  if (__pdf_entry_cff_get_sid(cff, str, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_get_seac_sid(cff_font *cff, const char *str, int *_result);
int cff_get_seac_sid(cff_font *cff, const char *str) {
  int _result;
  if (__pdf_entry_cff_get_seac_sid(cff, str, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_update_string(cff_font *cff);
void cff_update_string(cff_font *cff) {
  if (__pdf_entry_cff_update_string(cff) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_cff_add_string(cff_font *cff, const char *str, int unique, s_SID *_result);
s_SID cff_add_string(cff_font *cff, const char *str, int unique) {
  s_SID _result;
  if (__pdf_entry_cff_add_string(cff, str, unique, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_read_encoding(cff_font *cff, int *_result);
int cff_read_encoding(cff_font *cff) {
  int _result;
  if (__pdf_entry_cff_read_encoding(cff, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_pack_encoding(cff_font *cff, card8 *dest, int destlen, int *_result);
int cff_pack_encoding(cff_font *cff, card8 *dest, int destlen) {
  int _result;
  if (__pdf_entry_cff_pack_encoding(cff, dest, destlen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_encoding_lookup(cff_font *cff, card8 code, card16 *_result);
card16 cff_encoding_lookup(cff_font *cff, card8 code) {
  card16 _result;
  if (__pdf_entry_cff_encoding_lookup(cff, code, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_release_encoding(cff_encoding *encoding);
void cff_release_encoding(cff_encoding *encoding) {
  if (__pdf_entry_cff_release_encoding(encoding) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_cff_read_charsets(cff_font *cff, int *_result);
int cff_read_charsets(cff_font *cff) {
  int _result;
  if (__pdf_entry_cff_read_charsets(cff, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_pack_charsets(cff_font *cff, card8 *dest, int destlen, int *_result);
int cff_pack_charsets(cff_font *cff, card8 *dest, int destlen) {
  int _result;
  if (__pdf_entry_cff_pack_charsets(cff, dest, destlen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_get_glyphname(cff_font *cff, card16 gid, char **_result);
char *cff_get_glyphname(cff_font *cff, card16 gid) {
  char *_result;
  if (__pdf_entry_cff_get_glyphname(cff, gid, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_glyph_lookup(cff_font *cff, const char *glyph, card16 *_result);
card16 cff_glyph_lookup(cff_font *cff, const char *glyph) {
  card16 _result;
  if (__pdf_entry_cff_glyph_lookup(cff, glyph, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_charsets_lookup(cff_font *cff, card16 cid, card16 *_result);
card16 cff_charsets_lookup(cff_font *cff, card16 cid) {
  card16 _result;
  if (__pdf_entry_cff_charsets_lookup(cff, cid, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_charsets_lookup_gid(cff_charsets *charset, card16 cid, card16 *_result);
card16 cff_charsets_lookup_gid(cff_charsets *charset, card16 cid) {
  card16 _result;
  if (__pdf_entry_cff_charsets_lookup_gid(charset, cid, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_charsets_lookup_inverse(cff_font *cff, card16 gid, card16 *_result);
card16 cff_charsets_lookup_inverse(cff_font *cff, card16 gid) {
  card16 _result;
  if (__pdf_entry_cff_charsets_lookup_inverse(cff, gid, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_charsets_lookup_cid(cff_charsets *charset, card16 gid, card16 *_result);
card16 cff_charsets_lookup_cid(cff_charsets *charset, card16 gid) {
  card16 _result;
  if (__pdf_entry_cff_charsets_lookup_cid(charset, gid, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_release_charsets(cff_charsets *charset);
void cff_release_charsets(cff_charsets *charset) {
  if (__pdf_entry_cff_release_charsets(charset) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_cff_read_fdselect(cff_font *cff, int *_result);
int cff_read_fdselect(cff_font *cff) {
  int _result;
  if (__pdf_entry_cff_read_fdselect(cff, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_pack_fdselect(cff_font *cff, card8 *dest, int destlen, int *_result);
int cff_pack_fdselect(cff_font *cff, card8 *dest, int destlen) {
  int _result;
  if (__pdf_entry_cff_pack_fdselect(cff, dest, destlen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_release_fdselect(cff_fdselect *fdselect);
void cff_release_fdselect(cff_fdselect *fdselect) {
  if (__pdf_entry_cff_release_fdselect(fdselect) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_cff_fdselect_lookup(cff_font *cff, card16 gid, card8 *_result);
card8 cff_fdselect_lookup(cff_font *cff, card16 gid) {
  card8 _result;
  if (__pdf_entry_cff_fdselect_lookup(cff, gid, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_read_subrs(cff_font *cff, int *_result);
int cff_read_subrs(cff_font *cff) {
  int _result;
  if (__pdf_entry_cff_read_subrs(cff, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_read_fdarray(cff_font *cff, int *_result);
int cff_read_fdarray(cff_font *cff) {
  int _result;
  if (__pdf_entry_cff_read_fdarray(cff, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_cff_read_private(cff_font *cff, int *_result);
int cff_read_private(cff_font *cff) {
  int _result;
  if (__pdf_entry_cff_read_private(cff, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
