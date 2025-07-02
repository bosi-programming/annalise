use leptos::prelude::*;
use leptos_router::path;
use leptos_router::components::*;

use crate::components::footer::Footer;
use crate::screens::circulo_vip::CirculoVip;

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router>
        <main>
          <Routes fallback=|| "Not found.">
            <Route path=path!("/circulo-vip") view=CirculoVip/>
          </Routes>
        </main>
        <Footer />
      </Router>
    }
}
