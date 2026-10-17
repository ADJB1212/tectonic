/* Copyright 2026 the Tectonic Project
 * Licensed under the MIT License.
 */

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#ifdef _WIN32
#include <fcntl.h>
#include <io.h>
#include <sys/stat.h>
#include <string.h>
#else
#include <unistd.h>
#endif

int *tt_pdf_errno(void) { return &errno; }
void *tt_pdf_stdout(void) { return stdout; }
void *tt_pdf_stderr(void) { return stderr; }

int tt_pdf_mkstemp(char *template) {
#ifdef _WIN32
    if (_mktemp_s(template, strlen(template) + 1) != 0) return -1;
    return _open(template, _O_CREAT | _O_EXCL | _O_RDWR | _O_BINARY, _S_IREAD | _S_IWRITE);
#else
    return mkstemp(template);
#endif
}
