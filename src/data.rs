//! Каталог-заглушка. Позже эти данные будут приходить из базы (PostgreSQL),
//! а пока — чтобы все страницы и корзина работали «по-настоящему».

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Category {
    pub slug: &'static str,
    pub name: &'static str,
    /// Подпись «Все …» в меню.
    pub all_label: &'static str,
    /// Есть ли у пункта раскрывающееся меню в шапке (у «Игрушек» нет — как у них «Fragrance free»).
    pub has_menu: bool,
}

pub const CATEGORIES: [Category; 4] = [
    Category { slug: "cages", name: "Клетки", all_label: "Все клетки", has_menu: true },
    Category { slug: "perches", name: "Жёрдочки", all_label: "Все жёрдочки", has_menu: true },
    Category { slug: "feeders", name: "Кормушки и поилки", all_label: "Все кормушки и поилки", has_menu: true },
    Category { slug: "toys", name: "Игрушки", all_label: "Все игрушки", has_menu: false },
];

/// Подкатегории для мега-меню — пока заглушки, как в вайрфрейме.
pub const SUBCATEGORIES: [&str; 3] = ["Подкатегория", "Подкатегория", "Подкатегория"];

pub fn category(slug: &str) -> Option<&'static Category> {
    CATEGORIES.iter().find(|c| c.slug == slug)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Product {
    pub slug: &'static str,
    /// Первая строка названия (как у них «Exfoliating Lavender Leaf»).
    pub name: &'static str,
    /// Вторая строка — тип товара (как у них «Hand wash»).
    pub kind: &'static str,
    pub price: u32,
    pub category: &'static str,
    pub badge: Option<&'static str>,
}

macro_rules! p {
    ($slug:literal, $kind:literal, $price:literal, $cat:literal, $badge:expr) => {
        Product { slug: $slug, name: "Название товара", kind: $kind, price: $price, category: $cat, badge: $badge }
    };
}

/// 16 товаров-заглушек: по 4 в каждой категории.
pub const PRODUCTS: [Product; 16] = [
    p!("cage-1", "Клетка", 18_900, "cages", Some("Новинка")),
    p!("cage-2", "Клетка", 24_500, "cages", None),
    p!("cage-3", "Клетка", 12_900, "cages", Some("Хит продаж")),
    p!("cage-4", "Клетка", 31_000, "cages", None),
    p!("perch-1", "Жёрдочка", 1_490, "perches", Some("Хит продаж")),
    p!("perch-2", "Жёрдочка", 1_890, "perches", None),
    p!("perch-3", "Жёрдочка", 990, "perches", None),
    p!("perch-4", "Набор жёрдочек", 3_200, "perches", Some("Новинка")),
    p!("feeder-1", "Кормушка", 1_290, "feeders", None),
    p!("feeder-2", "Поилка", 1_590, "feeders", Some("Хит продаж")),
    p!("feeder-3", "Кормушка", 990, "feeders", None),
    p!("feeder-4", "Поилка", 2_100, "feeders", None),
    p!("toy-1", "Игрушка", 690, "toys", None),
    p!("toy-2", "Игрушка", 890, "toys", Some("Новинка")),
    p!("toy-3", "Игрушка", 1_190, "toys", None),
    p!("toy-4", "Игрушка", 540, "toys", None),
];

pub fn product(slug: &str) -> Option<&'static Product> {
    PRODUCTS.iter().find(|p| p.slug == slug)
}

/// «4 990 ₽» — неразрывный пробел между тысячами, как принято в России.
pub fn rub(amount: u32) -> String {
    let digits = amount.to_string();
    let mut out = String::new();
    for (i, ch) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push('\u{00a0}');
        }
        out.push(ch);
    }
    out + "\u{00a0}₽"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rub_formats_thousands() {
        assert_eq!(rub(990), "990\u{a0}₽");
        assert_eq!(rub(4_990), "4\u{a0}990\u{a0}₽");
        assert_eq!(rub(31_000), "31\u{a0}000\u{a0}₽");
    }

    #[test]
    fn every_product_has_a_known_category() {
        assert!(PRODUCTS.iter().all(|p| category(p.category).is_some()));
    }
}
