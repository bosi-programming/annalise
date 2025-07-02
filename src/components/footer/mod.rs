use crate::components::text::{Text, TextSize};
use leptos:: prelude::*;
use newsletter::Newsletter;
use social::Social;

pub mod newsletter;
pub mod social;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="px-8">
            <Newsletter class="pb-22" />
            <Social />
            <Text class="text-white text-center" size={TextSize::Small}>
                "Feito com suor, lágrimas e muita ajuda de gente boa demais."
            </Text>
            <Text class="text-white text-center pb-5" size={TextSize::Small}>
                "Obrigada pela visita! Volte mais vezes 💜"
            </Text>
            <hr class="text-neutral-300" />
            <Text class="text-white text-center pt-6" size={TextSize::Details}>
                "Annalise Cerqueira-Maia é um pseudônimo."
            </Text>
            <Text class="text-white text-center pb-5" size={TextSize::Details}>
                "© 2025 todos os direitos reservados. Design by Anna & @DaBruOficial."
            </Text>
            <Text class="text-white pb-10" size={TextSize::Terms}>
                "Oi, você que chegou até aqui! Esta é a área de letras miúdas em que eu falo sobre um assunto que quase ninguém se importa, mas que considero essencial. Como você está aqui, me parece que compartilhamos esse entendimento. É o seguinte: estou me organizando para fazer o pré-lançamento e o lançamento de ADE em 2025. Isso significa que criei uma estratégia para fazer com que você me conheça, saiba que o livro existe e (talvez) queria dar uma chance para ele. Ou seja, preciso falar sobre como cuido dos seus dados porque privacidade é algo que levo muito a sério. Os dados que você compartilhar comigo serão utilizados exclusivamente para o envio de informações relacionadas ao livro. Seu email, nome e qualquer outro dado fornecido estão protegidos e jamais serão compartilhados com terceiros. Se você participar de sorteios, promoções ou do clube do livro, recomendo que leia os Termos e Condições, em que estão detalhadas as regras e responsabilidades de cada parte. Estou comprometida com as diretrizes da Lei Geral de Proteção de Dados (LGPD), garantindo que você tenha total controle sobre suas informações. Caso deseje excluir seus dados, entre em contato a qualquer momento via annalisecerqueira@gmail.com. Obrigada pelo voto de confiança e, claro, por ler até aqui. Prometo que escrevo prosa bem melhor do que as letras miúdas de um rodapé de site. ✨"
            </Text>
        </div>
    }
}
