use leptos::prelude::*;
use leptos_meta::Title;

/// Заготовка: страница переносится из wireframe/legal.html.
#[component]
pub fn LegalPage() -> impl IntoView {
    view! {
        <Title text="Документы" />
        <section class="px-3 md:px-6 py-20 md:py-[100px] min-h-[50vh]">
            <h1 class="h3">"Документы"</h1>
        </section>
    }
}
