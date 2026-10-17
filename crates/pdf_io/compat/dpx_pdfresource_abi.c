#include "tectonic_bridge_core.h"
#include "dpx-pdfresource.h"
#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-pdfobj.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_init_resources(void);
void pdf_init_resources(void) {
  if (__pdf_entry_pdf_init_resources() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_close_resources(void);
void pdf_close_resources(void) {
  if (__pdf_entry_pdf_close_resources() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_defineresource(const char *category, const char *resname, pdf_obj *object, int flags, int *_result);
int pdf_defineresource(const char *category, const char *resname, pdf_obj *object, int flags) {
  int _result;
  if (__pdf_entry_pdf_defineresource(category, resname, object, flags, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_findresource(const char *category, const char *resname, int *_result);
int pdf_findresource(const char *category, const char *resname) {
  int _result;
  if (__pdf_entry_pdf_findresource(category, resname, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_get_resource_reference(int rc_id, pdf_obj **_result);
pdf_obj *pdf_get_resource_reference(int rc_id) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_get_resource_reference(rc_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
