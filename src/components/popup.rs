use std::time::Duration;

use leptos::{ev, prelude::*};

const SEEN_KEY: &str = "atb-popup-seen";

/// Окно подписки: через 3 секунды при первом визите (один раз за визит).
/// До 700px — узкая карточка 320px: текст сверху, фото 5:4 снизу. Шире — две колонки.
#[island]
pub fn SignupPopup() -> impl IntoView {
    let open = RwSignal::new(false);

    Effect::new(move |_| {
        let session = window().session_storage().ok().flatten();
        let seen = session.as_ref().and_then(|s| s.get_item(SEEN_KEY).ok().flatten()).is_some();
        if !seen {
            set_timeout(
                move || {
                    open.set(true);
                    if let Some(s) = session {
                        let _ = s.set_item(SEEN_KEY, "1");
                    }
                },
                Duration::from_secs(3),
            );
        }
        let esc = window_event_listener(ev::keydown, move |e| {
            if e.key() == "Escape" {
                open.set(false);
            }
        });
        on_cleanup(move || esc.remove());
    });

    view! {
        <div
            class="fixed inset-0 z-[70] justify-center overflow-y-auto bg-[rgba(255,251,251,.6)] p-3"
            class:hidden=move || !open.get()
            class:flex=open
            // клик мимо карточки закрывает окно
            on:click=move |e| {
                if e.target() == e.current_target() {
                    open.set(false);
                }
            }
        >
            <div class="relative my-auto bg-white w-[320px] flex flex-col min-[701px]:w-full min-[701px]:max-w-[760px] min-[701px]:grid min-[701px]:grid-cols-2">
                <div class="ph aspect-[5/4] order-last min-[701px]:order-first min-[701px]:aspect-auto min-[701px]:min-h-[340px]">
                    "Фото"
                </div>
                <form class="pt-5 pb-6 min-[701px]:p-6 flex flex-col justify-center" on:submit=move |e| {
                    e.prevent_default();
                    // TODO: отправка в рассылку, когда подключим сервис писем
                    open.set(false);
                }>
                    <button
                        type="button"
                        class="absolute top-4 right-2.5 min-[701px]:right-4"
                        aria-label="Закрыть"
                        on:click=move |_| open.set(false)
                    >
                        <svg width="12" height="12" viewBox="0 0 12 12" stroke="currentColor">
                            <path d="M1 1l10 10M11 1L1 11" />
                        </svg>
                    </button>
                    <h2 class="text-[14px] min-[701px]:text-[13px] font-medium uppercase mb-2 px-2.5 min-[701px]:px-0">
                        "Слоган"
                    </h2>
                    <p class="text-[13px] mb-5 min-[701px]:mb-8 px-2.5 min-[701px]:px-0">
                        "Подпишитесь и получите −10% на первый заказ и ранний доступ к новинкам."
                    </p>
                    <input
                        type="email"
                        name="email"
                        required
                        placeholder="Email"
                        class="border-b border-black py-2 pl-4 min-[701px]:pl-0 text-[14px] outline-none mb-5"
                    />
                    <button class="h-11 border-2 border-black text-[13px] hover:bg-black hover:text-white transition-colors min-[701px]:h-auto min-[701px]:border min-[701px]:self-center min-[701px]:px-10 min-[701px]:py-2 min-[701px]:text-[12px] min-[701px]:uppercase">
                        "Подписаться"
                    </button>
                    <p class="text-[11px] mt-6 px-1 min-[701px]:px-0">
                        "Подписываясь, вы соглашаетесь с "
                        <a href="/legal/privacy" class="underline">"политикой конфиденциальности"</a>
                        "."
                    </p>
                </form>
            </div>
        </div>
    }
}
