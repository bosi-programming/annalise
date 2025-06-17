use leptos::prelude::*;

use crate::components::text::{ElementType, Text, TextSize, TextWeight};

#[component]
pub fn Title(
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    view! {
        <Text size={TextSize::BigTitle} r#as={ElementType::H1} weight={TextWeight::Bold} class={class.unwrap_or_default()}>
            "Descubra #ADE antes de todo mundo"
        </Text>
    }
}
