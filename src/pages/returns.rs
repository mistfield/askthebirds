use leptos::prelude::*;
use leptos_meta::Title;

/// Заготовка: страница переносится из wireframe/returns.html.
#[component]
pub fn ReturnsPage() -> impl IntoView {
    view! {
        <Title text="Возврат" />
        <section class="px-3 md:px-6 py-20 md:py-[100px] min-h-[50vh]">
            <h1 class="h3">"Возврат"</h1>
        </section>
    }
}
