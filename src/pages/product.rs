//! Страница товара — по вайрфрейму wireframe/product.html (как у natureofthings).
//! Сервер собирает разметку, интерактивные части — острова в подмодулях:
//! галерея с полноэкранным просмотром, форма покупки, «Вам может понравиться», вопросы.

use leptos::prelude::*;
use leptos_meta::Title;
#[cfg(feature = "ssr")]
use leptos_router::hooks::use_params_map;

use crate::data::{PRODUCTS, Product, product};

mod accordion;
mod also;
mod buy;
mod gallery;

use accordion::Faq;
use also::AlsoLike;
use buy::BuyForm;
use gallery::Gallery;

// ----- Заглушки: всего этого пока нет в data.rs -----

/// «Почему это безопасно»: материал и пояснение.
const SAFETY: [(&str, &str); 7] = [
    ("Натуральная берёза", "Откуда этот материал и почему он безопасен для клюва и лап птицы."),
    ("Нержавеющая сталь", "Откуда этот материал и почему он безопасен для клюва и лап птицы."),
    ("Без цинка и свинца", "Откуда этот материал и почему он безопасен для клюва и лап птицы."),
    ("Пищевые красители", "Откуда этот материал и почему он безопасен для клюва и лап птицы."),
    ("Сизаль", "Чем полезен и как служит."),
    ("Без клея", "Откуда этот материал и почему он безопасен для клюва и лап птицы."),
    ("Гладкие края", "Откуда этот материал и почему он безопасен для клюва и лап птицы."),
];

/// Отзывы: оценка звёздами, когда, кто, текст.
const REVIEWS: [(&str, &str, &str); 2] = [
    ("1 день назад", "Анна К.", "Текст отзыва в одну-две строки."),
    (
        "4 месяца назад",
        "Игорь С.",
        "Текст отзыва подлиннее: что понравилось, как попугай принял покупку, удобно ли чистить.",
    ),
];

/// Товар этой страницы. Острова не получают параметров (для них нужен serde), поэтому узнают
/// товар сами: на сервере — из маршрута, в браузере — из адреса страницы.
fn current_product() -> Option<&'static Product> {
    #[cfg(feature = "ssr")]
    let slug = use_params_map().read_untracked().get("slug").unwrap_or_default();
    #[cfg(not(feature = "ssr"))]
    let slug = window()
        .location()
        .pathname()
        .ok()
        .and_then(|p| p.strip_prefix("/product/").map(|s| s.trim_end_matches('/').to_string()))
        .unwrap_or_default();
    product(&slug)
}

/// «Дополните»: первые товары двух других категорий.
fn complements(p: &Product) -> Vec<&'static Product> {
    PRODUCTS.iter().filter(|o| o.category != p.category).step_by(4).take(2).collect()
}

/// «Вам может понравиться»: остальные товары этой категории, затем — других.
fn recommendations(p: &Product) -> Vec<&'static Product> {
    let same = PRODUCTS.iter().filter(|o| o.category == p.category && o.slug != p.slug);
    let other = PRODUCTS.iter().filter(|o| o.category != p.category);
    same.chain(other).take(4).collect()
}

#[component]
pub fn ProductPage() -> impl IntoView {
    match current_product() {
        Some(p) => view! { <ProductView p /> }.into_any(),
        None => view! { <NotFound /> }.into_any(),
    }
}

/// Нет такого товара — «Страница не найдена» со статусом 404.
#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    if let Some(resp) = use_context::<leptos_axum::ResponseOptions>() {
        resp.set_status(axum::http::StatusCode::NOT_FOUND);
    }
    view! {
        <Title text="Страница не найдена" />
        <section class="px-3 md:px-6 py-20 md:py-[100px] min-h-[50vh] flex flex-col items-start gap-6">
            <h1 class="h3">"Страница не найдена"</h1>
            <a href="/" class="btn">"На главную"</a>
        </section>
    }
}

#[component]
fn ProductView(p: &'static Product) -> impl IntoView {
    view! {
        <Title text=format!("{} {}", p.name, p.kind) />

        // 1. Товар. Блок заходит под шапку на 56px, фото начинается на 16px ниже шапки.
        // Компьютер (от 1024px): фото слева столбиком на 60% ширины, справа колонка до 583px, липкая.
        // Телефон и планшет: фото листаются пальцем, под ними полоса 2px, ниже колонка до 583px по центру.
        <section id="pdp" class="-mt-14 pt-[72px] pb-[100px] overflow-clip">
            <div class="lg:flex">
                <Gallery />
                <div class="flex-1 max-w-[583px] mx-auto">
                    <BuyForm />
                </div>
            </div>
        </section>

        // 2. Почему это безопасно. С 768px: слева 5/12, строки через 100px; справа 7/12 — большое фото,
        // прилипает в 100px от верха. Телефон: фото сверху, под ним материалы листаются вбок.
        <section class="relative pb-8 md:pt-14 md:pb-14 overflow-clip md:grid md:grid-cols-12">
            <div class="md:col-span-7 md:order-last">
                <div class="ph aspect-[1660/1387] sticky top-[100px]">"Большое фото"</div>
            </div>
            <div class="md:col-span-5 px-3 md:px-6 pt-10 md:py-20 flex gap-x-[70px] md:flex-col md:space-y-[100px] overflow-x-auto md:overflow-visible hide-scroll snap-x snap-mandatory scroll-pl-3 md:scroll-pl-0">
                {SAFETY
                    .iter()
                    .map(|(title, text)| {
                        view! {
                            <div class="max-w-3/4 last:max-w-full last:w-full md:w-full md:max-w-[410px] shrink-0 md:shrink snap-start">
                                <h2 class="max-w-[270px] md:max-w-full text-[22px] md:text-[26px] leading-[1.1]">
                                    {*title}
                                </h2>
                                <p class="max-w-[270px] md:max-w-full mt-4 text-[16px] leading-[1.5]">{*text}</p>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </section>

        // 3. Отзывы: отступ 80px сверху и снизу, поля 10px
        <section class="my-20 p-2.5 max-w-[1440px]">
            // место под сводку оценок (у них пустое)
            <div class="h-6"></div>
            <div class="flex flex-row-reverse border-b border-oatmilk mb-4">
                <div class="ml-auto pb-5">
                    <button class="btn btn-brown !px-2 bg-white">"Написать отзыв"</button>
                </div>
                <div class="relative">
                    <button class="px-2 pt-2 pb-[13px] text-[18px] font-medium uppercase leading-[1.1]">
                        "Отзывы " <small class="text-[12px] font-normal">{REVIEWS.len()}</small>
                    </button>
                    <span class="absolute bottom-0 inset-x-0 h-[3px] bg-black"></span>
                </div>
            </div>
            <div class="flex gap-2 mb-5 ml-px">
                <button class="bg-oatmilk pl-4 pr-9 py-2 text-[12px] font-medium leading-[1.66] relative">
                    "Все оценки"
                    <svg class="absolute right-4 top-1/2 -translate-y-1/2" width="9" height="6" viewBox="0 0 9 6" fill="none">
                        <path d="M1 1l3.5 3.5L8 1" stroke="currentColor" stroke-width="1.5" />
                    </svg>
                </button>
                <button class="bg-oatmilk px-4 py-2 text-[12px] font-medium leading-[1.66]">"С фото"</button>
            </div>
            <div>
                {REVIEWS
                    .iter()
                    .map(|(when, who, text)| {
                        view! {
                            <div class="pb-2.5 mb-[30px]">
                                <div class="flex justify-between items-center h-6">
                                    <span class="text-[12px] tracking-[0] text-brown leading-none">"★★★★★"</span>
                                    <span class="text-[12px] leading-[1.5] text-[#606a72]">{*when}</span>
                                </div>
                                <div class="mt-4 flex items-center gap-2 text-[12px] leading-[1.5]">
                                    {*who}
                                    <span class="flex items-center gap-1">
                                        <svg width="15" height="16" viewBox="0 0 15 16" aria-hidden="true">
                                            <circle cx="7.5" cy="8" r="7" fill="#696159" />
                                            <path d="M4.5 8l2 2 4-4" stroke="#fff" fill="none" />
                                        </svg>
                                        "Проверенный покупатель"
                                    </span>
                                </div>
                                <p class="mt-1.5 mb-6 text-[14px] leading-[1.2]">{*text}</p>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </section>

        // 4. Вам может понравиться
        <AlsoLike />

        // 5. Частые вопросы: с 768px три колонки — текст слева, вопросы на 2 колонки справа
        <section class="pt-20 pb-20 md:pt-[100px] md:pb-[100px] overflow-clip">
            <div class="px-3 md:px-6 md:grid md:grid-cols-3 gap-x-3 gap-y-[45px]">
                <div class="col-span-full flex items-center justify-between">
                    <h2 class="h3 max-w-[160px] md:max-w-none">"Частые вопросы"</h2>
                    <a href="/faq" class="btn btn-brown !px-7">"Все вопросы"</a>
                </div>
                <div class="max-w-[340px] mt-8 md:mt-0 text-[16px] leading-[1.5]">
                    <p>
                        "Короткий текст о том, что здесь собраны ответы на частые вопросы, а если ответа нет — можно написать нам."
                    </p>
                </div>
                <div class="md:col-span-2 w-full max-w-[924px] md:ml-auto mt-8 md:mt-0">
                    <Faq />
                </div>
            </div>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn related_products_never_include_the_product_itself() {
        for p in &PRODUCTS {
            let c = complements(p);
            assert_eq!(c.len(), 2);
            assert!(c.iter().all(|o| o.category != p.category));
            let r = recommendations(p);
            assert_eq!(r.len(), 4);
            assert!(r.iter().all(|o| o.slug != p.slug));
        }
    }
}
