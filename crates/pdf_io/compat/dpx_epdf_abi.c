#include "tectonic_bridge_core.h"
#include "dpx-epdf.h"
#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-error.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfparse.h"
#include "dpx-pdfximage.h"
#include "dpx-dpxutil.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_include_page(pdf_ximage *ximage, rust_input_handle_t handle, const char *ident, load_options options, int *_result);
int pdf_include_page(pdf_ximage *ximage, rust_input_handle_t handle, const char *ident, load_options options) {
  int _result;
  if (__pdf_entry_pdf_include_page(ximage, handle, ident, options, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_copy_clip(rust_input_handle_t image_file, int pageNo, double x_user, double y_user, int *_result);
int pdf_copy_clip(rust_input_handle_t image_file, int pageNo, double x_user, double y_user) {
  int _result;
  if (__pdf_entry_pdf_copy_clip(image_file, pageNo, x_user, y_user, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
