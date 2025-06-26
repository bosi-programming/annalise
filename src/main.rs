use leptos::prelude::*;
use screens::{circulo_vip::CirculoVip, home::sinopse::Sinopse};

mod components;
mod screens;
mod utils;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <CirculoVip/> });
}
