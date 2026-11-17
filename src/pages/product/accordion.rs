//! Раскрывающиеся блоки (описание товара и вопросы): плавная высота 0,25 с,
//! стрелка сменяется минусом. Работают внутри островов.

use leptos::{html, prelude::*};

/// Частые вопросы (заглушки).
const QUESTIONS: [(&str, &str); 2] = [
    ("Подойдёт ли это моему попугаю?", "Ответ на вопрос в несколько строк."),
    ("Безопасны ли материалы для клюва?", "Ответ на вопрос в несколько строк."),
];

/// Состояние одного блока: открыт ли и где его содержимое.
#[derive(Clone, Copy)]
pub struct Acc {
    pub open: RwSignal<bool>,
    pub body: NodeRef<html::Div>,
}

impl Acc {
    pub fn new() -> Self {
        Self { open: RwSignal::new(false), body: NodeRef::new() }
    }

    /// Открыть/закрыть: высота плавно от 0 до содержимого (или обратно).
    pub fn toggle(self) {
        let open = !self.open.get_untracked();
        self.open.set(open);
        let Some(body) = self.body.get_untracked() else { return };
        if open {
            let _ = body.remove_attribute("hidden");
        }
        let full = body.scroll_height();
        let _ = body.set_attribute("style", &format!("height:{}px", if open { 0 } else { full }));
        let _ = body.offset_height(); // применить высоту до смены — иначе перехода не будет
        let _ = body.set_attribute("style", &format!("height:{}px", if open { full } else { 0 }));
    }

    /// Конец перехода: закрытый — спрятать, открытый — высота по содержимому.
    pub fn settle(self) {
        let Some(body) = self.body.get_untracked() else { return };
        if body.get_attribute("style").as_deref() == Some("height:0px") {
            let _ = body.set_attribute("hidden", "");
        } else {
            let _ = body.set_attribute("style", "height:auto");
        }
    }
}

/// Стрелка вниз, при открытии — минус.
#[component]
pub fn AccIcon() -> impl IntoView {
    view! {
        <span class="relative">
            <svg class="ic-chev" width="14" height="13" viewBox="0 0 14 13" fill="none">
                <path d="M13 3.5L7 9.5L1 3.5" stroke="currentColor" />
            </svg>
            <svg class="ic-minus" width="12" height="13" viewBox="0 0 12 13" fill="none">
                <path stroke="currentColor" d="M0 6h12" />
            </svg>
        </span>
    }
}

/// Содержимое блока: 16/18px, отступ 24px сверху.
#[component]
pub fn AccBody(acc: Acc, text: &'static str) -> impl IntoView {
    view! {
        <div class="acc-body" hidden node_ref=acc.body on:transitionend=move |_| acc.settle()>
            <div class="pt-6 text-[16px] md:text-[18px] leading-[1.2]">
                <p>{text}</p>
            </div>
        </div>
    }
}

/// Вопросы на странице товара: строки 22/32px, разделители коричневые 20%.
#[island]
pub fn Faq() -> impl IntoView {
    view! {
        <ul class="border-t border-brown/20">
            {QUESTIONS
                .iter()
                .map(|(q, a)| {
                    let acc = Acc::new();
                    view! {
                        <li class="acc py-[22px] md:py-8 border-b border-brown/20">
                            <h3>
                                <button
                                    type="button"
                                    class="acc-btn h6 w-full flex items-center gap-6 text-left justify-between hover:underline"
                                    aria-expanded=move || acc.open.get().to_string()
                                    on:click=move |_| acc.toggle()
                                >
                                    <span>{*q}</span>
                                    <AccIcon />
                                </button>
                            </h3>
                            <AccBody acc text=a />
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}
