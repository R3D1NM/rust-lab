mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str); // This function is provided by the browser.
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    alert(&format!("Hello, {}!", s)); // Call the alert function provided by the browser.
}
