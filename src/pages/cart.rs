use leptos::prelude::*;
use leptos_meta::Title;

/// Заготовка: страница переносится из wireframe/cart.html.
#[component]
pub fn CartPage() -> impl IntoView {
    view! {
        <Title text="Корзина" />
        <section class="px-3 md:px-6 py-20 md:py-[100px] min-h-[50vh]">
            <h1 class="h3">"Корзина"</h1>
        </section>
    }
}
