use crate::components::icons::{amazon::AmazonIcon, bluesky::BlueskyIcon, instagram::InstagramIcon, tiktok::TikTokIcon};
use leptos:: prelude::*;

#[component]
pub fn Social() -> impl IntoView {
    view! {
        <div class="mx-auto w-fit pb-7 grid grid-cols-4 gap-9">
            <a href="https://amzn.to/40xthXT" target="_blank" rel="external">
                <AmazonIcon />
            </a>
            <a href="https://bsky.app/profile/annalisecmaia.bsky.social" target="_blank" rel="external">
                <BlueskyIcon />
            </a>
            <a href="https://www.instagram.com/annalisecmaia/" target="_blank" rel="external">
                <InstagramIcon />
            </a>
            <a href="https://www.tiktok.com/@annalisecmaia?is_from_webapp=1&sender_device=pc" target="_blank" rel="external">
                <TikTokIcon />
            </a>
        </div>
    }
}

