use leptos::prelude::*;

use crate::components::text::{Text, TextSize};

#[component]
pub fn Input(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] r#type: Option<String>,
    #[prop(into, optional)] disabled: bool,
    #[prop(into, optional)] pattern: Option<String>,
    error: ReadSignal<String>,
    name: String,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(into, optional)] onfocus: Option<String>,
    #[prop(into, optional)] onblur: Option<String>,
) -> impl IntoView {
    let final_class = format!("focus:outline-1 focus:outline-offset-2 focus:outline-purple-light bg-input h-12 rounded-full w-full text-white placeholder:text-gray-300 px-5 py-4 {}", class.unwrap_or_default());
    let placeholder = placeholder.unwrap_or_default();
    let r#type = r#type.unwrap_or("text".to_string());

    view! {
        <div>
            <input
                name=name
                autocomplete="on"
                bind:value=(value, set_value)
                type=r#type
                class=final_class
                placeholder=placeholder
                pattern=pattern
                disabled=disabled
                onfocus=onfocus
                onblur=onblur
            />
            <Show when=move || !error.read().is_empty()>
                <Text class="text-err ml-4" size=TextSize::Small>
                    {error}
                </Text>
            </Show>
        </div>
    }
}
