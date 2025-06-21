use validator::ValidationError;
use regex::Regex;

pub fn validate_name(name: &str) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Ok(());
    }
    if name.len() < 3 {
        return Err(ValidationError::new("Digite seu nome completo"));
    }
    Ok(())
}
pub fn validate_date(date: &str) -> Result<(), ValidationError> {
    if date.is_empty() {
        return Ok(());
    }
    let date_regex = Regex::new(r"^(0[1-9]|[12][0-9]|3[01])\/(0[1-9]|1[0-2])\/\d{4}$").unwrap();
    if !date_regex.is_match(date) {
        return Err(ValidationError::new(""));
    }
    Ok(())
}
pub fn validate_phone(date: &str) -> Result<(), ValidationError> {
    let date_regex = Regex::new(r"\(\d{2}\)\d{5}-\d{4}").unwrap();
    if !date_regex.is_match(date) {
        return Err(ValidationError::new(""));
    }
    Ok(())
}

