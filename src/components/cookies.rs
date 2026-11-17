use leptos::prelude::*;

/// Выбор посетителя: "all" или "necessary". Пока выбора нет — баннер показывается.
pub const CONSENT_KEY: &str = "atb-cookies";
/// Событие окна, которое снова открывает баннер (кнопка «Настройки cookies» в подвале).
pub const OPEN_EVENT: &str = "atb:cookies";

/// Баннер cookies внизу на всю ширину, поверх окна подписки.
#[island]
pub fn CookieBanner() -> impl IntoView {
    let open = RwSignal::new(false);

    let storage = || window().local_storage().ok().flatten();
    let choose = move |value: &'static str| {
        if let Some(s) = storage() {
            let _ = s.set_item(CONSENT_KEY, value);
        }
        open.set(false);
    };

    Effect::new(move |_| {
        let chosen = storage().and_then(|s| s.get_item(CONSENT_KEY).ok().flatten()).is_some();
        open.set(!chosen);
        let h = window_event_listener_untyped(OPEN_EVENT, move |_| open.set(true));
        on_cleanup(move || h.remove());
    });

    view! {
        <div class="fixed bottom-0 inset-x-0 z-[75] bg-oatmilk px-3 lg:px-6 py-5" class:hidden=move || !open.get()>
            <h2 class="text-[14px] font-medium mb-2">"Этот сайт использует cookies"</h2>
            <p class="text-[12px] mb-4">"Мы используем cookies, чтобы сайт работал лучше."</p>
            <div class="flex flex-wrap gap-2">
                <button class="bg-brown text-white text-[12px] uppercase px-3 py-1.5" on:click=move |_| choose("all")>
                    "Разрешить все"
                </button>
                <button class="bg-brown text-white text-[12px] uppercase px-3 py-1.5" on:click=move |_| choose("necessary")>
                    "Только необходимые"
                </button>
                <a href="/legal/cookies" class="bg-brown text-white text-[12px] uppercase px-3 py-1.5">"Настройки"</a>
            </div>
        </div>
    }
}
