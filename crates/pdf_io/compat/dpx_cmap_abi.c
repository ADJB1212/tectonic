#include "tectonic_bridge_core.h"
#include "dpx-cmap.h"
#include <assert.h>
#include <stdlib.h>
#include <string.h>
#include "tectonic_bridge_core.h"
#include "dpx-cmap_p.h"
#include "dpx-dpxconf.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-cmap_read.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_CMap_set_silent(int value);
void CMap_set_silent(int value) {
  if (__pdf_entry_CMap_set_silent(value) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_CMap_new(CMap **_result);
CMap *CMap_new(void) {
  CMap *_result;
  if (__pdf_entry_CMap_new(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_release(CMap *cmap);
void CMap_release(CMap *cmap) {
  if (__pdf_entry_CMap_release(cmap) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_CMap_is_Identity(CMap *cmap, bool *_result);
bool CMap_is_Identity(CMap *cmap) {
  bool _result;
  if (__pdf_entry_CMap_is_Identity(cmap, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_is_valid(CMap *cmap, bool *_result);
bool CMap_is_valid(CMap *cmap) {
  bool _result;
  if (__pdf_entry_CMap_is_valid(cmap, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_get_profile(CMap *cmap, int type_0, int *_result);
int CMap_get_profile(CMap *cmap, int type_0) {
  int _result;
  if (__pdf_entry_CMap_get_profile(cmap, type_0, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_decode_char(CMap *cmap, const unsigned char **inbuf, size_t *inbytesleft, unsigned char **outbuf, size_t *outbytesleft);
void CMap_decode_char(CMap *cmap, const unsigned char **inbuf, size_t *inbytesleft, unsigned char **outbuf, size_t *outbytesleft) {
  if (__pdf_entry_CMap_decode_char(cmap, inbuf, inbytesleft, outbuf, outbytesleft) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_CMap_decode(CMap *cmap, const unsigned char **inbuf, size_t *inbytesleft, unsigned char **outbuf, size_t *outbytesleft, size_t *_result);
size_t CMap_decode(CMap *cmap, const unsigned char **inbuf, size_t *inbytesleft, unsigned char **outbuf, size_t *outbytesleft) {
  size_t _result;
  if (__pdf_entry_CMap_decode(cmap, inbuf, inbytesleft, outbuf, outbytesleft, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_get_name(CMap *cmap, char **_result);
char *CMap_get_name(CMap *cmap) {
  char *_result;
  if (__pdf_entry_CMap_get_name(cmap, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_get_type(CMap *cmap, int *_result);
int CMap_get_type(CMap *cmap) {
  int _result;
  if (__pdf_entry_CMap_get_type(cmap, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_get_wmode(CMap *cmap, int *_result);
int CMap_get_wmode(CMap *cmap) {
  int _result;
  if (__pdf_entry_CMap_get_wmode(cmap, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_get_CIDSysInfo(CMap *cmap, CIDSysInfo **_result);
CIDSysInfo *CMap_get_CIDSysInfo(CMap *cmap) {
  CIDSysInfo *_result;
  if (__pdf_entry_CMap_get_CIDSysInfo(cmap, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_set_name(CMap *cmap, const char *name);
void CMap_set_name(CMap *cmap, const char *name) {
  if (__pdf_entry_CMap_set_name(cmap, name) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_CMap_set_type(CMap *cmap, int type_0);
void CMap_set_type(CMap *cmap, int type_0) {
  if (__pdf_entry_CMap_set_type(cmap, type_0) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_CMap_set_wmode(CMap *cmap, int wmode);
void CMap_set_wmode(CMap *cmap, int wmode) {
  if (__pdf_entry_CMap_set_wmode(cmap, wmode) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_CMap_set_CIDSysInfo(CMap *cmap, const CIDSysInfo *csi);
void CMap_set_CIDSysInfo(CMap *cmap, const CIDSysInfo *csi) {
  if (__pdf_entry_CMap_set_CIDSysInfo(cmap, csi) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_CMap_set_usecmap(CMap *cmap, CMap *ucmap);
void CMap_set_usecmap(CMap *cmap, CMap *ucmap) {
  if (__pdf_entry_CMap_set_usecmap(cmap, ucmap) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_CMap_add_codespacerange(CMap *cmap, const unsigned char *codelo, const unsigned char *codehi, size_t dim, int *_result);
int CMap_add_codespacerange(CMap *cmap, const unsigned char *codelo, const unsigned char *codehi, size_t dim) {
  int _result;
  if (__pdf_entry_CMap_add_codespacerange(cmap, codelo, codehi, dim, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_add_notdefchar(CMap *cmap, const unsigned char *src, size_t srcdim, CID dst, int *_result);
int CMap_add_notdefchar(CMap *cmap, const unsigned char *src, size_t srcdim, CID dst) {
  int _result;
  if (__pdf_entry_CMap_add_notdefchar(cmap, src, srcdim, dst, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_add_notdefrange(CMap *cmap, const unsigned char *srclo, const unsigned char *srchi, size_t srcdim, CID dst, int *_result);
int CMap_add_notdefrange(CMap *cmap, const unsigned char *srclo, const unsigned char *srchi, size_t srcdim, CID dst) {
  int _result;
  if (__pdf_entry_CMap_add_notdefrange(cmap, srclo, srchi, srcdim, dst, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_add_bfchar(CMap *cmap, const unsigned char *src, size_t srcdim, const unsigned char *dst, size_t dstdim, int *_result);
int CMap_add_bfchar(CMap *cmap, const unsigned char *src, size_t srcdim, const unsigned char *dst, size_t dstdim) {
  int _result;
  if (__pdf_entry_CMap_add_bfchar(cmap, src, srcdim, dst, dstdim, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_add_bfrange(CMap *cmap, const unsigned char *srclo, const unsigned char *srchi, size_t srcdim, const unsigned char *base, size_t dstdim, int *_result);
int CMap_add_bfrange(CMap *cmap, const unsigned char *srclo, const unsigned char *srchi, size_t srcdim, const unsigned char *base, size_t dstdim) {
  int _result;
  if (__pdf_entry_CMap_add_bfrange(cmap, srclo, srchi, srcdim, base, dstdim, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_add_cidchar(CMap *cmap, const unsigned char *src, size_t srcdim, CID dst, int *_result);
int CMap_add_cidchar(CMap *cmap, const unsigned char *src, size_t srcdim, CID dst) {
  int _result;
  if (__pdf_entry_CMap_add_cidchar(cmap, src, srcdim, dst, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_add_cidrange(CMap *cmap, const unsigned char *srclo, const unsigned char *srchi, size_t srcdim, CID base, int *_result);
int CMap_add_cidrange(CMap *cmap, const unsigned char *srclo, const unsigned char *srchi, size_t srcdim, CID base) {
  int _result;
  if (__pdf_entry_CMap_add_cidrange(cmap, srclo, srchi, srcdim, base, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_cache_init(void);
void CMap_cache_init(void) {
  if (__pdf_entry_CMap_cache_init() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_CMap_cache_get(int id, CMap **_result);
CMap *CMap_cache_get(int id) {
  CMap *_result;
  if (__pdf_entry_CMap_cache_get(id, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_cache_find(const char *cmap_name, int *_result);
int CMap_cache_find(const char *cmap_name) {
  int _result;
  if (__pdf_entry_CMap_cache_find(cmap_name, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_cache_add(CMap *cmap, int *_result);
int CMap_cache_add(CMap *cmap) {
  int _result;
  if (__pdf_entry_CMap_cache_add(cmap, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_CMap_cache_close(void);
void CMap_cache_close(void) {
  if (__pdf_entry_CMap_cache_close() != 0) _tt_abort("%s", tt_pdf_last_error());
}
