//! Корзина в браузере покупателя (localStorage): «slug:кол-во,slug:кол-во».
//! Острова общаются через событие окна `atb:cart` — например, страница товара
//! добавляет товар, а шапка открывает шторку корзины.

use crate::data::{Product, product};

pub const CART_EVENT: &str = "atb:cart";
const KEY: &str = "atb-cart";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    pub product: &'static Product,
    pub qty: u32,
}

pub fn parse(raw: &str) -> Vec<Line> {
    raw.split(',')
        .filter_map(|part| {
            let (slug, qty) = part.split_once(':')?;
            let qty: u32 = qty.parse().ok().filter(|q| *q > 0)?;
            Some(Line { product: product(slug)?, qty })
        })
        .collect()
}

pub fn serialize(lines: &[Line]) -> String {
    lines.iter().map(|l| format!("{}:{}", l.product.slug, l.qty)).collect::<Vec<_>>().join(",")
}

pub fn total(lines: &[Line]) -> u32 {
    lines.iter().map(|l| l.product.price * l.qty).sum()
}

pub fn count(lines: &[Line]) -> u32 {
    lines.iter().map(|l| l.qty).sum()
}

/// Изменить количество товара (0 — удалить); новый товар добавляется в конец.
pub fn set_qty(mut lines: Vec<Line>, slug: &str, qty: u32) -> Vec<Line> {
    if let Some(i) = lines.iter().position(|l| l.product.slug == slug) {
        if qty == 0 {
            lines.remove(i);
        } else {
            lines[i].qty = qty;
        }
    } else if let (Some(p), true) = (product(slug), qty > 0) {
        lines.push(Line { product: p, qty });
    }
    lines
}

// ----- Работа с браузером (только в браузере; на сервере корзина всегда пустая) -----

fn storage() -> Option<web_sys::Storage> {
    leptos::prelude::window().local_storage().ok().flatten()
}

pub fn load() -> Vec<Line> {
    storage().and_then(|s| s.get_item(KEY).ok().flatten()).map(|raw| parse(&raw)).unwrap_or_default()
}

/// Сохранить и сообщить всем островам. `open` — открыть шторку корзины.
pub fn save(lines: &[Line], open: bool) {
    if let Some(s) = storage() {
        let _ = s.set_item(KEY, &serialize(lines));
    }
    let name = if open { "atb:cart-open" } else { CART_EVENT };
    if let Ok(ev) = web_sys::Event::new(name) {
        let _ = leptos::prelude::window().dispatch_event(&ev);
    }
}

/// Добавить товар и открыть шторку (кнопка «В корзину»).
pub fn add(slug: &str, qty: u32) {
    let lines = load();
    let current = lines.iter().find(|l| l.product.slug == slug).map_or(0, |l| l.qty);
    save(&set_qty(lines, slug, current + qty), true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_and_math() {
        let lines = parse("cage-1:2,perch-3:1,unknown:5,toy-1:0");
        assert_eq!(lines.len(), 2);
        assert_eq!(serialize(&lines), "cage-1:2,perch-3:1");
        assert_eq!(count(&lines), 3);
        assert_eq!(total(&lines), 18_900 * 2 + 990);
    }

    #[test]
    fn set_qty_adds_updates_and_removes() {
        let lines = set_qty(vec![], "toy-1", 1);
        let lines = set_qty(lines, "toy-1", 3);
        assert_eq!(lines[0].qty, 3);
        assert!(set_qty(lines, "toy-1", 0).is_empty());
    }
}
