//! «Вам может понравиться»: 4 карточки без зазоров (2 в ряд на телефоне).
//! При появлении слова заголовка выезжают снизу, карточки проявляются из размытия;
//! когда низ блока поднимается выше 15% экрана, карточки уходят вверх, гаснут и размываются.

use leptos::{ev, html, prelude::*};

use super::{current_product, recommendations};
use crate::{
    cart,
    data::{Product, rub},
};

/// Каких товаров нет в наличии (заглушка: в data.rs остатков пока нет).
const SOLD_OUT: [&str; 4] = ["cage-4", "perch-3", "feeder-3", "toy-4"];
const WORDS: [&str; 3] = ["Вам", "может", "понравиться"];

fn inner_height() -> f64 {
    window().inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(0.0)
}

/// Расстояние от верха страницы до элемента.
fn page_top(el: &web_sys::HtmlElement) -> f64 {
    use leptos::wasm_bindgen::JsCast;
    let mut top = el.offset_top();
    let mut parent = el.offset_parent();
    while let Some(p) = parent.and_then(|p| p.dyn_into::<web_sys::HtmlElement>().ok()) {
        top += p.offset_top();
        parent = p.offset_parent();
    }
    top as f64
}

#[island]
pub fn AlsoLike() -> impl IntoView {
    current_product().map(|p| view! { <Cards p /> })
}

#[component]
fn Cards(p: &'static Product) -> impl IntoView {
    let section = NodeRef::<html::Section>::new();
    let shown = RwSignal::new(false);
    // насколько карточки «ушли» вверх: 0 — на месте, 1 — исчезли
    let gone = RwSignal::new(0.0_f64);

    Effect::new(move |_| {
        let update = move || {
            let Some(s) = section.get_untracked() else { return };
            // где блок на экране (как getBoundingClientRect, которого нет в нашей сборке web-sys)
            let top = page_top(&s) - window().scroll_y().unwrap_or(0.0);
            let bottom = top + s.offset_height() as f64;
            let h = inner_height();
            if !shown.get_untracked() && top < h && bottom > 0.0 {
                shown.set(true);
            }
            let start = h * 0.15;
            let p = ((start - bottom) / start).clamp(0.0, 1.0);
            if p != gone.get_untracked() {
                gone.set(p);
            }
        };
        update();
        let h1 = window_event_listener(ev::scroll, move |_| update());
        let h2 = window_event_listener(ev::resize, move |_| update());
        on_cleanup(move || {
            h1.remove();
            h2.remove();
        });
    });

    let inner_style = move || {
        let p = gone.get();
        if p == 0.0 {
            String::new()
        } else {
            format!(
                "transform-origin:top;transform:translateY({}px) scale({});opacity:{};filter:blur({}px)",
                -100.0 * p,
                1.0 - 0.05 * p,
                1.0 - p,
                2.0 * p
            )
        }
    };

    view! {
        <section node_ref=section class="pt-20 pb-20 md:pt-[100px] md:pb-[100px]" class:in=shown id="also">
            <div class="px-3 md:px-6 flex items-center justify-between gap-6">
                <h2 class="h3">
                    {WORDS
                        .iter()
                        .enumerate()
                        .map(|(k, w)| {
                            let delay = format!("transition-delay:{}s", k as f64 / 10.0);
                            view! {
                                {(k > 0).then_some(" ")}
                                <span class="word">
                                    <span style=delay>{*w}</span>
                                </span>
                            }
                        })
                        .collect_view()}
                </h2>
                <div class="also-btn shrink-0">
                    <a href="/catalog" class="btn btn-brown whitespace-nowrap">"Смотреть все"</a>
                </div>
            </div>
            <ul class="grid grid-cols-2 md:grid-cols-4 mt-6 md:mt-10 gap-y-10 px-3 md:px-6">
                {recommendations(p)
                    .into_iter()
                    .enumerate()
                    .map(|(k, p)| {
                        let sold_out = SOLD_OUT.contains(&p.slug);
                        let delay = format!("transition-delay:{}s", k as f64 / 10.0);
                        view! {
                            <li class="also-card" style=delay>
                                <div class="also-inner relative flex flex-col group h-full overflow-hidden" style=inner_style>
                                    <div class="relative aspect-[360/540]">
                                        <div class="ph absolute inset-0">"Фото"</div>
                                        <div class="ph !bg-[#dcd8d3] absolute inset-0 opacity-0 group-hover:opacity-100 transition duration-300 ease-out">
                                            "Фото 2"
                                        </div>
                                        {(!sold_out)
                                            .then(|| p.badge)
                                            .flatten()
                                            .map(|b| {
                                                view! {
                                                    <span class="absolute top-2.5 md:top-6 left-2.5 md:left-6 h6 z-[1]">{b}</span>
                                                }
                                            })}
                                        {if sold_out {
                                            view! {
                                                <span class="absolute bottom-2.5 inset-x-0 text-center h6 text-black/80">
                                                    "Нет в наличии"
                                                </span>
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <button
                                                    aria-label="В корзину"
                                                    class="absolute bottom-2.5 md:bottom-6 right-2.5 md:right-6 size-[30px] md:size-[35px] border border-current flex items-center justify-center opacity-0 translate-x-full group-hover:translate-x-0 group-hover:opacity-100 focus:opacity-100 focus:translate-x-0 z-[2] hover:bg-brown hover:border-brown hover:text-white transition duration-300 ease-out"
                                                    on:click=move |_| cart::add(p.slug, 1)
                                                >
                                                    <svg width="9" height="9" viewBox="0 0 9 9" fill="none" aria-hidden="true">
                                                        <path stroke="currentColor" d="M4.43.27v8M8.5 4.2h-8" />
                                                    </svg>
                                                </button>
                                            }
                                                .into_any()
                                        }}
                                    </div>
                                    <div class="flex-1 pt-4 px-1 md:px-4 flex flex-col gap-1 text-center">
                                        <p class="h6">
                                            <a href=format!("/product/{}", p.slug) class="before:absolute before:inset-0 before:z-[1]">
                                                {p.name}
                                            </a>
                                        </p>
                                        <p class="h6 font-medium">{p.kind}</p>
                                        <p class="pt-1 mt-auto text-[11px] md:text-[13px] leading-[1.3]">{rub(p.price)}</p>
                                    </div>
                                </div>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </section>
    }
}
