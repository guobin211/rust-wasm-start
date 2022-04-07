use js_sys::{
    decode_uri, decode_uri_component, encode_uri, encode_uri_component, escape, unescape, JsString,
};

#[allow(dead_code)]
pub fn is_safe_number(n: Option<f64>) {
    // todo
}

#[allow(dead_code)]
pub fn is_safe_string(n: Option<f64>) {
    // todo
}

#[allow(dead_code)]
pub fn is_safe_map(n: Option<f64>) {
    // todo
}

#[allow(dead_code)]
pub fn is_safe_object(n: Option<f64>) {
    // todo
}

#[allow(dead_code)]
pub fn decode_url(encoded: &str) -> JsString {
    return match decode_uri_component(encoded) {
        Ok(decoded) => decoded,
        Err(_) => "".into(),
    };
}

#[allow(dead_code)]
pub fn encode_url(encoded: &str) -> JsString {
    encode_uri_component(encoded)
}

#[allow(dead_code)]
pub fn escape_str(input: &str) -> JsString {
    escape(input)
}

#[allow(dead_code)]
pub fn unescape_str(input: &str) -> JsString {
    unescape(input)
}
