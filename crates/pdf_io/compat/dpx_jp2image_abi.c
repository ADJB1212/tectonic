#include "tectonic_bridge_core.h"
#include "dpx-jp2image.h"
#include <math.h>
#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-error.h"
#include "dpx-mfileio.h"
#include "dpx-numbers.h"
#include "dpx-pdfobj.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_check_for_jp2(FILE *fp, int *_result);
int check_for_jp2(FILE *fp) {
  int _result;
  if (__pdf_entry_check_for_jp2(fp, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_jp2_include_image(pdf_ximage *ximage, FILE *fp, int *_result);
int jp2_include_image(pdf_ximage *ximage, FILE *fp) {
  int _result;
  if (__pdf_entry_jp2_include_image(ximage, fp, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_jp2_get_bbox(FILE *fp, int *width, int *height, double *xdensity, double *ydensity, int *_result);
int jp2_get_bbox(FILE *fp, int *width, int *height, double *xdensity, double *ydensity) {
  int _result;
  if (__pdf_entry_jp2_get_bbox(fp, width, height, xdensity, ydensity, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
