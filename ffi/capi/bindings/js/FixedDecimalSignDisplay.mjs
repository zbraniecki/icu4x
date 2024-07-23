// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

// Base enumerator definition
/** ECMA-402 compatible sign display preference.
*
*See the [Rust documentation for `SignDisplay`](https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.SignDisplay.html) for more information.
*/
export class FixedDecimalSignDisplay {
    #value = undefined;

    static values = new Map([
        ["Auto", 0],
        ["Never", 1],
        ["Always", 2],
        ["ExceptZero", 3],
        ["Negative", 4]
    ]);
    constructor(value) {
        if (value instanceof FixedDecimalSignDisplay) {
            this.#value = value.value;
            return;
        }

        if (FixedDecimalSignDisplay.values.has(value)) {
            this.#value = value;
            return;
        }

        throw TypeError(value + " is not a FixedDecimalSignDisplay and does not correspond to any of its enumerator values.");
    }

    get value() {
        return this.#value;
    }

    get ffiValue() {
        return FixedDecimalSignDisplay.values.get(this.#value);
    }

    static Auto = new FixedDecimalSignDisplay("Auto");

    static Never = new FixedDecimalSignDisplay("Never");

    static Always = new FixedDecimalSignDisplay("Always");

    static ExceptZero = new FixedDecimalSignDisplay("ExceptZero");

    static Negative = new FixedDecimalSignDisplay("Negative");


    

}