use crate::utils::{info_to_object, inputs_from_js, make_pair, outputs_to_js};
use tulip_rs::indicator_types::TIndicatorState as _;
use tulip_rs::indicators::md as rust_md;
use tulip_rs::indicators::md::{Indicator, IndicatorState, Md, INPUTS, OPTIONS};
use wasm_bindgen::prelude::*;

const IW: usize = INPUTS;
const OW: usize = OPTIONS;

// ── State class ──────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct MdState {
    inner: IndicatorState,
}

#[wasm_bindgen]
impl MdState {
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
    pub fn from_json(json: String) -> Result<MdState, JsError> {
        serde_json::from_str::<rust_md::IndicatorState>(&json)
            .map(|inner| MdState { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

// ── Top-level functions ───────────────────────────────────────────────────────

#[wasm_bindgen(js_name = "mdIndicator")]
pub fn md_indicator(
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
    let (outputs, inner) = Md::indicator(&input_arr, &option_arr, opt_outs.as_deref())
        .map_err(|e| JsError::new(&format!("{e:?}")))?;
    make_pair(outputs_to_js(outputs)?, JsValue::from(MdState { inner }))
}

#[wasm_bindgen(js_name = "mdInfo")]
pub fn md_info() -> JsValue {
    info_to_object(Md::INFO)
}

#[wasm_bindgen(js_name = "mdMinData")]
pub fn md_min_data(options: Vec<f64>) -> u32 {
    let option_arr: [f64; OW] = options
        .try_into()
        .unwrap_or_else(|_| panic!("Expected {OW} options"));
    Md::min_data(&option_arr) as u32
}
