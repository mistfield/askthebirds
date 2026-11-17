use leptos::prelude::*;
use leptos_meta::Title;

/// Заготовка: страница переносится из wireframe/product.html.
#[component]
pub fn ProductPage() -> impl IntoView {
    view! {
        <Title text="Товар" />
        <section class="px-3 md:px-6 py-20 md:py-[100px] min-h-[50vh]">
            <h1 class="h3">"Товар"</h1>
        </section>
    }
}
