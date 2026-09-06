use crate::utils::{info_to_object, inputs_from_js, make_pair, outputs_to_js};
use tulip_rs::indicator_types::TIndicatorState as _;
use tulip_rs::indicators::instantaneoustrendline as rust_instantaneoustrendline;
use tulip_rs::indicators::instantaneoustrendline::{
    Indicator, IndicatorState, InstantaneousTrendline, INPUTS, OPTIONS,
};
use wasm_bindgen::prelude::*;

const IW: usize = INPUTS;
const OW: usize = OPTIONS;

// ── State class ──────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct InstantaneoustrendlineState {
    inner: IndicatorState,
}

#[wasm_bindgen]
impl InstantaneoustrendlineState {
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
    pub fn from_json(json: String) -> Result<InstantaneoustrendlineState, JsError> {
        serde_json::from_str::<rust_instantaneoustrendline::IndicatorState>(&json)
            .map(|inner| InstantaneoustrendlineState { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

// ── Top-level functions ───────────────────────────────────────────────────────

/// Run the Instantaneous Trendline indicator. Returns `[outputs, state]` as a JS array.
/// `inputs`: `[[source]]`
/// Mandatory outputs: `trendline` | Optional: `[want_trigger, want_dc_period, want_alpha]`
#[wasm_bindgen(js_name = "instantaneoustrendlineIndicator")]
pub fn instantaneoustrendline_indicator(
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
    let (outputs, inner) =
        InstantaneousTrendline::indicator(&input_arr, &option_arr, opt_outs.as_deref())
            .map_err(|e| JsError::new(&format!("{e:?}")))?;
    make_pair(
        outputs_to_js(outputs)?,
        JsValue::from(InstantaneoustrendlineState { inner }),
    )
}

/// Static metadata for Instantaneous Trendline.
#[wasm_bindgen(js_name = "instantaneoustrendlineInfo")]
pub fn instantaneoustrendline_info() -> JsValue {
    info_to_object(InstantaneousTrendline::INFO)
}

/// Minimum number of input bars needed to produce at least one output bar.
#[wasm_bindgen(js_name = "instantaneoustrendlineMinData")]
pub fn instantaneoustrendline_min_data(options: Vec<f64>) -> u32 {
    let option_arr: [f64; OW] = options
        .try_into()
        .unwrap_or_else(|_| panic!("Expected {OW} options"));
    InstantaneousTrendline::min_data(&option_arr) as u32
}
