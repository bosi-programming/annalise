use leptos::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Variant {
    Primary,
    Secondary,
}

#[component]
pub fn Button(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] r#type: Option<String>,
    #[prop(into, optional)] variant: Signal<Option<Variant>>,
    children: Children,
) -> impl IntoView {
    let variant_class = move || match *variant.read() {
        Some(Variant::Primary) => "bg-purple-light",
        Some(Variant::Secondary) => "bg-purple-secondary",
        None => "bg-purple-light",
    };
    let final_class = format!("cursor-pointer text-white font-bold rounded-full px-5 py-4 hover:bg-purple-900 {} {}", class.unwrap_or_default(), variant_class());
    let r#type = r#type.unwrap_or("button".to_string());

    view! {
        <button
            type=r#type
            class=final_class
        >
            {children()}
        </button>
    }
}

