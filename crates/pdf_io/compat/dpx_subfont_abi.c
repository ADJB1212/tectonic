#include "tectonic_bridge_core.h"
#include "dpx-subfont.h"
#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_sfd_get_subfont_ids(const char *sfd_name, int *num_ids, char ***_result);
char **sfd_get_subfont_ids(const char *sfd_name, int *num_ids) {
  char **_result;
  if (__pdf_entry_sfd_get_subfont_ids(sfd_name, num_ids, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_sfd_load_record(const char *sfd_name, const char *subfont_id, int *_result);
int sfd_load_record(const char *sfd_name, const char *subfont_id) {
  int _result;
  if (__pdf_entry_sfd_load_record(sfd_name, subfont_id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_lookup_sfd_record(int rec_id, unsigned char c, unsigned short *_result);
unsigned short lookup_sfd_record(int rec_id, unsigned char c) {
  unsigned short _result;
  if (__pdf_entry_lookup_sfd_record(rec_id, c, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_release_sfd_record(void);
void release_sfd_record(void) {
  if (__pdf_entry_release_sfd_record() != 0) _tt_abort("%s", tt_pdf_last_error());
}
