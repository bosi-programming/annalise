use app::App;
use leptos::prelude::*;

mod components;
mod screens;
mod app;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App /> });
}
