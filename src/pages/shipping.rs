use leptos::prelude::*;
use leptos_meta::Title;

/// Заготовка: страница переносится из wireframe/shipping.html.
#[component]
pub fn ShippingPage() -> impl IntoView {
    view! {
        <Title text="Доставка" />
        <section class="px-3 md:px-6 py-20 md:py-[100px] min-h-[50vh]">
            <h1 class="h3">"Доставка"</h1>
        </section>
    }
}
