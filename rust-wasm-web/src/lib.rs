use wasm_bindgen::prelude::*;

use crate::logger::{log, Logger};
use crate::utils::set_panic_hook;

mod logger;
mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // namespace is window in browser
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error(a: &str);
}

#[wasm_bindgen]
pub fn call_alert(text: &str) {
    set_panic_hook();
    let data = format!("Alert By Rust! {}", text);
    log(&data);
    alert(&data);
}

#[wasm_bindgen]
pub fn create_logger() {
    Logger::new().log("Hello from Rust!");
}
