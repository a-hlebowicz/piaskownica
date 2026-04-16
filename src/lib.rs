use wasm_bindgen::prelude::*;

//wypisywanie do przeglądarki
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    };
}

#[wasm_bindgen]
pub fn hello() {
    log!("hello world");
}