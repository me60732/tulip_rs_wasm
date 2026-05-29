# tulip-rs-wasm

[![npm](https://img.shields.io/npm/v/tulip-rs-wasm.svg)](https://www.npmjs.com/package/tulip-rs-wasm)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Docs](https://img.shields.io/badge/docs-me60732.github.io-blue)](https://me60732.github.io/tulip_rs/)

**Technical analysis for the browser — powered by Rust and WebAssembly.**

WebAssembly bindings for [TulipRS](https://github.com/me60732/tulip_rs).
Implements 70+ technical indicators and 60+ candlestick patterns that run
directly in the browser with no backend, no native dependencies, and no
build step required (CDN delivery available).

Full API documentation: [me60732.github.io/tulip_rs](https://me60732.github.io/tulip_rs/)

---

## Why tulip-rs-wasm?

| | tulip-rs-wasm | technicalindicators | tulipindicators (JS) |
|---|---|---|---|
| **Runs in browser** | ✅ WASM — no backend | ✅ pure JS | ✅ pure JS |
| **Performance** | ✅ Rust WASM | ❌ interpreted JS | ❌ interpreted JS |
| **Stateful streaming** | ✅ resume from saved state | ❌ full recompute each tick | ❌ full recompute each tick |
| **State serialisation** | ✅ JSON | ❌ | ❌ |
| **Candlestick patterns** | ✅ 60+ patterns | limited | ❌ |
| **CDN delivery** | ✅ jsDelivr / unpkg | ✅ | ✅ |
| **No build step** | ✅ | ✅ | ✅ |

---

## Installation

### npm / bundler (Vite, webpack, Rollup, esbuild)

```bash
npm install tulip-rs-wasm
```

### CDN — plain HTML, no build step

```html
<script type="module">
  import { init, sma } from
    'https://cdn.jsdelivr.net/npm/tulip-rs-wasm@0.1.0/index.js';

  await init(
    'https://cdn.jsdelivr.net/npm/tulip-rs-wasm@0.1.0/pkg/tulip_rs_wasm_bg.wasm'
  );

  const close = [81.59, 81.06, 82.87, 83.00, 83.61, 83.15, 82.84, 83.99, 84.55, 84.36];
  const [outputs] = sma.indicator([close], [5]);
  console.log(Array.from(outputs[0]));
</script>
```

---

## Quick Start

`await init()` must be called **once** before any indicator is used.
The rest of the API is identical to `tulip-rs-node`.

```js
import { init, sma, ema, macd } from 'tulip-rs-wasm';

await init(); // load and compile the WASM module once

const close = [81.59, 81.06, 82.87, 83.00, 83.61,
               83.15, 82.84, 83.99, 84.55, 84.36];

// inputs: number[][]  |  options: number[]
// outputs: Float64Array[]  (use Array.from(outputs[0]) for a plain array)
const [outputs, state] = sma.indicator([close], [5]);

console.log(Array.from(outputs[0])); // SMA(5) values

// Streaming: feed new bars without reprocessing history
const newBar = [85.10];
const nextOutputs = state.batchIndicator([newBar]);
```

---

## API

### Initialisation

```js
// Bundler (Vite, webpack) — bundler resolves the WASM asset automatically
await init();

// Plain HTML / CDN — pass the WASM URL explicitly
await init('https://cdn.jsdelivr.net/npm/tulip-rs-wasm@0.1.0/pkg/tulip_rs_wasm_bg.wasm');

// Local web-target build
await init(new URL('./pkg/tulip_rs_wasm_bg.wasm', import.meta.url));
```

`init()` is a no-op on subsequent calls — safe to call more than once.

### Indicator info

```js
const info = sma.info;
// {
//   name: 'sma',
//   fullName: 'Simple Moving Average',
//   inputs: ['real'],
//   options: ['period'],
//   outputs: ['sma'],
//   optionalOutputs: [],
//   indicatorType: 'Trend',
//   displayType: 'Overlay'
// }

sma.minData([5]);            // minimum bars needed to produce output
sma.minDataAccuracy([5], 6); // bars needed for 6-decimal accuracy
```

### Running an indicator

```js
const [outputs, state] = sma.indicator([close], [5]);
// outputs: Float64Array[]  — one typed array per output series
// state:   SmaState        — snapshot of internal state after the last bar

// Convert to a plain number array if needed
const values = Array.from(outputs[0]);
```

### Streaming continuation

Save the state after an initial batch, then feed new bars incrementally without
touching the history:

```js
const [outputs, state] = sma.indicator([close.slice(0, -5)], [5]);

// later — new bars arrive
const newOutputs = state.batchIndicator([close.slice(-5)]);
```

### State serialisation

States can be round-tripped to JSON (human-readable, useful for persistence or
cross-environment transfer):

```js
// save
const json = state.toJson();

// restore
const restored = sma.State.fromJson(json);
const continued = restored.batchIndicator([close.slice(-5)]);
```

### Multi-input indicators

Indicators that need more than one price series take them as additional arrays
in the `inputs` argument:

```js
// STOCH — high, low, close
const [outputs] = stoch.indicator([high, low, close], [5, 3, 3]);
// outputs[0] → %K line
// outputs[1] → %D line

// MACD — close only, three output series
const [outputs] = macd.indicator([close], [2, 5, 9]);
// outputs[0] → MACD line
// outputs[1] → Signal line
// outputs[2] → Histogram

// AD — high, low, close, volume
const [outputs] = ad.indicator([high, low, close, volume], []);
```

### Candlestick patterns

The candlestick indicator returns pattern objects per bar instead of numeric
series:

```js
const [result, state] = candlestick.indicator(
  [open, high, low, close],
  [5, 1, 1], // candle_period, trend_period, trend_signal_period
);

result.forEach((patterns, bar) => {
  if (patterns && patterns.length > 0) {
    patterns.forEach(p => {
      console.log(`Bar ${bar}: ${p.fullName} (${p.forecast})`);
    });
  }
});

// Streaming — append new bars
const newPatterns = state.batchIndicator([[newOpen, newHigh, newLow, newClose]]);
```

Each pattern object has:

```js
{
  name:         'ThreeWhiteSoldiers',
  fullName:     'Three White Soldiers',
  japaneseName: 'akasankuusen',
  bars:         3,
  forecast:     'BullishReversal'
}
```

---

## Differences from tulip-rs-node

| | tulip-rs-wasm | tulip-rs-node |
|---|---|---|
| Initialisation | `await init()` required | immediate, synchronous |
| Output type | `Float64Array[]` | `number[][]` |
| State serialisation | `toJson()` / `fromJson()` | `toBuffer()` / `fromBuffer()` + JSON |
| SIMD — multiple assets | ❌ not available | ✅ `simdByAssets` |
| SIMD — multiple options | ❌ not available | ✅ `simdByOptions` |
| Runtime | Browser / WASM runtime | Node.js |

> **Note:** `Float64Array` values are zero-copy WASM memory views.
> Use `Array.from(outputs[0])` if you need a plain JS number array.

---

## Indicators

| Category | Indicators |
|---|---|
| **Trend (Overlay)** | SMA, EMA, DEMA, TEMA, WMA, HMA, KAMA, TRIMA, ZLEMA, WILDERS, VIDYA, LINREG, TSF, PSAR |
| **Momentum** | RSI, CMO, MOM, ROC, ROCR, STOCH, STOCHRSI, DPO, FOSC, MACD, APO, PPO |
| **Volatility (Overlay)** | BBANDS |
| **Volatility (Oscillator)** | ATR, NATR, VOLATILITY, STDDEV, MD |
| **Volume** | OBV, AD, ADOSC, MFI, EMV, NVI, PVI, KVO, VWMA, VOSC |
| **Directional** | ADX, ADXR, DI, DM, DX, AROON, AROONOSC |
| **Price (Overlay)** | AVGPRICE, MEDPRICE, TYPPRICE, WCPRICE |
| **Other** | AO, BOP, CCI, CVI, FISHER, MASS, MARKETFI, MSW, QSTICK, TR, VHF, WAD, WILLR, PIVOTPOINT, ULTOSC |
| **Candlestick** | 60+ patterns via `candlestick` |

---

## Running the Examples

Build the package first, then serve the directory and open any example in a browser:

```bash
# Build WASM (web target) and compile TypeScript
npm run build:web

# Serve with any static file server
npx serve .
# then open e.g. http://localhost:3000/examples/ti_sma_example.html
```

Examples are provided for every indicator under `examples/`.

---

## Build from Source

Requires Rust nightly (pinned via `rust-toolchain.toml`), the
`wasm32-unknown-unknown` target, and
[`wasm-pack`](https://rustwasm.github.io/wasm-pack/).

```bash
# Install wasm-pack (if not already installed)
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

git clone https://github.com/me60732/tulip-rs-wasm
cd tulip-rs-wasm
npm install
npm run build:web   # WASM (web target) + TypeScript
```

---

## Language Support

| Language | Status | Package |
|---|---|---|
| **Browser (WASM)** | ✅ Supported | [`tulip-rs-wasm`](https://www.npmjs.com/package/tulip-rs-wasm) (this repo) |
| **Node.js** | ✅ Supported | [`tulip-rs-node`](https://www.npmjs.com/package/tulip-rs-node) |
| **Rust** | ✅ Native | [`tulip_rs`](https://github.com/me60732/tulip_rs) |
| **Python** | ✅ Supported | [`tulip-rs`](https://pypi.org/project/tulip-rs/) · `pip install tulip-rs` |
| R | 🔜 Planned | — |
| Julia | 🔜 Planned | — |

---

## License

[MIT](LICENSE)
