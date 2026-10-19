/* Copyright 2026 the Tectonic Project
 * Licensed under the MIT License.
 */

#include <unicode/ubrk.h>
#include <unicode/ubidi.h>
#include <unicode/ucnv.h>

UConverter *tt_icu_ucnv_open(const char *name, UErrorCode *status)
{
    return ucnv_open(name, status);
}

void tt_icu_ucnv_close(UConverter *converter)
{
    ucnv_close(converter);
}

int32_t tt_icu_ucnv_to_algorithmic(UConverterType type, UConverter *converter,
                                 char *target, int32_t capacity,
                                 const char *source, int32_t length,
                                 UErrorCode *status)
{
    return ucnv_toAlgorithmic(type, converter, target, capacity, source, length, status);
}

UBreakIterator *tt_icu_ubrk_open(UBreakIteratorType type, const char *locale,
                               const UChar *text, int32_t length, UErrorCode *status)
{
    return ubrk_open(type, locale, text, length, status);
}

void tt_icu_ubrk_close(UBreakIterator *iterator)
{
    ubrk_close(iterator);
}

void tt_icu_ubrk_set_text(UBreakIterator *iterator, const UChar *text,
                          int32_t length, UErrorCode *status)
{
    ubrk_setText(iterator, text, length, status);
}

int32_t tt_icu_ubrk_next(UBreakIterator *iterator)
{
    return ubrk_next(iterator);
}

UBiDi *tt_icu_ubidi_open(void)
{
    return ubidi_open();
}

void tt_icu_ubidi_close(UBiDi *bidi)
{
    ubidi_close(bidi);
}

void tt_icu_ubidi_set_para(UBiDi *bidi, const UChar *text, int32_t length,
                          UBiDiLevel level, UBiDiLevel *embedding_levels,
                          UErrorCode *status)
{
    ubidi_setPara(bidi, text, length, level, embedding_levels, status);
}

UBiDiDirection tt_icu_ubidi_get_direction(const UBiDi *bidi)
{
    return ubidi_getDirection(bidi);
}

int32_t tt_icu_ubidi_count_runs(UBiDi *bidi, UErrorCode *status)
{
    return ubidi_countRuns(bidi, status);
}

UBiDiDirection tt_icu_ubidi_get_visual_run(UBiDi *bidi, int32_t run,
                                        int32_t *start, int32_t *length)
{
    return ubidi_getVisualRun(bidi, run, start, length);
}
