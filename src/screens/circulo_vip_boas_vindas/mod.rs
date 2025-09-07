use leptos::prelude::*;
use leptos_meta::*;
use leptos_use::{use_window_size, UseWindowSizeReturn};

use crate::components::text::{ElementType, Text, TextSize, TextWeight};
use crate::components::button::{Button, Variant};

#[component]
pub fn CirculoVipBoasVindas() -> impl IntoView {
    let UseWindowSizeReturn { width, .. } = use_window_size();
    let is_large_screen = move || width.get() >= 1024.00;
    let title_size = Signal::derive( move || if is_large_screen() {TextSize::EvenBiggerTitle} else {TextSize::BigTitle});

    view! {
        <main class="bg-[url(/starbg.gif)] bg-blend-color-burn bg-purple-900 lg:h-[calc(100dvh-81px)] flex flex-col justify-center lg:justify-center lg:gap-8 lg:px-26 h-full lg:pt-10 px-8 max-lg:min-h-[calc(100dvh-65px)]">
            <div class="lg:grid lg:grid-cols-2 lg:gap-10 lg:items-end">
                <Title text="Círculo VIP - Boas Vindas" />
                <div>
                    <Text r#as=ElementType::H1 size={title_size} weight=TextWeight::Bold class="text-white max-lg:pb-13">
                        "Boas-vindas ao Círculo VIP"
                    </Text>
                </div>
                <Text r#as=ElementType::H2 size=TextSize::H2 class="text-white max-lg:pb-18">
                    "É muito bom ter você comigo nessa jornada. Prometo que envio novidades sobre o lançamentos de #ADE assim que tudo for definido."
                </Text>
            </div>
            <Button class="w-65" variant=Variant::Secondary><a href="/circulo-vip">"Voltar para formulário"</a></Button>
        </main>
    }
}
