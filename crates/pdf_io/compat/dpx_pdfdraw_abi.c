#include "tectonic_bridge_core.h"
#include "dpx-pdfdraw.h"
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "dpx-dpxconf.h"
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfparse.h"
#include "dpx-pdfobj.h"

extern const char *tt_pdf_last_error(void);
extern int __pdf_entry_pdf_invertmatrix(pdf_tmatrix *M);
void pdf_invertmatrix(pdf_tmatrix *M) {
  if (__pdf_entry_pdf_invertmatrix(M) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_reset_xgstate(int force, int *_result);
int pdf_dev_reset_xgstate(int force) {
  int _result;
  if (__pdf_entry_pdf_dev_reset_xgstate(force, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_xgstate_push(pdf_obj *object);
void pdf_dev_xgstate_push(pdf_obj *object) {
  if (__pdf_entry_pdf_dev_xgstate_push(object) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_xgstate_pop(void);
void pdf_dev_xgstate_pop(void) {
  if (__pdf_entry_pdf_dev_xgstate_pop() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_init_gstates(void);
void pdf_dev_init_gstates(void) {
  if (__pdf_entry_pdf_dev_init_gstates() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_clear_gstates(void);
void pdf_dev_clear_gstates(void) {
  if (__pdf_entry_pdf_dev_clear_gstates() != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_gsave(int *_result);
int pdf_dev_gsave(void) {
  int _result;
  if (__pdf_entry_pdf_dev_gsave(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_grestore(int *_result);
int pdf_dev_grestore(void) {
  int _result;
  if (__pdf_entry_pdf_dev_grestore(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_push_gstate(int *_result);
int pdf_dev_push_gstate(void) {
  int _result;
  if (__pdf_entry_pdf_dev_push_gstate(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_pop_gstate(int *_result);
int pdf_dev_pop_gstate(void) {
  int _result;
  if (__pdf_entry_pdf_dev_pop_gstate(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_current_depth(int *_result);
int pdf_dev_current_depth(void) {
  int _result;
  if (__pdf_entry_pdf_dev_current_depth(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_grestore_to(int depth);
void pdf_dev_grestore_to(int depth) {
  if (__pdf_entry_pdf_dev_grestore_to(depth) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_currentpoint(pdf_coord *p, int *_result);
int pdf_dev_currentpoint(pdf_coord *p) {
  int _result;
  if (__pdf_entry_pdf_dev_currentpoint(p, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_currentmatrix(pdf_tmatrix *M, int *_result);
int pdf_dev_currentmatrix(pdf_tmatrix *M) {
  int _result;
  if (__pdf_entry_pdf_dev_currentmatrix(M, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_set_color(const pdf_color *color, char mask, int force);
void pdf_dev_set_color(const pdf_color *color, char mask, int force) {
  if (__pdf_entry_pdf_dev_set_color(color, mask, force) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_concat(const pdf_tmatrix *M, int *_result);
int pdf_dev_concat(const pdf_tmatrix *M) {
  int _result;
  if (__pdf_entry_pdf_dev_concat(M, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_setmiterlimit(double mlimit, int *_result);
int pdf_dev_setmiterlimit(double mlimit) {
  int _result;
  if (__pdf_entry_pdf_dev_setmiterlimit(mlimit, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_setlinecap(int capstyle, int *_result);
int pdf_dev_setlinecap(int capstyle) {
  int _result;
  if (__pdf_entry_pdf_dev_setlinecap(capstyle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_setlinejoin(int joinstyle, int *_result);
int pdf_dev_setlinejoin(int joinstyle) {
  int _result;
  if (__pdf_entry_pdf_dev_setlinejoin(joinstyle, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_setlinewidth(double width, int *_result);
int pdf_dev_setlinewidth(double width) {
  int _result;
  if (__pdf_entry_pdf_dev_setlinewidth(width, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_setdash(int count, double *pattern, double offset, int *_result);
int pdf_dev_setdash(int count, double *pattern, double offset) {
  int _result;
  if (__pdf_entry_pdf_dev_setdash(count, pattern, offset, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_clip(int *_result);
int pdf_dev_clip(void) {
  int _result;
  if (__pdf_entry_pdf_dev_clip(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_eoclip(int *_result);
int pdf_dev_eoclip(void) {
  int _result;
  if (__pdf_entry_pdf_dev_eoclip(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_flushpath(char p_op, int fill_rule, int *_result);
int pdf_dev_flushpath(char p_op, int fill_rule) {
  int _result;
  if (__pdf_entry_pdf_dev_flushpath(p_op, fill_rule, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_newpath(int *_result);
int pdf_dev_newpath(void) {
  int _result;
  if (__pdf_entry_pdf_dev_newpath(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_moveto(double x, double y, int *_result);
int pdf_dev_moveto(double x, double y) {
  int _result;
  if (__pdf_entry_pdf_dev_moveto(x, y, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_rmoveto(double x, double y, int *_result);
int pdf_dev_rmoveto(double x, double y) {
  int _result;
  if (__pdf_entry_pdf_dev_rmoveto(x, y, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_lineto(double x, double y, int *_result);
int pdf_dev_lineto(double x, double y) {
  int _result;
  if (__pdf_entry_pdf_dev_lineto(x, y, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_rlineto(double x, double y, int *_result);
int pdf_dev_rlineto(double x, double y) {
  int _result;
  if (__pdf_entry_pdf_dev_rlineto(x, y, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_curveto(double x0, double y0, double x1, double y1, double x2, double y2, int *_result);
int pdf_dev_curveto(double x0, double y0, double x1, double y1, double x2, double y2) {
  int _result;
  if (__pdf_entry_pdf_dev_curveto(x0, y0, x1, y1, x2, y2, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_rcurveto(double x0, double y0, double x1, double y1, double x2, double y2, int *_result);
int pdf_dev_rcurveto(double x0, double y0, double x1, double y1, double x2, double y2) {
  int _result;
  if (__pdf_entry_pdf_dev_rcurveto(x0, y0, x1, y1, x2, y2, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_closepath(int *_result);
int pdf_dev_closepath(void) {
  int _result;
  if (__pdf_entry_pdf_dev_closepath(&_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_dtransform(pdf_coord *p, const pdf_tmatrix *M);
void pdf_dev_dtransform(pdf_coord *p, const pdf_tmatrix *M) {
  if (__pdf_entry_pdf_dev_dtransform(p, M) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_idtransform(pdf_coord *p, const pdf_tmatrix *M);
void pdf_dev_idtransform(pdf_coord *p, const pdf_tmatrix *M) {
  if (__pdf_entry_pdf_dev_idtransform(p, M) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_transform(pdf_coord *p, const pdf_tmatrix *M);
void pdf_dev_transform(pdf_coord *p, const pdf_tmatrix *M) {
  if (__pdf_entry_pdf_dev_transform(p, M) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_itransform(pdf_coord *p, const pdf_tmatrix *M);
void pdf_dev_itransform(pdf_coord *p, const pdf_tmatrix *M) {
  if (__pdf_entry_pdf_dev_itransform(p, M) != 0) _tt_abort("%s", tt_pdf_last_error());
}
extern int __pdf_entry_pdf_dev_arc(double c_x, double c_y, double r, double a_0, double a_1, int *_result);
int pdf_dev_arc(double c_x, double c_y, double r, double a_0, double a_1) {
  int _result;
  if (__pdf_entry_pdf_dev_arc(c_x, c_y, r, a_0, a_1, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_arcn(double c_x, double c_y, double r, double a_0, double a_1, int *_result);
int pdf_dev_arcn(double c_x, double c_y, double r, double a_0, double a_1) {
  int _result;
  if (__pdf_entry_pdf_dev_arcn(c_x, c_y, r, a_0, a_1, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_arcx(double c_x, double c_y, double r_x, double r_y, double a_0, double a_1, int a_d, double xar, int *_result);
int pdf_dev_arcx(double c_x, double c_y, double r_x, double r_y, double a_0, double a_1, int a_d, double xar) {
  int _result;
  if (__pdf_entry_pdf_dev_arcx(c_x, c_y, r_x, r_y, a_0, a_1, a_d, xar, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_bspline(double x0, double y0, double x1, double y1, double x2, double y2, int *_result);
int pdf_dev_bspline(double x0, double y0, double x1, double y1, double x2, double y2) {
  int _result;
  if (__pdf_entry_pdf_dev_bspline(x0, y0, x1, y1, x2, y2, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_rectstroke(double x, double y, double w, double h, const pdf_tmatrix *M, int *_result);
int pdf_dev_rectstroke(double x, double y, double w, double h, const pdf_tmatrix *M) {
  int _result;
  if (__pdf_entry_pdf_dev_rectstroke(x, y, w, h, M, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_rectfill(double x, double y, double w, double h, int *_result);
int pdf_dev_rectfill(double x, double y, double w, double h) {
  int _result;
  if (__pdf_entry_pdf_dev_rectfill(x, y, w, h, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
extern int __pdf_entry_pdf_dev_rectclip(double x, double y, double w, double h, int *_result);
int pdf_dev_rectclip(double x, double y, double w, double h) {
  int _result;
  if (__pdf_entry_pdf_dev_rectclip(x, y, w, h, &_result) != 0) _tt_abort("%s", tt_pdf_last_error());
  return _result;
}
