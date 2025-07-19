use crate::components::text::{Text, TextSize};
use leptos:: prelude::*;
use newsletter::Newsletter;
use social::Social;

pub mod newsletter;
pub mod social;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="lg:px-30 lg:pt-15 px-8">
            <Newsletter class="pb-22" />
            <Social />
            <Text class="text-white text-center" size={TextSize::Small}>
                "Feito com suor, lágrimas e muita ajuda de gente boa demais."
            </Text>
            <Text class="text-white text-center pb-5 lg:pb-18" size={TextSize::Small}>
                "Obrigada pela visita! Volte mais vezes 💜"
            </Text>
            <hr class="text-neutral-300" />
            <Text class="text-white text-center pt-6" size={TextSize::Small}>
                "Annalise Cerqueira-Maia é um pseudônimo."
            </Text>
            <Text class="text-white text-center pb-5 lg:pb-12" size={TextSize::Small}>
                "© 2025 todos os direitos reservados. Design by Anna & @DaBruOficial."
            </Text>
            <Text class="text-white pb-10 lg:pb-35" size={TextSize::Details}>
                "Oi, você que chegou até aqui! Essa é a parte das letras miúdas (pouca gente lê, mas é importante): seus dados são usados apenas pra te enviar novidades sobre o livro ADE. Nada de terceiros. Se participar de sorteios ou do clube do livro, recomendo ler os "
            <a class="underline" href="https://docs.google.com/document/d/1L113L91ScvYjhnnP0RLjZ8TeO6e0RaszeWQ3duWmJ88/edit?usp=sharing" target="_blank" rel="license">"Termos e Condições."</a>
           " Dúvidas ou quer excluir seus dados? Escreva para annalisecerqueira@gmail.com. Obrigada pelo voto de confiança — e por ler até aqui. Prometo que minha prosa é bem mais interessante que esse rodapé. ✨"
            </Text>
        </div>
    }
}
