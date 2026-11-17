use leptos::prelude::*;
use leptos_meta::Title;

/// Доставка: текст-заглушка из wireframe/shipping.html.
#[component]
pub fn ShippingPage() -> impl IntoView {
    view! {
        <Title text="Доставка" />
        <section class="px-3 md:px-6 pt-10 pb-10 md:pt-20 md:pb-20">
            <h1 class="text-[26px] md:text-[40px] font-medium uppercase leading-[1.1]">"Доставка"</h1>
        </section>
        // Текст: колонка до 1040px, слева, 18px на компьютере, 16px на телефоне
        <section class="reveal px-3 md:px-6 pb-20 md:pb-[100px] text-[16px] md:text-[18px] leading-[1.2]">
            <div class="max-w-[1040px] mr-auto">
                <p>
                    "Текст о доставке СДЭКом по России — заглушка. Подробнее о сроках и стоимости — в разделе "
                    <a href="/faq" class="underline">"Вопросы и ответы"</a>"."
                </p>
            </div>
        </section>
    }
}
