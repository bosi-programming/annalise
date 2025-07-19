use leptos_meta::*;
use leptos::prelude::*;
use leptos_router::path;
use leptos_router::components::*;
use leptos_use::{use_preferred_dark, UseFaviconOptions, use_favicon_with_options};

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::screens::circulo_vip::CirculoVip;

#[component]
pub fn App() -> impl IntoView {
    let is_dark = use_preferred_dark();

    let (_icon, _) = use_favicon_with_options(
        UseFaviconOptions::default().new_icon(
            Signal::derive(move || {
                Some((if is_dark.get() { "/assets/favicon.png" } else { "/assets/favicon-light.png" }).to_string())
            }),
        )
    );
    view! {
        <div class="bg-purple">
            <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <Meta charset="UTF-8"/>
            <Meta name="author" content="Felipe Azevedo Bosi"/>
            <Header />
            <Router>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=CirculoVip/>
                    <Route path=path!("/circulo-vip") view=CirculoVip/>
                </Routes>
            </Router>
            <Footer />
        </div>
    }
}
