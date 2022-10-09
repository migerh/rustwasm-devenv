use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace=console, js_name="log")]
    fn console_log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) -> JsValue {
    console::log_1(&format!("Hello from Rust!").into());

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    
    let val = document.create_element("p").unwrap();
    val.set_inner_html("Hello from Rust!");
    
    body.append_child(&val).unwrap();
}
