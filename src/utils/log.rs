use leptos::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys::{self, Object, JSON}, wasm_bindgen::{JsCast, JsValue}, Headers, Request, RequestInit, RequestMode, Response};

pub async fn log(level: String, message: String, context: JsValue) {
    let body = Object::new();
    js_sys::Reflect::set(&body, &"message".into(), &message.into()).unwrap();
    js_sys::Reflect::set(&body, &"status".into(), &level.into()).unwrap();
    js_sys::Reflect::set(&body, &"service".into(), &"leptos-app".into()).unwrap();

    if context.is_object() {
        let context_obj = context.dyn_into::<Object>().unwrap();
        let keys = js_sys::Reflect::own_keys(&context_obj).unwrap();
        for key in keys.iter() {
            let value = js_sys::Reflect::get(&context_obj, &key).unwrap();
            js_sys::Reflect::set(&body, &key, &value).unwrap();
        }
    }

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(Some(&JSON::stringify(&body).unwrap().into()).unwrap());
    let headers = Headers::new().unwrap();
    headers.append("Content-Type", "application/json").unwrap();
    headers.append("DD-API-KEY", "559f5f179597e886d93bd98e908e544f").unwrap();
    opts.set_headers(&headers.into());

    let request = Request::new_with_str_and_init("https://http-intake.logs.datadoghq.com/api/v2/logs", &opts);
    let window = web_sys::window().expect("no global window");
    let resp_value = JsFuture::from(window.fetch_with_request(&request.clone().unwrap())).await;
    let resp: Response = resp_value.unwrap().dyn_into().unwrap();

    match JsFuture::from(resp.json().unwrap()).await {
       Ok(resp) => {
            let resp: Response = resp.dyn_into().unwrap();
            if !resp.ok() {
                web_sys::console::error_1(&"Datadog submission failed".into());
            }
        }
        Err(e) => web_sys::console::error_1(&e), 
    }
}

