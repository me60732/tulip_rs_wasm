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
export interface DisplayGroup {
  /** Stable machine-readable key, e.g. `"adx_dx"` or `"true_range"`. */
  id: string;
  /** Human-readable pane title, e.g. `"Directional Index"`. */
  label: string;
  /** Where to render: `"Overlay"` | `"Indicator"` | `"Volume"`. */
  displayType: string;
  /** Output names belonging to this group (may include optional outputs). */
  outputs: string[];
}

/** Indicator metadata — shape identical to tulip-rs-node's `IndicatorInfo`. */
export interface IndicatorInfo {
  name: string;
  fullName: string;
  inputs: string[];
  options: string[];
  outputs: string[];
  optionalOutputs: string[];
  indicatorType: string;
  /** Groups of outputs that should be rendered together on the same pane. */
  displayGroups: DisplayGroup[];
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
type AnyWasm = Record<string, any>;

/**
 * Wraps a single tulip_rs indicator's flat wasm-bindgen exports into a typed
 * namespaced object.
 *
 * `S` is the wasm-bindgen State class for this indicator (e.g. `SmaState`).
 * Defaults to `unknown` so the class is usable before `wasm-pack build` has
 * been run.
 */
export class Indicator<S = unknown> {
  private readonly _name: string;
  private readonly _wasm: AnyWasm;
  private _info: IndicatorInfo | null = null;

  constructor(name: string, wasm: AnyWasm) {
    this._name = name;
    this._wasm = wasm;
  }

  /**
   * Static metadata — fetched lazily on first access after WASM is initialised.
   * Shape is identical to `tulip-rs-node`.
   */
  get info(): IndicatorInfo {
    if (!this._info) {
      this._info = (this._wasm[`${this._name}Info`] as () => IndicatorInfo)();
    }
    return this._info;
  }

  /**
   * Run the indicator on a batch of data.
   * Returns `[outputs, state]` where `outputs` is an array of `Float64Array`
   * series (one per output channel) and `state` is the streaming state object.
   *
   * Identical call signature to `tulip-rs-node`.
   */
  indicator(
    inputs: number[][],
    options: number[],
    optionalOutputs?: boolean[],
  ): [Float64Array[], S] {
    return (this._wasm[`${this._name}Indicator`] as Function)(
      inputs,
      options,
      optionalOutputs,
    ) as [Float64Array[], S];
  }

  /**
   * Minimum number of input bars required to produce at least one output bar.
   * Identical call signature to `tulip-rs-node`.
   */
  minData(options: number[]): number {
    return (this._wasm[`${this._name}MinData`] as Function)(options) as number;
  }

  /**
   * Minimum input bars needed to achieve a given decimal accuracy.
   * Identical call signature to `tulip-rs-node`.
   */
  minDataAccuracy(options: number[], decimals: number): number {
    return (this._wasm[`${this._name}MinDataAccuracy`] as Function)(
      options,
      decimals,
    ) as number;
  }

  /**
   * The wasm-bindgen State class for this indicator.
   * Use `sma.State.fromJson(json)` to restore a previously serialised state.
   */
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  get State(): any {
    const cap = this._name.charAt(0).toUpperCase() + this._name.slice(1);
    return this._wasm[`${cap}State`];
  }
}
