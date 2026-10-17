#ifndef TECTONIC_TFM_H
#define TECTONIC_TFM_H

#include <stdbool.h>
#include <stdint.h>

/* kind is 0 for a message and 1 for a warning. The returned text stays valid
 * until the next call. */
const char *tt_tfm_next_diagnostic(int *kind);

int tt_tfm_open(const char *name, int must_exist, int verbose, const char **error);
void tt_tfm_close_all(void);
void tt_tfm_reset(void);

int32_t tt_tfm_get_fw_width(int font_id, int32_t ch, const char **error);
int32_t tt_tfm_get_fw_height(int font_id, int32_t ch, const char **error);
int32_t tt_tfm_get_fw_depth(int font_id, int32_t ch, const char **error);
int32_t tt_tfm_string_width(int font_id, const unsigned char *text, unsigned length,
                            const char **error);
double tt_tfm_get_width(int font_id, int32_t ch, const char **error);
double tt_tfm_get_design_size(int font_id, const char **error);
int tt_tfm_is_jfm(int font_id);
bool tt_tfm_exists(const char *name);

#endif
