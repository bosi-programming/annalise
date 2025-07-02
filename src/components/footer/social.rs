use crate::components::icons::{amazon::AmazonIcon, bluesky::BlueskyIcon, instagram::InstagramIcon, tiktok::TikTokIcon};
use leptos:: prelude::*;

#[component]
pub fn Social() -> impl IntoView {
    view! {
        <div class="pb-7 grid grid-cols-4 gap-9">
            <AmazonIcon />
            <BlueskyIcon />
            <InstagramIcon />
            <TikTokIcon />
        </div>
    }
}

