use std::time::Duration;

use leptos::prelude::*;
use leptos_meta::Title;

use crate::cart::{self, Line};
use crate::data::{product, rub, Product};

/// Что предложить к набору (в вайрфрейме — четыре карточки-заглушки).
const SUGGESTED: [&str; 4] = ["perch-1", "perch-2", "feeder-2", "toy-1"];

/// «1 товар», «2 товара», «5 товаров».
fn items_label(n: u32) -> String {
    let word = if n % 10 == 1 && n % 100 != 11 {
        "товар"
    } else if (2..=4).contains(&(n % 10)) && !(10..20).contains(&(n % 100)) {
        "товара"
    } else {
        "товаров"
    };
    format!("{n} {word}")
}

#[component]
pub fn CartPage() -> impl IntoView {
    view! {
        <Title text="Корзина" />
        <CartBody />
    }
}

/// Вся страница — один остров: корзина живёт в браузере (localStorage), на сервере она пустая.
#[island]
fn CartBody() -> impl IntoView {
    let lines = RwSignal::new(Vec::<Line>::new());
    // пока корзина не прочитана из браузера, левую колонку не показываем (иначе мелькнёт «пусто»)
    let ready = RwSignal::new(false);
    // строка, которая схлопывается после «Удалить»
    let removing = RwSignal::new(None::<&'static str>);

    Effect::new(move |_| {
        lines.set(cart::load());
        ready.set(true);
        let h = window_event_listener_untyped(cart::CART_EVENT, move |_| lines.set(cart::load()));
        on_cleanup(move || h.remove());
    });

    let update = move |slug: &'static str, qty: u32| {
        let next = cart::set_qty(lines.get_untracked(), slug, qty);
        cart::save(&next, false);
        lines.set(next);
    };
    let add = move |slug: &'static str| {
        let current = lines.get_untracked().iter().find(|l| l.product.slug == slug).map_or(0, |l| l.qty);
        update(slug, (current + 1).min(999));
    };
    // «Удалить» и «−» на единице: строка схлопывается за 0,3 с и пропадает
    let remove = move |slug: &'static str| {
        removing.set(Some(slug));
        set_timeout(
            move || {
                removing.set(None);
                update(slug, 0);
            },
            Duration::from_millis(300),
        );
    };

    let is_empty = move || lines.get().is_empty();
    let total = move || rub(cart::total(&lines.get()));

    view! {
        // 1. Корзина: фон — «овсяное молоко» 40%, отступы 80/100px (телефон 40/12)
        <section class="relative overflow-hidden bg-oatmilk/40 pt-10 pb-3 md:pt-20 md:pb-[100px]">
            <div class="px-3 md:px-6 flex flex-col md:flex-row justify-between gap-10">
                // слева: пусто
                <div class="w-full" class=("hidden", move || !is_empty()) class=("invisible", move || !ready.get())>
                    <div class="w-full h-full flex flex-col items-center justify-center">
                        <h1 class="text-[22px] md:text-[26px] leading-[1.1] uppercase font-medium text-center">
                            "Ваша корзина пуста"
                        </h1>
                        <a href="/catalog?sort=bestsellers" class="btn mt-10">"Хиты продаж"</a>
                    </div>
                </div>

                // слева: список товаров
                <div class="w-full max-w-[812px]" class=("hidden", is_empty)>
                    <div class="flex items-center gap-2 justify-between">
                        <h1 class="text-[13px] md:text-[14px] leading-[1.2] uppercase font-medium flex items-end gap-2">
                            "Ваша корзина "
                            <span class="text-[12px] leading-[1.25] tracking-[0.04em] font-normal opacity-60">
                                {move || items_label(cart::count(&lines.get()))}
                            </span>
                        </h1>
                        <div class="size-6"></div>
                    </div>
                    <ul>
                        <For each=move || lines.get() key=|l| (l.product.slug, l.qty) let:line>
                            <CartRow line removing update remove />
                        </For>
                    </ul>
                </div>

                // справа: итог заказа (до 480px)
                <div class="w-full max-w-[480px]">
                    <div class="bg-white p-3 md:p-6 pb-6 md:pb-10">
                        <h2 class="text-[13px] md:text-[14px] leading-[1.2] uppercase font-medium">"Итог заказа"</h2>
                        <div class="mt-4 md:mt-6 pt-4 md:pt-6 border-t border-brown/20 flex items-center justify-between caps">
                            <p>"Подытог"</p>
                            <p>{total}</p>
                        </div>
                        <div class="my-4 md:my-6 pb-4 md:pb-6 border-b border-brown/20 flex items-center justify-between caps">
                            <p>"Доставка"</p>
                            <p>"Рассчитывается при оформлении"</p>
                        </div>
                        <div class="pt-8 md:pt-[165px]">
                            // пока пусто — кнопка бледная и неактивная
                            <a
                                href="#"
                                class=move || {
                                    if is_empty() {
                                        "w-full flex items-center justify-center gap-[22px] md:gap-8 px-10 pt-3 pb-[14px] border text-white leading-none text-[12px] md:text-[13px] uppercase tracking-[0.01em] transition-colors border-brown/20 bg-brown/20 pointer-events-none cursor-not-allowed"
                                    } else {
                                        "w-full flex items-center justify-center gap-[22px] md:gap-8 px-10 pt-3 pb-[14px] border text-white leading-none text-[12px] md:text-[13px] uppercase tracking-[0.01em] transition-colors border-brown bg-brown hover:bg-black hover:border-black"
                                    }
                                }
                                aria-disabled=move || is_empty().to_string()
                            >
                                "Оформить заказ "
                                <span>{total}</span>
                            </a>
                        </div>
                        // у них здесь блок рассрочки стороннего сервиса (высота ~31px); у нас пока заглушка
                        <div class="pt-3 md:pt-4">
                            <div class="ph mt-4 h-[59px] lg:h-[30px]">"Блок рассрочки (если понадобится)"</div>
                        </div>
                        <div class="mt-6 md:mt-10">
                            <h3 class="caps text-center">"Безопасная оплата"</h3>
                            <ul class="mt-3 md:mt-4 flex flex-wrap justify-center gap-2 md:gap-4">
                                {(0..13)
                                    .map(|_| view! { <li class="ph w-[38px] h-6 !text-[8px]">"Карта"</li> })
                                    .collect_view()}
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        // 2. «Дополните набор»: отступы 100px (телефон 80), 2 колонки на телефоне, 4 от 768px
        <section class="relative py-20 md:py-[100px]">
            <div class="px-3 md:px-6 flex items-center justify-between gap-6">
                <h2 class="h3">"Дополните набор"</h2>
                <a href="/catalog" class="btn">"Смотреть все"</a>
            </div>
            <ul class="grid grid-cols-2 md:grid-cols-4 mt-6 md:mt-10 gap-y-10 min-h-[100px]">
                {SUGGESTED
                    .iter()
                    .filter_map(|slug| product(slug))
                    .map(|p| view! { <Suggested p add /> })
                    .collect_view()}
            </ul>
        </section>
    }
}

/// Одна строка корзины: фото, название, цена, «−/число/+» и «Удалить».
#[component]
fn CartRow(
    line: Line,
    removing: RwSignal<Option<&'static str>>,
    update: impl Fn(&'static str, u32) + Copy + Send + Sync + 'static,
    remove: impl Fn(&'static str) + Copy + Send + Sync + 'static,
) -> impl IntoView {
    let p = line.product;
    let gone = move || removing.get() == Some(p.slug);
    view! {
        <li
            class="flex relative gap-8 overflow-hidden transition-[max-height,opacity] duration-300"
            style=move || if gone() { "max-height:0;opacity:0" } else { "max-height:300px" }
        >
            <div class="absolute bottom-0 inset-x-0 h-px bg-brown/20"></div>
            <a href=format!("/product/{}", p.slug) class="shrink-0 py-5 md:py-6 w-[86px] md:w-[114px]">
                <div class="ph w-full aspect-[114/148]"></div>
            </a>
            <div class="w-full flex flex-col justify-between py-8 md:py-10 gap-4">
                <div class="flex items-start gap-2 justify-between">
                    <div>
                        <p class="text-[12px] md:text-[13px] leading-[1.2] uppercase tracking-[0.01em]">
                            <a href=format!("/product/{}", p.slug) class="hover:underline">{p.name}</a>
                        </p>
                        <p class="text-[12px] md:text-[13px] leading-[1.2] uppercase tracking-[0.01em] font-medium mt-1">
                            {p.kind}
                        </p>
                    </div>
                    <p class="shrink-0 text-[12px] leading-[1.3]">{rub(p.price * line.qty)}</p>
                </div>
                <div class="flex items-end gap-2 justify-between">
                    <div class="flex">
                        <button
                            type="button"
                            aria-label="Меньше"
                            class="size-7 bg-oatmilk/60 flex items-center justify-center hover:bg-brown hover:text-white transition"
                            on:click=move |_| {
                                if line.qty > 1 {
                                    update(p.slug, line.qty - 1);
                                } else {
                                    remove(p.slug);
                                }
                            }
                        >
                            <svg width="9" height="8" viewBox="0 0 9 8" fill="none" stroke="currentColor">
                                <path d="M8.85 3.93h-8" />
                            </svg>
                        </button>
                        <span class="size-7 bg-oatmilk/60 flex items-center justify-center caps text-center">{line.qty}</span>
                        <button
                            type="button"
                            aria-label="Больше"
                            class="size-7 bg-oatmilk/60 flex items-center justify-center hover:bg-brown hover:text-white transition"
                            on:click=move |_| update(p.slug, (line.qty + 1).min(999))
                        >
                            <svg width="9" height="9" viewBox="0 0 9 9" fill="none" stroke="currentColor">
                                <path d="M4.43.27v8M8.5 4.2h-8" />
                            </svg>
                        </button>
                    </div>
                    <a
                        href="#"
                        class="caps hover:underline transition"
                        on:click=move |e| {
                            e.prevent_default();
                            remove(p.slug);
                        }
                    >
                        "Удалить"
                    </a>
                </div>
            </div>
        </li>
    }
}

/// Карточка «Дополните набор»: кнопка «+» выезжает справа при наведении.
#[component]
fn Suggested(p: &'static Product, add: impl Fn(&'static str) + Copy + Send + Sync + 'static) -> impl IntoView {
    let href = format!("/product/{}", p.slug);
    view! {
        <li>
            <div class="relative flex flex-col group h-full overflow-hidden">
                <div class="relative aspect-[360/540]">
                    <a href=href.clone() class="ph absolute inset-0">"Фото"</a>
                    {p.badge.map(|b| view! { <span class="absolute top-[10px] md:top-6 left-[10px] md:left-6 caps text-black/80">{b}</span> })}
                    <button
                        type="button"
                        aria-label="В корзину"
                        class="absolute bottom-[10px] md:bottom-6 right-[10px] md:right-6 size-[30px] md:size-[35px] border border-current flex items-center justify-center opacity-0 translate-x-full group-hover:translate-x-0 group-hover:opacity-100 focus:opacity-100 focus:translate-x-0 hover:bg-brown hover:text-white transition duration-300 ease-out"
                        on:click=move |_| add(p.slug)
                    >
                        <svg width="9" height="9" viewBox="0 0 9 9" fill="none" stroke="currentColor">
                            <path d="M4.43.27v8M8.5 4.2h-8" />
                        </svg>
                    </button>
                </div>
                <div class="flex-1 pt-4 px-1 md:px-4 flex flex-col gap-1 text-center">
                    <p class="text-[12px] md:text-[13px] leading-[1.2] uppercase tracking-[0.01em]">
                        <a href=href class="hover:underline">{p.name}</a>
                    </p>
                    <p class="text-[12px] md:text-[13px] leading-[1.2] uppercase tracking-[0.01em] font-medium">{p.kind}</p>
                    <p class="text-[11px] md:text-[13px] leading-[1.3] pt-1 mt-auto">{rub(p.price)}</p>
                </div>
            </div>
        </li>
    }
}
