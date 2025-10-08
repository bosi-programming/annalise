mod utils;
mod validation;
use gloo_net::http::Request;
use leptos::{ev::SubmitEvent, prelude::*, task::spawn_local};
use serde::Serialize;
use validation::FormData;

use validator::Validate;
use web_sys::window;
use crate::{components::{button::Button, input::Input, text::{Text,TextWeight}}, env::{FORM_TEMPLATE, RESPONDER}};

#[derive(Serialize)]
struct Body {
    pub formTemplate: String,
    pub formAnswers: FormData,
}

#[component]
pub fn Form() -> impl IntoView {
    let (name, set_name) = signal("".to_string());
    let (name_error, set_name_error) = signal("".to_string());
    let (dob, set_dob) = signal("".to_string());
    let (dob_error, set_dob_error) = signal("".to_string());
    let (email, set_email) = signal("".to_string());
    let (email_error, set_email_error) = signal("".to_string());
    let (phone, set_phone) = signal("".to_string());
    let (phone_error, set_phone_error) = signal("".to_string());

    let on_submit = move | e: SubmitEvent| {
        e.prevent_default();
        let form_data = FormData {
            name: name.get(),
            dob: dob.get(),
            email: email.get(),
            phone: phone.get(),
        };
        set_email_error.set(String::new());
        set_phone_error.set(String::new());
        set_dob_error.set(String::new());
        set_name_error.set(String::new());
        let data = form_data.validate().map_err(|e| {
            for (field, error) in e.field_errors() {
                match field.as_ref() {
                    "name" => set_name_error.set(error.get(0).map_or("".to_string(), |e| e.to_string())),
                    "dob" => set_dob_error.set(error.get(0).map_or("".to_string(), |e| e.to_string())),
                    "email" => set_email_error.set(error.get(0).map_or("".to_string(), |e| e.to_string())),
                    "phone" => set_phone_error.set(error.get(0).map_or("".to_string(), |e| e.to_string())),
                    _ => {}
                }
            }
        });
        if data.is_ok() {
            let responder = RESPONDER;
            let form_template = FORM_TEMPLATE;
            let body = Body {
                formTemplate: form_template.to_string(),
                formAnswers: form_data
            };
            let json = serde_json::to_string(&body).expect("Form data serialization should work");
            spawn_local(async move {
                let res = Request::post("https://form-service.felipebosi.com/form-response")
                    .header("Content-Type", "application/json")
                    .header("responder", responder)
                    .body(json)
                    .unwrap()
                    .send()
                    .await;
                match res {
                    Ok(_) => window().unwrap().location().set_href("/circulo-vip/boas-vindas").unwrap(),
                    Err(_) => todo!(),
                }
            })
        }
    };

    view! {
        <form on:submit=on_submit class="py-17 lg:py-0 grid grid-cols-1 gap-3">
            <Input placeholder="Nome completo".to_string() value={name} set_value={set_name} name="name".to_string() error={name_error} />
            <Input placeholder="Data de nascimento: DD/MM/AAAA".to_string() value={dob} set_value={set_dob} name="bday".to_string() error={dob_error} onfocus="(this.type='date')" onblur="if(this.value==''){this.type='text'}" />
            <Input placeholder="E-mail*".to_string() value={email} set_value={set_email} name="email".to_string() error={email_error} />
            <Input placeholder="Celular: (DD)00000-0000*".to_string() value={phone} set_value={set_phone} pattern="\\([0-9]{2}\\)[0-9]{5}-[0-9]{4}" name="tel-national".to_string() error={phone_error} />
            <Button r#type="submit".to_string()><Text weight=TextWeight::Bold>Entrar para o Círculo VIP</Text></Button>
        </form>
    }
}

