// generated by diplomat-tool
import { WeekRelativeUnit } from "./WeekRelativeUnit.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `WeekOf`](https://docs.rs/icu/latest/icu/calendar/week/struct.WeekOf.html) for more information.
*/
export class WeekOf {
    #week;
    get week()  {
        return this.#week;
    }
    
    #unit;
    get unit()  {
        return this.#unit;
    }
    

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    
    _intoFFI(
        slice_cleanup_callbacks,
        appendArrayMap
    ) {
        return [this.#week, this.#unit.ffiValue]
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    _fromFFI(ptr) {
        const weekDeref = (new Uint16Array(wasm.memory.buffer, ptr, 1))[0];
        this.#week = weekDeref;
        const unitDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 4);
        this.#unit = WeekRelativeUnit[Array.from(WeekRelativeUnit.values.keys())[unitDeref]];

        return this;
    }
    // This is an out struct. You need to call other methods to be able to get this struct.
    constructor(ptr) {
        this._fromFFI(ptr);
    }
    

}