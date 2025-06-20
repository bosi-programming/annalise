use leptos::prelude::*;

use crate::components::{button::Button, input::Input, text::{Text, TextWeight}};

#[component]
pub fn Form() -> impl IntoView {
    let (name, set_name) = signal("".to_string());
    let (dob, set_dob) = signal("".to_string());
    let (email, set_email) = signal("".to_string());
    let (phone, set_phone) = signal("".to_string());
    view! {
        <form class="my-17 grid grid-cols-1 gap-3">
            <Input placeholder="Nome completo".to_string() value={name} set_value={set_name} name="name".to_string() />
            <Input placeholder="Data de nascimento: DD/MM/AAAA".to_string() value={dob} set_value={set_dob} name="bday".to_string() />
            <Input placeholder="E-mail".to_string() value={email} set_value={set_email} name="email".to_string() />
            <Input placeholder="Celular: (DDD) 00000-0000".to_string() value={phone} set_value={set_phone} name="tel-national".to_string() />
            <Button r#type="submit".to_string()><Text weight=TextWeight::Bold>Entrar para o Círculo VIP</Text></Button>
        </form>
    }
}
