#include "tectonic_bridge_core.h"
#include "dpx-tt_glyf.h"
#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-sfnt.h"
#include "dpx-tt_table.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_tt_find_glyph(struct tt_glyphs *g, USHORT gid, USHORT *_result);
USHORT tt_find_glyph(struct tt_glyphs *g, USHORT gid) {
  USHORT _result;
  if (__pdf_entry_tt_find_glyph(g, gid, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_get_index(struct tt_glyphs *g, USHORT gid, USHORT *_result);
USHORT tt_get_index(struct tt_glyphs *g, USHORT gid) {
  USHORT _result;
  if (__pdf_entry_tt_get_index(g, gid, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_add_glyph(struct tt_glyphs *g, USHORT gid, USHORT new_gid, USHORT *_result);
USHORT tt_add_glyph(struct tt_glyphs *g, USHORT gid, USHORT new_gid) {
  USHORT _result;
  if (__pdf_entry_tt_add_glyph(g, gid, new_gid, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_build_init(struct tt_glyphs **_result);
struct tt_glyphs *tt_build_init(void) {
  struct tt_glyphs *_result;
  if (__pdf_entry_tt_build_init(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_build_finish(struct tt_glyphs *g);
void tt_build_finish(struct tt_glyphs *g) {
  if (__pdf_entry_tt_build_finish(g) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_tt_build_tables(sfnt *sfont, struct tt_glyphs *g, int *_result);
int tt_build_tables(sfnt *sfont, struct tt_glyphs *g) {
  int _result;
  if (__pdf_entry_tt_build_tables(sfont, g, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_get_metrics(sfnt *sfont, struct tt_glyphs *g, int *_result);
int tt_get_metrics(sfnt *sfont, struct tt_glyphs *g) {
  int _result;
  if (__pdf_entry_tt_get_metrics(sfont, g, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
