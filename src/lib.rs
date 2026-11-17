pub mod app;
pub mod cart;
pub mod components;
pub mod data;
pub mod pages;

/// Точка входа в браузере: оживляет только «острова» (#[island]),
/// остальная страница остаётся обычным HTML с сервера.
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
