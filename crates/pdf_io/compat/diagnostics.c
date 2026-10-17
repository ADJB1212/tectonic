/* Copyright 2026 the Tectonic Project
 * Licensed under the MIT License.
 */

#include <stdarg.h>
#include <stdio.h>
#include "tectonic_bridge_core.h"
#include "dpx-error.h"
#include "dpx-specials.h"

extern int tt_pdf_emit_message(const char *text, int warning);
extern const char *tt_pdf_last_error(void);

static void emit(const char *format, va_list args, int warning) {
    char text[1024];
    vsnprintf(text, sizeof(text), format, args);
    if (tt_pdf_emit_message(text, warning) != 0)
        _tt_abort("%s", tt_pdf_last_error());
}

void dpx_message(const char *format, ...) {
    va_list args;
    va_start(args, format);
    emit(format, args, 0);
    va_end(args);
}

void dpx_warning(const char *format, ...) {
    va_list args;
    va_start(args, format);
    emit(format, args, 1);
    va_end(args);
}

void spc_warn(struct spc_env *env, const char *format, ...) {
    va_list args;
    (void)env;
    va_start(args, format);
    emit(format, args, 1);
    va_end(args);
}
