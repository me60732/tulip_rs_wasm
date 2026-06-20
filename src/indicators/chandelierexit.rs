use crate::utils::{info_to_object, inputs_from_js, make_pair, outputs_to_js};
use tulip_rs::indicator_types::TIndicatorState as _;
use tulip_rs::indicators::chandelierexit as rust_chandelierexit;
use wasm_bindgen::prelude::*;

const IW: usize = rust_chandelierexit::INPUTS_WIDTH;
const OW: usize = rust_chandelierexit::OPTIONS_WIDTH;

// ── State class ──────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct ChandelierexitState {
    inner: rust_chandelierexit::IndicatorState,
}

#[wasm_bindgen]
impl ChandelierexitState {
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
    pub fn from_json(json: String) -> Result<ChandelierexitState, JsError> {
        serde_json::from_str::<rust_chandelierexit::IndicatorState>(&json)
            .map(|inner| ChandelierexitState { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

// ── Top-level functions ───────────────────────────────────────────────────────

/// Run the CHANDELIEREXIT indicator. Returns `[outputs, state]` as a JS array.
/// `inputs`: `[high, low, close]`   `options`: `[period, multiplier]`
#[wasm_bindgen(js_name = "chandelierexitIndicator")]
pub fn chandelierexit_indicator(
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
        rust_chandelierexit::indicator(&input_arr, &option_arr, opt_outs.as_deref())
            .map_err(|e| JsError::new(&format!("{e:?}")))?;
    make_pair(
        outputs_to_js(outputs)?,
        JsValue::from(ChandelierexitState { inner }),
    )
}

/// Static metadata for CHANDELIEREXIT.
#[wasm_bindgen(js_name = "chandelierexitInfo")]
pub fn chandelierexit_info() -> JsValue {
    info_to_object(rust_chandelierexit::INFO)
}

/// Minimum number of input bars needed to produce at least one output bar.
#[wasm_bindgen(js_name = "chandelierexitMinData")]
pub fn chandelierexit_min_data(options: Vec<f64>) -> u32 {
    rust_chandelierexit::min_data(&options) as u32
}

