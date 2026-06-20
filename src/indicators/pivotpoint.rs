use crate::utils::{info_to_object, inputs_from_js, make_pair, outputs_to_js};
use tulip_rs::indicator_types::TIndicatorState as _;
use tulip_rs::indicators::pivotpoint as rust_pivotpoint;
use wasm_bindgen::prelude::*;

const IW: usize = rust_pivotpoint::INPUTS_WIDTH;
const OW: usize = rust_pivotpoint::OPTIONS_WIDTH;

// ── State class ──────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct PivotpointState {
    inner: rust_pivotpoint::IndicatorState,
}

#[wasm_bindgen]
impl PivotpointState {
    /// Continue streaming: feed new bars into an existing state.
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
    pub fn from_json(json: String) -> Result<PivotpointState, JsError> {
        serde_json::from_str::<rust_pivotpoint::IndicatorState>(&json)
            .map(|inner| PivotpointState { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

// ── Top-level functions ───────────────────────────────────────────────────────

/// Run the PivotPoint indicator. Returns `[outputs, state]` as a JS array.
/// `inputs`: `[[high, low, close]]`   `options`: indicator-specific options
/// Note: SIMD variants are not available for this indicator.
#[wasm_bindgen(js_name = "pivotpointIndicator")]
pub fn pivotpoint_indicator(
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
    let (outputs, inner) = rust_pivotpoint::indicator(&input_arr, &option_arr, opt_outs.as_deref())
        .map_err(|e| JsError::new(&format!("{e:?}")))?;
    make_pair(
        outputs_to_js(outputs)?,
        JsValue::from(PivotpointState { inner }),
    )
}

/// Static metadata for PivotPoint.
#[wasm_bindgen(js_name = "pivotpointInfo")]
pub fn pivotpoint_info() -> JsValue {
    info_to_object(rust_pivotpoint::INFO)
}

/// Minimum number of input bars needed to produce at least one output bar.
#[wasm_bindgen(js_name = "pivotpointMinData")]
pub fn pivotpoint_min_data(options: Vec<f64>) -> u32 {
    rust_pivotpoint::min_data(&options) as u32
}


// Note: SIMD variants are not available for this indicator.
