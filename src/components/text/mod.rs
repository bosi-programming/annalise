use leptos::prelude::*;

pub enum TextSize {
    Big,
    Medium,
    Small,
    Details,
    Terms,
    BigTitle,
    H1,
    H2,
    H3,
}

pub enum TextWeight {
    Normal,
    Bold,
}

#[derive(Clone, Default)]
pub enum ElementType {
    Span,
    #[default]
    P,
    H1,
    H2,
    H3,
}

#[component]
pub fn Text(
    #[prop(into, optional)] size: Option<TextSize>,
    #[prop(into, optional)] weight: Option<TextWeight>,
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] r#as: Option<ElementType>,
    children: Children,
) -> impl IntoView {
    let size_class = match size {
        Some(TextSize::Big) => "text-big",
        Some(TextSize::Medium) => "text-medium",
        Some(TextSize::Small) => "text-small",
        Some(TextSize::Details) => "text-details",
        Some(TextSize::Terms) => "text-terms",
        Some(TextSize::BigTitle) => "text-big-title",
        Some(TextSize::H1) => "text-h1",
        Some(TextSize::H2) => "text-h2",
        Some(TextSize::H3) => "text-h3",
        None => "text-big",
    };
    let weight_class = match weight {
        Some(TextWeight::Normal) => "font-normal",
        Some(TextWeight::Bold) => "font-bold",
        None => "font-normal",
    };
    let class = class.unwrap_or_default();
    let final_class = format!("font-[Lora] {} {} {}", size_class, weight_class, class).trim().to_string();

    match r#as {
        Some(ElementType::Span) => view! {
                <span class=final_class>
                    {children()}
                </span>
            }.into_any(),
        Some(ElementType::H1) => view! {
            <h1 class=final_class>
                {children()}
            </h1>
        }.into_any(),
        Some(ElementType::H2) => view! {
            <h2 class=final_class>
                {children()}
            </h2>
        }.into_any(),
        Some(ElementType::H3) => view! {
            <h3 class=final_class>
                {children()}
            </h3>
        }.into_any(),
        None | Some(ElementType::P) => view! {
            <p class=final_class>
                {children()}
            </p>
        }.into_any(),
    }
}
