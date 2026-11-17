use leptos::prelude::*;

const CATEGORIES: &[(&str, &str)] = &[
    ("Клетки", "/catalog/cages"),
    ("Жёрдочки", "/catalog/perches"),
    ("Кормушки и поилки", "/catalog/feeders"),
    ("Игрушки", "/catalog/toys"),
];

/// Главная: первый экран, категории, видео-блок — как у natureofthings.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        // первый экран: 100vh, шапка лежит поверх фото
        <section class="relative -mt-14 h-screen min-h-[560px]">
            <div class="ph absolute inset-0 !items-start !justify-start pt-24 px-6">
                "Фото на весь экран (на телефоне — отдельное вертикальное)"
            </div>
            <div class="absolute bottom-8 md:bottom-14 left-1/2 -translate-x-1/2">
                <a href="/catalog" class="btn">"Смотреть все"</a>
            </div>
        </section>

        // категории: отступы 100/80px, плитки 346×521 без зазоров: 1 / 2 / 4 колонки
        <section class="reveal px-3 md:px-6 py-20 md:py-[100px]">
            <div class="flex items-center justify-between mb-6 md:mb-10">
                <h2 class="h3">"Категории"</h2>
                <a href="/catalog" class="btn">"Смотреть все"</a>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4">
                {CATEGORIES
                    .iter()
                    .map(|(name, href)| {
                        view! {
                            <a href=*href class="group/tile relative block aspect-[346/521] overflow-hidden">
                                <div class="ph absolute inset-0 transition-transform duration-300 group-hover/tile:scale-105">
                                    "Фото"
                                </div>
                                <h3 class="h3 absolute bottom-8 inset-x-0 text-center text-white [text-shadow:0_0_12px_rgba(0,0,0,.25)] transition-transform duration-300 group-hover/tile:-translate-y-2.5">
                                    {*name}
                                </h3>
                            </a>
                        }
                    })
                    .collect_view()}
            </div>
        </section>

        // видео-блок: 100vh, слоган и светлая кнопка внизу по центру
        <section class="reveal relative h-screen min-h-[560px]">
            <div class="ph !bg-ph-dark !text-white/70 absolute inset-0 !items-start !justify-start p-6">
                "Видео на весь экран"
            </div>
            <div class="absolute inset-x-0 bottom-8 md:bottom-14 flex flex-col items-center gap-8 px-6 text-center">
                <h3 class="h3 text-white">"Слоган"</h3>
                <a href="/catalog" class="btn btn-light">"Смотреть все"</a>
            </div>
        </section>
    }
}
