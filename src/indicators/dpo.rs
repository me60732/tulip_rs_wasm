use crate::utils::{info_to_object, inputs_from_js, make_pair, outputs_to_js};
use tulip_rs::indicator_types::TIndicatorState as _;
use tulip_rs::indicators::dpo as rust_dpo;
use wasm_bindgen::prelude::*;

const IW: usize = rust_dpo::INPUTS_WIDTH;
const OW: usize = rust_dpo::OPTIONS_WIDTH;

// ── State class ──────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct DpoState {
    inner: rust_dpo::IndicatorState,
}

#[wasm_bindgen]
impl DpoState {
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
    pub fn from_json(json: String) -> Result<DpoState, JsError> {
        serde_json::from_str::<rust_dpo::IndicatorState>(&json)
            .map(|inner| DpoState { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

// ── Top-level functions ───────────────────────────────────────────────────────

/// Run the DPO indicator. Returns `[outputs, state]` as a JS array.
/// `inputs`: `[[close]]`   `options`: `[period]`
#[wasm_bindgen(js_name = "dpoIndicator")]
pub fn dpo_indicator(
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
    let (outputs, inner) = rust_dpo::indicator(&input_arr, &option_arr, opt_outs.as_deref())
        .map_err(|e| JsError::new(&format!("{e:?}")))?;
    make_pair(outputs_to_js(outputs)?, JsValue::from(DpoState { inner }))
}

/// Static metadata for DPO.
#[wasm_bindgen(js_name = "dpoInfo")]
pub fn dpo_info() -> JsValue {
    info_to_object(rust_dpo::info())
}

/// Minimum number of input bars needed to produce at least one output bar.
#[wasm_bindgen(js_name = "dpoMinData")]
pub fn dpo_min_data(options: Vec<f64>) -> u32 {
    rust_dpo::min_data(&options) as u32
}

/// Minimum input bars needed to achieve a given decimal accuracy.
#[wasm_bindgen(js_name = "dpoMinDataAccuracy")]
pub fn dpo_min_data_accuracy(options: Vec<f64>, decimals: u32) -> u32 {
    rust_dpo::min_data_accuracy(&options, decimals as usize) as u32
}
