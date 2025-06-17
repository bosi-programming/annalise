use leptos::prelude::*;
use screens::{circulo_vip::CirculoVip, home::sinopse::Sinopse};

mod components;
mod screens;

fn main() {
    leptos::mount::mount_to_body(|| view! { <CirculoVip/> });
}
