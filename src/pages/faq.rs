use leptos::prelude::*;
use leptos_meta::Title;

/// Тело ответа: один абзац, два абзаца или абзац со ссылкой.
#[derive(Clone, Copy)]
enum Answer {
    One(&'static str),
    Two(&'static str, &'static str),
    Link(&'static str),
    Lists,
}

/// Раздел и его вопросы (тексты — заглушки из wireframe/faq.html).
struct Group {
    id: &'static str,
    title: &'static str,
    items: &'static [(&'static str, Answer)],
}

const STD: &str = "Текст ответа — заглушка, примерно такого же объёма, как у оригинала: один абзац в одну–две строки.";

const GROUPS: &[Group] = &[
    Group {
        id: "r1",
        title: "Раздел 1",
        items: &[
            ("Вопрос 1.1 — заглушка?", Answer::Two("Текст ответа — заглушка, первый абзац того же объёма.", "Второй абзац — заглушка.")),
            ("Вопрос 1.2 — заглушка?", Answer::One(STD)),
        ],
    },
    Group {
        id: "r2",
        title: "Раздел 2",
        items: &[
            ("Вопрос 2.1 — заглушка?", Answer::One(STD)),
            ("Вопрос 2.2 — заглушка?", Answer::One(STD)),
            ("Вопрос 2.3 — заглушка?", Answer::Two("Текст ответа — заглушка, первый абзац того же объёма.", "Второй абзац — заглушка.")),
            ("Вопрос 2.4 — заглушка?", Answer::One(STD)),
            ("Вопрос 2.5 — заглушка?", Answer::One(STD)),
            ("Вопрос 2.6 — заглушка?", Answer::One(STD)),
        ],
    },
    Group {
        id: "r3",
        title: "Раздел 3",
        items: &[
            ("Вопрос 3.1 — заглушка?", Answer::One(STD)),
            ("Вопрос 3.2 — заглушка?", Answer::Link("Текст со ссылкой — заглушка. Подробнее ")),
            ("Вопрос 3.3 — заглушка?", Answer::One(STD)),
            ("Вопрос 3.4 — заглушка?", Answer::One(STD)),
        ],
    },
    Group {
        id: "r4",
        title: "Раздел 4",
        items: &[
            ("Вопрос 4.1 — заглушка?", Answer::One(STD)),
            ("Вопрос 4.2 — заглушка?", Answer::One(STD)),
            ("Вопрос 4.3 — заглушка?", Answer::Link("Текст со ссылкой — заглушка. Подробнее ")),
        ],
    },
    Group {
        id: "r5",
        title: "Раздел 5",
        items: &[
            ("Вопрос 5.1 — заглушка?", Answer::Lists),
            ("Вопрос 5.2 — заглушка?", Answer::One(STD)),
            ("Вопрос 5.3 — заглушка?", Answer::One(STD)),
            ("Вопрос 5.4 — заглушка?", Answer::Link("Текст со ссылкой — заглушка. Подробнее ")),
            ("Вопрос 5.5 — заглушка?", Answer::One(STD)),
            ("Вопрос 5.6 — заглушка?", Answer::Two("Текст ответа — заглушка, первый абзац того же объёма.", "Второй абзац — заглушка.")),
            ("Вопрос 5.7 — заглушка?", Answer::One(STD)),
        ],
    },
    Group {
        id: "r6",
        title: "Раздел 6",
        items: &[("Вопрос 6.1 — заглушка?", Answer::Link("Текст со ссылкой — заглушка. Подробнее "))],
    },
];

/// Тело ответа в вайрфрейме: отступ сверху 24/32px, текст 16/18px.
#[component]
fn AnswerBody(answer: Answer) -> impl IntoView {
    let body = match answer {
        Answer::One(text) => view! { <p>{text}</p> }.into_any(),
        Answer::Two(first, second) => view! {
            <p class="mb-4">{first}</p>
            <p>{second}</p>
        }
        .into_any(),
        Answer::Link(text) => view! {
            <p>{text}<a href="#" class="underline">"здесь"</a>"."</p>
        }
        .into_any(),
        Answer::Lists => view! {
            <p class="mb-2"><strong class="font-medium">"Подзаголовок — заглушка:"</strong></p>
            <ul class="list-disc pl-5 space-y-1">
                <li>"Пункт списка — заглушка."</li>
                <li>"Пункт списка — заглушка."</li>
                <li>"Пункт списка — заглушка."</li>
            </ul>
        }
        .into_any(),
    };
    view! {
        <div class="pt-6 text-[16px] md:text-[18px] leading-[1.2]">{body}</div>
    }
}

/// Вопросы и ответы: раскрытие — нативный <details>, без JavaScript.
#[component]
pub fn FaqPage() -> impl IntoView {
    view! {
        <Title text="Вопросы и ответы" />
        <section class="px-3 md:px-6 pt-20 pb-20 md:pt-[100px] md:pb-[100px]">
            <h1 class="text-[26px] md:text-[40px] font-medium uppercase leading-[1.1]">"Вопросы и ответы"</h1>
        </section>

        // Вопросы: слева оглавление (3 колонки, 1/3), справа раскрывающиеся ответы (2/3). Телефон: оглавление сверху.
        <div class="reveal md:grid md:grid-cols-3 md:gap-[12px] px-3 md:px-6 md:mt-14">
            <nav class="md:col-span-1 pb-4 md:pb-0">
                <ul class="flex flex-col gap-4 text-[12px] leading-[1.2] md:sticky md:top-[100px]">
                    {GROUPS
                        .iter()
                        .map(|g| {
                            view! {
                                <li>
                                    <a href=format!("#{}", g.id) class="block uppercase tracking-[0.01em] hover:underline">{g.title}</a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </nav>
            <div class="mt-8 md:mt-0 md:col-span-2 md:ml-auto w-full max-w-[924px]">
                {GROUPS
                    .iter()
                    .map(|g| {
                        view! {
                            <section id=g.id class="reveal scroll-mt-20 md:scroll-mt-[100px] py-8 md:py-14 first:pt-0">
                                <h2 class="h3">{g.title}</h2>
                                <ul class="mt-6 md:mt-10 border-t border-brown/20">
                                    {g.items
                                        .iter()
                                        .map(|(question, answer)| {
                                            view! {
                                                <li class="py-[22px] md:py-8 border-b border-brown/20">
                                                    <details class="group">
                                                        <summary class="w-full flex items-center justify-between gap-6 text-left uppercase text-[12px] md:text-[13px] leading-[1.2] tracking-[0.01em] hover:underline cursor-pointer list-none [&::-webkit-details-marker]:hidden">
                                                            <h3 class="font-normal">{*question}</h3>
                                                            <svg class="shrink-0 transition-transform duration-200 group-open:rotate-180" width="14" height="13" viewBox="0 0 14 13" fill="none" stroke="currentColor" aria-hidden="true">
                                                                <path d="M13 3.5L7 9.5L1 3.5" />
                                                            </svg>
                                                        </summary>
                                                        <AnswerBody answer=*answer />
                                                    </details>
                                                </li>
                                            }
                                        })
                                        .collect_view()}
                                </ul>
                            </section>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
