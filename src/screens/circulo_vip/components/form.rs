use leptos::{ev::SubmitEvent, leptos_dom::logging::console_log, prelude::*};
use regex::Regex;
use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::components::{button::Button, input::Input, text::{Text, TextSize, TextWeight}};

#[derive(Debug, Validate, Deserialize)]
struct FormData {
    #[validate(custom(function = "validate_name", message = "Digite seu nome completo"))]
    name: String,
    #[validate(custom(function = "validate_date", message = "A data de nascimento deve estar no formato DD/MM/AAAA"))]
    dob: String,
    #[validate(email(message = "Digite um e-mail válido"))]
    email: String,
    #[validate(custom(function = "validate_phone", message = "Digite um número de celular válido no formato (DD) 00000-0000"))]
    phone: String,
}

#[derive(Debug,Clone)]
struct FormErrors {
    name: Option<String>,
    dob: Option<String>,
    email: Option<String>,
    phone: Option<String>,
}


fn validate_name(name: &str) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Ok(());
    }
    if name.len() < 3 {
        return Err(ValidationError::new("Digite seu nome completo"));
    }
    Ok(())
}
fn validate_date(date: &str) -> Result<(), ValidationError> {
    if date.is_empty() {
        return Ok(());
    }
    let date_regex = Regex::new(r"^(0[1-9]|[12][0-9]|3[01])\/(0[1-9]|1[0-2])\/\d{4}$").unwrap();
    if !date_regex.is_match(date) {
        return Err(ValidationError::new(""));
    }
    Ok(())
}
fn validate_phone(date: &str) -> Result<(), ValidationError> {
    let date_regex = Regex::new(r"\(\d{2}\)\d{5}-\d{4}").unwrap();
    if !date_regex.is_match(date) {
        return Err(ValidationError::new(""));
    }
    Ok(())
}


#[component]
pub fn Form() -> impl IntoView {
    let (name, set_name) = signal("".to_string());
    let (dob, set_dob) = signal("".to_string());
    let (email, set_email) = signal("".to_string());
    let (phone, set_phone) = signal("".to_string());
    let (errors, set_errors) = signal(FormErrors {
        name: None,
        dob: None,
        email: None,
        phone: None,
    });

    let on_submit = move | e: SubmitEvent| {
        e.prevent_default();
        let form_data = FormData {
            name: name.get(),
            dob: dob.get(),
            email: email.get(),
            phone: phone.get(),
        };
        console_log(&format!("Form data: {:?}", form_data));
        form_data.validate().map_err(|e| {
            let mut temp_errors = FormErrors {
                name: None,
                dob: None,
                email: None,
                phone: None,
            };
            for (field, error) in e.field_errors() {
                match field.as_ref() {
                    "name" => temp_errors.name = error.get(0).map_or(None, |e| Some(e.to_string())),
                    "dob" => temp_errors.dob = Some(error.get(0).map_or("".to_string(), |e| e.to_string())),
                    "email" => temp_errors.email = Some(error.get(0).map_or("".to_string(), |e| e.to_string())),
                    "phone" => temp_errors.phone = Some(error.get(0).map_or("".to_string(), |e| e.to_string())),
                    _ => {}
                }
            }
            set_errors.set(temp_errors);
            console_log(&format!("State error: {:?}", errors.get()));
            console_log(&format!("Validation error: {:?}", e.errors()));
        }).ok();
    };

    view! {
        <form on:submit=on_submit class="my-17 grid grid-cols-1 gap-3">
            <Input placeholder="Nome completo".to_string() value={name} set_value={set_name} name="name".to_string() />
            <Show when=move || errors.read().name.is_some()>
                <Text class="text-err ml-4" size=TextSize::Small>
                    {move || errors.read().name.clone().unwrap_or("".to_string())}
                </Text>
            </Show>
            <Input placeholder="Data de nascimento: DD/MM/AAAA".to_string() value={dob} set_value={set_dob} name="bday".to_string() />
            <Show when=move || errors.read().dob.is_some()>
                <Text class="text-err ml-4" size=TextSize::Small>
                    {move || errors.read().dob.clone().unwrap_or("".to_string())}
                </Text>
            </Show>
            <Input placeholder="E-mail*".to_string() value={email} set_value={set_email} name="email".to_string() />
            <Show when=move || errors.read().email.is_some()>
                <Text class="text-err ml-4" size=TextSize::Small>
                    {move || errors.read().email.clone().unwrap_or("".to_string())}
                </Text>
            </Show>
            <Input placeholder="Celular: (DD) 00000-0000*".to_string() value={phone} set_value={set_phone} name="tel-national".to_string() />
            <Show when=move || errors.read().phone.is_some()>
                <Text class="text-err ml-4" size=TextSize::Small>
                    {move || errors.read().phone.clone().unwrap_or("".to_string())}
                </Text>
            </Show>
            <Button r#type="submit".to_string()><Text weight=TextWeight::Bold>Entrar para o Círculo VIP</Text></Button>
        </form>
    }
}
