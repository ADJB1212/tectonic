#ifndef TECTONIC_XETEX_CHECKPOINT_H
#define TECTONIC_XETEX_CHECKPOINT_H

#include "xetex-core.h"
#include "xetex_bindings.h"

typedef struct {
    unsigned char *bytes;
    size_t length, capacity, position;
    bool restoring, valid;
} tt_checkpoint;
extern bool tt_checkpoint_resumed;
extern bool tt_checkpoint_enabled;
extern bool tt_checkpoint_dispatch_fetch;
void tt_checkpoint_before_input(void);
void tt_checkpoint_continue(void);

void tt_checkpoint_field(tt_checkpoint *cp, const char *name, void *value, size_t length);
void tt_checkpoint_array(tt_checkpoint *cp, const char *name, void **value, size_t count, size_t item_size);
void tt_checkpoint_used_array(tt_checkpoint *cp, const char *name, void **value, size_t capacity, size_t used, size_t item_size);
void tt_checkpoint_string(tt_checkpoint *cp, const char *name, char **value);
void tt_checkpoint_engine(tt_checkpoint *cp);
void tt_checkpoint_shipout(tt_checkpoint *cp);
void tt_checkpoint_pagebuilder(tt_checkpoint *cp);
void tt_checkpoint_synctex(tt_checkpoint *cp);
void tt_checkpoint_clock(tt_checkpoint *cp);
void tt_checkpoint_ext_reset(void);
void tt_checkpoint_fonts(tt_checkpoint *cp);
void tt_checkpoint_memory(tt_checkpoint *cp);
void tt_checkpoint_rle(tt_checkpoint *cp, const char *name, void **value, size_t count, size_t item_size);
void tt_checkpoint_try(void);
bool tt_checkpoint_resume(void);
void tt_checkpoint_disallow(void);
void tt_checkpoint_release(void);
void tt_checkpoint_register_glyph(int32_t node);
void tt_checkpoint_unregister_glyph(int32_t node);

#define CP_VALUE(value) tt_checkpoint_field(cp, #value, &(value), sizeof(value))
#define CP_ARRAY(value, count) tt_checkpoint_array(cp, #value, (void **)&(value), (count), sizeof(*(value)))
#define CP_STRING(value) tt_checkpoint_string(cp, #value, &(value))
#define CP_USED(value, capacity, used) tt_checkpoint_used_array(cp, #value, (void **)&(value), (capacity), (used), sizeof(*(value)))
#define CP_RLE(value, count) tt_checkpoint_rle(cp, #value, (void **)&(value), (count), sizeof(*(value)))

#endif
