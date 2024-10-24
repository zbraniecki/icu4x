#ifndef ICU4XIsoWeekday_D_HPP
#define ICU4XIsoWeekday_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XIsoWeekday.d.h"


class ICU4XIsoWeekday {
public:
  enum Value {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
  };

  ICU4XIsoWeekday() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XIsoWeekday(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XIsoWeekday AsFFI() const;
  inline static ICU4XIsoWeekday FromFFI(capi::ICU4XIsoWeekday c_enum);
private:
    Value value;
};


#endif // ICU4XIsoWeekday_D_HPP
