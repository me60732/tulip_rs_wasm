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

import initWasm, * as wasmExports from "./pkg/tulip_rs_wasm.js";

import { Indicator } from "./src-ts/indicator.js";

export { Indicator };
export type { IndicatorInfo, DisplayGroup } from "./src-ts/indicator.js";

// ── Initialisation ────────────────────────────────────────────────────────────

let _initialised = false;

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
export async function init(wasmPath?: string | URL | Request): Promise<void> {
  if (_initialised) return;
  await initWasm(wasmPath);
  _initialised = true;
}

// ── Internal wasm module reference ───────────────────────────────────────────
// `wasmExports` is the live namespace imported from the wasm-pack generated
// module.  Indicator instances capture it at module-load time; all method
// calls are deferred until after init() so the WASM is ready.
const _wasm = wasmExports as Record<string, unknown>;

// ── Indicators ────────────────────────────────────────────────────────────────
export const ad = new Indicator("ad", _wasm);
export const adosc = new Indicator("adosc", _wasm);
export const adx = new Indicator("adx", _wasm);
export const adxr = new Indicator("adxr", _wasm);
export const ao = new Indicator("ao", _wasm);
export const apo = new Indicator("apo", _wasm);
export const aroon = new Indicator("aroon", _wasm);
export const aroonosc = new Indicator("aroonosc", _wasm);
export const atr = new Indicator("atr", _wasm);
export const avgprice = new Indicator("avgprice", _wasm);
export const bbands = new Indicator("bbands", _wasm);
export const bop = new Indicator("bop", _wasm);
export const candlestick = new Indicator("candlestick", _wasm);
export const cci = new Indicator("cci", _wasm);
export const chandelierexit = new Indicator("chandelierexit", _wasm);
export const cmo = new Indicator("cmo", _wasm);
export const cvi = new Indicator("cvi", _wasm);
export const dema = new Indicator("dema", _wasm);
export const di = new Indicator("di", _wasm);
export const dm = new Indicator("dm", _wasm);
export const donchianchannel = new Indicator("donchianchannel", _wasm);
export const dpo = new Indicator("dpo", _wasm);
export const dx = new Indicator("dx", _wasm);
export const ef = new Indicator("ef", _wasm);
export const ema = new Indicator("ema", _wasm);
export const emv = new Indicator("emv", _wasm);
export const fisher = new Indicator("fisher", _wasm);
export const fosc = new Indicator("fosc", _wasm);
export const hma = new Indicator("hma", _wasm);
export const kama = new Indicator("kama", _wasm);
export const keltnerchannel = new Indicator("keltnerchannel", _wasm);
export const kvo = new Indicator("kvo", _wasm);
export const linreg = new Indicator("linreg", _wasm);
export const macd = new Indicator("macd", _wasm);
export const marketfi = new Indicator("marketfi", _wasm);
export const mass = new Indicator("mass", _wasm);
export const max = new Indicator("max", _wasm);
export const md = new Indicator("md", _wasm);
export const medprice = new Indicator("medprice", _wasm);
export const mfi = new Indicator("mfi", _wasm);
export const min = new Indicator("min", _wasm);
export const mom = new Indicator("mom", _wasm);
export const msw = new Indicator("msw", _wasm);
export const natr = new Indicator("natr", _wasm);
export const nvi = new Indicator("nvi", _wasm);
export const obv = new Indicator("obv", _wasm);
export const pivotpoint = new Indicator("pivotpoint", _wasm);
export const ppo = new Indicator("ppo", _wasm);
export const psar = new Indicator("psar", _wasm);
export const pvi = new Indicator("pvi", _wasm);
export const qstick = new Indicator("qstick", _wasm);
export const roc = new Indicator("roc", _wasm);
export const rocr = new Indicator("rocr", _wasm);
export const rsi = new Indicator("rsi", _wasm);
export const sma = new Indicator("sma", _wasm);
export const smaenvelope = new Indicator("smaenvelope", _wasm);
export const stddev = new Indicator("stddev", _wasm);
export const stoch = new Indicator("stoch", _wasm);
export const stochrsi = new Indicator("stochrsi", _wasm);
export const tema = new Indicator("tema", _wasm);
export const tr = new Indicator("tr", _wasm);
export const trima = new Indicator("trima", _wasm);
export const trix = new Indicator("trix", _wasm);
export const tsf = new Indicator("tsf", _wasm);
export const typprice = new Indicator("typprice", _wasm);
export const ultosc = new Indicator("ultosc", _wasm);
export const vhf = new Indicator("vhf", _wasm);
export const vidya = new Indicator("vidya", _wasm);
export const volatility = new Indicator("volatility", _wasm);
export const vosc = new Indicator("vosc", _wasm);
export const vwma = new Indicator("vwma", _wasm);
export const wad = new Indicator("wad", _wasm);
export const wcprice = new Indicator("wcprice", _wasm);
export const wilders = new Indicator("wilders", _wasm);
export const willr = new Indicator("willr", _wasm);
export const wma = new Indicator("wma", _wasm);
export const zlema = new Indicator("zlema", _wasm);
