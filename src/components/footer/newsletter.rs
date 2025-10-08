use gloo_net::http::Request;
use leptos::{ev::SubmitEvent, prelude::*, task::spawn_local};
use validator::Validate;
use web_sys::window;
use serde::{Deserialize, Serialize};

use crate::{components::{button::Button, email::Email, text::{ElementType, Text, TextSize, TextWeight}}, env::{FORM_TEMPLATE, RESPONDER}};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct FormData {
    #[validate(email(message = "Digite um e-mail válido"))]
    pub email: String,
}
#[derive(Debug,Clone)]
struct FormErrors {
    email: Option<String>,
}

#[derive(Serialize)]
struct Body {
    pub formTemplate: String,
    pub formAnswers: FormData,
}

#[component]
pub fn Newsletter(
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let (email, set_email) = signal("".to_string());
    let (errors, set_errors) = signal(FormErrors {
        email: None,
    });
    let on_submit = move | e: SubmitEvent| {
        e.prevent_default();
        let form_data = FormData {
            email: email.get(),
        };
        let data = form_data.validate().map_err(|e| {
            let mut temp_errors = FormErrors {
                email: None,
            };
            for (field, error) in e.field_errors() {
                match field.as_ref() {
                    "email" => temp_errors.email = Some(error.get(0).map_or("".to_string(), |e| e.to_string())),
                    _ => {}
                }
            }
            set_errors.set(temp_errors);
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
        <div class={class.unwrap_or_default()}>
            <Text size={TextSize::H1} r#as={ElementType::H2} weight={TextWeight::Bold} class="text-white leading-none text-center pb-5">
                "Newsletter"
            </Text>
            <Text class="text-white leading-none text-center pb-10">
                "Vamos ficar pertinho? Compartilho por email o processo de escrita, recomendações literárias, material extra do livro e fotos do Chicó."
            </Text>
            <form on:submit=on_submit class="flex items-center max-w-md m-auto">
                <Email placeholder="Email" name="email".into() value=email set_value=set_email class="flex-auto w-10" />
                <Button r#type="submit" class="rounded-none rounded-e-full flex-none">
                    "Enviar"
                </Button>
            </form>
            <Show when=move || errors.read().email.is_some()>
                <Text class="text-err ml-4 mt-3" size=TextSize::Small>
                    {move || errors.read().email.clone().unwrap_or("".to_string())}
                </Text>
            </Show>
        </div>
    }
}


