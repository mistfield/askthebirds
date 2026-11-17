use leptos::prelude::*;

/// Кнопка чата как у natureofthings: белый круг 60px, справа 30px / снизу 33px,
/// внутри круглое облачко #333 с тремя точками.
#[component]
pub fn ChatButton() -> impl IntoView {
    view! {
        <button
            aria-label="Открыть чат"
            class="fixed bottom-[33px] right-[30px] z-50 w-[60px] h-[60px] rounded-full bg-white shadow-[0_2px_4px_rgba(22,22,22,.16)] flex items-center justify-center"
        >
            <svg width="28" height="28" viewBox="0 0 56 56" aria-hidden="true">
                <path
                    fill="#333"
                    d="M28 0a28 28 0 1 1-12.4 53.1C10.3 55.3 4.6 56.1.7 56c-.7 0-.9-.9-.3-1.4 3.4-2.4 5.6-5 6.9-7.8A28 28 0 0 1 28 0Z"
                />
                <circle cx="18" cy="28" r="2.5" fill="#F4F4F6" />
                <circle cx="28" cy="28" r="2.5" fill="#F4F4F6" />
                <circle cx="38" cy="28" r="2.5" fill="#F4F4F6" />
            </svg>
        </button>
    }
}
