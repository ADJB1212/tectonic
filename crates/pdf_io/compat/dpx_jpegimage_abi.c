#include "tectonic_bridge_core.h"
#include "dpx-jpegimage.h"
#include <fcntl.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-numbers.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfximage.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_check_for_jpeg(rust_input_handle_t handle, int *_result);
int check_for_jpeg(rust_input_handle_t handle) {
  int _result;
  if (__pdf_entry_check_for_jpeg(handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_jpeg_include_image(pdf_ximage *ximage, rust_input_handle_t handle, int *_result);
int jpeg_include_image(pdf_ximage *ximage, rust_input_handle_t handle) {
  int _result;
  if (__pdf_entry_jpeg_include_image(ximage, handle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_jpeg_get_bbox(rust_input_handle_t handle, unsigned int *width, unsigned int *height, double *xdensity, double *ydensity, int *_result);
int jpeg_get_bbox(rust_input_handle_t handle, unsigned int *width, unsigned int *height, double *xdensity, double *ydensity) {
  int _result;
  if (__pdf_entry_jpeg_get_bbox(handle, width, height, xdensity, ydensity, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
