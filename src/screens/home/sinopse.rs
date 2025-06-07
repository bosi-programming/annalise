use leptos::prelude::*;

use crate::{components::text::Text, screens::home::sinopse_messages::get_sinopse_messages};

#[component]
pub fn Sinopse(
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let messages = get_sinopse_messages();
    view! {
    <section class=class>
      <Text>
        {messages.title}
      </Text>
        {messages.messages.into_iter().map(|message| {
            view! {
                <Text class="message">
                    {message}
                </Text>
            }})
        .collect::<Vec<_>>()}
    </section>
    }
}

