use leptos::prelude::*;
use leptos_meta::Title;

/// Заготовка: страница переносится из wireframe/faq.html.
#[component]
pub fn FaqPage() -> impl IntoView {
    view! {
        <Title text="Вопросы и ответы" />
        <section class="px-3 md:px-6 py-20 md:py-[100px] min-h-[50vh]">
            <h1 class="h3">"Вопросы и ответы"</h1>
        </section>
    }
}
