use validator::Validate;
use serde::Deserialize;
use super::utils::{validate_date, validate_name, validate_phone};

#[derive(Debug, Validate, Deserialize)]
pub struct FormData {
    #[validate(custom(function = "validate_name", message = "Digite seu nome completo"))]
    pub name: String,
    #[validate(custom(function = "validate_date", message = "Digite a data no formato dia/mês/ano."))]
    pub dob: String,
    #[validate(email(message = "Esse email parece incorreto. Confira e tente novamente."))]
    pub email: String,
    #[validate(custom(function = "validate_phone", message = "O número está incompleto. Inclua o DDD como em: (00) 0 0000-0000."))]
    pub phone: String,
}

