#ifndef TECTONIC_PDF_PIPELINE_H
#define TECTONIC_PDF_PIPELINE_H

#include <stddef.h>
#include <stdint.h>

enum {
    TT_PDF_RAW_BYTES,
    TT_PDF_TEXT,
    TT_PDF_RULE,
    TT_PDF_GRAPHICS_MODE,
    TT_PDF_DIRECTION,
    TT_PDF_RESET_FONTS
};

typedef struct {
    char name[16];
    int32_t size;
    int format;
    int wmode;
    double extend;
    double slant;
    double bold;
} tt_pdf_render_font;

typedef struct {
    int kind;
    int font;
    int32_t x, y, width, height;
    const unsigned char *data;
    size_t length;
} tt_pdf_render_command;

typedef struct {
    void *target;
    const unsigned char *data;
    size_t length;
    int kind;
    int status;
    void *handle;
} tt_pdf_task_result;

int tt_pdf_pipeline_active(void);
void tt_pdf_phase_start(int phase);
void tt_pdf_phase_end(void);
int tt_pdf_special_supported(const unsigned char *data, size_t length);
void tt_pdf_pipeline_enable_pages(int enabled);
int tt_pdf_page_begin(double unit, int precision, int autorotate, int direction);
int tt_pdf_page_recording(void);
void tt_pdf_page_text(const tt_pdf_render_font *font, int id, int32_t x, int32_t y,
                      int32_t width, const unsigned char *data, size_t length);
void tt_pdf_page_command(int kind, int32_t x, int32_t y, int32_t width, int32_t height);
int tt_pdf_page_bytes(const char *data, size_t length);
size_t tt_pdf_page_size(void);
int tt_pdf_pipeline_has_capacity(size_t bytes);
int tt_pdf_page_submit(void *target, unsigned int compression_level);
int tt_pdf_stream_submit(void *target, const unsigned char *data, size_t length,
                        unsigned int level, int predictor, int columns, int bpc, int colors);
int tt_pdf_pipeline_take(tt_pdf_task_result *result);
void tt_pdf_pipeline_release(void *result);
void tt_pdf_render_output(void *output, const char *data, size_t length);
int tt_pdf_render_page(const tt_pdf_render_command *commands, size_t count,
                       const tt_pdf_render_font *fonts, size_t font_count,
                       double unit, int precision, int autorotate, int direction, void *output);
void pdf_pipeline_drain(int all);

#endif
