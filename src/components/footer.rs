use leptos::prelude::*;

/// Подвал — обычный HTML (не «остров»), поэтому баннер cookies открываем событием окна.
const REOPEN_COOKIES: &str = "window.dispatchEvent(new Event('atb:cookies'))";

const SHOP: &[(&str, &str)] = &[
    ("Смотреть все", "/catalog"),
    ("Клетки", "/catalog/cages"),
    ("Жёрдочки", "/catalog/perches"),
    ("Кормушки и поилки", "/catalog/feeders"),
    ("Игрушки", "/catalog/toys"),
];
const HELP: &[(&str, &str)] = &[
    ("Вопросы и ответы", "/faq"),
    ("Доставка", "/shipping"),
    ("Отследить заказ", "/account/orders"),
    ("Мой аккаунт", "/account"),
    ("Возврат", "/returns"),
];
const ABOUT: &[(&str, &str)] = &[("Подход", "/approach"), ("Контакты", "/contact"), ("Telegram", "#")];
const LEGAL: &[(&str, &str)] = &[
    ("Политика конфиденциальности", "/legal/privacy"),
    ("Пользовательское соглашение", "/legal/terms"),
    ("Политика cookies", "/legal/cookies"),
];

/// Колонка ссылок подвала: заголовок 12px, ссылки с шагом 36px.
#[component]
fn LinkColumn(title: &'static str, links: &'static [(&'static str, &'static str)]) -> impl IntoView {
    view! {
        <div>
            <p class="caps mb-6">{title}</p>
            <ul class="flex flex-col gap-[19px] text-[13px] md:text-[14px] leading-[1.3]">
                {links
                    .iter()
                    .map(|(text, href)| view! { <li><a href=*href class="hover:underline">{*text}</a></li> })
                    .collect_view()}
            </ul>
        </div>
    }
}

/// Подвал. Компьютер: колонки слева через 80px, подписка справа 445px, юридическая строка внизу.
/// Телефон: подписка сверху, ниже сетка 2×2, юридические ссылки — колонкой рядом с «О нас».
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="page-in bg-brown text-white pt-10 lg:pt-20 pb-4 lg:pb-6 px-3 md:px-6">
            <div class="flex flex-col lg:flex-row-reverse lg:justify-between gap-10 lg:gap-12">
                <div class="lg:w-[445px]">
                    <h2 class="caps mb-[9px]">"Подписка"</h2>
                    <p class="text-[14px] md:text-[16px] font-medium uppercase leading-[1.2] mb-10 max-w-[200px] lg:max-w-none">
                        "Подпишитесь на рассылку, чтобы быть в курсе."
                    </p>
                    <form class="flex items-center border-b border-white h-[44px] lg:h-[41px]" action="#">
                        <input
                            type="email"
                            name="email"
                            placeholder="Email"
                            class="bg-transparent flex-1 text-[16px] lg:text-[14px] placeholder:text-white/80 outline-none"
                        />
                        <button aria-label="Подписаться" class="px-1">
                            <svg width="14" height="10" viewBox="0 0 14 10" fill="none" stroke="currentColor">
                                <path d="M0 5h13M9 1l4 4-4 4" />
                            </svg>
                        </button>
                    </form>
                    <p class="text-[14px] leading-[1.3] text-white/80 mt-2.5">
                        "Подписываясь, вы соглашаетесь с "
                        <a href="/legal/privacy" class="underline">"политикой конфиденциальности"</a>
                        "."
                    </p>
                </div>
                <div class="grid grid-cols-2 gap-x-3 gap-y-10 lg:flex lg:gap-20">
                    <LinkColumn title="Магазин" links=SHOP />
                    <LinkColumn title="Заказы и помощь" links=HELP />
                    <LinkColumn title="О нас" links=ABOUT />
                    // юридические ссылки на телефоне: колонкой рядом с «О нас»
                    <ul class="lg:hidden flex flex-col gap-[19px] text-[13px] md:text-[14px] leading-[1.3] pt-10">
                        {LEGAL
                            .iter()
                            .map(|(text, href)| view! { <li><a href=*href class="hover:underline">{*text}</a></li> })
                            .collect_view()}
                        <li><button class="hover:underline" onclick=REOPEN_COOKIES>"Настройки cookies"</button></li>
                    </ul>
                </div>
            </div>
            // юридическая строка: на компьютере ссылки + ©, на телефоне только ©
            <div class="mt-12 lg:mt-[196px] flex flex-wrap items-center gap-x-8 gap-y-2 text-[12px] leading-[1.3] text-white/80">
                {LEGAL
                    .iter()
                    .map(|(text, href)| view! { <a href=*href class="hidden lg:inline hover:underline">{*text}</a> })
                    .collect_view()}
                <button class="hidden lg:inline hover:underline" onclick=REOPEN_COOKIES>"Настройки cookies"</button>
                <span>"©2026 askthebirds"</span>
            </div>
        </footer>
    }
}
