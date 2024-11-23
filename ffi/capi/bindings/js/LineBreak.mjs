// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

// Base enumerator definition
/** See the [Rust documentation for `LineBreak`](https://docs.rs/icu/latest/icu/properties/props/struct.LineBreak.html) for more information.
*/
export class LineBreak {
    #value = undefined;

    static #values = new Map([
        ["Unknown", 0],
        ["Ambiguous", 1],
        ["Alphabetic", 2],
        ["BreakBoth", 3],
        ["BreakAfter", 4],
        ["BreakBefore", 5],
        ["MandatoryBreak", 6],
        ["ContingentBreak", 7],
        ["ClosePunctuation", 8],
        ["CombiningMark", 9],
        ["CarriageReturn", 10],
        ["Exclamation", 11],
        ["Glue", 12],
        ["Hyphen", 13],
        ["Ideographic", 14],
        ["Inseparable", 15],
        ["InfixNumeric", 16],
        ["LineFeed", 17],
        ["Nonstarter", 18],
        ["Numeric", 19],
        ["OpenPunctuation", 20],
        ["PostfixNumeric", 21],
        ["PrefixNumeric", 22],
        ["Quotation", 23],
        ["ComplexContext", 24],
        ["Surrogate", 25],
        ["Space", 26],
        ["BreakSymbols", 27],
        ["ZwSpace", 28],
        ["NextLine", 29],
        ["WordJoiner", 30],
        ["H2", 31],
        ["H3", 32],
        ["Jl", 33],
        ["Jt", 34],
        ["Jv", 35],
        ["CloseParenthesis", 36],
        ["ConditionalJapaneseStarter", 37],
        ["HebrewLetter", 38],
        ["RegionalIndicator", 39],
        ["EBase", 40],
        ["EModifier", 41],
        ["Zwj", 42],
        ["Aksara", 43],
        ["AksaraPrebase", 44],
        ["AksaraStart", 45],
        ["ViramaFinal", 46],
        ["Virama", 47]
    ]);

    static getAllEntries() {
        return LineBreak.#values.entries();
    }

    constructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return;
            }
            return LineBreak.#objectValues[arguments[1]];
        }

        if (value instanceof LineBreak) {
            return value;
        }

        let intVal = LineBreak.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal == null) {
            return LineBreak.#objectValues[intVal];
        }

        throw TypeError(value + " is not a LineBreak and does not correspond to any of its enumerator values.");
    }

    get value() {
        return [...LineBreak.#values.keys()][this.#value];
    }

    get ffiValue() {
        return this.#value;
    }
    static #objectValues = [
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 3),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 4),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 5),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 6),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 7),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 8),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 9),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 10),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 11),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 12),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 13),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 14),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 15),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 16),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 17),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 18),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 19),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 20),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 21),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 22),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 23),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 24),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 25),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 26),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 27),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 28),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 29),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 30),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 31),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 32),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 33),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 34),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 35),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 36),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 37),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 38),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 39),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 40),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 41),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 42),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 43),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 44),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 45),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 46),
        new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 47),
    ];

    static Unknown = LineBreak.#objectValues[0];
    static Ambiguous = LineBreak.#objectValues[1];
    static Alphabetic = LineBreak.#objectValues[2];
    static BreakBoth = LineBreak.#objectValues[3];
    static BreakAfter = LineBreak.#objectValues[4];
    static BreakBefore = LineBreak.#objectValues[5];
    static MandatoryBreak = LineBreak.#objectValues[6];
    static ContingentBreak = LineBreak.#objectValues[7];
    static ClosePunctuation = LineBreak.#objectValues[8];
    static CombiningMark = LineBreak.#objectValues[9];
    static CarriageReturn = LineBreak.#objectValues[10];
    static Exclamation = LineBreak.#objectValues[11];
    static Glue = LineBreak.#objectValues[12];
    static Hyphen = LineBreak.#objectValues[13];
    static Ideographic = LineBreak.#objectValues[14];
    static Inseparable = LineBreak.#objectValues[15];
    static InfixNumeric = LineBreak.#objectValues[16];
    static LineFeed = LineBreak.#objectValues[17];
    static Nonstarter = LineBreak.#objectValues[18];
    static Numeric = LineBreak.#objectValues[19];
    static OpenPunctuation = LineBreak.#objectValues[20];
    static PostfixNumeric = LineBreak.#objectValues[21];
    static PrefixNumeric = LineBreak.#objectValues[22];
    static Quotation = LineBreak.#objectValues[23];
    static ComplexContext = LineBreak.#objectValues[24];
    static Surrogate = LineBreak.#objectValues[25];
    static Space = LineBreak.#objectValues[26];
    static BreakSymbols = LineBreak.#objectValues[27];
    static ZwSpace = LineBreak.#objectValues[28];
    static NextLine = LineBreak.#objectValues[29];
    static WordJoiner = LineBreak.#objectValues[30];
    static H2 = LineBreak.#objectValues[31];
    static H3 = LineBreak.#objectValues[32];
    static Jl = LineBreak.#objectValues[33];
    static Jt = LineBreak.#objectValues[34];
    static Jv = LineBreak.#objectValues[35];
    static CloseParenthesis = LineBreak.#objectValues[36];
    static ConditionalJapaneseStarter = LineBreak.#objectValues[37];
    static HebrewLetter = LineBreak.#objectValues[38];
    static RegionalIndicator = LineBreak.#objectValues[39];
    static EBase = LineBreak.#objectValues[40];
    static EModifier = LineBreak.#objectValues[41];
    static Zwj = LineBreak.#objectValues[42];
    static Aksara = LineBreak.#objectValues[43];
    static AksaraPrebase = LineBreak.#objectValues[44];
    static AksaraStart = LineBreak.#objectValues[45];
    static ViramaFinal = LineBreak.#objectValues[46];
    static Virama = LineBreak.#objectValues[47];

    toInteger() {
        const result = wasm.icu4x_LineBreak_to_integer_mv1(this.ffiValue);
    
        try {
            return result;
        }
        
        finally {}
    }

    static fromInteger(other) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);
        
        const result = wasm.icu4x_LineBreak_from_integer_mv1(diplomatReceive.buffer, other);
    
        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return new LineBreak(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
        }
        
        finally {
            diplomatReceive.free();
        }
    }
}