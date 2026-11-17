use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

/// Какая из пяти «страниц» аккаунта показана — определяется адресом.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum View {
    Login,
    Register,
    Orders,
    OrdersEmpty,
    Profile,
}

impl View {
    /// `/account` — вход; `/account/register`, `/account/orders`, `/account/orders-empty`, `/account/profile`.
    fn from_param(param: Option<&str>) -> Self {
        match param {
            Some("register") => View::Register,
            Some("orders") => View::Orders,
            Some("orders-empty") => View::OrdersEmpty,
            Some("profile") => View::Profile,
            _ => View::Login,
        }
    }

    fn title(self) -> &'static str {
        match self {
            View::Login => "Вход",
            View::Register => "Регистрация",
            View::Orders | View::OrdersEmpty => "Заказы",
            View::Profile => "Профиль",
        }
    }
}

const INPUT: &str = "border-b border-black py-2 text-[14px] outline-none";
const SUBMIT: &str = "w-full px-10 pt-3 pb-[14px] border border-brown bg-brown text-white leading-none text-[12px] uppercase tracking-[0.01em] transition-colors hover:bg-black hover:border-black";

/// Аккаунт: вход, регистрация, заказы, «заказов нет», профиль.
/// Настоящего входа пока нет: формы ничего не отправляют, внутренние страницы — заглушки.
#[component]
pub fn AccountPage() -> impl IntoView {
    let params = use_params_map();
    let view_of = move || View::from_param(params.read().get("view").as_deref());

    view! {
        <Title text=move || view_of().title() />
        <div class="acct">
            {move || match view_of() {
                View::Login => view! { <Login /> }.into_any(),
                View::Register => view! { <Register /> }.into_any(),
                v => view! { <Inside view=v /> }.into_any(),
            }}
        </div>
    }
}

// Вход. НЕ СВЕРЕНО (как и в вайрфрейме): форму входа без входа в аккаунт у них увидеть не удалось.
#[component]
fn Login() -> impl IntoView {
    view! {
        <section class="px-3 md:px-6 py-16 md:py-[100px]">
            <div class="bg-white mx-auto w-full max-w-[440px] p-6 md:p-10">
                <h1 class="text-[22px] md:text-[26px] leading-[1.1] uppercase font-medium text-center">"Вход"</h1>
                <p class="text-[13px] md:text-[14px] text-center mt-3">"Введите email — мы отправим код для входа."</p>
                <form class="mt-8 flex flex-col gap-6" onsubmit="return false">
                    <input type="email" placeholder="Email" class=INPUT />
                    <button class=SUBMIT>"Продолжить"</button>
                </form>
                <p class="text-[12px] text-center mt-6">
                    "Нет аккаунта? " <a href="/account/register" class="underline">"Создать"</a>
                </p>
            </div>
        </section>
    }
}

#[component]
fn Register() -> impl IntoView {
    view! {
        <section class="px-3 md:px-6 py-16 md:py-[100px]">
            <div class="bg-white mx-auto w-full max-w-[440px] p-6 md:p-10">
                <h1 class="text-[22px] md:text-[26px] leading-[1.1] uppercase font-medium text-center">"Регистрация"</h1>
                <p class="text-[13px] md:text-[14px] text-center mt-3">"Следите за заказами и оформляйте покупки быстрее."</p>
                <form class="mt-8 flex flex-col gap-6" onsubmit="return false">
                    <input type="text" placeholder="Имя" class=INPUT />
                    <input type="text" placeholder="Фамилия" class=INPUT />
                    <input type="email" placeholder="Email" class=INPUT />
                    <label class="flex items-start gap-2 text-[12px]">
                        <input type="checkbox" class="mt-0.5" />
                        " Подписаться на рассылку"
                    </label>
                    <button class=SUBMIT>"Создать аккаунт"</button>
                </form>
                <p class="text-[12px] text-center mt-6">
                    "Уже есть аккаунт? " <a href="/account" class="underline">"Войти"</a>
                </p>
            </div>
        </section>
    }
}

// Страницы после входа: слева меню «Заказы / Профиль», справа содержимое. Заглушки.
#[component]
fn Inside(view: View) -> impl IntoView {
    view! {
        <div class="px-3 md:px-6 py-10 md:py-[60px] md:grid md:grid-cols-[200px_1fr] gap-10 max-w-[1100px] mx-auto">
            <nav class="flex md:flex-col gap-6 md:gap-3 mb-8 md:mb-0 text-[14px]">
                <a href="/account/orders" class="hover:underline">"Заказы"</a>
                <a href="/account/profile" class="hover:underline">"Профиль"</a>
                <a href="/account" class="hover:underline md:mt-6">"Выйти"</a>
            </nav>
            {match view {
                View::Orders => view! { <Orders /> }.into_any(),
                View::OrdersEmpty => view! { <OrdersEmpty /> }.into_any(),
                _ => view! { <Profile /> }.into_any(),
            }}
        </div>
    }
}

#[component]
fn Orders() -> impl IntoView {
    view! {
        <div>
            <h1 class="text-[18px] font-medium uppercase mb-6">"Заказы"</h1>
            <ul class="flex flex-col gap-3">
                <OrderRow number="0001" note="12 ноября · Доставлен" sum="4\u{a0}650\u{a0}₽" />
                <OrderRow number="0002" note="2 октября · В пути" sum="1\u{a0}450\u{a0}₽" />
            </ul>
        </div>
    }
}

#[component]
fn OrderRow(number: &'static str, note: &'static str, sum: &'static str) -> impl IntoView {
    view! {
        <li class="bg-white p-4 md:p-6 flex gap-4 items-center">
            <div class="ph w-14 h-16 shrink-0"></div>
            <div class="flex-1 text-[13px] leading-[1.4]">
                <b class="font-medium">{format!("Заказ № {number}")}</b>
                <br />
                {note}
            </div>
            <div class="text-[13px]">{sum}</div>
        </li>
    }
}

// У них мельком виден пустой экран заказов («Добро пожаловать, пора за покупками»). НЕ СВЕРЕНО.
#[component]
fn OrdersEmpty() -> impl IntoView {
    view! {
        <div>
            <h1 class="text-[18px] font-medium uppercase mb-6">"Заказы"</h1>
            <div class="bg-white p-6 md:p-10 max-w-[760px]">
                <h2 class="text-[14px] font-medium">"Добро пожаловать"</h2>
                <p class="text-[14px] text-neutral-600 mb-6">"Пора за покупками?"</p>
                <a href="/catalog" class="btn mb-8">"В магазин"</a>
                <div class="grid grid-cols-3 md:grid-cols-5 gap-3">
                    <div class="ph aspect-[2/3]"></div>
                    <div class="ph aspect-[2/3]"></div>
                    <div class="ph aspect-[2/3]"></div>
                    <div class="ph aspect-[2/3] hidden md:flex"></div>
                    <div class="ph aspect-[2/3] hidden md:flex"></div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Profile() -> impl IntoView {
    view! {
        <div>
            <h1 class="text-[18px] font-medium uppercase mb-6">"Профиль"</h1>
            <form class="bg-white p-6 md:p-10 max-w-[560px] flex flex-col gap-6" onsubmit="return false">
                <input type="text" placeholder="Имя" class=INPUT />
                <input type="text" placeholder="Фамилия" class=INPUT />
                <input type="email" placeholder="Email" class=INPUT />
                <input type="tel" placeholder="Телефон" class=INPUT />
                <button class="btn self-start">"Сохранить"</button>
            </form>
        </div>
    }
}
