use leptos::prelude::*;

use crate::components::{button::Button, email::Email, text::{ElementType, Text, TextSize, TextWeight}};

#[component]
pub fn Newsletter(
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let (email, set_email) = signal("".to_string());
    view! {
        <div class={class.unwrap_or_default()}>
            <Text size={TextSize::H1} r#as={ElementType::H2} weight={TextWeight::Bold} class="text-white leading-none text-center pb-5">
                "Newsletter"
            </Text>
            <Text class="text-white leading-none text-center pb-10">
                "Vamos ficar pertinho? Compartilho por email o processo de escrita, recomendações literárias, material extra do livro e fotos do Chicó."
            </Text>
            <div class="flex items-center">
                <Email placeholder="Email" name="email".into() value=email set_value=set_email class="flex-auto w-10" />
                <Button class="rounded-none rounded-e-full flex-none">
                    "Enviar"
                </Button>
            </div>
        </div>
    }
}

