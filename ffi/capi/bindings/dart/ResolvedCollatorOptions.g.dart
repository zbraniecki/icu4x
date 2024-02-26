// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class _ResolvedCollatorOptionsFfi extends ffi.Struct {
  @ffi.Int32()
  external int strength;
  @ffi.Int32()
  external int alternateHandling;
  @ffi.Int32()
  external int caseFirst;
  @ffi.Int32()
  external int maxVariable;
  @ffi.Int32()
  external int caseLevel;
  @ffi.Int32()
  external int numeric;
  @ffi.Int32()
  external int backwardSecondLevel;
}

/// See the [Rust documentation for `ResolvedCollatorOptions`](https://docs.rs/icu/latest/icu/collator/struct.ResolvedCollatorOptions.html) for more information.
final class ResolvedCollatorOptions {
  final CollatorStrength strength;
  final CollatorAlternateHandling alternateHandling;
  final CollatorCaseFirst caseFirst;
  final CollatorMaxVariable maxVariable;
  final CollatorCaseLevel caseLevel;
  final CollatorNumeric numeric;
  final CollatorBackwardSecondLevel backwardSecondLevel;

  // ignore: unused_element
  // Internal constructor from FFI.
  // This struct contains borrowed fields, so this takes in a list of
  // "edges" corresponding to where each lifetime's data may have been borrowed from
  // and passes it down to individual fields containing the borrow.
  // This method does not attempt to handle any dependencies between lifetimes, the caller
  // should handle this when constructing edge arrays.
  ResolvedCollatorOptions._(_ResolvedCollatorOptionsFfi underlying) :
    strength = CollatorStrength.values[underlying.strength],
    alternateHandling = CollatorAlternateHandling.values[underlying.alternateHandling],
    caseFirst = CollatorCaseFirst.values[underlying.caseFirst],
    maxVariable = CollatorMaxVariable.values[underlying.maxVariable],
    caseLevel = CollatorCaseLevel.values[underlying.caseLevel],
    numeric = CollatorNumeric.values[underlying.numeric],
    backwardSecondLevel = CollatorBackwardSecondLevel.values[underlying.backwardSecondLevel];

  // ignore: unused_element
  _ResolvedCollatorOptionsFfi _pointer(ffi.Allocator temp) {
    final pointer = temp<_ResolvedCollatorOptionsFfi>();
    pointer.ref.strength = strength.index;
    pointer.ref.alternateHandling = alternateHandling.index;
    pointer.ref.caseFirst = caseFirst.index;
    pointer.ref.maxVariable = maxVariable.index;
    pointer.ref.caseLevel = caseLevel.index;
    pointer.ref.numeric = numeric.index;
    pointer.ref.backwardSecondLevel = backwardSecondLevel.index;
    return pointer.ref;
  }

  @override
  bool operator ==(Object other) =>
      other is ResolvedCollatorOptions &&
      other.strength == this.strength &&
      other.alternateHandling == this.alternateHandling &&
      other.caseFirst == this.caseFirst &&
      other.maxVariable == this.maxVariable &&
      other.caseLevel == this.caseLevel &&
      other.numeric == this.numeric &&
      other.backwardSecondLevel == this.backwardSecondLevel;

  @override
  int get hashCode => Object.hashAll([
        this.strength,
        this.alternateHandling,
        this.caseFirst,
        this.maxVariable,
        this.caseLevel,
        this.numeric,
        this.backwardSecondLevel,
      ]);
}