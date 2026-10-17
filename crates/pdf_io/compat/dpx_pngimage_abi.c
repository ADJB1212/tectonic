#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfobj.h"
#include "dpx-pngimage.h"
#include <fcntl.h>
#include <png.h>
#include <pngconf.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include "dpx-pdfximage.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_check_for_png(rust_input_handle_t handle, int *_result);
int check_for_png(rust_input_handle_t handle) {
  int _result;
  if (__pdf_entry_check_for_png(handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_png_include_image(pdf_ximage *ximage, rust_input_handle_t handle, int *_result);
int png_include_image(pdf_ximage *ximage, rust_input_handle_t handle) {
  int _result;
  if (__pdf_entry_png_include_image(ximage, handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_png_get_bbox(rust_input_handle_t handle, uint32_t *width, uint32_t *height, double *xdensity, double *ydensity, int *_result);
int png_get_bbox(rust_input_handle_t handle, uint32_t *width, uint32_t *height, double *xdensity, double *ydensity) {
  int _result;
  if (__pdf_entry_png_get_bbox(handle, width, height, xdensity, ydensity, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
