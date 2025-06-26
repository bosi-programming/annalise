use leptos::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{wasm_bindgen::{JsCast, JsValue}, Request, RequestInit, RequestMode, Response};

pub async fn fetch_data(url: &str) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);  // Enable CORS

    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = web_sys::window().expect("no global window");
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    // Parse JSON response
    JsFuture::from(resp.json()?).await
}
