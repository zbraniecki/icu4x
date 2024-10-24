#ifndef ICU4XIsoTimeZoneFormat_D_H
#define ICU4XIsoTimeZoneFormat_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XIsoTimeZoneFormat {
  ICU4XIsoTimeZoneFormat_Basic = 0,
  ICU4XIsoTimeZoneFormat_Extended = 1,
  ICU4XIsoTimeZoneFormat_UtcBasic = 2,
  ICU4XIsoTimeZoneFormat_UtcExtended = 3,
} ICU4XIsoTimeZoneFormat;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XIsoTimeZoneFormat_D_H
