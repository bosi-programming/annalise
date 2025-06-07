use leptos::prelude::*;

use crate::{components::text::Text, screens::home::sinopse_messages::get_sinopse_messages};

#[component]
pub fn Sinopse(
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let messages = get_sinopse_messages();
    let class = class.unwrap_or_else(|| "".to_string());
    view! {
    <section class={"px-8 pt-10 pb-8 bg-gray-100 ".to_string() + &class}>
      <Text class="mb-4">
        {messages.title}
      </Text>
        {messages.messages.into_iter().map(|message| {
            view! {
                <Text class="mb-4">
                    {message}
                </Text>
            }})
        .collect::<Vec<_>>()}
    </section>
    }
}

