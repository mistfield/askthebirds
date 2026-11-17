use leptos::prelude::*;
use leptos_meta::Title;

/// Заготовка: страница переносится из wireframe/contact.html.
#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <Title text="Контакты" />
        <section class="px-3 md:px-6 py-20 md:py-[100px] min-h-[50vh]">
            <h1 class="h3">"Контакты"</h1>
        </section>
    }
}
