pub mod filter;
mod utils;

use filter::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn filter(s: &str, words: Box<[JsValue]>) -> JsValue {
    let filter = Filter::from_js(words);
    let filtered = filter.filter(s);
    if let Ok(val) = JsValue::from_serde(&filtered) {
        val
    } else {
        JsValue::null()
    }
}
