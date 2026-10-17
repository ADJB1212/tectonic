#include "tectonic_bridge_core.h"
#include "dpx-dpxcrypt.h"
#include <stdlib.h>
#include <string.h>
#include "dpx-mem.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_MD5_init(MD5_CONTEXT *ctx);
void MD5_init(MD5_CONTEXT *ctx) {
  if (__pdf_entry_MD5_init(ctx) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_MD5_write(MD5_CONTEXT *hd, const unsigned char *inbuf, unsigned int inlen);
void MD5_write(MD5_CONTEXT *hd, const unsigned char *inbuf, unsigned int inlen) {
  if (__pdf_entry_MD5_write(hd, inbuf, inlen) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_MD5_final(unsigned char *outbuf, MD5_CONTEXT *hd);
void MD5_final(unsigned char *outbuf, MD5_CONTEXT *hd) {
  if (__pdf_entry_MD5_final(outbuf, hd) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_SHA256_init(SHA256_CONTEXT *hd);
void SHA256_init(SHA256_CONTEXT *hd) {
  if (__pdf_entry_SHA256_init(hd) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_SHA256_write(SHA256_CONTEXT *hd, const unsigned char *inbuf, unsigned int inlen);
void SHA256_write(SHA256_CONTEXT *hd, const unsigned char *inbuf, unsigned int inlen) {
  if (__pdf_entry_SHA256_write(hd, inbuf, inlen) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_SHA256_final(unsigned char *outbuf, SHA256_CONTEXT *hd);
void SHA256_final(unsigned char *outbuf, SHA256_CONTEXT *hd) {
  if (__pdf_entry_SHA256_final(outbuf, hd) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_SHA512_init(SHA512_CONTEXT *ctx);
void SHA512_init(SHA512_CONTEXT *ctx) {
  if (__pdf_entry_SHA512_init(ctx) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_SHA384_init(SHA512_CONTEXT *ctx);
void SHA384_init(SHA512_CONTEXT *ctx) {
  if (__pdf_entry_SHA384_init(ctx) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_SHA512_write(SHA512_CONTEXT *hd, const unsigned char *inbuf, unsigned int inlen);
void SHA512_write(SHA512_CONTEXT *hd, const unsigned char *inbuf, unsigned int inlen) {
  if (__pdf_entry_SHA512_write(hd, inbuf, inlen) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_SHA512_final(unsigned char *outbuf, SHA512_CONTEXT *hd);
void SHA512_final(unsigned char *outbuf, SHA512_CONTEXT *hd) {
  if (__pdf_entry_SHA512_final(outbuf, hd) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_ARC4(ARC4_CONTEXT *ctx, unsigned int len, const unsigned char *inbuf, unsigned char *outbuf);
void ARC4(ARC4_CONTEXT *ctx, unsigned int len, const unsigned char *inbuf, unsigned char *outbuf) {
  if (__pdf_entry_ARC4(ctx, len, inbuf, outbuf) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_ARC4_set_key(ARC4_CONTEXT *ctx, unsigned int keylen, const unsigned char *key);
void ARC4_set_key(ARC4_CONTEXT *ctx, unsigned int keylen, const unsigned char *key) {
  if (__pdf_entry_ARC4_set_key(ctx, keylen, key) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_AES_ecb_encrypt(const unsigned char *key, size_t key_len, const unsigned char *plain, size_t plain_len, unsigned char **cipher, size_t *cipher_len);
void AES_ecb_encrypt(const unsigned char *key, size_t key_len, const unsigned char *plain, size_t plain_len, unsigned char **cipher, size_t *cipher_len) {
  if (__pdf_entry_AES_ecb_encrypt(key, key_len, plain, plain_len, cipher, cipher_len) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_AES_cbc_encrypt_tectonic(const unsigned char *key, size_t key_len, const unsigned char *iv, int padding, const unsigned char *plain, size_t plain_len, unsigned char **cipher, size_t *cipher_len);
void AES_cbc_encrypt_tectonic(const unsigned char *key, size_t key_len, const unsigned char *iv, int padding, const unsigned char *plain, size_t plain_len, unsigned char **cipher, size_t *cipher_len) {
  if (__pdf_entry_AES_cbc_encrypt_tectonic(key, key_len, iv, padding, plain, plain_len, cipher, cipher_len) != 0) _tt_abort("%s", tt_pdf_last_error());
}
