use leptos::prelude::*;

use crate::{components::text::{ElementType, Text, TextSize, TextWeight}, screens::home::sinopse_messages::get_sinopse_messages};

#[component]
pub fn Sinopse(
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let messages = get_sinopse_messages();
    let class = class.unwrap_or_else(|| "".to_string());
    view! {
    <section class={"px-8 pt-10 pb-8 bg-gray-100 ".to_string() + &class}>
      <Text size=TextSize::H2 r#as=ElementType::H2 weight=TextWeight::Bold class="mb-4">
        {messages.title}
      </Text>
        {messages.messages.clone().into_iter().enumerate().map(|(idx, message)| {
            view! {
                <Text weight={if idx + 1 == messages.messages.len() {TextWeight::Bold} else {TextWeight::Normal}} class="mb-4">
                    {message}
                </Text>
            }})
        .collect::<Vec<_>>()}
    </section>
    }
}

