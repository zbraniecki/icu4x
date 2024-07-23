// generated by diplomat-tool
import type { pointer, char } from "./diplomat-runtime.d.ts";

// Base enumerator definition
/** See the [Rust documentation for `Direction`](https://docs.rs/icu/latest/icu/locale/enum.Direction.html) for more information.
*/
export class LocaleDirection {
    constructor(value : LocaleDirection | string);

    get value() : string;

    get ffiValue() : number;

    static LeftToRight : LocaleDirection;

    static RightToLeft : LocaleDirection;

    static Unknown : LocaleDirection;


    

}