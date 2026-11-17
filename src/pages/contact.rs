use leptos::prelude::*;
use leptos_meta::Title;

/// Контакты: только текст, формы нет (как у natureofthings) — из wireframe/contact.html.
#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <Title text="Контакты" />
        <section class="px-3 md:px-6 pt-10 pb-10 md:pt-20 md:pb-20">
            <h1 class="text-[26px] md:text-[40px] font-medium uppercase leading-[1.1]">"Контакты"</h1>
        </section>
        // Абзацы через 16px на телефоне и 24px на компьютере
        <section class="reveal px-3 md:px-6 pb-20 md:pb-[100px] text-[16px] md:text-[18px] leading-[1.2]">
            <div class="max-w-[1040px] mr-auto space-y-4 md:space-y-6">
                <p>
                    "Вопрос? Напишите нам на "
                    <a href="mailto:mail@example.com" class="underline">"mail@example.com"</a>
                    " — заглушка. Ответим в течение 2–5 рабочих дней."
                </p>
                <p>
                    "По вопросам возврата — заглушка на "
                    <a href="mailto:mail@example.com" class="underline">"mail@example.com"</a>"."
                </p>
                <p>
                    "Для прессы, партнёров и оптовых запросов — заглушка на "
                    <a href="mailto:mail@example.com" class="underline">"mail@example.com"</a>"."
                </p>
            </div>
        </section>
    }
}
