#ifndef TECTONIC_FONTMAP_H
#define TECTONIC_FONTMAP_H

#include "dpx-fontmap.h"

void tt_fontmap_reset(void);
void tt_fontmap_clear(void);
fontmap_rec *tt_fontmap_lookup(const char *key);
fontmap_rec *tt_fontmap_insert(const char *key, const fontmap_rec *record, int replace);
void tt_fontmap_remove(const char *key);

int tt_fontmap_load_begin(rust_input_handle_t handle, int mode, int verbose);
/* Returns 1 for a legacy record, 2 for a format mismatch, or 0 at EOF. */
int tt_fontmap_load_next(const char **line, int *line_number, int *format);
void tt_fontmap_load_end(void);

#endif
