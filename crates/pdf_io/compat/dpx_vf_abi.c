#include "tectonic_bridge_core.h"
#include "dpx-vf.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-dvi.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfdev.h"
#include "dpx-tfm.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_vf_reset_global_state(void);
void vf_reset_global_state(void) {
  if (__pdf_entry_vf_reset_global_state() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_vf_locate_font(const char *tex_name, spt_t ptsize, int *_result);
int vf_locate_font(const char *tex_name, spt_t ptsize) {
  int _result;
  if (__pdf_entry_vf_locate_font(tex_name, ptsize, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_vf_set_char(int32_t ch, int vf_font);
void vf_set_char(int32_t ch, int vf_font) {
  if (__pdf_entry_vf_set_char(ch, vf_font) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_vf_close_all_fonts(void);
void vf_close_all_fonts(void) {
  if (__pdf_entry_vf_close_all_fonts() != 0) _tt_abort("%s", tt_pdf_last_error());
}
