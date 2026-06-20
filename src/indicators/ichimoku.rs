use crate::utils::{info_to_object, inputs_from_js, make_pair, outputs_to_js};
use tulip_rs::indicator_types::TIndicatorState as _;
use tulip_rs::indicators::ichimoku as rust_ichimoku;
use wasm_bindgen::prelude::*;

const IW: usize = rust_ichimoku::INPUTS_WIDTH;
const OW: usize = rust_ichimoku::OPTIONS_WIDTH;

// ── State class ──────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct IchimokuState {
    inner: rust_ichimoku::IndicatorState,
}

#[wasm_bindgen]
impl IchimokuState {
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
    pub fn from_json(json: String) -> Result<IchimokuState, JsError> {
        serde_json::from_str::<rust_ichimoku::IndicatorState>(&json)
            .map(|inner| IchimokuState { inner })
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

// ── Top-level functions ───────────────────────────────────────────────────────

/// Run the Ichimoku Cloud indicator. Returns `[outputs, state]` as a JS array.
/// `inputs`: `[[high], [low], [close]]` | `options`: `[short_period, long_period]`
/// Mandatory outputs: `conversion`, `base`, `leading_span_a`, `leading_span_b`
/// Optional: `[want_lagging_span]`
/// Note: leading span outputs are offset forward; the binding returns them as-is.
#[wasm_bindgen(js_name = "ichimokuIndicator")]
pub fn ichimoku_indicator(
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
    let (outputs, inner) = rust_ichimoku::indicator(&input_arr, &option_arr, opt_outs.as_deref())
        .map_err(|e| JsError::new(&format!("{e:?}")))?;
    make_pair(
        outputs_to_js(outputs)?,
        JsValue::from(IchimokuState { inner }),
    )
}

/// Static metadata for Ichimoku Cloud.
#[wasm_bindgen(js_name = "ichimokuInfo")]
pub fn ichimoku_info() -> JsValue {
    info_to_object(rust_ichimoku::INFO)
}

/// Minimum number of input bars needed to produce at least one output bar.
#[wasm_bindgen(js_name = "ichimokuMinData")]
pub fn ichimoku_min_data(options: Vec<f64>) -> u32 {
    rust_ichimoku::min_data(&options) as u32
}

