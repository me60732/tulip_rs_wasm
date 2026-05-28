/**
 * Shared types and the `Indicator<S>` wrapper class for tulip-rs-wasm.
 *
 * The class mirrors the `Indicator<S>` in `tulip-rs-node` as closely as
 * possible.  The main differences are:
 *
 *  • No `simdByAssets` / `simdByOptions` — excluded to minimise the WASM
 *    bundle size.
 *  • `info` is a lazy getter, not an eager property, because the wasm module
 *    may not be initialised when `Indicator` instances are first created.
 *  • Outputs are `Float64Array[]` (zero-copy WASM memory views) instead of
 *    `number[][]`.  Use `Array.from(outputs[0])` to convert if needed.
 *  • State serialisation uses `toJson()` / `fromJson()` only — no binary
 *    buffer methods.
 */
/** Indicator metadata — shape identical to tulip-rs-node's `IndicatorInfo`. */
export interface IndicatorInfo {
    name: string;
    fullName: string;
    inputs: string[];
    options: string[];
    outputs: string[];
    optionalOutputs: string[];
    indicatorType: string;
    displayType: string;
}
type AnyWasm = Record<string, any>;
/**
 * Wraps a single tulip_rs indicator's flat wasm-bindgen exports into a typed
 * namespaced object.
 *
 * `S` is the wasm-bindgen State class for this indicator (e.g. `SmaState`).
 * Defaults to `unknown` so the class is usable before `wasm-pack build` has
 * been run.
 */
export declare class Indicator<S = unknown> {
    private readonly _name;
    private readonly _wasm;
    private _info;
    constructor(name: string, wasm: AnyWasm);
    /**
     * Static metadata — fetched lazily on first access after WASM is initialised.
     * Shape is identical to `tulip-rs-node`.
     */
    get info(): IndicatorInfo;
    /**
     * Run the indicator on a batch of data.
     * Returns `[outputs, state]` where `outputs` is an array of `Float64Array`
     * series (one per output channel) and `state` is the streaming state object.
     *
     * Identical call signature to `tulip-rs-node`.
     */
    indicator(inputs: number[][], options: number[], optionalOutputs?: boolean[]): [Float64Array[], S];
    /**
     * Minimum number of input bars required to produce at least one output bar.
     * Identical call signature to `tulip-rs-node`.
     */
    minData(options: number[]): number;
    /**
     * Minimum input bars needed to achieve a given decimal accuracy.
     * Identical call signature to `tulip-rs-node`.
     */
    minDataAccuracy(options: number[], decimals: number): number;
    /**
     * The wasm-bindgen State class for this indicator.
     * Use `sma.State.fromJson(json)` to restore a previously serialised state.
     */
    get State(): any;
}
export {};
