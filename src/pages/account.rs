use leptos::prelude::*;
use leptos_meta::Title;

/// Заготовка: страница переносится из wireframe/account.html.
#[component]
pub fn AccountPage() -> impl IntoView {
    view! {
        <Title text="Аккаунт" />
        <section class="px-3 md:px-6 py-20 md:py-[100px] min-h-[50vh]">
            <h1 class="h3">"Аккаунт"</h1>
        </section>
    }
}
