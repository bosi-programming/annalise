use leptos:: prelude::*;

use crate::components::{icons::{logo::Logo, tiktok::TikTokIcon}, text::{Text, TextWeight}};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="lg:py-6 lg:px-13 py-4 px-7 bg-bg-header flex items-center justify-between">
            <div class="flex items-center">
                <Logo />
                <Text weight=TextWeight::Bold class="text-white">
                    "Annalise Cerqueira-Maia"
                </Text>
            </div>
            <a href="https://www.tiktok.com/@annalisecmaia?is_from_webapp=1&sender_device=pc" target="_blank" rel="external">
                <TikTokIcon />
            </a>
        </div>
    }
}
