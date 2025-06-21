use validator::Validate;
use serde::Deserialize;
use super::utils::{validate_date, validate_name, validate_phone};

#[derive(Debug, Validate, Deserialize)]
pub struct FormData {
    #[validate(custom(function = "validate_name", message = "Digite seu nome completo"))]
    pub name: String,
    #[validate(custom(function = "validate_date", message = "A data de nascimento deve estar no formato DD/MM/AAAA"))]
    pub dob: String,
    #[validate(email(message = "Digite um e-mail válido"))]
    pub email: String,
    #[validate(custom(function = "validate_phone", message = "Digite um número de celular válido no formato (DD) 00000-0000"))]
    pub phone: String,
}

