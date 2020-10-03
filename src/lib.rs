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
pub fn filter(
    s: &str,
    words: Box<[JsValue]>,
    ignore_whitespace: bool,
    case_insensitive: bool,
) -> JsValue {
    let filter = Filter::from_js(words);
    let filtered = filter.filter_opts(s, ignore_whitespace, case_insensitive);
    if let Ok(val) = JsValue::from_serde(&filtered) {
        val
    } else {
        JsValue::null()
    }
}
