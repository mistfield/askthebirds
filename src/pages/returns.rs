use leptos::prelude::*;
use leptos_meta::Title;

/// Возврат: текст-заглушка из wireframe/returns.html.
#[component]
pub fn ReturnsPage() -> impl IntoView {
    view! {
        <Title text="Возврат" />
        <section class="px-3 md:px-6 pt-10 pb-10 md:pt-20 md:pb-20">
            <h1 class="text-[26px] md:text-[40px] font-medium uppercase leading-[1.1]">"Возврат"</h1>
        </section>
        // Текст: та же вёрстка, что у доставки
        <section class="reveal px-3 md:px-6 pb-20 md:pb-[100px] text-[16px] md:text-[18px] leading-[1.2]">
            <div class="max-w-[1040px] mr-auto">
                <p>
                    "Текст об условиях возврата — заглушка. Подробнее — в разделе "
                    <a href="/faq" class="underline">"Вопросы и ответы"</a>"."
                </p>
            </div>
        </section>
    }
}
