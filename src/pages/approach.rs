use leptos::prelude::*;
use leptos_meta::Title;

/// Заготовка: страница переносится из wireframe/approach.html.
#[component]
pub fn ApproachPage() -> impl IntoView {
    view! {
        <Title text="Подход" />
        <section class="px-3 md:px-6 py-20 md:py-[100px] min-h-[50vh]">
            <h1 class="h3">"Подход"</h1>
        </section>
    }
}
