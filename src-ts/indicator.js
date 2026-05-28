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
/**
 * Wraps a single tulip_rs indicator's flat wasm-bindgen exports into a typed
 * namespaced object.
 *
 * `S` is the wasm-bindgen State class for this indicator (e.g. `SmaState`).
 * Defaults to `unknown` so the class is usable before `wasm-pack build` has
 * been run.
 */
export class Indicator {
  _name;
  _wasm;
  _info = null;
  constructor(name, wasm) {
    this._name = name;
    this._wasm = wasm;
  }
  /**
   * Static metadata — fetched lazily on first access after WASM is initialised.
   * Shape is identical to `tulip-rs-node`.
   */
  get info() {
    if (!this._info) {
      this._info = this._wasm[`${this._name}Info`]();
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
  indicator(inputs, options, optionalOutputs) {
    return this._wasm[`${this._name}Indicator`](
      inputs,
      options,
      optionalOutputs,
    );
  }
  /**
   * Minimum number of input bars required to produce at least one output bar.
   * Identical call signature to `tulip-rs-node`.
   */
  minData(options) {
    return this._wasm[`${this._name}MinData`](options);
  }
  /**
   * Minimum input bars needed to achieve a given decimal accuracy.
   * Identical call signature to `tulip-rs-node`.
   */
  minDataAccuracy(options, decimals) {
    return this._wasm[`${this._name}MinDataAccuracy`](options, decimals);
  }
  /**
   * The wasm-bindgen State class for this indicator.
   * Use `sma.State.fromJson(json)` to restore a previously serialised state.
   */
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  get State() {
    const cap = this._name.charAt(0).toUpperCase() + this._name.slice(1);
    return this._wasm[`${cap}State`];
  }
}
