use crate::utils::{info_to_object, inputs_from_js, make_pair};
use serde::Serialize;
use tulip_rs::candle_indicators::candle_patterns::CandlePattern;
use tulip_rs::candle_indicators::types::ForecastType as RustForecastType;
use tulip_rs::indicators::candlestick as rust_cdl;
use wasm_bindgen::prelude::*;

const IW: usize = rust_cdl::INPUTS_WIDTH;
const OW: usize = rust_cdl::OPTIONS_WIDTH;

// ── ForecastType ─────────────────────────────────────────────────────────────

/// Parse a string into the Rust `ForecastType`. Accepted values:
/// `"BearishReversal"`, `"BullishReversal"`, `"BearishContinuation"`,
/// `"BullishContinuation"`, `"BearishReversalOrContinuation"`,
/// `"BullishReversalOrContinuation"`.
fn parse_forecast_type(s: &str) -> Result<RustForecastType, JsError> {
    match s {
        "BearishReversal" => Ok(RustForecastType::BearishReversal),
        "BullishReversal" => Ok(RustForecastType::BullishReversal),
        "BearishContinuation" => Ok(RustForecastType::BearishContinuation),
        "BullishContinuation" => Ok(RustForecastType::BullishContinuation),
        "BearishReversalOrContinuation" => Ok(RustForecastType::BearishReversalOrContinuation),
        "BullishReversalOrContinuation" => Ok(RustForecastType::BullishReversalOrContinuation),
        _ => Err(JsError::new(&format!(
            "Unknown ForecastType: '{s}'. Expected one of: BearishReversal, BullishReversal, \
             BearishContinuation, BullishContinuation, BearishReversalOrContinuation, \
             BullishReversalOrContinuation"
        ))),
    }
}

// ── Pattern output type ───────────────────────────────────────────────────────

/// A single detected candlestick pattern on a bar.
/// Serialised as a plain JS object (identical shape to `tulip-rs-node`).
#[derive(Serialize)]
struct CandlePatternObject {
    pub name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    #[serde(rename = "japaneseName")]
    pub japanese_name: String,
    pub bars: u32,
    pub forecast: String,
}

fn pattern_to_object(p: CandlePattern) -> CandlePatternObject {
    let info = p.get_info();
    CandlePatternObject {
        name: format!("{p:?}"),
        full_name: info.full_name.to_string(),
        japanese_name: info.japanese_name.to_string(),
        bars: info.bars as u32,
        forecast: format!("{:?}", info.forecast),
    }
}

fn convert_patterns(raw: Vec<Option<Vec<CandlePattern>>>) -> Vec<Option<Vec<CandlePatternObject>>> {
    raw.into_iter()
        .map(|entry| entry.map(|ps| ps.into_iter().map(pattern_to_object).collect()))
        .collect()
}

// ── State class ──────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct CandlestickState {
    inner: rust_cdl::IndicatorState,
}

#[wasm_bindgen]
impl CandlestickState {
    /// Continue streaming: feed new bars into an existing state.
    /// `forecast_type`: optional string filter — see `parse_forecast_type`.
    /// Returns a JS array where each element is `null` or an array of pattern objects.
    #[wasm_bindgen(js_name = "batchIndicator")]
    pub fn batch_indicator(
        &mut self,
        inputs: JsValue,
        forecast_type: Option<String>,
    ) -> Result<JsValue, JsError> {
        let inputs = inputs_from_js(inputs)?;
        let input_arr: [&[f64]; IW] = inputs
            .iter()
            .map(|v| v.as_slice())
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| JsError::new(&format!("Expected {IW} input series")))?;
        let forecast = forecast_type.map(|s| parse_forecast_type(&s)).transpose()?;
        let raw = self
            .inner
            .batch_indicator(&input_arr, forecast)
            .map_err(|e| JsError::new(&format!("{e:?}")))?;
        let patterns = convert_patterns(raw);
        serde_wasm_bindgen::to_value(&patterns).map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> Result<String, JsError> {
        serde_json::to_string(&self.inner).map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = "fromJson")]
    pub fn from_json(json: String) -> Result<CandlestickState, JsError> {
        serde_json::from_str::<rust_cdl::IndicatorState>(&json)
            .map(|inner| CandlestickState { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

// ── Top-level functions ───────────────────────────────────────────────────────

/// Run the candlestick indicator. Returns `[patterns, state]` as a JS array.
/// `inputs`:  `[open, high, low, close]`
/// `options`: `[candle_period, trend_period, trend_signal_period]`
/// Each element of `patterns` is `null` (no match) or an array of pattern objects.
/// `forecast_type`: optional string — see `parse_forecast_type` for accepted values.
/// Note: SIMD variants are not available for this indicator.
#[wasm_bindgen(js_name = "candlestickIndicator")]
pub fn candlestick_indicator(
    inputs: JsValue,
    options: Vec<f64>,
    forecast_type: Option<String>,
) -> Result<js_sys::Array, JsError> {
    let inputs = inputs_from_js(inputs)?;
    let input_arr: [&[f64]; IW] = inputs
        .iter()
        .map(|v| v.as_slice())
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| JsError::new(&format!("Expected {IW} input series")))?;
    let option_arr: [f64; OW] = options
        .try_into()
        .map_err(|_| JsError::new(&format!("Expected {OW} options")))?;
    let forecast = forecast_type.map(|s| parse_forecast_type(&s)).transpose()?;
    let (raw_patterns, inner) = rust_cdl::indicator(&input_arr, &option_arr, forecast)
        .map_err(|e| JsError::new(&format!("{e:?}")))?;
    let patterns = convert_patterns(raw_patterns);
    let patterns_js =
        serde_wasm_bindgen::to_value(&patterns).map_err(|e| JsError::new(&e.to_string()))?;
    make_pair(patterns_js, JsValue::from(CandlestickState { inner }))
}

/// Static metadata for the candlestick indicator.
#[wasm_bindgen(js_name = "candlestickInfo")]
pub fn candlestick_info() -> JsValue {
    info_to_object(rust_cdl::INFO)
}

/// Minimum number of input bars needed to produce at least one output bar.
#[wasm_bindgen(js_name = "candlestickMinData")]
pub fn candlestick_min_data(options: Vec<f64>) -> u32 {
    rust_cdl::min_data(&options) as u32
}
// candlestick does not expose min_data_accuracy (not present in tulip_rs for this indicator)
