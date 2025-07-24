use leptos_meta::*;
use leptos::prelude::*;
use leptos_router::path;
use leptos_router::components::*;
use leptos_use::use_window;
use leptos_use::{use_preferred_dark, UseFaviconOptions, use_favicon_with_options};

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::screens::circulo_vip::CirculoVip;
use crate::screens::circulo_vip_boas_vindas::CirculoVipBoasVindas;

fn get_url_on_client() -> Option<String> {
    match window().location().pathname() {
        Ok(url) => Some(url),
        Err(_) => None,
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let window = use_window();
    let is_dark = use_preferred_dark();

    let (_icon, _) = use_favicon_with_options(
        UseFaviconOptions::default().new_icon(
            Signal::derive(move || {
                Some((if is_dark.get() { "/assets/favicon.png" } else { "/assets/favicon-light.png" }).to_string())
            }),
        )
    );

    let mut pathname = "Loading...".to_string();
    if window.is_some() {
        if let Some(url) = get_url_on_client() {
            pathname = url;
        } else {
            pathname = "Could not get URL.".to_string();
        }
    }
    let should_hide_footer = !pathname.contains("/circulo-vip/boas-vindas");
    view! {
        <div class="bg-purple">
            <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <Meta charset="UTF-8"/>
            <Meta name="author" content="Felipe Azevedo Bosi"/>
            <Header />
            <div class="max-w-7xl m-auto">
                <Router>
                    <Routes fallback=|| "Not found.">
                        <Route path=path!("/") view=CirculoVip/>
                        <Route path=path!("/circulo-vip") view=CirculoVip/>
                        <Route path=path!("/circulo-vip/boas-vindas") view=CirculoVipBoasVindas/>
                    </Routes>
                </Router>
            <Show when={move || should_hide_footer}>
                <Footer />
            </Show>
            </div>
        </div>
    }
}
