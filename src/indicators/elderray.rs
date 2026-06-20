use crate::utils::{info_to_object, inputs_from_js, make_pair, outputs_to_js};
use tulip_rs::indicator_types::TIndicatorState as _;
use tulip_rs::indicators::elderray as rust_elderray;
use wasm_bindgen::prelude::*;

const IW: usize = rust_elderray::INPUTS_WIDTH;
const OW: usize = rust_elderray::OPTIONS_WIDTH;

// ── State class ──────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct ElderrayState {
    inner: rust_elderray::IndicatorState,
}

#[wasm_bindgen]
impl ElderrayState {
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
    pub fn from_json(json: String) -> Result<ElderrayState, JsError> {
        serde_json::from_str::<rust_elderray::IndicatorState>(&json)
            .map(|inner| ElderrayState { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

// ── Top-level functions ───────────────────────────────────────────────────────

/// Run the ELDERRAY indicator. Returns `[outputs, state]` as a JS array.
/// `inputs`: `[high, low, close]`   `options`: `[period]`
/// `optional_outputs`: `[true]` to include the EMA series.
#[wasm_bindgen(js_name = "elderrayIndicator")]
pub fn elderray_indicator(
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
    let (outputs, inner) = rust_elderray::indicator(&input_arr, &option_arr, opt_outs.as_deref())
        .map_err(|e| JsError::new(&format!("{e:?}")))?;
    make_pair(
        outputs_to_js(outputs)?,
        JsValue::from(ElderrayState { inner }),
    )
}

/// Static metadata for ELDERRAY.
#[wasm_bindgen(js_name = "elderrayInfo")]
pub fn elderray_info() -> JsValue {
    info_to_object(rust_elderray::INFO)
}

/// Minimum number of input bars needed to produce at least one output bar.
#[wasm_bindgen(js_name = "elderrayMinData")]
pub fn elderray_min_data(options: Vec<f64>) -> u32 {
    rust_elderray::min_data(&options) as u32
}

