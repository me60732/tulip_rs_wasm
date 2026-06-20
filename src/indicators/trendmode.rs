use crate::utils::{info_to_object, inputs_from_js, make_pair, outputs_to_js};
use tulip_rs::indicator_types::TIndicatorState as _;
use tulip_rs::indicators::trendmode as rust_trendmode;
use wasm_bindgen::prelude::*;

const IW: usize = rust_trendmode::INPUTS_WIDTH;
const OW: usize = rust_trendmode::OPTIONS_WIDTH;

// ── State class ──────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct TrendmodeState {
    inner: rust_trendmode::IndicatorState,
}

#[wasm_bindgen]
impl TrendmodeState {
    #[wasm_bindgen(js_name = "batchIndicator")]
    pub fn batch_indicator(
        &mut self,
        inputs: JsValue,
        optional_outputs: JsValue,
    ) -> Result<JsValue, JsError> {
        let inputs = inputs_from_js(inputs)?;
        let input_arr: [&[f64]; IW] = inputs
            .iter()
            .map(|v| v.as_slice())
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| JsError::new(&format!("Expected {IW} input series")))?;
        let opt_outs = crate::utils::optional_outputs_from_js(optional_outputs)?;
        let outputs = self
            .inner
            .batch_indicator(&input_arr, opt_outs.as_deref())
            .map_err(|e| JsError::new(&format!("{e:?}")))?;
        outputs_to_js(outputs)
    }

    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> Result<String, JsError> {
        serde_json::to_string(&self.inner).map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = "fromJson")]
    pub fn from_json(json: String) -> Result<TrendmodeState, JsError> {
        serde_json::from_str::<rust_trendmode::IndicatorState>(&json)
            .map(|inner| TrendmodeState { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

// ── Top-level functions ───────────────────────────────────────────────────────

/// Run the Trend Mode indicator. Returns `[outputs, state]` as a JS array.
/// `inputs`: `[[source]]` | `options`: `[alpha]` (0.0 = adaptive)
/// Mandatory outputs: `trendmode` | Optional: `[want_cycle, want_peak]`
#[wasm_bindgen(js_name = "trendmodeIndicator")]
pub fn trendmode_indicator(
    inputs: JsValue,
    options: Vec<f64>,
    optional_outputs: JsValue,
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
    let opt_outs = crate::utils::optional_outputs_from_js(optional_outputs)?;
    let (outputs, inner) = rust_trendmode::indicator(&input_arr, &option_arr, opt_outs.as_deref())
        .map_err(|e| JsError::new(&format!("{e:?}")))?;
    make_pair(
        outputs_to_js(outputs)?,
        JsValue::from(TrendmodeState { inner }),
    )
}

/// Static metadata for Trend Mode.
#[wasm_bindgen(js_name = "trendmodeInfo")]
pub fn trendmode_info() -> JsValue {
    info_to_object(rust_trendmode::INFO)
}

/// Minimum number of input bars needed to produce at least one output bar.
#[wasm_bindgen(js_name = "trendmodeMinData")]
pub fn trendmode_min_data(options: Vec<f64>) -> u32 {
    rust_trendmode::min_data(&options) as u32
}

