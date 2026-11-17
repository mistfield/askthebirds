use leptos::{ev, html, prelude::*, web_sys};
use leptos_meta::Title;

fn clamp(v: f64) -> f64 {
    v.clamp(0.0, 1.0)
}

/// Страница «Подход».
#[component]
pub fn ApproachPage() -> impl IntoView {
    view! {
        <Title text="Подход" />
        <ApproachBody />
    }
}

/// Всё содержимое страницы — один остров: анимации при прокрутке считаются в браузере.
/// Первый экран проявляется за 1 с и при прокрутке тает (до размытия 4px); текстовые блоки
/// появляются снизу из размытия при входе в экран и тают, уезжая вверх на 100px.
#[island]
fn ApproachBody() -> impl IntoView {
    let hero_wrap = NodeRef::<html::Div>::new();
    // блоки, которые тают при уходе вверх (последний блок не тает)
    let leave = [
        NodeRef::<html::Section>::new(),
        NodeRef::<html::Section>::new(),
        NodeRef::<html::Section>::new(),
    ];
    let last = NodeRef::<html::Section>::new();
    let sections = [leave[0], leave[1], leave[2], last];

    let hero_in = RwSignal::new(false);
    let hero_p = RwSignal::new(0.0_f64);
    let leave_p = [RwSignal::new(0.0_f64), RwSignal::new(0.0_f64), RwSignal::new(0.0_f64)];
    let shown = [RwSignal::new(false), RwSignal::new(false), RwSignal::new(false), RwSignal::new(false)];

    // нижний и верхний край блока относительно окна (без getBoundingClientRect — его нет в подключённых
    // возможностях web-sys); сдвиг `shift` — на сколько блок уже уехал вверх от прокрутки
    let edges = |el: &web_sys::HtmlElement, scroll: f64, shift: f64| {
        let top = f64::from(el.offset_top()) - scroll + shift;
        (top, top + f64::from(el.offset_height()))
    };

    let update = move || {
        let w = window();
        let vh = w.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(800.0);
        let scroll = w.scroll_y().unwrap_or(0.0);
        if let Some(el) = hero_wrap.get_untracked() {
            let (_, bottom) = edges(&el, scroll, 0.0);
            hero_p.set(clamp((vh * 0.5 - bottom) / (vh * 0.5)));
        }
        for (i, r) in leave.iter().enumerate() {
            if let Some(el) = r.get_untracked() {
                let (_, bottom) = edges(&el, scroll, -100.0 * leave_p[i].get_untracked());
                leave_p[i].set(clamp((vh * 0.25 - bottom) / (vh * 0.25)));
            }
        }
        for (i, r) in sections.iter().enumerate() {
            if shown[i].get_untracked() {
                continue;
            }
            if let Some(el) = r.get_untracked() {
                let (top, bottom) = edges(&el, scroll, 0.0);
                if top < vh && bottom > 0.0 {
                    shown[i].set(true);
                }
            }
        }
    };

    Effect::new(move |_| {
        hero_in.set(true);
        update();
        let s = window_event_listener(ev::scroll, move |_| update());
        let r = window_event_listener(ev::resize, move |_| update());
        on_cleanup(move || {
            s.remove();
            r.remove();
        });
    });

    // стиль таящего блока: прозрачность, сдвиг вверх и размытие
    let leave_style = move |p: RwSignal<f64>| {
        move || {
            let p = p.get();
            format!("opacity:{};transform:translateY({}px);filter:blur({}px)", 1.0 - p, -100.0 * p, 4.0 * p)
        }
    };

    view! {
        // 1. Первый экран: фото на 100vh (минимум 600px), шапка лежит поверх
        <div
            node_ref=hero_wrap
            class="relative -mt-14 transition-opacity duration-1000 ease-linear"
            class=("opacity-0", move || !hero_in.get())
            class=("opacity-100", move || hero_in.get())
        >
            <div
                class="relative h-screen min-h-[600px]"
                style=move || {
                    let p = hero_p.get();
                    format!("opacity:{};filter:blur({}px)", 1.0 - p, 4.0 * p)
                }
            >
                <div class="ph absolute inset-0 !items-start !justify-start pt-24 px-6">
                    "Фото на весь экран (на телефоне — отдельное вертикальное, 2:3)"
                </div>
                // затемнение сверху, чтобы читалась шапка
                <div class="absolute top-0 inset-x-0 h-[130px] bg-linear-to-b from-black to-transparent opacity-15"></div>
                // затемнение снизу
                <div class="absolute bottom-0 inset-x-0 h-[218px] md:h-[223px] bg-linear-to-t from-black/25 to-transparent"></div>
            </div>
        </div>

        // 2. Текст «Подход»: колонка 450px слева
        <section node_ref=leave[0] class="relative overflow-clip py-14 md:py-[70px]" style=leave_style(leave_p[0])>
            <div class="px-3 md:px-6 lg:px-[50px]">
                <div class="max-w-[450px] mr-auto text-left">
                    <div class="mt-6">
                        <h3
                            class="rise-md h3 mb-4 md:mb-6"
                            class=("in", move || shown[0].get())
                        >
                            "Подход"
                        </h3>
                        <p
                            class="rise-md text-[16px] md:text-[18px] leading-[1.2] mb-4 md:mb-6"
                            style="transition-delay:0.1s"
                            class=("in", move || shown[0].get())
                        >
                            "Мы проводим три уровня проверки каждого товара вместе с поставщиками материалов. Так появляется наша оценка безопасности. Мы изучаем состав изделий с независимой лабораторией, чтобы понять, из чего они сделаны. Так появляется оценка натуральности. Оба числа мы публикуем открыто, чтобы вы ясно понимали, что именно покупаете. Текст-заглушка."
                        </p>
                    </div>
                </div>
            </div>
        </section>

        // 3. Два фото рядом: 1 колонка до 768px, 2 колонки дальше; пропорция 1376×2476
        <section node_ref=leave[1] class="relative overflow-clip py-10 md:py-[70px]" style=leave_style(leave_p[1])>
            <PhotoPair shown=shown[1] />
        </section>

        // 4. Текст «Упаковка»
        <section node_ref=leave[2] class="relative overflow-clip py-14 md:py-[70px]" style=leave_style(leave_p[2])>
            <div class="px-3 md:px-6 lg:px-[50px]">
                <div class="max-w-[450px] mr-auto text-left">
                    <div class="mt-6">
                        <h3
                            class="rise-md h3 mb-4 md:mb-6"
                            class=("in", move || shown[2].get())
                        >
                            "Упаковка"
                        </h3>
                        <p
                            class="rise-md text-[16px] md:text-[18px] leading-[1.2]"
                            style="transition-delay:0.1s"
                            class=("in", move || shown[2].get())
                        >
                            "Мы думаем об экологичности на каждом этапе разработки. Упаковка перерабатываемая и сделана ответственно. Текст-заглушка того же объёма."
                        </p>
                    </div>
                </div>
            </div>
        </section>

        // 5. Ещё два фото (последний блок — не тает при прокрутке)
        <section node_ref=last class="relative overflow-clip py-10 md:py-[70px]">
            <PhotoPair shown=shown[3] />
        </section>
    }
}

#[component]
fn PhotoPair(shown: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="max-w-[1440px] mx-auto px-8 md:px-[50px] grid gap-6 md:grid-cols-2">
            <div class="rise mx-auto w-full max-w-[660px]" class=("in", move || shown.get())>
                <div class="ph w-full aspect-[1376/2476]">"Фото 1376×2476"</div>
            </div>
            <div class="rise mx-auto w-full max-w-[660px]" style="transition-delay:0.1s" class=("in", move || shown.get())>
                <div class="ph w-full aspect-[1376/2476]">"Фото 1376×2476"</div>
            </div>
        </div>
    }
}
