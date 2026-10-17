#ifndef TECTONIC_BRIDGE_BUFFERED_H
#define TECTONIC_BRIDGE_BUFFERED_H

#include "tectonic_bridge_core.h"

/* Exclusive sequential reads; do not mix with raw reads or seeks after buffering. */
typedef struct {
    rust_input_handle_t handle;
    size_t position;
    size_t length;
    unsigned char bytes[8192];
} ttbc_buffered_input;

static inline void
ttbc_buffered_input_init(ttbc_buffered_input *input, rust_input_handle_t handle)
{
    input->handle = handle;
    input->position = input->length = 0;
}

static inline int
ttbc_buffered_input_getc(ttbc_buffered_input *input)
{
    if (input->position == input->length) {
        ssize_t count = ttstub_input_read_partial(input->handle, (char *) input->bytes,
                                                  sizeof(input->bytes));
        input->position = input->length = 0;
        if (count <= 0)
            return -1;
        input->length = (size_t) count;
    }
    return input->bytes[input->position++];
}

static inline void
ttbc_buffered_input_ungetc(ttbc_buffered_input *input, int byte)
{
    assert(input->position > 0);
    input->bytes[--input->position] = (unsigned char) byte;
}

#endif
