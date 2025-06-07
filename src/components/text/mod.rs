use leptos::prelude::*;

#[component]
pub fn Text(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <p class=class>
            {children()}
        </p>
    }
}
