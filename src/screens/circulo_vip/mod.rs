use components::form::Form;
use components::heading::Heading;
use leptos::prelude::*;
use leptos_meta::*;

use crate::components::text::Text;

pub mod components;

#[component]
pub fn CirculoVip() -> impl IntoView {
    provide_meta_context();
    view! {
        <main class="lg:grid lg:grid-cols-2 lg:py-40 lg:gap-30 lg:px-30 h-full pt-10 px-8">
            <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <Meta charset="UTF-8"/>
            <Meta name="author" content="Felipe Azevedo Bosi"/>
            <Title text="Círculo VIP - #ADE" />
            <div>
                <Heading class="text-white leading-none" />
                <Text class="text-white mt-10 mr-16">
                    "Entre para o Círculo VIP e seja a primeira a receber novidades sobre o universo de #ADE. Prometo pela Lua que brilha no céu: nada de spam."
                </Text>
            </div>
            <Form />
        </main>
    }
}
