use crate::utils::{info_to_object, inputs_from_js, make_pair, outputs_to_js};
use tulip_rs::indicator_types::TIndicatorState as _;
use tulip_rs::indicators::wilders as rust_wilders;
use tulip_rs::indicators::wilders::{Indicator, IndicatorState, Wilders, INPUTS, OPTIONS};
use wasm_bindgen::prelude::*;

const IW: usize = INPUTS;
const OW: usize = OPTIONS;

// ── State class ──────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct WildersState {
    inner: IndicatorState,
}

#[wasm_bindgen]
impl WildersState {
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
    pub fn from_json(json: String) -> Result<WildersState, JsError> {
        serde_json::from_str::<rust_wilders::IndicatorState>(&json)
            .map(|inner| WildersState { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

// ── Top-level functions ───────────────────────────────────────────────────────

#[wasm_bindgen(js_name = "wildersIndicator")]
pub fn wilders_indicator(
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
    let (outputs, inner) = Wilders::indicator(&input_arr, &option_arr, opt_outs.as_deref())
        .map_err(|e| JsError::new(&format!("{e:?}")))?;
    make_pair(
        outputs_to_js(outputs)?,
        JsValue::from(WildersState { inner }),
    )
}

#[wasm_bindgen(js_name = "wildersInfo")]
pub fn wilders_info() -> JsValue {
    info_to_object(Wilders::INFO)
}

#[wasm_bindgen(js_name = "wildersMinData")]
pub fn wilders_min_data(options: Vec<f64>) -> u32 {
    let option_arr: [f64; OW] = options
        .try_into()
        .unwrap_or_else(|_| panic!("Expected {OW} options"));
    Wilders::min_data(&option_arr) as u32
}
