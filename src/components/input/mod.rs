use leptos::prelude::*;

#[component]
pub fn Input(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] r#type: Option<String>,
    #[prop(into, optional)] disabled: bool,
    name: String,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>
) -> impl IntoView {
    let final_class = format!("focus:outline-1 focus:outline-offset-2 focus:outline-purple-light bg-input h-12 rounded-full w-full text-white placeholder:text-gray-300 px-5 py-4 {}", class.unwrap_or_default());
    let placeholder = placeholder.unwrap_or_default();
    let r#type = r#type.unwrap_or("text".to_string());

    view! {
        <input
            name=name
            autocomplete="on"
            bind:value=(value, set_value)
            type=r#type
            class=final_class
            placeholder=placeholder
            disabled=disabled
        />
    }
}
