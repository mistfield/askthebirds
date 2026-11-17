//! Шапка и всё, что из неё открывается: мега-меню, поиск, корзина-шторка, мобильное меню.
//! Один «остров», потому что эти панели управляются из шапки и закрывают друг друга.

use leptos::{ev, html, prelude::*};

use crate::cart::{self, Line};
use crate::data::{CATEGORIES, Category, PRODUCTS, SUBCATEGORIES, rub};

/// Какой пункт мега-меню открыт: "shop" или slug категории.
type MenuKey = Option<&'static str>;

#[island]
pub fn Header() -> impl IntoView {
    let scrolled = RwSignal::new(false);
    let hovering = RwSignal::new(false);
    let menu = RwSignal::new(None::<&'static str>);
    let search_open = RwSignal::new(false);
    let cart_open = RwSignal::new(false);
    let mobile_open = RwSignal::new(false);
    let lines = RwSignal::new(Vec::<Line>::new());

    let close_all = move || {
        menu.set(None);
        search_open.set(false);
        cart_open.set(false);
        mobile_open.set(false);
    };

    // Только в браузере: прокрутка, Esc, события корзины от других островов.
    Effect::new(move |_| {
        let update = move || scrolled.set(window().scroll_y().unwrap_or(0.0) > 10.0);
        update();
        lines.set(cart::load());
        let h1 = window_event_listener(ev::scroll, move |_| update());
        let h2 = window_event_listener(ev::keydown, move |e| {
            if e.key() == "Escape" {
                close_all();
            }
        });
        let h3 = window_event_listener_untyped(cart::CART_EVENT, move |_| lines.set(cart::load()));
        let h4 = window_event_listener_untyped("atb:cart-open", move |_| {
            lines.set(cart::load());
            menu.set(None);
            search_open.set(false);
            cart_open.set(true);
        });
        on_cleanup(move || {
            h1.remove();
            h2.remove();
            h3.remove();
            h4.remove();
        });
    });

    let solid = move || scrolled.get() || hovering.get() || menu.get().is_some() || search_open.get();
    let cart_count = move || cart::count(&lines.get());

    let search_input = NodeRef::<html::Input>::new();
    let toggle_search = move |_| {
        menu.set(None);
        search_open.update(|o| *o = !*o);
        if search_open.get_untracked() {
            request_animation_frame(move || {
                if let Some(input) = search_input.get_untracked() {
                    let _ = input.focus();
                }
            });
        }
    };
    let open_cart = move |_| {
        lines.set(cart::load());
        menu.set(None);
        search_open.set(false);
        cart_open.set(true);
    };

    view! {
        <header
            class="page-in sticky top-0 z-50 py-3 px-3 lg:px-6 flex items-center gap-6 transition-colors duration-300"
            class:bg-white=solid
            on:mouseenter=move |_| hovering.set(true)
            on:mouseleave=move |_| {
                hovering.set(false);
                menu.set(None);
            }
        >
            // слева: «Магазин» и категории; у кого есть меню — открывают его при наведении
            <nav class="hidden lg:flex items-center gap-6 flex-1 min-w-0 [&>button]:inline-flex [&>button]:items-center [&>button]:text-left">
                <MenuTrigger key="shop" label="Магазин" menu />
                {CATEGORIES
                    .iter()
                    .map(|c| {
                        if c.has_menu {
                            view! { <MenuTrigger key=c.slug label=c.name menu /> }.into_any()
                        } else {
                            view! {
                                <a href=format!("/catalog/{}", c.slug) class="nav" on:mouseenter=move |_| menu.set(None)>
                                    {c.name}
                                </a>
                            }
                                .into_any()
                        }
                    })
                    .collect_view()}
            </nav>
            <div class="lg:hidden flex-1">
                <button class="nav" on:click=move |_| mobile_open.set(true)>"Меню"</button>
            </div>

            // центр: логотип — виден, только когда шапка белая
            <a
                href="/"
                class="block shrink-0 w-[92px] lg:w-[122px] transition-opacity duration-300"
                class:opacity-0=move || !solid()
            >
                <img src="/logo.svg" alt="askthebirds" class="block w-full h-auto" />
            </a>

            // справа: текстом, без иконок
            <nav class="flex items-center justify-end gap-6 flex-1 min-w-0" on:mouseenter=move |_| menu.set(None)>
                <a href="/approach" class="nav hidden lg:inline-block">"Подход"</a>
                <button class="nav hidden lg:inline-block" class:border-current=search_open on:click=toggle_search>
                    "Поиск"
                </button>
                <a href="/account" class="nav hidden lg:inline-block">"Аккаунт"</a>
                <button class="nav" on:click=open_cart>
                    "Корзина"
                    {move || (cart_count() > 0).then(|| format!(" ({})", cart_count()))}
                </button>
            </nav>

            <MegaMenu menu />

            // поиск: панель под шапкой, страница затемняется
            <div class="absolute left-0 right-0 top-full bg-white px-6 py-10" class:hidden=move || !search_open.get()>
                <form action="/catalog" method="get" class="grid grid-cols-2 items-center gap-6">
                    <h2 class="text-[13px] uppercase tracking-[0.01em]">"Поиск по сайту"</h2>
                    <label class="flex items-center border-b border-black">
                        <input
                            node_ref=search_input
                            type="search"
                            name="q"
                            placeholder="Поиск"
                            class="flex-1 py-2 text-[14px] outline-none placeholder:text-neutral-500"
                        />
                        <SearchIcon />
                    </label>
                </form>
            </div>
        </header>

        // затемнение: у корзины ложится и на шапку, у поиска — нет
        <div
            class="fixed inset-0 bg-black/40 transition-opacity duration-300"
            class=(["opacity-100", "pointer-events-auto"], move || search_open.get() || cart_open.get())
            class=(["opacity-0", "pointer-events-none"], move || !(search_open.get() || cart_open.get()))
            class=("z-[55]", move || cart_open.get())
            class=("z-40", move || !cart_open.get())
            on:click=move |_| close_all()
        ></div>

        <CartDrawer open=cart_open lines />
        <MobileMenu open=mobile_open />
    }
}

/// Пункт шапки, который открывает мега-меню при наведении.
#[component]
fn MenuTrigger(key: &'static str, label: &'static str, menu: RwSignal<MenuKey>) -> impl IntoView {
    view! {
        <button
            class="nav"
            aria-expanded=move || (menu.get() == Some(key)).to_string()
            on:mouseenter=move |_| menu.set(Some(key))
            on:click=move |_| menu.update(|m| *m = if *m == Some(key) { None } else { Some(key) })
        >
            {label}
            <span class="chev"></span>
        </button>
    }
}

/// Мега-меню на всю ширину: слева «Смотреть все / Хиты продаж», в центре список, справа фото.
#[component]
fn MegaMenu(menu: RwSignal<MenuKey>) -> impl IntoView {
    let content = move || {
        let key = menu.get()?;
        let (all_label, all_href, title, items): (&str, String, &str, Vec<(String, String)>) = if key == "shop" {
            (
                "Смотреть все",
                "/catalog".into(),
                "По категориям",
                CATEGORIES.iter().map(|c| (c.name.to_string(), format!("/catalog/{}", c.slug))).collect(),
            )
        } else {
            let c: &Category = CATEGORIES.iter().find(|c| c.slug == key)?;
            (
                c.all_label,
                format!("/catalog/{}", c.slug),
                "По типу",
                SUBCATEGORIES.iter().map(|s| (s.to_string(), format!("/catalog/{}", c.slug))).collect(),
            )
        };
        Some(view! {
            <div class="grid grid-cols-12 gap-6">
                <div class="col-span-3 flex flex-col gap-3">
                    <a href=all_href.clone() class="text-[13px] uppercase tracking-[0.01em] hover:underline">{all_label}</a>
                    <a href=format!("{all_href}?sort=bestsellers") class="text-[13px] uppercase tracking-[0.01em] hover:underline">
                        "Хиты продаж"
                    </a>
                </div>
                <div class="col-span-3 flex flex-col gap-3">
                    <div class="caps text-neutral-400 mb-2">{title}</div>
                    {items
                        .into_iter()
                        .map(|(text, href)| view! { <a href=href class="text-[14px] hover:underline">{text}</a> })
                        .collect_view()}
                </div>
                <div class="col-span-6 relative">
                    <div class="ph aspect-[16/9] !items-start !justify-start p-4">"Фото"</div>
                    <a href=all_href class="btn btn-light absolute bottom-6 left-1/2 -translate-x-1/2">"Смотреть все"</a>
                </div>
            </div>
        })
    };
    view! {
        <div class="absolute left-0 right-0 top-full bg-white px-6 pt-8 pb-6" class:hidden=move || menu.get().is_none()>
            {content}
        </div>
    }
}

/// Корзина-шторка справа (540px): пустая — фото и «Хиты продаж», с товарами — список, количество, итог.
#[component]
fn CartDrawer(open: RwSignal<bool>, lines: RwSignal<Vec<Line>>) -> impl IntoView {
    let update = move |slug: &'static str, qty: u32| {
        let next = cart::set_qty(lines.get_untracked(), slug, qty);
        cart::save(&next, false);
        lines.set(next);
    };
    let add = move |slug: &'static str| {
        let current = lines.get_untracked().iter().find(|l| l.product.slug == slug).map_or(0, |l| l.qty);
        update(slug, current + 1);
    };
    let count = move || cart::count(&lines.get());
    let total = move || rub(cart::total(&lines.get()));

    view! {
        <aside
            // закрытая шторка ещё и невидима: не попадает под Tab и не даёт «люфт» вбок на iPhone
            class="fixed top-0 right-0 bottom-0 w-full max-w-[540px] bg-white z-[60] transition-[transform,visibility] duration-300 flex flex-col px-6 py-5"
            class=(["translate-x-full", "invisible"], move || !open.get())
            aria-label="Корзина"
            aria-hidden=move || (!open.get()).to_string()
        >
            <Show
                when=move || !lines.get().is_empty()
                fallback=move || {
                    view! {
                        <div class="flex flex-col flex-1">
                            <div class="flex items-center justify-between mb-5">
                                <h2 class="text-[13px] font-medium uppercase">"Корзина пуста"</h2>
                                <CloseButton on_close=move || open.set(false) />
                            </div>
                            <div class="relative">
                                <div class="ph aspect-[16/9] !items-start !justify-start p-4">"Фото"</div>
                                <a href="/catalog?sort=bestsellers" class="btn btn-light absolute bottom-6 left-1/2 -translate-x-1/2 whitespace-nowrap">
                                    "Хиты продаж"
                                </a>
                            </div>
                            <div class="mt-auto">
                                <Suggestions add />
                                <div class="flex justify-between border-t border-neutral-200 pt-4 mb-4 text-[12px] uppercase">
                                    <span>"Доставка"</span>
                                    <span>"Рассчитывается при оформлении"</span>
                                </div>
                                <button disabled class="w-full h-10 bg-brown/20 text-white text-[12px] uppercase">
                                    "Оформить заказ · 0 ₽"
                                </button>
                            </div>
                        </div>
                    }
                }
            >
                <div class="flex flex-col flex-1 min-h-0 -mx-3 -my-2 md:mx-0 md:my-1">
                    <div class="flex items-center justify-between gap-2">
                        <h2 class="text-[13px] md:text-[14px] leading-[1.2] font-medium uppercase flex items-end gap-2">
                            "Ваша корзина"
                            <span class="caps font-normal opacity-60">{move || items_label(count())}</span>
                        </h2>
                        <CloseButton on_close=move || open.set(false) />
                    </div>
                    <ul class="mt-3 md:mt-4 flex-1 overflow-auto">
                        <For each=move || lines.get() key=|l| (l.product.slug, l.qty) let:line>
                            <CartLine line update />
                        </For>
                    </ul>
                    <div class="mt-auto py-3 shrink-0 overflow-hidden">
                        <Suggestions add />
                    </div>
                    <div class="shrink-0 bg-white pt-3 md:pt-4">
                        <div class="flex items-center justify-between">
                            <p class="caps">"Доставка"</p>
                            <p class="caps opacity-60">"Рассчитывается при оформлении"</p>
                        </div>
                        <a
                            href="/cart"
                            class="mt-3 md:mt-[18px] pt-3 pb-3.5 w-full flex items-center justify-center gap-[22px] md:gap-8 bg-brown border border-brown text-white text-[12px] uppercase tracking-[0.01em] leading-none hover:bg-black hover:border-black transition-colors"
                        >
                            "Оформить заказ"
                            <span>{total}</span>
                        </a>
                    </div>
                </div>
            </Show>
        </aside>
    }
}

#[component]
fn CartLine(line: Line, update: impl Fn(&'static str, u32) + Copy + Send + Sync + 'static) -> impl IntoView {
    let p = line.product;
    view! {
        <li class="relative flex gap-4">
            <div class="absolute bottom-0 inset-x-0 h-px bg-brown/20"></div>
            <a href=format!("/product/{}", p.slug) class="shrink-0 py-5 w-[68px] md:w-[86px]">
                <div class="ph aspect-[86/112]"></div>
            </a>
            <div class="w-full flex flex-col justify-between gap-4 md:gap-5 py-5">
                <div class="flex items-start gap-2 justify-between">
                    <div>
                        <p class="h6">{p.name}</p>
                        <p class="h6 font-medium mt-1">{p.kind}</p>
                    </div>
                    <p class="text-[12px] leading-[1.3] shrink-0">{rub(p.price * line.qty)}</p>
                </div>
                <div class="flex items-end gap-2 justify-between">
                    <div class="flex">
                        <button
                            aria-label="Меньше"
                            class="size-7 bg-oatmilk/60 flex items-center justify-center hover:bg-brown hover:text-white transition"
                            on:click=move |_| update(p.slug, line.qty - 1)
                        >
                            <svg width="9" height="8" viewBox="0 0 9 8" fill="none">
                                <path stroke="currentColor" d="M8.85 3.93h-8" />
                            </svg>
                        </button>
                        <span class="size-7 bg-oatmilk/60 flex items-center justify-center caps">{line.qty}</span>
                        <button
                            aria-label="Больше"
                            class="size-7 bg-oatmilk/60 flex items-center justify-center hover:bg-brown hover:text-white transition"
                            on:click=move |_| update(p.slug, line.qty + 1)
                        >
                            <PlusIcon />
                        </button>
                    </div>
                    <button class="caps hover:underline" on:click=move |_| update(p.slug, 0)>"Удалить"</button>
                </div>
            </div>
        </li>
    }
}

/// Сколько товаров видно в «Дополните» и до какого можно долистать.
const SHOWN: usize = 3;
const MAX_START: usize = PRODUCTS.len() - SHOWN;

/// «Дополните»: лента товаров со стрелками и кнопкой «+».
#[component]
fn Suggestions(add: impl Fn(&'static str) + Copy + Send + Sync + 'static) -> impl IntoView {
    let start = RwSignal::new(0usize);
    view! {
        <div class="flex items-center justify-between gap-3 mb-3.5">
            <h3 class="caps font-medium">"Дополните"</h3>
            <button
                aria-label="Назад"
                class="size-4 ml-auto flex items-center justify-center disabled:opacity-20"
                disabled=move || start.get() == 0
                on:click=move |_| start.update(|s| *s = s.saturating_sub(1))
            >
                <ArrowIcon class="rotate-180" />
            </button>
            <button
                aria-label="Вперёд"
                class="size-4 flex items-center justify-center disabled:opacity-20"
                disabled=move || { start.get() >= MAX_START }
                on:click=move |_| start.update(|s| *s = (*s + 1).min(MAX_START))
            >
                <ArrowIcon class="" />
            </button>
        </div>
        <ul class="flex gap-4 mb-6">
            {move || {
                PRODUCTS
                    .iter()
                    .skip(start.get())
                    .take(SHOWN)
                    .map(|p| {
                        view! {
                            <li class="w-[210px] shrink-0 flex gap-2">
                                <a href=format!("/product/{}", p.slug) class="ph w-12 aspect-[48/72] shrink-0"></a>
                                <div class="flex-1 flex flex-col">
                                    <h4 class="caps">{p.name}</h4>
                                    <p class="caps font-medium">{p.kind}</p>
                                    <button
                                        class="mt-2 flex items-center justify-between gap-1 py-1.5 px-3 w-full max-w-[110px] border border-current caps hover:bg-brown hover:border-brown hover:text-white transition"
                                        on:click=move |_| add(p.slug)
                                    >
                                        {rub(p.price)}
                                        <PlusIcon />
                                    </button>
                                </div>
                            </li>
                        }
                    })
                    .collect_view()
            }}
        </ul>
    }
}

/// Мобильное меню на весь экран: поиск, разделы с раскрытием, внизу «Смотреть все» и «Аккаунт».
#[component]
fn MobileMenu(open: RwSignal<bool>) -> impl IntoView {
    let expanded = RwSignal::new(None::<&'static str>);
    let toggle = move |key: &'static str| expanded.update(|e| *e = if *e == Some(key) { None } else { Some(key) });
    let section = move |key: &'static str, label: &'static str, links: Vec<(String, String)>| {
        view! {
            <div>
                <button
                    class="w-full flex justify-between items-center uppercase"
                    aria-expanded=move || (expanded.get() == Some(key)).to_string()
                    on:click=move |_| toggle(key)
                >
                    {label}
                    <span class="chev"></span>
                </button>
                <div class="flex flex-col gap-3 pt-3 pl-3 normal-case font-normal" class:hidden=move || expanded.get() != Some(key)>
                    {links.into_iter().map(|(t, h)| view! { <a href=h>{t}</a> }).collect_view()}
                </div>
            </div>
        }
    };
    let shop_links: Vec<(String, String)> = std::iter::once(("Смотреть все".to_string(), "/catalog".to_string()))
        .chain(CATEGORIES.iter().map(|c| (c.name.to_string(), format!("/catalog/{}", c.slug))))
        .collect();

    view! {
        <div class="fixed inset-0 bg-white z-[60] flex-col px-3 py-5" class:hidden=move || !open.get() class:flex=open>
            <button class="nav self-start mb-6" on:click=move |_| open.set(false)>"Закрыть"</button>
            <form action="/catalog" method="get">
                <label class="flex items-center border-b border-black mb-8">
                    <input type="search" name="q" placeholder="Поиск" class="flex-1 py-2 text-[14px] outline-none" />
                    <SearchIcon />
                </label>
            </form>
            <nav class="flex flex-col gap-4 text-[14px] font-medium uppercase">
                {section("shop", "Магазин", shop_links)}
                {CATEGORIES
                    .iter()
                    .filter(|c| c.has_menu)
                    .map(|c| {
                        let links = std::iter::once((c.all_label.to_string(), format!("/catalog/{}", c.slug)))
                            .chain(SUBCATEGORIES.iter().map(|s| (s.to_string(), format!("/catalog/{}", c.slug))))
                            .collect();
                        section(c.slug, c.name, links)
                    })
                    .collect_view()}
                {CATEGORIES
                    .iter()
                    .filter(|c| !c.has_menu)
                    .map(|c| view! { <a href=format!("/catalog/{}", c.slug) class="mt-2">{c.name}</a> })
                    .collect_view()}
                <a href="/approach">"Подход"</a>
            </nav>
            <div class="mt-auto flex flex-col items-center gap-4">
                <a href="/catalog" class="btn w-full text-center">"Смотреть все"</a>
                <a href="/account" class="text-[12px] uppercase">"Аккаунт"</a>
            </div>
        </div>
    }
}

fn items_label(n: u32) -> String {
    let word = match (n % 10, n % 100) {
        (1, r) if r != 11 => "товар",
        (2..=4, r) if !(12..=14).contains(&r) => "товара",
        _ => "товаров",
    };
    format!("{n} {word}")
}

#[component]
fn CloseButton(on_close: impl Fn() + 'static) -> impl IntoView {
    view! {
        <button aria-label="Закрыть" on:click=move |_| on_close()>
            <svg width="12" height="12" viewBox="0 0 12 12" stroke="currentColor">
                <path d="M1 1l10 10M11 1L1 11" />
            </svg>
        </button>
    }
}

#[component]
fn SearchIcon() -> impl IntoView {
    view! {
        <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor">
            <circle cx="6" cy="6" r="5" />
            <path d="M10 10l3.5 3.5" />
        </svg>
    }
}

#[component]
fn PlusIcon() -> impl IntoView {
    view! {
        <svg width="9" height="9" viewBox="0 0 9 9" fill="none">
            <path stroke="currentColor" d="M4.43.27v8M8.5 4.2h-8" />
        </svg>
    }
}

#[component]
fn ArrowIcon(class: &'static str) -> impl IntoView {
    view! {
        <svg class=class width="12" height="12" viewBox="0 0 12 12" fill="none">
            <path stroke="currentColor" d="M1 6h9.5M6 1l5 5-5 5" />
        </svg>
    }
}

#[cfg(test)]
mod tests {
    use super::items_label;

    #[test]
    fn russian_plurals() {
        assert_eq!(items_label(1), "1 товар");
        assert_eq!(items_label(3), "3 товара");
        assert_eq!(items_label(5), "5 товаров");
        assert_eq!(items_label(11), "11 товаров");
        assert_eq!(items_label(22), "22 товара");
    }
}
