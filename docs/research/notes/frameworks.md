# Full-stack Rust web frameworks for an SEO-critical e-commerce shop (state as of Nov 2026)

Scope: Leptos, Dioxus (fullstack), Yew, Sycamore, newer contenders (Topcoat), and the server-first Axum + Maud/Askama + htmx/Datastar approach. Research date: 2026-11-16. Release-page dates on GitHub often omit the year, so years below are inferred from ordering and context where marked "(inferred)".

## 1. Versions, release cadence, activity, maintainers/funding, API stability

### Takeaway
Leptos is the most actively released web-focused framework: a stable 0.8 line with frequent patches (0.8.23 on 31 Oct 2026) and 0.9 at RC stage, described as "mostly compatible". Dioxus 0.7 is stable (0.7.10). 0.8 is in alpha with real breaking changes, and the team was acqui-hired by Cognition in Sep 2026 while promising continued maintenance. Yew (0.23) and Sycamore (0.9.4, 0.10 beta) are alive but slower and smaller. Topcoat (Tokio team, Jul 2026) is new and explicitly experimental.

### Cited Findings
**Leptos**
- Latest releases (GitHub, years inferred as 2026): v0.9.0-rc1 (31 Oct), v0.8.23 marked "Latest" (31 Oct), v0.9.0-rc0 (15 Oct), v0.8.22 (05 Oct), v0.8.21 (26 Sep), v0.9.0-beta (18 Jul), v0.8.19 (25 Jun), v0.9.0-alpha (19 May), v0.8.18 (16 Apr), v0.8.17 (01 Mar). That is roughly one patch release a month — [Leptos GitHub releases](https://github.com/leptos-rs/leptos/releases)
- lib.rs lists 0.9.0-alpha on May 19, 2026 and 0.8.7 on Feb 16, 2026 — [lib.rs leptos_server](https://lib.rs/crates/leptos_server)
- 0.8.0 (tagged around 1 May 2025, inferred) brought Axum 0.8 support, which forced the major bump, and compile-time improvements via `erase_components` — [Leptos v0.8.0 release](https://github.com/leptos-rs/leptos/releases/tag/v0.8.0)
- 0.9 notes say it should be "mostly compatible with most user application code". Breaking changes:
  - server-function body limits now apply to all server functions, including multipart uploads (the Axum default is 2 MB)
  - code splitting moved behind a `lazy` feature flag
  - form encoding moved to `serde_qs` 1.0
  - `on:toggle` now gets a `ToggleEvent`
  - calling signals as `foo()` now works on stable Rust
  - `Signal<T>` passed where `impl Fn() -> T` is expected may need `*signal`
  - "many semver-breaking internal changes"

  — [Leptos GitHub releases](https://github.com/leptos-rs/leptos/releases)
- Leptos 0.7 introduced islands. A third-party comparison table says "Islands: Yes, since 0.7" — [rustify.rs Topcoat vs Leptos](https://rustify.rs/articles/topcoat-vs-leptos-2026) (secondary, SEO-style site; treat with caution)

**Dioxus**
- Releases (asset timestamps indicate 2026): v0.8.0-alpha.3 (16 Oct), alpha.2 (14 Oct), alpha.1 (31 Jul), v0.7.10 "Latest" (30 Jul), v0.8.0-alpha.0 (19 May), v0.7.9 (08 May), v0.7.8 (07 May), v0.7.7 (01 May), v0.7.6 (22 Apr), v0.7.5 (06 Apr) — [Dioxus GitHub releases](https://github.com/DioxusLabs/dioxus/releases)
- 0.8 alphas include breaking changes:
  - alpha.2 moves elements out of `dioxus_elements`
  - alpha.2 also changes the rendering internals ("no longer uses comment-node placeholders and markers, either at runtime or during hydration")
  - alpha.3 makes `dioxus::launch` install ring as the rustls provider
  - the 0.8 CLI cannot run apps built with Dioxus older than 0.7.2

  — [Dioxus GitHub releases](https://github.com/DioxusLabs/dioxus/releases)
- Dioxus 0.7.0 was published on 8 Sep 2025 by Jonathan Kelley and merged more than 350 PRs — [Dioxus 0.7 release blog](https://dioxuslabs.com/blog/release-070/)
- The 0.7 fullstack rewrite "was originally planned for 0.8, but given the severity of the issues, we decided to pull it forward to 0.7" — [newreleases 0.7.0-rc.1](https://newreleases.io/project/github/DioxusLabs/dioxus/release/v0.7.0-rc.1)
- Funding: the GitHub org credits Satellite.im, FutureWei, Y Combinator, GitHub and donors — [DioxusLabs GitHub](https://github.com/dioxuslabs)
- The team reportedly raised $3.5M (YC, Khosla) — [Unite.ai](https://www.unite.ai/?p=465293). CB Insights reports only about $500K over 3 rounds — [CB Insights](https://www.cbinsights.com/company/dioxus-labs/financials). The two figures conflict.
- **10 Sep 2026: the Dioxus team joined Cognition (maker of Devin).** Dioxus says it "will keep working on Dioxus, Blitz, Taffy, and Subsecond". Cognition is "increasing investments in Dioxus-Native and Blitz", and Nico Burns will be "devoted full time to maintaining and improving Dioxus" — [Unite.ai](https://www.unite.ai/?p=465293)

**Yew**
- 0.22 "For Real This Time" was released 29 Nov 2025 (`#[component]` rename, for-loops in `html!`) — [Yew blog](https://yew.rs/blog/2025/11/29/release-0-22)
- 0.23.0 followed about 8 months before the page snapshot (2026), with this note: "bumping from 0.22 should require no code changes for most users". Changes include `use_reducer` skipping re-render on the same Rc and the scheduler yielding to the main thread — [newreleases yew-v0.23.0](https://newreleases.io/project/github/yewstack/yew/release/yew-v0.23.0)
- The 0.22 docs are marked "no longer actively maintained" in favour of 0.23 — [Yew 0.22 migration guides](https://yew.rs/docs/0.22/category/migration-guides)
- lib.rs shows "37 releases (22 breaking)" for Yew — [lib.rs yew](https://lib.rs/crates/yew)

**Sycamore**
- Latest is v0.9.4 (06 Oct). v0.10.0-beta.1 is a pre-release (09 Nov, inferred 2026). The 0.9.x patches include hydration fixes for Keyed, Indexed and Portal. The release notes don't mention streaming SSR — [Sycamore GitHub releases](https://github.com/sycamore-rs/sycamore/releases)

**Topcoat (new contender)**
- Announced 22 Jul 2026 on the Tokio blog by Carl Lerche and Julien Scholz. It is a "modular, batteries-included Rust framework for building full-stack, reactive web apps". Its own text calls it "just the first release" and says client reactivity "is still in the early stages of development and has some limitations" — [Tokio blog](https://tokio.rs/blog/2026-07-22-announcing-topcoat)
- 0.1.0 reached crates.io on 16 Jul 2026 and 0.5.0 on 27 Jul 2026, with breaking router/layout changes. The README calls it "early-stage and experimental". Auth/validation, background jobs, **streaming SSR** and deployment docs are still roadmap items — [XenoSpectrum](https://xenospectrum.com/en/topcoat-rust-fullstack/)
- DevClass coverage (30 Oct 2026) — [DevClass](https://www.devclass.com/devops/2026/10/30/topcoat-enables-rust-to-render-server-side-web-apps/5306490)

### Inferences
- For API stability over a multi-year shop project, Leptos 0.8 to 0.9 looks like the gentlest upgrade path. Dioxus 0.7 to 0.8 involves more churn: element namespace, rendering internals, TLS provider.
- After the Cognition move, Dioxus's corporate priorities lean toward Dioxus-Native/Blitz (native rendering) rather than SEO web SSR. That is a strategic risk for a web-only shop, though not a fatal one.
- Topcoat comes from Tokio, which is credible, but at 0.x weeks-old maturity it isn't a 2026 production choice. It is worth tracking for 2027.

### Gaps
- Exact GitHub star counts and contributor counts for late 2026 weren't retrieved.
- Leptos funding: I found no source on Greg Johnston's sponsorship or funding as of 2026. Leptos appears community- and sponsor-driven, but this is unverified.
- I couldn't confirm whether Leptos 0.9.0 final shipped after rc1 (31 Oct 2026).

## 2. SSR / streaming / hydration / islands / SSG; SEO; TTFB and Core Web Vitals with WASM

### Takeaway
Leptos has the most complete SEO-relevant feature set: SSR, out-of-order and in-order streaming, hydration, islands, and meta tags via `leptos_meta`. Dioxus 0.7 has SSR, hydration and Axum-based fullstack, but its 0.7 release blog doesn't focus on SSR or SSG. Server-first stacks deliver complete HTML with no hydration cost. With WASM, the cost falls on time-to-interactive rather than on first paint.

### Cited Findings
- Leptos book: in a server-rendered app, WASM binary size "affects only one thing: time to interactivity on the first load" — quoted via search summary of [Leptos book, binary size](https://book.leptos.dev/deployment/binary_size.html)
- Leptos book: WASM binaries are "significantly larger than the JavaScript bundles" for the same app. WASM compiles faster per KB thanks to streaming compilation — [Leptos book, binary size](https://book.leptos.dev/deployment/binary_size.html)
- Dioxus 0.7 fullstack:
  - server functions rebuilt on Axum, and any Axum handler can be a server function
  - SSE (`ServerEvents<T>`), WebSockets, `Streaming<T>`, typed `Form<T>`/`MultipartFormData`, `FileStream`
  - can target wasm32 (e.g. Cloudflare Workers)
  - the blog post itself doesn't discuss SSR, hydration or SSG

  — [Dioxus 0.7 release blog](https://dioxuslabs.com/blog/release-070/)
- Dioxus 0.7.0-rc.1 notes list hot-reloading for pure SSR apps and full `axum::extract::Request` access in server functions — [newreleases 0.7.0-rc.1](https://newreleases.io/project/github/DioxusLabs/dioxus/release/v0.7.0-rc.1)
- Dioxus 0.8-alpha.1 fixes `#[css_module]` styles going missing from `<head>` after the first SSR request. Alpha.2 removes hydration comment markers — [Dioxus GitHub releases](https://github.com/DioxusLabs/dioxus/releases)
- A Leptos core contributor (benw.is) noted that a "sledgehammer" rendering mode increased bundle size, "causing a fairly significant regression in TTI". This is an older blog post — [benw.is via reader](https://hn.jakechampion.name/reader/https://benw.is/posts/full-stack-rust-with-leptos).
- Topcoat renders all markup on the server, ships no WASM, and compiles Rust closures inside `$(...)` to JS signals. `#[shard]` functions re-render page fragments. It also has HTMX/Alpine integrations — [Tokio blog](https://tokio.rs/blog/2026-07-22-announcing-topcoat)
- Topcoat's repo contains a catalog benchmark (Next.js 15 vs Leptos 0.8 vs Axum 0.8 + Maud). It measures only server time to full HTML, and "no official score table has been published" — [XenoSpectrum](https://xenospectrum.com/en/topcoat-rust-fullstack/)

### Inferences
- For SEO, all SSR options (Leptos, Dioxus fullstack, server-first templates) hand crawlers full HTML. LCP and TTFB are governed by server render and DB time, not WASM. INP and TTI on the first visit are where WASM hydration hurts on low-end Android devices, which matters for the Russian mobile audience.
- Leptos islands (hydrate only interactive parts such as the cart button, filters and 3D configurator) is the closest WASM-framework analogue to a server-first page and is the best fit for catalog pages.
- No published Lighthouse or CWV benchmark compares these frameworks. The shop should run its own Lighthouse CI on a prototype.

### Gaps
- I found no rigorous 2026 Lighthouse/CWV measurements for Leptos or Dioxus production sites. One DEV post claims "~5KB gzipped, Lighthouse 95-100" for Leptos with no methodology ([DEV](https://dev.to/0012303/leptos-has-a-free-api-build-full-stack-web-apps-in-rust-with-fine-grained-reactivity-22c)). It is unreliable and shouldn't be used.
- Sitemap generation: no framework has a built-in sitemap generator. These are typically generated by an Axum route or the `sitemap-rs` crate (from general knowledge, not verified this session).
- SSG/static generation support status in Dioxus 0.7/0.8 and Leptos 0.8 (Leptos has had `StaticRoute`/SSG modes since 0.7, per memory) wasn't verified this session.

## 3. WASM bundle sizes and mitigation

### Takeaway
Both Leptos (0.8.5+, `#[lazy]` / `#[lazy_route]`, `cargo leptos --split`) and Dioxus (0.7, `#[wasm_split("/path")]` router-based chunks via `dx`) now support WASM code splitting. Both are young (introduced in summer 2025). Neither project publishes official size numbers, so you need to measure.

### Cited Findings
- Leptos 0.8.5 added WASM code splitting:
  - `#[lazy]` turns a function into a lazily loaded async function in a separate chunk
  - `#[lazy_route]` with `LazyRoute` splits a route into `data()` and `view()`, which load concurrently with no waterfall
  - 0.8.6 added `#[server] #[lazy]`

  — [Leptos v0.8.5 release](https://github.com/leptos-rs/leptos/releases/tag/v0.8.5); [lib.rs leptos_server](https://lib.rs/crates/leptos_server)
- Leptos book:
  - code splitting was "released in summer 2025, so it may still have bugs" and is enabled with `cargo leptos (serve|watch|build) --split`
  - serve WASM compressed (typically under 50% of raw size)
  - use a `wasm-release` profile with `opt-level='z'`, `lto=true`, `codegen-units=1`
  - nightly `build-std` with `panic=immediate-abort`
  - `miniserde`/`serde-lite` resources
  - default `regex` adds about 500 KB
  - avoid heavy generics (monomorphization)

  — [Leptos book, binary size](https://book.leptos.dev/deployment/binary_size.html)
- Leptos 0.9 moves code splitting behind a `lazy` feature flag, which reduces size for apps that don't use it — [Leptos GitHub releases](https://github.com/leptos-rs/leptos/releases)
- Dioxus 0.7: `dx` "automatically splits WebAssembly apps into chunks based on the router". It is enabled per route with `#[wasm_split("/path")]`, and users see a loading state while a chunk loads — [Dioxus 0.7 release blog](https://dioxuslabs.com/blog/release-070/)
- Dioxus 0.7.6 added content-hashed filenames for wasm_split bundles. 0.8-alpha.3 trimmed lightningcss features and removed duplicate crate versions, with no stated size impact — [Dioxus GitHub releases](https://github.com/DioxusLabs/dioxus/releases)

### Inferences
- For the 3D configurator, a lazy-loaded chunk (Leptos `#[lazy]`/`#[lazy_route]` or a Dioxus `wasm_split` route), or a separate JS/WebGL bundle loaded only on the configurator page, keeps catalog and product pages light.
- Realistic Leptos/Dioxus SSR+hydrate apps commonly land in the hundreds of KB of WASM before compression (community experience, not verified with a source this session). Islands mode plus splitting are the main levers.

### Gaps
- I found no authoritative 2026 numbers for gzipped/brotli WASM sizes of real Leptos or Dioxus apps.
- js-framework-benchmark results for 2026 weren't retrieved.

## 4. Axum integration, server functions, routing, forms, i18n

### Takeaway
Leptos (leptos_axum, `#[server]`, leptos_router, `<ActionForm>`) and Dioxus 0.7 (server functions rebuilt *on* Axum, typed forms) both integrate deeply with Axum. Dioxus 0.7's model, where any Axum handler can be a server function, is the most Axum-native. Leptos also supports Actix.

### Cited Findings
- Leptos 0.8 was driven by Axum 0.8 support — [Leptos v0.8.0 release](https://github.com/leptos-rs/leptos/releases/tag/v0.8.0)
- Leptos 0.9:
  - server function body limits apply everywhere (Axum default 2 MB, which matters for uploads)
  - form-data encoding moved to `serde_qs` 1.0, giving better empty-collection handling

  — [Leptos GitHub releases](https://github.com/leptos-rs/leptos/releases)
- Leptos runs on Axum or Actix and uses `leptos_router` — [rustify.rs](https://rustify.rs/articles/topcoat-vs-leptos-2026) (secondary)
- Dioxus 0.7:
  - Rocket-style endpoint attributes with path and query params
  - any Axum handler can serve as a server function
  - `Form<T>`, `MultipartFormData`, SSE, WebSockets, streaming
  - a new `dioxus::serve` entrypoint allows hot-patching Axum routers

  — [Dioxus 0.7 release blog](https://dioxuslabs.com/blog/release-070/); [alternativeto news](https://alternativeto.net/news/2025/11/dioxus-0-7-brings-hot-patching-wgpu-rendering-and-axum-integration)
- Topcoat: pages declared with `#[page("/")]` plus `Router::builder().discover()`, async components querying the DB via its Toasty ORM, Tailwind and shadcn-style components. Axum is positioned as the lower-level router for APIs — [Tokio blog](https://tokio.rs/blog/2026-07-22-announcing-topcoat)

### Inferences
- Progressive enhancement (forms working without JS or WASM) is important for checkout robustness. Leptos `ActionForm` posts to server functions as plain HTML forms (per Leptos design, not re-verified this session), which suits checkout.

### Gaps
- i18n: I didn't retrieve 2026 sources for `leptos_i18n` (the community crate) or Dioxus i18n options (`dioxus-i18n`). Russian-only launches can defer this. Fluent-based crates exist (general knowledge, unverified).

## 5. Developer experience: hot reload, tooling, compile times, IDE

### Takeaway
Dioxus has the strongest DX story with Subsecond Rust hot-patching (`dx serve`), which works on web WASM and on Axum routers. Leptos relies on cargo-leptos with view-macro hot reload and full rebuilds otherwise. Compile times remain the top complaint across all Rust web stacks, including server-first ones.

### Cited Findings
- Subsecond patches Rust code at runtime without losing state on Web (WASM), desktop, iOS and Android, via `dx serve` and `subsecond::call`. It doesn't migrate struct state when fields change. Bevy and Iced have adopted it — [Dioxus 0.7 release blog](https://dioxuslabs.com/blog/release-070/)
- Dioxus 0.7.6 made `dx serve --hotpatch` start faster. 0.7.8 fixed wasm hot-patch UB. 0.7.10 fixed hot-patching for crate names that differ from package names — [Dioxus GitHub releases](https://github.com/DioxusLabs/dioxus/releases)
- Leptos 0.8 improved compile times via `erase_components` — [Leptos v0.8.0 release](https://github.com/leptos-rs/leptos/releases/tag/v0.8.0). 0.9 makes `signal()` call syntax work on stable, which helps IDE and rust-analyzer ergonomics — [Leptos GitHub releases](https://github.com/leptos-rs/leptos/releases)
- A 2026 Japanese guide reports that the first build of a simple Leptos counter took over four minutes. This matters for CI — via search summary of [issoh.co.jp](https://www.issoh.co.jp/md/tech/9089) (secondary; not fetched directly)
- MASH stack (Maud + Axum + SQLx + htmx): compile time is "the main drawback". The author got save-to-reload down to about 2.5 s using Bacon and tower-livereload. Maud template changes require recompiling — [Evan Schwartz, Mar 2025](https://emschwartz.me/building-a-fast-website-with-the-mash-stack-in-rust/)
- DevClass on WASM approaches: "the upfront loading time can be a real drag" — via search summary of [DevClass](https://www.devclass.com/devops/2026/10/30/topcoat-enables-rust-to-render-server-side-web-apps/5306490)

### Gaps
- I found no 2026 head-to-head compile-time benchmarks.
- cargo-leptos 2026 hot-reload capabilities weren't re-verified. I don't know whether cargo-leptos adopted Subsecond.

## 6. Real-world production sites / case studies

### Takeaway
Public production evidence is thin for all Rust WASM frameworks, and I found **no verified Rust-WASM e-commerce storefront case study**. Tech trackers count about 130–213 sites using Leptos. Dioxus's flagship users are more app- and desktop-oriented.

### Cited Findings
- Wappalyzer tracks about 130 live sites using Leptos — [Wappalyzer](https://www.wappalyzer.com/technologies/web-frameworks/leptos/). Another tracker claims 213 — [wmtips](https://www.wmtips.com/technologies/backend-frameworks/leptos/). The counts conflict.
- Intelbee lists novartis.com among Leptos sites. This is unverified and probably detection noise — [intelbee](https://intelbee.com/technologies/web-frameworks/leptos)
- A small production Leptos app at thkitchensappliances.com (a developer blog case study) — [baileylutcd blog](https://baileylutcd.github.io/blog/rust-leptos-full-stack-web-app)
- Scour (a content feed) runs in production on the server-first MASH stack — [Evan Schwartz](https://emschwartz.me/building-a-fast-website-with-the-mash-stack-in-rust/)
- The Dioxus 0.7 blog names no production users, only ecosystem adopters of Subsecond (Bevy, Iced) — [Dioxus 0.7 release blog](https://dioxuslabs.com/blog/release-070/)

### Inferences
- An e-commerce shop in Rust/WASM would be pioneering. Expect to build cart, checkout, payment integration (YooKassa/CloudPayments) and sitemap/feeds (Yandex YML) yourself.

### Gaps
- No verified e-commerce case studies for any Rust framework.
- I didn't search Reddit r/rust 2026 production threads or the awesome-leptos/awesome-dioxus showcase lists in depth.

## 7. Server-first alternative: Axum + Maud/Askama + htmx/Datastar vs WASM frameworks

### Takeaway
For a catalog-heavy, SEO-critical shop, a server-first stack gives full HTML, zero hydration cost, small client JS, and the simplest mental model. It loses shared client/server Rust components and fine-grained client reactivity. The usual hybrid is server-first pages plus an isolated interactive island (JS/WebGL or a standalone WASM module) for the 3D configurator.

### Cited Findings
- MASH stack:
  - Maud gives compile-time-checked HTML in Rust, and its `Markup` is an Axum response
  - Axum extractors provide shared layout data
  - htmx swaps fragments with no custom JS, and its script is cacheable as `immutable`
  - `axum-htmx` handles headers and `Vary`
  - the author moved from Askama to Maud because Askama needs a struct plus a template per change
  - cons: compile times, clunky static-fragment patterns
  - the author still says TypeScript may be better for typical startups

  — [Evan Schwartz, Mar 2025](https://emschwartz.me/building-a-fast-website-with-the-mash-stack-in-rust/)
- Datastar is "SSE first", with HTML fragments and signals over the same SSE connection. A 2026 tutorial pairs it with Rust and Axum — [hamy.xyz, Mar 2026](https://hamy.xyz/blog/2026-03_datastar-rust-todo)
- The HARM stack (htmx, Axum, Rust, Maud, plus Alpine.js) was discussed on r/rust in Jan 2025 — [r/rust mirror](https://redlib.hackliberty.org/r/rust/comments/1i76s8t/the_harm_stack_htmx_axumalpinejs_rust_maud/m8l0rll/?context=3); [engineered.at](https://engineered.at/articles/the-harm-stack-htmx-axum-alpinejs-rust-maud-considered-unharmful)
- Topcoat formalizes the server-first approach (server-rendered async components, JS-compiled signals, `#[shard]` partial re-render, HTMX/Alpine integrations) but is experimental — [Tokio blog](https://tokio.rs/blog/2026-07-22-announcing-topcoat); [XenoSpectrum](https://xenospectrum.com/en/topcoat-rust-fullstack/)

### Inferences
**Pros of server-first for this shop:**
- best-case TTFB, LCP and INP on cheap phones
- trivially crawlable by Yandex and Google
- no WASM toolchain
- easy caching and CDN
- checkout works without JS

**Cons:**
- cart and filter interactivity must be done via htmx/Datastar or small JS
- no shared typed components between client and server
- the 3D configurator needs a separate client tech (three.js/Babylon via JS, or a standalone Bevy/wgpu WASM module), and that is outside "entirely Rust" for the glue

### Gaps
- No e-commerce benchmark comparing these approaches. The Topcoat catalog benchmark results are unpublished.

### Overall recommendation (synthesis, inference — placed here for the report writer)

These are inferences, not cited facts.

- **Primary pick: Leptos 0.8 (move to 0.9 once final) with Axum, SSR + islands, and `#[lazy]`/`#[lazy_route]` code splitting.** It is the most web/SEO-focused, most actively released, has the gentlest upgrade path (0.9 "mostly compatible"), streaming SSR, islands, and `leptos_meta` for meta tags. The 3D configurator would be an island or lazy route (Rust + `web-sys` WebGL/wgpu, or three.js via JS interop).
- **Strong alternative: server-first Axum + Maud (or Askama/Rinja) + htmx/Datastar,** with the 3D configurator as a separately loaded WASM or JS module only on its page. It is likely the best Core Web Vitals and simplest ops, and the right choice if the team values SEO and performance over shared reactive components.
- **Dioxus 0.7** is a credible option, with the best hot-patching DX and Axum-native server functions. Its strategic focus (native/Blitz, the Cognition acqui-hire in Sep 2026) and the breaking 0.8 alpha make it second for an SEO web shop.
- **Yew and Sycamore:** viable but smaller and slower-moving, with no advantage here.
- **Topcoat:** watch, but don't adopt in 2026.

#### 3D configurator gap
I didn't research Rust 3D options (Bevy WASM size, three-d, wgpu WebGPU support in browsers in 2026). This is presumably covered by another researcher.
