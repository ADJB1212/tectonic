#ifndef TECTONIC_COMPRESS_H
#define TECTONIC_COMPRESS_H

#include <stddef.h>
#include <stdint.h>

typedef struct {
    const unsigned char *input;
    uint64_t input_len;
    unsigned char *output;
    uint64_t output_len;
    int32_t result;
} tt_pdf_compression_job;

void tt_pdf_compress_streams(tt_pdf_compression_job *jobs, size_t count, uint32_t level);

#endif
