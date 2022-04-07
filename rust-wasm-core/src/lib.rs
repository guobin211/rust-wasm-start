mod utils;

use js_sys::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// 闭包函数
    fn takes_immutable_closure(f: &dyn Fn());

    fn takes_mutable_closure(f: &mut dyn FnMut());

    fn takes_closure_parse_int_to_string(x: &dyn Fn(u32) -> String);
}

#[wasm_bindgen]
pub fn parse_number(params: Number) {}

#[wasm_bindgen]
pub fn parse_int(params: BigInt) {}

#[wasm_bindgen]
pub fn parse_str(params: &str) {}

#[wasm_bindgen]
pub fn parse_string(params: &JsString) {}

#[wasm_bindgen]
pub fn parse_boolean(params: Boolean) -> bool {
    true
}

#[wasm_bindgen]
pub fn parse_null() {}

#[wasm_bindgen]
pub fn parse_undefined() {}

#[wasm_bindgen]
pub fn parse_symbol(params: &Symbol) {}

#[wasm_bindgen]
pub fn parse_object(params: &Object) {}

#[wasm_bindgen]
pub fn parse_set(params: &Set) {}

#[wasm_bindgen]
pub fn parse_map(params: &Map) {}

#[wasm_bindgen]
pub fn parse_array(params: &Array) {}

#[wasm_bindgen]
pub fn parse_array_buffer(params: &ArrayBuffer) {}
