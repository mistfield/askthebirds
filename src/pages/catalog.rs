use leptos::prelude::*;
use leptos_meta::Title;

/// Заготовка: страница переносится из wireframe/catalog.html.
#[component]
pub fn CatalogPage() -> impl IntoView {
    view! {
        <Title text="Каталог" />
        <section class="px-3 md:px-6 py-20 md:py-[100px] min-h-[50vh]">
            <h1 class="h3">"Каталог"</h1>
        </section>
    }
}
