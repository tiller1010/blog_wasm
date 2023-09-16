use wasm_bindgen::prelude::*;

#[wasm_bindgen]
//pub async fn get_page_content(url: &str) -> Result<JsValue, JsValue> {
//    let promise = js_sys::Promise::resolve(&42.into());
//    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
//    Ok(result)
//}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn im_blue() {
    log("I'm blue da ba dee da ba daa");
}

