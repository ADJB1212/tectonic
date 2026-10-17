#include "tectonic_bridge_core.h"
#include "dpx-tt_post.h"
#include <assert.h>
#include <stdlib.h>
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-sfnt.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_tt_read_post_table(sfnt *sfont, struct tt_post_table **_result);
struct tt_post_table *tt_read_post_table(sfnt *sfont) {
  struct tt_post_table *_result;
  if (__pdf_entry_tt_read_post_table(sfont, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_lookup_post_table(struct tt_post_table *post, const char *glyphname, USHORT *_result);
USHORT tt_lookup_post_table(struct tt_post_table *post, const char *glyphname) {
  USHORT _result;
  if (__pdf_entry_tt_lookup_post_table(post, glyphname, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_get_glyphname(struct tt_post_table *post, USHORT gid, char **_result);
char *tt_get_glyphname(struct tt_post_table *post, USHORT gid) {
  char *_result;
  if (__pdf_entry_tt_get_glyphname(post, gid, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_tt_release_post_table(struct tt_post_table *post);
void tt_release_post_table(struct tt_post_table *post) {
  if (__pdf_entry_tt_release_post_table(post) != 0) _tt_abort("%s", tt_pdf_last_error());
}
