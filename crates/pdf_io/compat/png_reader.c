#include <png.h>
#include <stdint.h>
#include <string.h>

extern int tt_pdf_png_input_read(uintptr_t handle, unsigned char *bytes, size_t length);

static void reader_error(png_structp png, const char *message) {
    char *error = png_get_error_ptr(png);
    if (error) {
        strncpy(error, message, 1023);
        error[1023] = 0;
    }
    png_longjmp(png, 1);
}

static void reader_warning(png_structp png, const char *message) {
    (void)png;
    (void)message;
}

static void reader_read(png_structp png, unsigned char *bytes, size_t length) {
    uintptr_t handle = (uintptr_t)png_get_io_ptr(png);
    /* Rust returns before libpng can jump to the C operation boundary. */
    if (tt_pdf_png_input_read(handle, bytes, length) != 0)
        png_error(png, "error reading PNG");
}

png_structp tt_pdf_png_create(char *error, uintptr_t handle) {
    png_structp png = png_create_read_struct(PNG_LIBPNG_VER_STRING, error, reader_error, reader_warning);
    if (png)
        png_set_read_fn(png, (void *)handle, reader_read);
    return png;
}

png_infop tt_pdf_png_create_info(png_structp png) {
    if (setjmp(png_jmpbuf(png))) return NULL;
    return png_create_info_struct(png);
}

void tt_pdf_png_destroy(png_structp png, png_infop info) {
    png_destroy_read_struct(&png, &info, NULL);
}

const char *tt_pdf_png_error(png_structp png) {
    return png_get_error_ptr(png);
}

int tt_pdf_png_read_info(png_structp png, png_infop info) {
    if (setjmp(png_jmpbuf(png))) return -1;
    png_read_info(png, info);
    return 0;
}

int tt_pdf_png_update_info(png_structp png, png_infop info) {
    if (setjmp(png_jmpbuf(png))) return -1;
    png_read_update_info(png, info);
    return 0;
}

int tt_pdf_png_read_image(png_structp png, png_bytepp rows) {
    if (setjmp(png_jmpbuf(png))) return -1;
    png_read_image(png, rows);
    return 0;
}

int tt_pdf_png_read_end(png_structp png, png_infop info) {
    if (setjmp(png_jmpbuf(png))) return -1;
    png_read_end(png, info);
    return 0;
}

int tt_pdf_png_set_gamma(png_structp png, double screen, double file) {
    if (setjmp(png_jmpbuf(png))) return -1;
    png_set_gamma(png, screen, file);
    return 0;
}

int tt_pdf_png_set_background(png_structp png, png_const_color_16p color, int code, int expand, double gamma) {
    if (setjmp(png_jmpbuf(png))) return -1;
    png_set_background(png, color, code, expand, gamma);
    return 0;
}
