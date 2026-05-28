use crate::utils::{info_to_object, inputs_from_js, make_pair, outputs_to_js};
use tulip_rs::indicator_types::TIndicatorState as _;
use tulip_rs::indicators::marketfi as rust_marketfi;
use wasm_bindgen::prelude::*;

const IW: usize = rust_marketfi::INPUTS_WIDTH;
const OW: usize = rust_marketfi::OPTIONS_WIDTH;

// ── State class ──────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct MarketfiState {
    inner: rust_marketfi::IndicatorState,
}

#[wasm_bindgen]
impl MarketfiState {
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
    pub fn from_json(json: String) -> Result<MarketfiState, JsError> {
        serde_json::from_str::<rust_marketfi::IndicatorState>(&json)
            .map(|inner| MarketfiState { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

// ── Top-level functions ───────────────────────────────────────────────────────

/// Run the MARKETFI indicator. Returns `[outputs, state]` as a JS array.
/// `inputs`: `[[high], [low], [volume]]`
#[wasm_bindgen(js_name = "marketfiIndicator")]
pub fn marketfi_indicator(
    inputs: JsValue,
    optional_outputs: JsValue,
) -> Result<js_sys::Array, JsError> {
    let inputs = inputs_from_js(inputs)?;
    let input_arr: [&[f64]; IW] = inputs
        .iter()
        .map(|v| v.as_slice())
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| JsError::new(&format!("Expected {IW} input series")))?;
    let option_arr: [f64; OW] = [];
    let opt_outs = crate::utils::optional_outputs_from_js(optional_outputs)?;
    let (outputs, inner) = rust_marketfi::indicator(&input_arr, &option_arr, opt_outs.as_deref())
        .map_err(|e| JsError::new(&format!("{e:?}")))?;
    make_pair(
        outputs_to_js(outputs)?,
        JsValue::from(MarketfiState { inner }),
    )
}

/// Static metadata for MARKETFI.
#[wasm_bindgen(js_name = "marketfiInfo")]
pub fn marketfi_info() -> JsValue {
    info_to_object(rust_marketfi::info())
}

/// Minimum number of input bars needed to produce at least one output bar.
#[wasm_bindgen(js_name = "marketfiMinData")]
pub fn marketfi_min_data() -> u32 {
    rust_marketfi::min_data(&[]) as u32
}

/// Minimum input bars needed to achieve a given decimal accuracy.
#[wasm_bindgen(js_name = "marketfiMinDataAccuracy")]
pub fn marketfi_min_data_accuracy(decimals: u32) -> u32 {
    rust_marketfi::min_data_accuracy(&[], decimals as usize) as u32
}
