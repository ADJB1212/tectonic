#include "tectonic_bridge_core.h"
#include "dpx-tt_table.h"
#include <stdio.h>
#include "tectonic_bridge_core.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-sfnt.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_tt_pack_head_table(struct tt_head_table *table, char **_result);
char *tt_pack_head_table(struct tt_head_table *table) {
  char *_result;
  if (__pdf_entry_tt_pack_head_table(table, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_read_head_table(sfnt *sfont, struct tt_head_table **_result);
struct tt_head_table *tt_read_head_table(sfnt *sfont) {
  struct tt_head_table *_result;
  if (__pdf_entry_tt_read_head_table(sfont, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_pack_maxp_table(struct tt_maxp_table *table, char **_result);
char *tt_pack_maxp_table(struct tt_maxp_table *table) {
  char *_result;
  if (__pdf_entry_tt_pack_maxp_table(table, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_read_maxp_table(sfnt *sfont, struct tt_maxp_table **_result);
struct tt_maxp_table *tt_read_maxp_table(sfnt *sfont) {
  struct tt_maxp_table *_result;
  if (__pdf_entry_tt_read_maxp_table(sfont, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_pack_hhea_table(struct tt_hhea_table *table, char **_result);
char *tt_pack_hhea_table(struct tt_hhea_table *table) {
  char *_result;
  if (__pdf_entry_tt_pack_hhea_table(table, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_read_hhea_table(sfnt *sfont, struct tt_hhea_table **_result);
struct tt_hhea_table *tt_read_hhea_table(sfnt *sfont) {
  struct tt_hhea_table *_result;
  if (__pdf_entry_tt_read_hhea_table(sfont, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_read_vhea_table(sfnt *sfont, struct tt_vhea_table **_result);
struct tt_vhea_table *tt_read_vhea_table(sfnt *sfont) {
  struct tt_vhea_table *_result;
  if (__pdf_entry_tt_read_vhea_table(sfont, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_read_VORG_table(sfnt *sfont, struct tt_VORG_table **_result);
struct tt_VORG_table *tt_read_VORG_table(sfnt *sfont) {
  struct tt_VORG_table *_result;
  if (__pdf_entry_tt_read_VORG_table(sfont, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_read_longMetrics(sfnt *sfont, USHORT numGlyphs, USHORT numLongMetrics, USHORT numExSideBearings, struct tt_longMetrics **_result);
struct tt_longMetrics *tt_read_longMetrics(sfnt *sfont, USHORT numGlyphs, USHORT numLongMetrics, USHORT numExSideBearings) {
  struct tt_longMetrics *_result;
  if (__pdf_entry_tt_read_longMetrics(sfont, numGlyphs, numLongMetrics, numExSideBearings, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_read_os2__table(sfnt *sfont, struct tt_os2__table **_result);
struct tt_os2__table *tt_read_os2__table(sfnt *sfont) {
  struct tt_os2__table *_result;
  if (__pdf_entry_tt_read_os2__table(sfont, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_get_ps_fontname(sfnt *sfont, char *dest, USHORT destlen, USHORT *_result);
USHORT tt_get_ps_fontname(sfnt *sfont, char *dest, USHORT destlen) {
  USHORT _result;
  if (__pdf_entry_tt_get_ps_fontname(sfont, dest, destlen, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
