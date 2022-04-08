use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error(a: &str);
}

pub fn log(message: &str) {
    let data = format!("scope[wasm]_log[info] : {}", message);
    console_log(&data);
}

pub fn error(message: &str) {
    let data = format!("scope[wasm]_log[error] : {}", message);
    console_error(&data);
}

pub struct Logger {
    pub scope: String,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            scope: "wasm".to_string(),
        }
    }

    pub fn log(&self, message: &str) {
        let data = format!("scope[{}]_log[info] : {}", self.scope, message);
        console_log(&data);
    }

    pub fn error(&self, message: &str) {
        let data = format!("scope[{}]_log[error] : {}", self.scope, message);
        console_error(&data);
    }
}
