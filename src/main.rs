use components::text::Text;
use leptos::prelude::*;
use screens::home::sinopse::Sinopse;

mod components;
mod screens;

fn main() {
    leptos::mount::mount_to_body(|| view! { <Sinopse/> });
}
