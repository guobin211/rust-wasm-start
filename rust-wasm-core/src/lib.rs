use std::collections::HashMap;

use js_sys::*;
use wasm_bindgen::prelude::*;

use crate::utils::safe_encode;

mod utils;

#[wasm_bindgen]
extern "C" {
    /// 闭包函数
    fn takes_immutable_closure(f: &dyn Fn());

    fn takes_mutable_closure(f: &mut dyn FnMut());

    fn takes_closure_parse_int_to_string(x: &dyn Fn(u32) -> String);
}

static CACHE_MAP: HashMap<String, u32> = HashMap::new();

#[wasm_bindgen]
pub fn parse_number(params: &Number) -> JsString {
    safe_encode(&format!("?number={}", params))
}

#[wasm_bindgen]
pub fn parse_int(params: &BigInt) -> JsString {
    safe_encode(&format!("{}", params))
}

#[wasm_bindgen]
pub fn parse_str(params: &str) -> JsString {
    safe_encode(&params)
}

#[wasm_bindgen]
pub fn parse_string(params: String) -> JsString {
    safe_encode(&params)
}

#[wasm_bindgen]
pub fn parse_boolean(params: Boolean) -> JsString {
    if params == true {
        return safe_encode("true");
    }
    safe_encode("false")
}

#[wasm_bindgen]
pub fn parse_null(params: &JsValue) -> bool {
    JsValue::is_null(params)
}

#[wasm_bindgen]
pub fn parse_undefined(params: &JsValue) -> bool {
    JsValue::is_undefined(params)
}

#[wasm_bindgen]
pub fn parse_symbol(params: &Symbol) -> String {
    let mut result: String = "Not a Symbol".to_owned();
    if JsValue::is_symbol(&params) {
        result = format!("{:?}", params.to_string());
    }
    result
}

#[wasm_bindgen]
pub fn parse_object(params: &Object) -> Object {
    let result = Object::new();
    if JsValue::is_null(params) {
        return result;
    }
    result
}

#[wasm_bindgen]
pub fn parse_set(params: &Set) -> Box<[JsValue]> {
    let mut arr: Vec<JsValue> = Vec::new();
    params.for_each(&mut |el, _, _| {
        arr.push(el);
    });
    arr.into_boxed_slice()
}

#[wasm_bindgen]
pub fn reverse_map(params: &Map) -> Map {
    let result = Map::new();
    params.for_each(&mut |v, k| {
        if JsValue::is_string(&k) {
            if JsValue::is_string(&v) {
                result.set(&v, &k);
            }
        }
    });
    result.set(&JsString::from("rust"), &JsString::from("1.59"));
    result
}

// 数组过滤bigint
#[wasm_bindgen]
pub fn parse_bigint_array(params: Option<Box<[JsValue]>>) -> Box<[JsValue]> {
    let mut result: Vec<JsValue> = Vec::new();
    if let Some(arr) = params {
        for el in arr.iter() {
            if el.is_bigint() {
                result.push(el.clone());
            }
        }
    }
    result.into_boxed_slice()
}

// ArrayBuffer = [u8]
#[wasm_bindgen]
pub fn parse_array_buffer(params: &[u8]) -> usize {
    params.len()
}

#[allow(dead_code)]
pub fn call_js_method() -> u32 {
    let mut value: u32 = 0;
    if let Some(v) = &CACHE_MAP.get(&"call_js_method".to_string()) {
        value = v.to_owned() + 1;
    }
    takes_immutable_closure(&|| {
        println!("call_js_method");
    });
    value
}
