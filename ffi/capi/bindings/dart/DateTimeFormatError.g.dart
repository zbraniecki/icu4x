// generated by diplomat-tool

part of 'lib.g.dart';

/// Additional information: [1](https://docs.rs/icu/latest/icu/datetime/enum.DateTimeWriteError.html)
enum DateTimeFormatError {
  unknown,

  missingInputField,

  zoneInfoMissingFields,

  invalidEra,

  invalidMonthCode,

  invalidCyclicYear,

  namesNotLoaded,

  fixedDecimalFormatterNotLoaded,

  unsupportedField;

  int get _ffi {
    switch (this) {
      case unknown:
        return 0;
      case missingInputField:
        return 1;
      case zoneInfoMissingFields:
        return 2;
      case invalidEra:
        return 3;
      case invalidMonthCode:
        return 4;
      case invalidCyclicYear:
        return 5;
      case namesNotLoaded:
        return 16;
      case fixedDecimalFormatterNotLoaded:
        return 17;
      case unsupportedField:
        return 18;
    }
  }
}