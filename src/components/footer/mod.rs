use leptos:: prelude::*;
use newsletter::Newsletter;
use social::Social;

pub mod newsletter;
pub mod social;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="bg-purple w-screen px-8">
            <Newsletter class="pb-22" />
            <Social />
        </div>
    }
}
