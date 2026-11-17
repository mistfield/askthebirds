//! Галерея товара и полноэкранный просмотр.
//! Компьютер: фото столбиком, кнопка «увеличить» прилипает к низу экрана.
//! Телефон и планшет: фото листаются пальцем или мышью, полоса под ними показывает номер фото.

use std::time::Duration;

use leptos::{ev, html, prelude::*};

/// Фото товара (заглушки) 740×760.
const SLIDES: [&str; 2] = ["Фото 1 — товар на белом фоне", "Фото 2 — товар с попугаем"];
const N: i32 = SLIDES.len() as i32;

fn inner_width() -> f64 {
    window().inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(0.0)
}

#[island]
pub fn Gallery() -> impl IntoView {
    // ----- лента на телефоне и планшете -----
    let index = RwSignal::new(0_i32);
    // палец/мышь прижаты: (где начали, на сколько сдвинули)
    let drag = RwSignal::new(None::<(i32, i32)>);
    let track = NodeRef::<html::Div>::new();
    let bar = NodeRef::<html::Div>::new();
    let go = move |i: i32| index.set(i.clamp(0, N - 1));

    let track_style = move || {
        let base = -index.get() * 100;
        match drag.get() {
            Some((_, dx)) => format!("transform:translateX(calc({base}% + {dx}px));transition:none"),
            None => format!("transform:translateX({base}%)"),
        }
    };
    let on_down = move |e: ev::PointerEvent| {
        if inner_width() < 1024.0 {
            drag.set(Some((e.client_x(), 0)));
            if let Some(t) = track.get_untracked() {
                let _ = t.set_pointer_capture(e.pointer_id());
            }
        }
    };
    let on_move = move |e: ev::PointerEvent| {
        if let Some((start, _)) = drag.get_untracked() {
            drag.set(Some((start, e.client_x() - start)));
        }
    };
    let on_up = move |e: ev::PointerEvent| {
        let Some((start, _)) = drag.get_untracked() else { return };
        let dx = e.client_x() - start;
        drag.set(None);
        let i = index.get_untracked();
        go(if dx < -50 {
            i + 1
        } else if dx > 50 {
            i - 1
        } else {
            i
        });
    };
    let on_bar = move |e: ev::MouseEvent| {
        // offset_x — от левого края полосы: ползунок клики не ловит
        if let Some(b) = bar.get_untracked() {
            let part = e.offset_x() as f64 / b.offset_width() as f64;
            go((part * N as f64).floor() as i32);
        }
    };

    // ----- полноэкранный просмотр: 3 копии ленты, по кругу -----
    let lb_open = RwSignal::new(false); // показан (display)
    let lb_shown = RwSignal::new(false); // проявлен (прозрачность)
    let lb_index = RwSignal::new(N);
    let lb_track = NodeRef::<html::Div>::new();

    // поставить ленту так, чтобы текущее фото было по центру экрана
    let place = move |animate: bool| {
        let Some(t) = lb_track.get_untracked() else { return };
        let w = t.first_element_child().map_or(0, |c| c.client_width()) as f64;
        let x = inner_width() / 2.0 - (lb_index.get_untracked() as f64 + 0.5) * w;
        let transition = if animate { "" } else { "transition:none;" };
        let _ = t.set_attribute("style", &format!("{transition}transform:translateX({x}px)"));
    };
    let html_overflow = |hidden: bool| {
        if let Some(root) = document().document_element() {
            let _ = if hidden { root.set_attribute("style", "overflow:hidden") } else { root.remove_attribute("style") };
        }
    };
    let lb_open_fn = move |_| {
        lb_index.set(N);
        lb_open.set(true);
        html_overflow(true);
        request_animation_frame(move || {
            place(false);
            lb_shown.set(true);
        });
    };
    let lb_close = move || {
        lb_shown.set(false);
        html_overflow(false);
        set_timeout(move || lb_open.set(false), Duration::from_millis(150));
    };
    let lb_step = move |e: ev::MouseEvent, d: i32| {
        e.stop_propagation();
        lb_index.update(|i| *i += d);
        place(true);
    };
    let lb_settle = move |e: ev::TransitionEvent| {
        if e.target() != e.current_target() {
            return;
        }
        let i = lb_index.get_untracked();
        if !(N..2 * N).contains(&i) {
            lb_index.set(N + i.rem_euclid(N));
            place(false);
        }
    };

    Effect::new(move |_| {
        let esc = window_event_listener(ev::keydown, move |e| {
            if e.key() == "Escape" && lb_open.get_untracked() {
                lb_close();
            }
        });
        let resize = window_event_listener(ev::resize, move |_| {
            if lb_open.get_untracked() {
                place(false);
            }
        });
        on_cleanup(move || {
            esc.remove();
            resize.remove();
        });
    });

    view! {
        <div class="relative w-full lg:max-w-[60vw] overflow-hidden lg:overflow-visible">
            <div
                node_ref=track
                class="flex lg:block transition-transform duration-500 lg:!transform-none touch-pan-y"
                style=track_style
                on:pointerdown=on_down
                on:pointermove=on_move
                on:pointerup=on_up
                on:pointercancel=move |_| drag.set(None)
            >
                {SLIDES
                    .iter()
                    .enumerate()
                    .map(|(k, label)| {
                        let k = k as i32;
                        view! {
                            <div class="gal-slide w-full shrink-0" class:active=move || index.get() == k>
                                <div class="ph aspect-[740/760]">{*label}</div>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
            // полоса листания: только телефон и планшет, до 690px по центру
            <div
                node_ref=bar
                class="lg:hidden relative h-[2px] max-w-[690px] mx-auto bg-brown/40 cursor-pointer"
                on:click=on_bar
            >
                <div
                    class="absolute inset-y-0 left-0 w-1/2 bg-black transition-transform duration-500 pointer-events-none"
                    style=move || format!("transform:translateX({}%)", index.get() * 100)
                ></div>
            </div>
            // увеличить: только компьютер, прилипает к низу экрана в 16px от края
            <button
                type="button"
                aria-label="Увеличить фото"
                class="hidden lg:flex sticky bottom-4 left-4 -mt-10 size-6 rounded-full items-center justify-center hover:bg-oatmilk transition z-[2]"
                on:click=lb_open_fn
            >
                <svg class="size-3" viewBox="0 0 15 15" fill="none" aria-hidden="true">
                    <path stroke="currentColor" d="M.5 10v4.5H5M10 14.5h4.5V10M14.5 5V.5H10M5 .5H.5V5" />
                </svg>
            </button>
        </div>

        // Полноэкранный просмотр: затемнение 40% + размытие фона, фото по высоте экрана минус 45px
        // сверху и снизу, соседние видны по краям, по кругу. Закрывается кликом в любом месте, «Закрыть» или Esc.
        <div
            class="fixed inset-0 z-[9999] h-screen py-[45px] bg-black/40 backdrop-blur-[24px] items-center justify-center opacity-0 transition-opacity duration-150"
            class:hidden=move || !lb_open.get()
            class:flex=lb_open
            class:opacity-100=lb_shown
            on:click=move |_| lb_close()
        >
            <div class="w-full h-full relative">
                <div class="overflow-hidden h-full">
                    <div node_ref=lb_track class="flex h-full transition-transform duration-300" on:transitionend=lb_settle>
                        {(0..3)
                            .flat_map(|_| 1..=SLIDES.len())
                            .map(|k| {
                                view! {
                                    <div class="h-full aspect-[740/760] shrink-0">
                                        <div class="ph h-full shadow-[inset_0_0_0_1px_rgba(255,255,255,.6)]">
                                            {format!("Фото {k}")}
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>
                <div class="absolute top-full inset-x-0 p-4 flex justify-center gap-6 items-center z-[2]">
                    // у них 60px под «Close»; у нас шире «Закрыть», чтобы стрелки остались по центру
                    <div class="w-[84px] mr-auto"></div>
                    <button aria-label="Назад" class="size-4 text-white group" on:click=move |e| lb_step(e, -1)>
                        <svg class="size-4 rotate-180 group-hover:scale-110 transition block" viewBox="0 0 12 12" fill="none">
                            <path stroke="currentColor" d="M1 6h9.5M6 1l5 5-5 5" />
                        </svg>
                    </button>
                    <button aria-label="Вперёд" class="size-4 text-white group" on:click=move |e| lb_step(e, 1)>
                        <svg class="size-4 group-hover:scale-110 transition block" viewBox="0 0 12 12" fill="none">
                            <path stroke="currentColor" d="M1 6h9.5M6 1l5 5-5 5" />
                        </svg>
                    </button>
                    <button class="flex items-center gap-[15px] text-white group ml-auto text-[13px] uppercase tracking-[0.01em] leading-[1.2]">
                        "Закрыть"
                        <svg class="size-2.5 group-hover:rotate-180 transition block" viewBox="0 0 10 10" fill="none">
                            <path stroke="currentColor" d="M.5.5l9 9M9.5.5l-9 9" />
                        </svg>
                    </button>
                </div>
            </div>
        </div>
    }
}
