//! Сервер: Axum отдаёт страницы, которые Leptos собирает в готовый HTML.

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use askthebirds::app::{App, shell};
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};

    let conf = get_configuration(None).expect("настройки Leptos из Cargo.toml");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.expect("свободный порт");
    leptos::logging::log!("askthebirds: http://{addr}");
    axum::serve(listener, app.into_make_service()).await.expect("сервер");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
