use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/dot-api.js")]
extern "C" {
    type WsProvider;

    #[wasm_bindgen(constructor)]
    fn new(s: &str) -> WsProvider;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

#[wasm_bindgen(start)]
pub fn run() {
    log(&format!("Hello!"));
    let x = WsProvider::new("wss://rpc.polkadot.io");
}

