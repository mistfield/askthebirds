use leptos::prelude::*;
use leptos_meta::{Link, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::components::{AnnouncementBar, ChatButton, CookieBanner, Footer, Header, SignupPopup};
use crate::pages::*;

/// Оболочка HTML-документа (собирается на сервере).
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="ru">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true />
                <MetaTags />
            </head>
            <body class="bg-white text-black font-sans antialiased flex flex-col min-h-screen">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/askthebirds.css" />
        // TODO: перенести Inter на свой сервер (без Google): быстрее в России и не передаём IP посетителей за границу
        <Link rel="preconnect" href="https://fonts.googleapis.com" />
        <Link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="" />
        <Link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&display=swap"
        />
        <Title formatter=|t: String| if t.is_empty() { "askthebirds — товары для попугаев".into() } else { format!("{t} — askthebirds") } />

        <Router>
            <AnnouncementBar />
            <Header />
            <main class="page-in flex-1">
                <Routes fallback=NotFoundPage>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/catalog") view=CatalogPage />
                    <Route path=path!("/catalog/:category") view=CatalogPage />
                    <Route path=path!("/product/:slug") view=ProductPage />
                    <Route path=path!("/approach") view=ApproachPage />
                    <Route path=path!("/cart") view=CartPage />
                    <Route path=path!("/account") view=AccountPage />
                    <Route path=path!("/account/:view") view=AccountPage />
                    <Route path=path!("/faq") view=FaqPage />
                    <Route path=path!("/shipping") view=ShippingPage />
                    <Route path=path!("/returns") view=ReturnsPage />
                    <Route path=path!("/contact") view=ContactPage />
                    <Route path=path!("/legal/:doc") view=LegalPage />
                </Routes>
            </main>
            <Footer />
            <ChatButton />
            <SignupPopup />
            <CookieBanner />
        </Router>
    }
}

/// Страница «не найдено».
#[component]
fn NotFoundPage() -> impl IntoView {
    view! {
        <Title text="Страница не найдена" />
        <section class="px-3 md:px-6 py-20 md:py-[100px] min-h-[50vh] flex flex-col items-start gap-6">
            <h1 class="h3">"Страница не найдена"</h1>
            <a href="/" class="btn">"На главную"</a>
        </section>
    }
}
