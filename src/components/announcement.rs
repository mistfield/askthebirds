use std::time::Duration;

use leptos::prelude::*;

const MESSAGES: [(&str, &str); 2] = [
    ("Сообщение 1: новость", "#"),
    ("Сообщение 2: −10% на первый заказ за подписку", "#"),
];

/// Плашка сверху (32px): сообщения сменяют друг друга снизу вверх каждые 4 секунды.
#[island]
pub fn AnnouncementBar() -> impl IntoView {
    let current = RwSignal::new(0usize);

    Effect::new(move |_| {
        if let Ok(handle) = set_interval_with_handle(
            move || current.update(|i| *i = (*i + 1) % MESSAGES.len()),
            Duration::from_secs(4),
        ) {
            on_cleanup(move || handle.clear());
        }
    });

    view! {
        <div class="bg-brown text-white h-8 overflow-hidden relative">
            {MESSAGES
                .iter()
                .enumerate()
                .map(|(i, (text, href))| {
                    // текущее — на месте; следующее ждёт снизу; предыдущее уходит вверх
                    let pos = move || {
                        let cur = current.get();
                        if i == cur {
                            "translate-y-0 opacity-100"
                        } else if i == (cur + 1) % MESSAGES.len() {
                            "translate-y-full opacity-0"
                        } else {
                            "-translate-y-full opacity-0"
                        }
                    };
                    view! {
                        <a
                            href=*href
                            class=move || {
                                format!(
                                    "absolute inset-0 flex items-center justify-center text-[12px] uppercase tracking-[0.01em] underline underline-offset-2 transition-[transform,opacity] duration-[600ms] {}",
                                    pos(),
                                )
                            }
                        >
                            {*text}
                        </a>
                    }
                })
                .collect_view()}
        </div>
    }
}
