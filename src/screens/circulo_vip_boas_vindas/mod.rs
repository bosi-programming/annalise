use leptos::prelude::*;
use leptos_meta::*;
use leptos_use::{use_window_size, UseWindowSizeReturn};

use crate::components::text::{ElementType, Text, TextSize, TextWeight};
use crate::components::button::Button;

#[component]
pub fn CirculoVipBoasVindas() -> impl IntoView {
    let UseWindowSizeReturn { width, .. } = use_window_size();
    let is_large_screen = move || width.get() >= 1024.00;
    let title_size = Signal::derive( move || if is_large_screen() {TextSize::BiggerTitle} else {TextSize::BigTitle});

    view! {
        <main class="lg:grid lg:grid-cols-2 lg:py-45 lg:mb-13 lg:gap-30 lg:px-30 h-full pt-10 px-8">
            <Title text="Círculo VIP - Boas Vindas" />
            <div>
                <Text r#as=ElementType::H1 size={title_size} weight=TextWeight::Bold class="text-white mt-10 mr-16">
                    "Boas-vindas ao Círculo VIP"
                </Text>
                <Text r#as=ElementType::H2 size=TextSize::H2 class="text-white mt-10 mr-16">
                    "É muito bom ter você comigo nessa jornada. Prometo que envio novidades sobre o lançamentos de #ADE assim que tudo for definido."
                </Text>
                <Button class="mt-10"><a href="/circulo-vip">"Voltar para formulário"</a></Button>
            </div>
        </main>
    }
}
