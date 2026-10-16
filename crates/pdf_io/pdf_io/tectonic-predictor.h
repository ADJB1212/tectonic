#ifndef TECTONIC_PREDICTOR_H
#define TECTONIC_PREDICTOR_H

#include <stdint.h>

void tt_pdf_filter_png_rows(const unsigned char *raster, unsigned char *destination,
                           int32_t row_bytes, int bytes_per_pixel,
                           int32_t first_row, int32_t last_row);
void tt_pdf_filter_png_parallel(const unsigned char *raster, unsigned char *destination,
                               int32_t row_bytes, int32_t rows, int bytes_per_pixel);

#endif
