use leptos::prelude::*;

#[component]
pub fn Button(
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] r#type: Option<String>,
    children: Children,
) -> impl IntoView {
    let final_class = format!("cursor-pointer bg-purple-light text-white font-bold rounded-full px-5 py-4 hover:bg-purple-900 {}", class.unwrap_or_default());
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

