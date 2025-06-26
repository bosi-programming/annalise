// src/datadog.rs
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{Object, Reflect};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = DD_RUM)]
    fn init_rum(options: &Object);

    #[wasm_bindgen(js_namespace = DD_LOGS)]
    fn init(options: &Object);
}

pub fn initialize_datadog() {
    let rum_options = Object::new();
    Reflect::set(
        &rum_options,
        &"applicationId".into(),
        &"annalise".into(),
    ).unwrap();
    Reflect::set(
        &rum_options,
        &"clientToken".into(),
        &"559f5f179597e886d93bd98e908e544f".into(),
    ).unwrap();
    Reflect::set(
        &rum_options,
        &"site".into(),
        &"datadoghq.com".into(),
    ).unwrap();
    Reflect::set(
        &rum_options,
        &"service".into(),
        &"leptos-app".into(),
    ).unwrap();
    Reflect::set(
        &rum_options,
        &"sampleRate".into(),
        &100.into(),
    ).unwrap();

    let logs_options = Object::new();
    Reflect::set(
        &logs_options,
        &"clientToken".into(),
        &"559f5f179597e886d93bd98e908e544f".into(),
    ).unwrap();
    Reflect::set(
        &logs_options,
        &"site".into(),
        &"datadoghq.com".into(),
    ).unwrap();
    Reflect::set(
        &logs_options,
        &"forwardErrorsToLogs".into(),
        &true.into(),
    ).unwrap();

    // init_rum(&rum_options);
    init(&logs_options);
}

