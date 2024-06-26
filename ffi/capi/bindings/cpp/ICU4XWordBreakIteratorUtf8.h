#ifndef ICU4XWordBreakIteratorUtf8_H
#define ICU4XWordBreakIteratorUtf8_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XSegmenterWordType.d.h"
#include "ICU4XSegmenterWordType.h"

#include "ICU4XWordBreakIteratorUtf8.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


int32_t ICU4XWordBreakIteratorUtf8_next(ICU4XWordBreakIteratorUtf8* self);

ICU4XSegmenterWordType ICU4XWordBreakIteratorUtf8_word_type(const ICU4XWordBreakIteratorUtf8* self);

bool ICU4XWordBreakIteratorUtf8_is_word_like(const ICU4XWordBreakIteratorUtf8* self);

void ICU4XWordBreakIteratorUtf8_destroy(ICU4XWordBreakIteratorUtf8* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XWordBreakIteratorUtf8_H
