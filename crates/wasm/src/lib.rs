use cryputil_core::dispatch;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct WasmResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    trace: Option<cryputil_core::core::trace::Trace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

// Input: Kommando-Name, Parameter-Objekt aus JS
// Calc:  über zentralen Dispatcher ausführen, Trace serialisieren
// Output: JsValue mit { ok, trace? , error? }
#[wasm_bindgen]
pub fn run(command: &str, params: JsValue) -> Result<JsValue, JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let value: serde_json::Value = if params.is_undefined() || params.is_null() {
        serde_json::Value::Object(Default::default())
    } else {
        serde_wasm_bindgen::from_value(params).map_err(|e| {
            serde_wasm_bindgen::to_value(&WasmResponse {
                ok: false,
                trace: None,
                error: Some(format!("Parameter-Parse-Fehler: {}", e)),
            })
            .unwrap_or(JsValue::NULL)
        })?
    };

    let response = match dispatch::run(command, &value) {
        Ok(trace) => WasmResponse {
            ok: true,
            trace: Some(trace),
            error: None,
        },
        Err(err) => WasmResponse {
            ok: false,
            trace: None,
            error: Some(err.to_string()),
        },
    };

    serde_wasm_bindgen::to_value(&response).map_err(|e| JsValue::from_str(&e.to_string()))
}

// Input: -
// Calc:  Liste aller Kommandos für die help-Anzeige
// Output: JsValue mit Array { name, params, description }
#[wasm_bindgen]
pub fn commands() -> Result<JsValue, JsValue> {
    #[derive(Serialize)]
    struct CmdView {
        name: String,
        params: Vec<String>,
        description: String,
    }
    let list: Vec<CmdView> = dispatch::commands()
        .iter()
        .map(|c| CmdView {
            name: c.name.to_string(),
            params: c.params.iter().map(|s| s.to_string()).collect(),
            description: c.description.to_string(),
        })
        .collect();
    serde_wasm_bindgen::to_value(&list).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
