use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

pub fn get_nested_property(
    node: &JsValue,
    keys: &Vec<&str>,
    depth: usize,
) -> Result<JsValue, JsValue> {
    if depth >= keys.len() {
        return Ok(node.clone());
    }
    match node.is_object() {
        true => {
            let next = Reflect::get(&node, &JsValue::from(keys[depth]))?;
            get_nested_property(&next, keys, depth + 1)
        }
        _ => Err(JsValue::from(format!(
            "current depth is not object, key: {}",
            keys[0.min(depth - 1)],
        ))),
    }
}
