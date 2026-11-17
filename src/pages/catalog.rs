//! Каталог: заголовок, вкладки категорий, сортировка, фильтры, сетка товаров, страницы.
//! Разметка и поведение — из wireframe/catalog.html. Большая часть — обычный HTML с сервера,
//! «острова» только там, где нужен браузер: фильтры, сортировка, кнопка «+», уход заголовка.

use leptos::{ev, html, prelude::*, wasm_bindgen::JsCast};
use leptos_meta::Title;
use leptos_router::hooks::{use_params_map, use_query_map};

use crate::cart;
use crate::data::{CATEGORIES, PRODUCTS, Product, category, rub};

const PER_PAGE: usize = 16;

/// Варианты сортировки: (значение в адресе, подпись). Первый — «Подборка», порядок как в данных.
const SORTS: [(&str, &str); 7] = [
    ("", "Подборка"),
    ("bestsellers", "Популярные"),
    ("new", "Новинки"),
    ("price-asc", "Дешевле"),
    ("price-desc", "Дороже"),
    ("name-asc", "А–Я"),
    ("name-desc", "Я–А"),
];

/// Группы фильтров: заголовок и пункты (пока заглушки, как в вайрфрейме).
const FILTERS: [(&str, &[&str]); 3] = [
    ("По типу", &["Подкатегория"; 9]),
    ("По размеру попугая", &["Маленькие", "Средние", "Крупные"]),
    ("По материалу", &["Дерево", "Металл"]),
];

const CLOSE_FILTERS: &str = "window.dispatchEvent(new Event('atb:filters-close'))";

const PLUS_ICON: &str = r#"<svg aria-hidden="true" width="9" height="9" fill="none" viewBox="0 0 9 9"><path stroke="currentColor" d="M4.43.27v8"/><path stroke="currentColor" d="M8.5 4.2h-8"/></svg>"#;
const MINUS_ICON: &str = r#"<svg aria-hidden="true" width="9" height="8" fill="none" viewBox="0 0 9 8"><path stroke="currentColor" d="M8.85 3.93h-8"/></svg>"#;

// ───────────────────────── выбор товаров ─────────────────────────

/// Товары для показа: категория, поиск по названию/типу, сортировка.
fn select(category: Option<&str>, q: &str, sort: &str) -> Vec<&'static Product> {
    let q = q.trim().to_lowercase();
    let mut list: Vec<&'static Product> = PRODUCTS
        .iter()
        .filter(|p| category.is_none_or(|c| p.category == c))
        .filter(|p| q.is_empty() || p.name.to_lowercase().contains(&q) || p.kind.to_lowercase().contains(&q))
        .collect();
    match sort {
        "bestsellers" => list.sort_by_key(|p| p.badge != Some("Хит продаж")),
        "new" => list.sort_by_key(|p| p.badge != Some("Новинка")),
        "price-asc" => list.sort_by_key(|p| p.price),
        "price-desc" => list.sort_by_key(|p| std::cmp::Reverse(p.price)),
        "name-asc" => list.sort_by(|a, b| (a.name, a.kind).cmp(&(b.name, b.kind))),
        "name-desc" => list.sort_by(|a, b| (b.name, b.kind).cmp(&(a.name, a.kind))),
        _ => {}
    }
    list
}

fn page_count(total: usize) -> usize {
    total.div_ceil(PER_PAGE).max(1)
}

/// Кодируем поисковый запрос для адреса.
fn encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(b as char),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Адрес каталога с поиском, сортировкой и страницей (пустые параметры не пишем).
fn href(base: &str, q: &str, sort: &str, page: usize) -> String {
    let mut params = Vec::new();
    if !q.is_empty() {
        params.push(format!("q={}", encode(q)));
    }
    if !sort.is_empty() {
        params.push(format!("sort={sort}"));
    }
    if page > 1 {
        params.push(format!("page={page}"));
    }
    if params.is_empty() { base.to_string() } else { format!("{base}?{}", params.join("&")) }
}

// ───────────────────────── страница ─────────────────────────

#[component]
pub fn CatalogPage() -> impl IntoView {
    let params = use_params_map();
    let query = use_query_map();

    move || {
        let slug = params.read().get("category");
        let q = query.read().get("q").unwrap_or_default().trim().to_string();
        let sort = query.read().get("sort").filter(|s| SORTS.iter().any(|(v, _)| v == s)).unwrap_or_default();
        let page_no: usize = query.read().get("page").and_then(|p| p.parse().ok()).unwrap_or(1);

        // категория из адреса; неизвестная — «не найдено»
        let cat = match slug.as_deref() {
            None => None,
            Some(s) => match category(s) {
                Some(c) => Some(c),
                None => return not_found().into_any(),
            },
        };
        let base = cat.map_or("/catalog".to_string(), |c| format!("/catalog/{}", c.slug));
        let title = if !q.is_empty() {
            format!("Результаты: «{q}»")
        } else {
            cat.map_or("Все товары", |c| c.name).to_string()
        };

        let found = select(cat.map(|c| c.slug), &q, &sort);
        let pages = page_count(found.len());
        let page_no = page_no.clamp(1, pages);
        let shown: Vec<&'static Product> = found.iter().skip((page_no - 1) * PER_PAGE).take(PER_PAGE).copied().collect();
        let empty = shown.is_empty();

        let sort_urls: Vec<String> = SORTS.iter().map(|(v, _)| href(&base, &q, v, 1)).collect();
        let current_sort = SORTS.iter().position(|(v, _)| *v == sort).unwrap_or(0);
        let crumb_href = href(&base, &q, "", 1);

        view! {
            <Title text=title.clone() />
            <div id="catalog-page">
                <CatalogClicks />
                // 3. Заголовок: хлебные крошки, название, описание справа. Отступы 56/56 (на телефоне 32/32),
                // при прокрутке уходит вверх и растворяется
                <HeroFade>
                    <div class="px-3 md:px-6 flex flex-col md:flex-row md:items-center justify-between gap-y-10 gap-6">
                        <div class="flex-1 catalog-in" style="--y:0px;--i:0">
                            <nav class="mb-3" aria-label="Хлебные крошки">
                                <ol class="caps text-black whitespace-nowrap overflow-hidden w-full py-[3px] pl-[3px] -ml-[3px] [&_li]:inline-block [&_li+li]:ml-[5px] [&_a:hover]:underline">
                                    <li>
                                        <a href="/">"Главная"</a>
                                    </li>
                                    <li aria-hidden="true">"—"</li>
                                    <li>
                                        <a href=crumb_href aria-current="page">
                                            {title.clone()}
                                        </a>
                                    </li>
                                </ol>
                            </nav>
                            <h1 class="text-[20px] md:text-[22px] font-medium uppercase leading-[1.1]">{title.clone()}</h1>
                        </div>
                        <div class="max-w-[486px] w-full catalog-in" style="--y:0px;--i:1">
                            <div class="text-[16px] md:text-[18px] leading-[1.2]">
                                "Текст-заглушка: пара предложений о том, какие товары здесь собраны и по каким правилам мы их выбираем. Безопасные материалы, продуманная конструкция, проверено на птицах. Описание будет заменено позже."
                            </div>
                        </div>
                    </div>
                </HeroFade>

                // 4. Вкладки категорий слева, сортировка и фильтры справа. Отступы 40/40 (на телефоне 32/32), сверху линия
                <section class="border-t border-brown/20 py-8 md:py-10">
                    <div class="px-3 md:px-6 flex flex-col lg:flex-row lg:items-center justify-between gap-y-10 gap-6">
                        <ul class="catalog-in flex gap-4 overflow-x-auto hide-scroll p-[3px] -mx-[3px] w-full" style="--i:0">
                            <li class="shrink-0">
                                <a
                                    href=href("/catalog", &q, "", 1)
                                    class="inline-block nav-text pb-2 border-b border-transparent hover:border-current"
                                >
                                    "Все товары"
                                </a>
                            </li>
                            {CATEGORIES
                                .iter()
                                .map(|c| {
                                    view! {
                                        <li class="shrink-0">
                                            <a
                                                href=href(&format!("/catalog/{}", c.slug), &q, "", 1)
                                                class="inline-block nav-text pb-2 border-b border-transparent hover:border-current"
                                            >
                                                {c.name}
                                            </a>
                                        </li>
                                    }
                                })
                                .collect_view()}
                        </ul>
                        <div class="flex items-center lg:justify-between w-full">
                            <div class="catalog-in flex items-center gap-x-1 lg:ml-auto group" style="--i:1">
                                <label for="sort" class="nav-text text-black/60 group-hover:text-black transition">
                                    "Сортировка"
                                </label>
                                <select
                                    id="sort"
                                    class="nav-text relative focus:outline-none border-b border-transparent focus-visible:border-current"
                                    onchange="location.href=this.value"
                                >
                                    {SORTS
                                        .iter()
                                        .zip(sort_urls)
                                        .enumerate()
                                        .map(|(i, ((_, label), url))| {
                                            view! {
                                                <option value=url selected=i == current_sort>
                                                    {*label}
                                                </option>
                                            }
                                        })
                                        .collect_view()}
                                </select>
                            </div>
                            <div
                                class="catalog-in w-full lg:max-w-[328px] pl-[14px] flex flex-row-reverse lg:flex-row items-center justify-start md:justify-end"
                                style="--i:2"
                            >
                                <span class="size-4"></span>
                                <FiltersToggle />
                            </div>
                        </div>
                    </div>
                </section>

                // 5. Сетка товаров: 2 колонки (4 от 1024px) без зазоров по горизонтали, 56px между рядами (32 на телефоне).
                // Фильтры открываются справа: 300px, сетка сжимается. Сверху 16px (на телефоне 0), снизу 100px (80).
                <section id="collection" class="pb-20 md:pb-[100px] md:pt-4">
                    <div class="px-3 md:px-6 flex items-start">
                        {if empty {
                            view! {
                                <p class="nav-text w-full py-20 text-center">
                                    "Ничего не найдено. Попробуйте изменить запрос."
                                </p>
                            }
                                .into_any()
                        } else {
                            view! {
                                <ul class="grid grid-cols-2 gap-y-8 md:gap-y-14 lg:grid-cols-4 w-full">
                                    {shown
                                        .into_iter()
                                        .enumerate()
                                        .map(|(i, p)| view! { <ProductCard p=p i=i /> })
                                        .collect_view()}
                                </ul>
                            }
                                .into_any()
                        }} <FiltersPanel />
                    </div>

                    // Постраничная навигация: по 16 товаров; номера страниц — от 768px
                    <Pagination base=base.clone() q=q.clone() sort=sort.clone() page=page_no pages=pages />
                </section>
            </div>
        }
        .into_any()
    }
}

/// Нет такой категории — «Страница не найдена» со статусом 404 (важно для поисковиков).
fn not_found() -> impl IntoView {
    #[cfg(feature = "ssr")]
    if let Some(resp) = use_context::<leptos_axum::ResponseOptions>() {
        resp.set_status(axum::http::StatusCode::NOT_FOUND);
    }
    view! {
        <Title text="Страница не найдена" />
        <section class="px-3 md:px-6 py-20 md:py-[100px] min-h-[50vh] flex flex-col items-start gap-6">
            <h1 class="h3">"Страница не найдена"</h1>
            <a href="/catalog" class="btn">
                "Все товары"
            </a>
        </section>
    }
}

/// Карточка товара: два фото (второе при наведении), бейдж, кнопка «+», название, цена.
#[component]
fn ProductCard(p: &'static Product, i: usize) -> impl IntoView {
    let link = format!("/product/{}", p.slug);
    view! {
        <li class="catalog-in" style=format!("--i:{i}")>
            <div class="relative flex flex-col group h-full overflow-hidden">
                <div class="relative aspect-[360/540]">
                    <div class="ph w-full h-full">"Фото"</div>
                    <div class="ph !bg-[#d6d2cd] absolute inset-0 opacity-0 group-hover:opacity-100 transition duration-300 ease-out">
                        "Фото 2 (при наведении)"
                    </div>
                    {p
                        .badge
                        .map(|b| {
                            view! {
                                <span class="badge absolute top-[10px] md:top-[24px] left-[10px] md:left-[24px]">{b}</span>
                            }
                        })}
                    <button
                        type="button"
                        aria-label="В корзину"
                        data-add=p.slug
                        class="absolute bottom-[10px] md:bottom-[24px] right-[10px] md:right-[24px] size-[30px] md:size-[35px] border border-current flex items-center justify-center opacity-0 translate-x-full group-hover:translate-x-0 group-hover:opacity-100 focus:opacity-100 focus:translate-x-0 z-[2] hover:bg-brown hover:text-white transition duration-300 ease-out"
                    >
                        <svg aria-hidden="true" width="9" height="9" fill="none" viewBox="0 0 9 9">
                            <path stroke="currentColor" d="M4.43.27v8" />
                            <path stroke="currentColor" d="M8.5 4.2h-8" />
                        </svg>
                    </button>
                </div>
                <div class="flex-1 pt-4 px-1 md:px-4 flex flex-col gap-1 text-center">
                    <p class="h6">
                        <a href=link class="box-link">
                            {p.name}
                        </a>
                    </p>
                    <p class="h6 font-medium">{p.kind}</p>
                    <p class="body-xs pt-1 mt-auto">{rub(p.price)}</p>
                </div>
            </div>
        </li>
    }
}

#[component]
fn Pagination(base: String, q: String, sort: String, page: usize, pages: usize) -> impl IntoView {
    let prev = (page > 1).then(|| href(&base, &q, &sort, page - 1));
    let next = (page < pages).then(|| href(&base, &q, &sort, page + 1));
    view! {
        <div class="overflow-hidden py-6 nav-text px-3 md:px-6 mt-10">
            <div class="overflow-x-auto hide-scroll">
                <div class="text-center flex justify-center gap-x-8 md:gap-x-6">
                    <a
                        href=prev.clone().unwrap_or_else(|| "#collection".into())
                        aria-label="Назад"
                        aria-disabled=prev.is_none().then_some("true")
                        class="shrink-0 p-2 hover:underline"
                    >
                        <span class="flex items-center gap-x-2">"Назад"</span>
                    </a>
                    <ul class="hidden md:flex items-center gap-x-2">
                        {(1..=pages)
                            .map(|n| {
                                if n == page {
                                    view! {
                                        <li>
                                            <span class="flex items-center justify-center border-b border-black p-2">{n}</span>
                                        </li>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <li>
                                            <a
                                                href=href(&base, &q, &sort, n)
                                                aria-label=format!("Страница {n}")
                                                class="shrink-0 inline-block p-2 hover:underline"
                                            >
                                                {n}
                                            </a>
                                        </li>
                                    }
                                        .into_any()
                                }
                            })
                            .collect_view()}
                    </ul>
                    <a
                        href=next.clone().unwrap_or_else(|| "#collection".into())
                        aria-label="Далее"
                        aria-disabled=next.is_none().then_some("true")
                        class="shrink-0 p-2 hover:underline"
                    >
                        "Далее"
                    </a>
                </div>
            </div>
        </div>
    }
}

/// Панель фильтров (открывается кнопкой «Фильтры») и затемнение для телефона.
/// Само открытие — в острове `FiltersToggle`: он ставит класс `open` и слушает `atb:filters-close`.
#[component]
fn FiltersPanel() -> impl IntoView {
    view! {
        <div id="filters-overlay" class="fov fixed inset-0 bg-black/60 backdrop-blur-[24px] z-[200] md:hidden" onclick=CLOSE_FILTERS></div>
        <div
            id="filters"
            class="fwrap w-full bg-white fixed inset-y-0 right-0 md:static shrink-0 z-[200] overflow-auto overflow-x-clip md:overflow-clip transition-all duration-300"
        >
            <div class="finner px-[14px] pt-4 md:pt-0 transition-opacity duration-200 w-[300px] h-full md:h-auto relative md:sticky md:top-[100px]">
                <button
                    type="button"
                    aria-label="Закрыть фильтры"
                    class="absolute top-3 right-3 md:hidden"
                    onclick=CLOSE_FILTERS
                >
                    <svg width="10" height="10" viewBox="0 0 12 12" stroke="currentColor">
                        <path d="M1 1l10 10M11 1L1 11" />
                    </svg>
                </button>
                <form class="flex flex-col h-full md:h-auto" onsubmit="return false">
                    {FILTERS
                        .iter()
                        .map(|(title, items)| {
                            view! {
                                <FilterGroup title=*title>
                                    <ul class="pt-4 flex flex-col gap-4">
                                        {items
                                            .iter()
                                            .map(|item| {
                                                view! {
                                                    <li>
                                                        <label class="w-full flex items-center justify-between gap-4">
                                                            <span class="body-xs text-black/80">{*item}</span>
                                                            <input type="checkbox" class="accent-black size-4" />
                                                        </label>
                                                    </li>
                                                }
                                            })
                                            .collect_view()}
                                    </ul>
                                </FilterGroup>
                            }
                        })
                        .collect_view()}
                    <div class="md:hidden sticky mt-auto bottom-0 p-3 w-full bg-white border-t border-brown/20">
                        <button type="button" class="btn w-full text-center" onclick=CLOSE_FILTERS>
                            "Применить"
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}

/// Группа фильтров: заголовок со значком «−/+» и сворачиваемый список (сворачивает `CatalogClicks`).
#[component]
fn FilterGroup(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="fgroup w-full border-b border-brown/20 py-6">
            <button
                type="button"
                data-fold
                class="caps font-medium text-black/60 w-full flex items-center justify-between gap-6"
                aria-expanded="true"
            >
                {title}
                <span class="relative">
                    <span class="plus block opacity-0" inner_html=PLUS_ICON></span>
                    <span class="minus absolute inset-0 m-auto block" inner_html=MINUS_ICON></span>
                </span>
            </button>
            <div class="fold">
                <div>{children()}</div>
            </div>
        </div>
    }
}

// ───────────────────────── острова ─────────────────────────

/// Заголовок уходит вверх и растворяется при прокрутке: от момента, когда его низ на 15% высоты экрана,
/// до верха экрана.
#[island]
fn HeroFade(children: Children) -> impl IntoView {
    let hero = NodeRef::<html::Section>::new();
    let progress = RwSignal::new(0.0_f64);

    Effect::new(move |_| {
        let update = move || {
            if let Some(el) = hero.get_untracked() {
                let height = window().inner_height().ok().and_then(|h| h.as_f64()).unwrap_or(0.0);
                let start = height * 0.15;
                // низ заголовка на экране; уже сдвинутую вверх часть (100px × прогресс) учитываем, как у них
                let bottom = f64::from(el.offset_top() + el.offset_height())
                    - window().scroll_y().unwrap_or(0.0)
                    - 100.0 * progress.get_untracked();
                let p = if start > 0.0 { ((start - bottom) / start).clamp(0.0, 1.0) } else { 0.0 };
                progress.set(p);
            }
        };
        update();
        let handle = window_event_listener(ev::scroll, move |_| update());
        on_cleanup(move || handle.remove());
    });

    view! {
        <section
            id="hero"
            node_ref=hero
            class="relative py-8 md:py-14 origin-top"
            style=move || {
                let p = progress.get();
                if p == 0.0 {
                    String::new()
                } else {
                    format!("opacity:{};transform:translateY({}px);filter:blur({}px)", 1.0 - p, -100.0 * p, 4.0 * p)
                }
            }
        >
            {children()}
        </section>
    }
}

/// Кнопка «Фильтры»: открывает и закрывает панель справа. На телефоне панель лежит поверх страницы,
/// и страница под ней не прокручивается. Закрыть можно ещё Esc, крестиком, «Применить» и затемнением
/// (они шлют событие `atb:filters-close`).
#[island]
fn FiltersToggle() -> impl IntoView {
    let open = RwSignal::new(false);

    Effect::new(move |_| {
        let is_open = open.get();
        let doc = document();
        for id in ["filters", "filters-overlay"] {
            if let Some(el) = doc.get_element_by_id(id) {
                let _ = el.class_list().toggle_with_force("open", is_open);
            }
        }
        let narrow = window().inner_width().ok().and_then(|w| w.as_f64()).is_some_and(|w| w < 768.0);
        if let Some(body) = doc.body() {
            let _ = body.style().set_property("overflow", if is_open && narrow { "hidden" } else { "" });
        }
    });

    Effect::new(move |_| {
        let h1 = window_event_listener_untyped("atb:filters-close", move |_| open.set(false));
        let h2 = window_event_listener(ev::keydown, move |e| {
            if e.key() == "Escape" {
                open.set(false);
            }
        });
        on_cleanup(move || {
            h1.remove();
            h2.remove();
        });
    });

    view! {
        <button
            id="filters-toggle"
            type="button"
            class="flex items-center gap-x-1.5 hover:underline"
            aria-expanded=move || open.get().to_string()
            on:click=move |_| open.update(|o| *o = !*o)
        >
            <span class="nav-text text-black relative">
                <span class="hidden md:inline transition" class:opacity-0=move || !open.get()>
                    "Скрыть "
                </span>
                "Фильтры"
            </span>
            <span class="relative">
                <span class="transition block" class:opacity-0=move || open.get()>
                    <svg aria-hidden="true" width="12" height="11" fill="none" viewBox="0 0 12 11">
                        <path fill="currentColor" d="M11.5 2v1h-2V2h2Z" />
                        <path fill="currentColor" d="M6 2v1H0V2h6Z" />
                        <path fill="currentColor" d="M11.5 8v1h-6V8h6Z" />
                        <path fill="currentColor" d="M3 8v1H0V8h3Z" />
                        <path fill="currentColor" d="M7 .5v4H6v-4h1Z" />
                        <path fill="currentColor" d="M3 6.5v4H2v-4h1Z" />
                    </svg>
                </span>
                <span class="transition absolute inset-0 m-auto mt-0.5" class:opacity-0=move || !open.get()>
                    <svg aria-hidden="true" width="9" height="8" fill="none" viewBox="0 0 9 8">
                        <path stroke="currentColor" d="M8.85 3.93h-8" />
                    </svg>
                </span>
            </span>
        </button>
    }
}

/// Клики внутри страницы без отдельного острова на каждый элемент (острова без свойств):
/// кнопка «+» на карточке кладёт товар в корзину и открывает её (`data-add` — адрес товара),
/// заголовок группы фильтров сворачивает и разворачивает её список (`data-fold`).
#[island]
fn CatalogClicks() -> impl IntoView {
    Effect::new(move |_| {
        let handle = window_event_listener(ev::click, move |e| {
            let Some(target) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else {
                return;
            };
            if let Ok(Some(button)) = target.closest("[data-add]") {
                if let Some(slug) = button.get_attribute("data-add") {
                    cart::add(&slug, 1);
                }
            } else if let Ok(Some(button)) = target.closest("[data-fold]") {
                let Ok(Some(group)) = button.closest(".fgroup") else { return };
                let Ok(Some(fold)) = group.query_selector(".fold") else { return };
                let closed = fold.class_list().toggle("closed").unwrap_or(false);
                let _ = button.set_attribute("aria-expanded", if closed { "false" } else { "true" });
                if let Ok(Some(plus)) = button.query_selector(".plus") {
                    let _ = plus.class_list().toggle_with_force("opacity-0", !closed);
                }
                if let Ok(Some(minus)) = button.query_selector(".minus") {
                    let _ = minus.class_list().toggle_with_force("opacity-0", closed);
                }
            }
        });
        on_cleanup(move || handle.remove());
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_filter_and_search() {
        assert_eq!(select(None, "", "").len(), 16);
        assert_eq!(select(Some("toys"), "", "").len(), 4);
        // поиск без учёта регистра, по типу товара
        assert_eq!(select(None, "ЖЁРДОЧ", "").len(), 4);
        assert!(select(None, "ничего такого", "").is_empty());
    }

    #[test]
    fn sorting() {
        let cheap = select(None, "", "price-asc");
        assert!(cheap.windows(2).all(|w| w[0].price <= w[1].price));
        let dear = select(None, "", "price-desc");
        assert!(dear.windows(2).all(|w| w[0].price >= w[1].price));
        let hits = select(None, "", "bestsellers");
        assert_eq!(hits[0].badge, Some("Хит продаж"));
        assert_eq!(select(None, "", "unknown")[0].slug, PRODUCTS[0].slug);
    }

    #[test]
    fn links_and_pages() {
        assert_eq!(href("/catalog", "", "", 1), "/catalog");
        assert_eq!(href("/catalog/toys", "корм", "new", 2), "/catalog/toys?q=%D0%BA%D0%BE%D1%80%D0%BC&sort=new&page=2");
        assert_eq!(page_count(0), 1);
        assert_eq!(page_count(16), 1);
        assert_eq!(page_count(17), 2);
    }
}
