//! Правая колонка товара: название, описание, цвет и размер, «В корзину» с ценой,
//! оплата частями, раскрывающиеся блоки и «Дополните». Липкая на компьютере.

use std::time::Duration;

use leptos::{ev, prelude::*};

use super::accordion::{Acc, AccBody, AccIcon};
use super::{complements, current_product};
use crate::{
    cart,
    data::{Product, rub},
};

// ----- Заглушки: вариантов, описаний и размеров пока нет в data.rs -----

const DESCRIPTION: &str = "Описание товара в четыре строки: из чего он сделан, чем удобен птице и хозяину, \
     как его собрать и поставить, почему ему можно доверять и чем он лучше обычных.";
const FREE_OF: &str = "Без краски, без клея, без острых краёв, без цинка, без свинца, без лака.";
const DIMENSIONS: &str = "45 × 35 × 60 см";
/// Цвета: название и оттенок кружка.
const COLORS: [(&str, &str); 2] = [("Натуральный", "#e3ddd5"), ("Тёмный", "#8f8983")];
const SIZES: [&str; 3] = ["S", "M", "L"];
const DEFAULT_SIZE: &str = "M";
const DETAILS: [(&str, &str); 5] = [
    ("Материалы", "Из чего сделаны все части: прутья, дно, дверцы, крепления. Каждый материал — отдельной строкой."),
    ("Размеры", "Ширина × глубина × высота, расстояние между прутьями, вес."),
    (
        "Для каких попугаев",
        "Каким птицам подходит по размеру и характеру: волнистые, неразлучники, кореллы и т.д.",
    ),
    ("Уход и чистка", "Как мыть, чем можно и нельзя, как часто менять."),
    ("Доставка", "Сроки и стоимость доставки, возврат."),
];

const PLUS: &str = "M4.43.27v8M8.5 4.2h-8";

#[island]
pub fn BuyForm() -> impl IntoView {
    current_product().map(|p| view! { <Form p /> })
}

#[component]
fn Form(p: &'static Product) -> impl IntoView {
    let (slug, name, kind, price) = (p.slug, p.name, p.kind, p.price);
    let color = RwSignal::new(0_usize);
    let busy = RwSignal::new(false);

    // В корзину: крутилка на кнопке, через 0,4 с — товар в корзине и шторка открыта, через 1,4 с — снова текст
    let on_submit = move |e: ev::SubmitEvent| {
        e.prevent_default();
        if busy.get_untracked() {
            return;
        }
        busy.set(true);
        set_timeout(move || cart::add(slug, 1), Duration::from_millis(400));
        set_timeout(move || busy.set(false), Duration::from_millis(1400));
    };

    let details = DETAILS.len();
    view! {
        <form class="pt-8 xl:pt-[120px] px-3 lg:px-6 xl:px-[72px] lg:sticky lg:top-0" on:submit=on_submit>
            // название: 16px на телефоне, 18px с 768px; размер справа серым
            <div class="mb-6">
                <h1 class="rise h3 !font-normal">{name}</h1>
                <div class="rise rise-2 flex flex-wrap items-end gap-4 justify-between mt-1.5">
                    <p class="h3">{kind}</p>
                    <p class="caps text-black/60">{DIMENSIONS}</p>
                </div>
            </div>
            // описание: 16/18px; строка «без чего» мельче
            <div class="mb-8">
                <div class="rise text-[16px] md:text-[18px] leading-[1.2]">
                    <p>{DESCRIPTION}</p>
                </div>
                <p class="rise rise-2 mt-2.5 text-[14px] md:text-[16px] leading-[1.3]">{FREE_OF}</p>
            </div>
            <div class="mb-5">
                <div class="rise h-px w-full bg-brown/20"></div>
            </div>
            // варианты: цвет кружками (подпись при наведении), размер квадратиками
            <div class="mb-5">
                <ul class="rise space-y-4">
                    <li class="flex items-center justify-between">
                        <p class="text-[12px] leading-[1.3] text-black/60 shrink-0">
                            "Цвет "
                            <span class="inline-block ml-1.5 text-black">{move || COLORS[color.get()].0}</span>
                        </p>
                        <ul class="ml-auto flex flex-wrap gap-2">
                            {COLORS
                                .iter()
                                .enumerate()
                                .map(|(k, (label, bg))| {
                                    view! {
                                        <li class="group block relative">
                                            <a
                                                href="#"
                                                aria-label=*label
                                                class="block size-[18px] m-1 rounded-full border border-brown/20 ring-black ring-offset-4"
                                                class:ring-1=move || color.get() == k
                                                class:ring-0=move || color.get() != k
                                                style=format!("background:{bg}")
                                                on:click=move |e| {
                                                    e.prevent_default();
                                                    color.set(k);
                                                }
                                            ></a>
                                            <span class="whitespace-nowrap absolute top-full left-1/2 -translate-x-1/2 -translate-y-1/2 py-0.5 px-1 mt-1 caps opacity-0 group-hover:opacity-100 group-hover:translate-y-0 transition ease-out duration-300 z-[2] pointer-events-none">
                                                {*label}
                                            </span>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    </li>
                    <li class="flex items-center justify-between">
                        <p class="text-[12px] leading-[1.3] text-black/60 shrink-0">"Размер"</p>
                        <ul class="mt-2 gap-1.5 flex flex-wrap">
                            {SIZES
                                .iter()
                                .map(|s| {
                                    view! {
                                        <li>
                                            <label class="group block relative">
                                                <input
                                                    type="radio"
                                                    name="size"
                                                    value=*s
                                                    class="sr-only peer"
                                                    checked=*s == DEFAULT_SIZE
                                                />
                                                <span class="caps text-center p-2 w-full block border border-brown/20 text-black/60 peer-checked:border-black hover:border-black peer-checked:text-black cursor-pointer">
                                                    {*s}
                                                </span>
                                            </label>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                    </li>
                </ul>
            </div>
            // В корзину: чёрная, 42px, цена через 32px; при наведении коричневая
            <div class="rise mb-3">
                <button
                    type="submit"
                    class="relative w-full flex justify-center items-center gap-8 py-[13px] px-10 border border-white bg-black text-white text-[12px] uppercase tracking-[0.01em] leading-[1.2] hover:bg-brown hover:border-brown transition-colors"
                >
                    <span class:opacity-0=busy>"В корзину"</span>
                    <span
                        class="absolute inset-0 m-auto size-5 rounded-full border-2 border-white/30 border-t-white animate-spin"
                        class:opacity-0=move || !busy.get()
                    ></span>
                    <span class:opacity-0=busy>{rub(price)}</span>
                </button>
            </div>
            // оплата частями: заглушка под «Долями / Сплит»
            <div class="rise mt-4 mb-4 md:mb-14">
                <p class="text-[14px] leading-[26px] flex items-center flex-wrap gap-x-1">
                    "или 4 платежа по " <b class="font-medium">{rub(price.div_ceil(4))}</b> " с "
                    <span class="inline-flex items-center h-[22px] px-2 border border-black rounded-full text-[11px] uppercase">
                        "Долями / Сплит"
                    </span>
                    <button
                        type="button"
                        aria-label="Подробнее"
                        class="size-3.5 rounded-full border border-black text-[9px] leading-none"
                    >
                        "i"
                    </button>
                </p>
            </div>
            // раскрывающиеся блоки: строки 50px, разделители коричневые 20%
            {DETAILS
                .iter()
                .enumerate()
                .map(|(k, (title, text))| {
                    let acc = Acc::new();
                    view! {
                        <div class:mb-14=k + 1 == details>
                            <div class="acc rise py-4 -mt-px border-y border-brown/20">
                                <h2>
                                    <button
                                        type="button"
                                        class="acc-btn caps w-full flex items-center justify-between hover:underline relative"
                                        aria-expanded=move || acc.open.get().to_string()
                                        on:click=move |_| acc.toggle()
                                    >
                                        <span class="absolute inset-x-0 -inset-y-6"></span>
                                        <span>{*title}</span>
                                        <AccIcon />
                                    </button>
                                </h2>
                                <AccBody acc text=text />
                            </div>
                        </div>
                    }
                })
                .collect_view()}
            // Дополните: 2 товара в ряд через 32px, над ними чёрная линия
            <div class="mb-2">
                <h2 class="rise text-[13px] md:text-[14px] leading-[1.2] font-medium uppercase pt-4 mb-5">"Дополните"</h2>
                <div class="rise rise-2 border-t border-black pt-4 grid grid-cols-2 gap-8">
                    {complements(p)
                        .into_iter()
                        .map(|o| {
                            let href = format!("/product/{}", o.slug);
                            view! {
                                <div class="group relative flex flex-col gap-3">
                                    <div class="h6">
                                        <h3>
                                            <a href=href class="before:absolute before:inset-0 before:z-[2]">{o.name}</a>
                                        </h3>
                                        <p class="font-medium">{o.kind}</p>
                                    </div>
                                    <div class="relative mt-auto overflow-hidden">
                                        <div class="ph aspect-[356/529]">"Фото"</div>
                                        <div class="ph !bg-[#dcd8d3] absolute inset-0 opacity-0 group-hover:opacity-100 transition duration-300 ease-out z-[1]">
                                            "Фото 2"
                                        </div>
                                    </div>
                                    <button
                                        type="button"
                                        class="relative z-[2] w-full flex justify-between items-center px-[14px] py-2 border border-black text-[12px] uppercase tracking-[0.01em] leading-[1.2] btn-brown transition-colors"
                                        on:click=move |_| cart::add(o.slug, 1)
                                    >
                                        {rub(o.price)}
                                        <svg width="9" height="9" viewBox="0 0 9 9" fill="none" aria-hidden="true">
                                            <path stroke="currentColor" d=PLUS />
                                        </svg>
                                    </button>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </form>
    }
}
