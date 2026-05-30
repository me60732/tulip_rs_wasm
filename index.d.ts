/**
 * tulip-rs-wasm — WebAssembly bindings for the tulip_rs technical analysis
 * library.  Runs directly in the browser with no native dependencies.
 *
 * ## Quick-start (bundler / Vite / webpack)
 *
 * ```ts
 * import { init, sma, ema } from 'tulip-rs-wasm';
 *
 * await init();  // load & compile the WASM module once
 *
 * const [outputs, state] = sma.indicator([[...close]], [14]);
 * console.log(Array.from(outputs[0])); // Float64Array → number[]
 * console.log(sma.info);
 *
 * // Streaming — feed new bars without reprocessing history
 * const next = state.batchIndicator([[newBar]]);
 *
 * // State serialisation (JSON)
 * const json = state.toJson();
 * const back = sma.State.fromJson(json);
 * ```
 *
 * ## Plain HTML / CDN (web target — no bundler)
 *
 * ```html
 * <script type="module">
 *   import { init, sma } from
 *     'https://cdn.jsdelivr.net/npm/tulip-rs-wasm@0.1.0/index.js';
 *   await init(
 *     'https://cdn.jsdelivr.net/npm/tulip-rs-wasm@0.1.0/pkg/tulip_rs_wasm_bg.wasm'
 *   );
 *   const [outputs] = sma.indicator([[...close]], [14]);
 *   console.log(Array.from(outputs[0]));
 * </script>
 * ```
 *
 * ## Key differences from tulip-rs-node
 *
 * - `await init()` must be called once before any indicator is used.
 * - `simdByAssets` / `simdByOptions` are **not** available — omitted to keep
 *   the WASM bundle small.
 * - Outputs are `Float64Array[]` (zero-copy WASM memory views) instead of
 *   `number[][]`.  Convert with `Array.from(outputs[0])` if a plain array is
 *   needed.
 * - State serialisation uses `toJson()` / `fromJson()` only — no binary
 *   buffer methods.
 *
 * Build sequence (Rust must compile before TypeScript):
 *   npm run build:wasm:web  →  wasm-pack build --target web, emits pkg/
 *   npm run build:ts        →  tsc, emits index.js + index.d.ts
 *   npm run build:web       →  both in one step
 */
import { Indicator } from "./src-ts/indicator.js";
export { Indicator };
export type { IndicatorInfo, DisplayGroup } from "./src-ts/indicator.js";
/**
 * Load and initialise the WASM module.  Must be awaited once before calling
 * any indicator function or accessing `indicator.info`.
 *
 * @param wasmPath  Optional URL / path to the `.wasm` file.  When omitted the
 *                  bundler (Vite, webpack, etc.) resolves the asset
 *                  automatically.  For plain `<script type="module">` usage
 *                  pass the URL explicitly, e.g.
 *                  `await init(new URL('./pkg/tulip_rs_wasm_bg.wasm', import.meta.url))`.
 */
export declare function init(wasmPath?: string | URL | Request): Promise<void>;
export declare const ad: Indicator<unknown>;
export declare const adosc: Indicator<unknown>;
export declare const adx: Indicator<unknown>;
export declare const adxr: Indicator<unknown>;
export declare const ao: Indicator<unknown>;
export declare const apo: Indicator<unknown>;
export declare const aroon: Indicator<unknown>;
export declare const aroonosc: Indicator<unknown>;
export declare const atr: Indicator<unknown>;
export declare const avgprice: Indicator<unknown>;
export declare const bbands: Indicator<unknown>;
export declare const bop: Indicator<unknown>;
export declare const candlestick: Indicator<unknown>;
export declare const cci: Indicator<unknown>;
export declare const cmo: Indicator<unknown>;
export declare const cvi: Indicator<unknown>;
export declare const dema: Indicator<unknown>;
export declare const di: Indicator<unknown>;
export declare const dm: Indicator<unknown>;
export declare const dpo: Indicator<unknown>;
export declare const dx: Indicator<unknown>;
export declare const ema: Indicator<unknown>;
export declare const emv: Indicator<unknown>;
export declare const fisher: Indicator<unknown>;
export declare const fosc: Indicator<unknown>;
export declare const hma: Indicator<unknown>;
export declare const kama: Indicator<unknown>;
export declare const kvo: Indicator<unknown>;
export declare const linreg: Indicator<unknown>;
export declare const macd: Indicator<unknown>;
export declare const marketfi: Indicator<unknown>;
export declare const mass: Indicator<unknown>;
export declare const max: Indicator<unknown>;
export declare const md: Indicator<unknown>;
export declare const medprice: Indicator<unknown>;
export declare const mfi: Indicator<unknown>;
export declare const min: Indicator<unknown>;
export declare const mom: Indicator<unknown>;
export declare const msw: Indicator<unknown>;
export declare const natr: Indicator<unknown>;
export declare const nvi: Indicator<unknown>;
export declare const obv: Indicator<unknown>;
export declare const pivotpoint: Indicator<unknown>;
export declare const ppo: Indicator<unknown>;
export declare const psar: Indicator<unknown>;
export declare const pvi: Indicator<unknown>;
export declare const qstick: Indicator<unknown>;
export declare const roc: Indicator<unknown>;
export declare const rocr: Indicator<unknown>;
export declare const rsi: Indicator<unknown>;
export declare const sma: Indicator<unknown>;
export declare const stddev: Indicator<unknown>;
export declare const stoch: Indicator<unknown>;
export declare const stochrsi: Indicator<unknown>;
export declare const tema: Indicator<unknown>;
export declare const tr: Indicator<unknown>;
export declare const trima: Indicator<unknown>;
export declare const trix: Indicator<unknown>;
export declare const tsf: Indicator<unknown>;
export declare const typprice: Indicator<unknown>;
export declare const ultosc: Indicator<unknown>;
export declare const vhf: Indicator<unknown>;
export declare const vidya: Indicator<unknown>;
export declare const volatility: Indicator<unknown>;
export declare const vosc: Indicator<unknown>;
export declare const vwma: Indicator<unknown>;
export declare const wad: Indicator<unknown>;
export declare const wcprice: Indicator<unknown>;
export declare const wilders: Indicator<unknown>;
export declare const willr: Indicator<unknown>;
export declare const wma: Indicator<unknown>;
export declare const zlema: Indicator<unknown>;
