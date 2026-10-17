#include "tectonic_bridge_core.h"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "tectonic_bridge_core.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxcrypt.h"
#include "dpx-dpxutil.h"
#include "dpx-dvipdfmx.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-numbers.h"
#include "dpx-pdfobj.h"
#include "dpx-system.h"
#include "dpx-unicode.h"
#include "dpx-pdfencrypt.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_enc_init(const unsigned char *id, int keybits, uint32_t permission, const char *opasswd, const char *upasswd, int use_aes, int encrypt_metadata, pdf_sec **_result);
pdf_sec *pdf_enc_init(const unsigned char *id, int keybits, uint32_t permission, const char *opasswd, const char *upasswd, int use_aes, int encrypt_metadata) {
  pdf_sec *_result;
  if (__pdf_entry_pdf_enc_init(id, keybits, permission, opasswd, upasswd, use_aes, encrypt_metadata, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_enc_close(pdf_sec **p_sec);
void pdf_enc_close(pdf_sec **p_sec) {
  if (__pdf_entry_pdf_enc_close(p_sec) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_encrypt_data(pdf_sec *p_sec, const unsigned char *plain, size_t plain_len, unsigned char **cipher, size_t *cipher_len);
void pdf_encrypt_data(pdf_sec *p_sec, const unsigned char *plain, size_t plain_len, unsigned char **cipher, size_t *cipher_len) {
  if (__pdf_entry_pdf_encrypt_data(p_sec, plain, plain_len, cipher, cipher_len) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_enc_get_encrypt_dict(pdf_sec *p_sec, pdf_obj **_result);
pdf_obj *pdf_enc_get_encrypt_dict(pdf_sec *p_sec) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_enc_get_encrypt_dict(p_sec, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_enc_get_extension_dict(pdf_sec *p_sec, pdf_obj **_result);
pdf_obj *pdf_enc_get_extension_dict(pdf_sec *p_sec) {
  pdf_obj *_result;
  if (__pdf_entry_pdf_enc_get_extension_dict(p_sec, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_enc_set_label(pdf_sec *p_sec, unsigned int label);
void pdf_enc_set_label(pdf_sec *p_sec, unsigned int label) {
  if (__pdf_entry_pdf_enc_set_label(p_sec, label) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_enc_set_generation(pdf_sec *p_sec, unsigned int generation);
void pdf_enc_set_generation(pdf_sec *p_sec, unsigned int generation) {
  if (__pdf_entry_pdf_enc_set_generation(p_sec, generation) != 0) _tt_abort("%s", tt_pdf_last_error());
}
