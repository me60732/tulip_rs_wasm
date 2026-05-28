use js_sys::Float64Array;
use serde::Serialize;
use tulip_rs::types::Info;
use wasm_bindgen::prelude::*;

// ── InfoObject ────────────────────────────────────────────────────────────────

/// Plain-JS-object shape returned by every `{name}Info()` function.
/// Serialised to a JS object via serde-wasm-bindgen so the shape is identical
/// to the napi-rs `InfoObject` produced by `tulip-rs-node`.
#[derive(Serialize)]
pub struct InfoObject {
    pub name: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
    pub inputs: Vec<String>,
    pub options: Vec<String>,
    pub outputs: Vec<String>,
    #[serde(rename = "optionalOutputs")]
    pub optional_outputs: Vec<String>,
    #[serde(rename = "indicatorType")]
    pub indicator_type: String,
    #[serde(rename = "displayType")]
    pub display_type: String,
}

/// Convert a tulip_rs `Info` struct into an `InfoObject` and serialise it as a
/// plain JS object.  Returns `null` on the (practically impossible) serialisation
/// failure rather than panicking.
pub fn info_to_object(info: Info<'static>) -> JsValue {
    let obj = InfoObject {
        name: info.name.to_string(),
        full_name: info.full_name.to_string(),
        inputs: info.inputs.iter().map(|s| s.to_string()).collect(),
        options: info.options.iter().map(|s| s.to_string()).collect(),
        outputs: info.outputs.iter().map(|s| s.to_string()).collect(),
        optional_outputs: info
            .optional_outputs
            .iter()
            .map(|s| s.to_string())
            .collect(),
        indicator_type: format!("{:?}", info.indicator_type),
        display_type: format!("{:?}", info.display_type),
    };
    serde_wasm_bindgen::to_value(&obj).unwrap_or(JsValue::NULL)
}

// ── Conversion helpers ────────────────────────────────────────────────────────

/// Deserialise a `JsValue` (JS `Array<Array<number>>`) into `Vec<Vec<f64>>`.
/// Accepts both regular JS arrays and typed arrays for each inner series.
pub fn inputs_from_js(val: JsValue) -> Result<Vec<Vec<f64>>, JsError> {
    serde_wasm_bindgen::from_value(val).map_err(|e| JsError::new(&e.to_string()))
}

/// Deserialise an optional `JsValue` (`boolean[] | null | undefined`) into
/// `Option<Vec<bool>>`.  `wasm-bindgen` does not natively support `Vec<bool>`
/// as a parameter type, so we accept a `JsValue` and decode it here.
/// Pass from JS as `[true, false, true]` or omit / pass `null`.
pub fn optional_outputs_from_js(val: JsValue) -> Result<Option<Vec<bool>>, JsError> {
    if val.is_null() || val.is_undefined() {
        Ok(None)
    } else {
        serde_wasm_bindgen::from_value::<Vec<bool>>(val)
            .map(Some)
            .map_err(|e| JsError::new(&e.to_string()))
    }
}

/// Convert `Vec<Vec<f64>>` output series into a JS `Array<Float64Array>`.
/// Using `Float64Array` keeps the data in a shared WASM memory view — callers
/// that only need to read the values can do so without copying, while
/// `Array.from(output[0])` converts to a plain JS number array if needed.
pub fn outputs_to_js(outputs: Vec<Vec<f64>>) -> Result<JsValue, JsError> {
    let arr = js_sys::Array::new();
    for series in outputs {
        arr.push(&Float64Array::from(series.as_slice()));
    }
    Ok(arr.into())
}

/// Build the `[outputs, state]` two-element JS array returned by every top-level
/// indicator function.  `state` must already be a `JsValue` (use
/// `JsValue::from(MyState { inner })` — wasm-bindgen generates `From<T>` for
/// every `#[wasm_bindgen]` struct).
pub fn make_pair(outputs_js: JsValue, state: JsValue) -> Result<js_sys::Array, JsError> {
    let pair = js_sys::Array::new();
    pair.push(&outputs_js);
    pair.push(&state);
    Ok(pair)
}
