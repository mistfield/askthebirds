# Animation-rich, "wow" web design with a Rust/WASM frontend (Leptos / Dioxus), state as of Nov 2026

Overall verdict, from the findings below: Apple/Awwwards-level visuals are achievable with Rust, but mostly because the browser does the animating. Use CSS scroll-driven animations, View Transitions, CSS transitions/keyframes, and the Web Animations API through web-sys. Rust handles state and orchestration. The Rust-native animation crates are young, small, and uneven in quality. No Rust crate today matches GSAP or Motion (Framer Motion) in depth.

## 1. Rust animation libraries / crates: maturity and activity

### Takeaway
There is no mature "Framer Motion for Rust". dioxus-motion is the most coherent option for Dioxus. On Leptos the field is fragmented: leptos_animate, animato, slik, spanda, and the yanked leptos-motion. leptos-use gives solid low-level primitives (rAF, intersection observer, scroll).

### Cited Findings
- **leptos-motion / leptos-motion-core** is billed as Framer-Motion-inspired. leptos-motion-core is now marked **yanked** on lib.rs: 19 releases with only 1 stable, latest 1.1.0 on 16 Sep 2025, about 21 downloads/month. The README claims "production-ready" but also reports "WebGL System: 27/52 tests passing" and "9/12 examples functional", and describes a remediation toward Motion.dev parity on an "8-month roadmap". The README contradicts itself. Do not use it. — [lib.rs leptos-motion-core](https://lib.rs/crates/leptos-motion-core); [docs.rs leptos-motion 0.9.0](https://docs.rs/leptos-motion/0.9.0)
- **leptos_animate** offers FLIP, in/out transitions and custom-animation helpers. Its README says it is early-stage, may have breaking changes, and uses unstable web-sys APIs. The latest version seen is 0.1.10. — [docs.rs leptos_animate 0.1.6 README](https://docs.rs/crate/leptos_animate/0.1.6/source/README.md); [docs.rs leptos_animate latest](https://docs.rs/crate/leptos_animate/latest/source/Cargo.toml)
- **animato** (1.7.x) is a recent tween/animation crate. It has a Leptos `AnimatedFor` list lifecycle: removed rows stay mounted for their exit animation, and the surviving rows use FLIP. — [docs.rs animato 1.7.2](https://docs.rs/crate/animato/1.7.2); [animato README](https://docs.rs/crate/animato/1.7.1/source/README.md)
- **slik** (0.2.0) is Motion-inspired for Leptos. It has three layers: a MotionValue for scalars, `use_motion` bound to typed NodeRefs, and HTML components. It handles opacity and transform. — [docs.rs slik 0.2.0](https://docs.rs/crate/slik/0.2.0)
- **spanda** is a tween library with a Leptos guide. An `on_update` callback bridges values into signals, and a `RafDriver` serves wasm. — [spanda leptos_guide](https://docs.rs/crate/spanda/0.9.0/source/docs/leptos_guide.md)
- **leptos_animation** is an older crate built on AnimationContext and animated signals, and appears to predate the current Leptos API. — [docs.rs leptos_animation](https://docs.rs/leptos_animation)
- **leptos-use** is the de-facto utility crate. It includes `use_raf_fn` (calls a function on every requestAnimationFrame, with pause/resume), `use_intersection_observer`, and `use_scroll` (reactive scroll position, scroll-end, scroll-to). The latest seen is 0.18.2, and its README maps each version to a Leptos version. — [leptos-use use_raf_fn](https://leptos-use.rs/animation/use_raf_fn.html); [use_intersection_observer](https://leptos-use.rs/elements/use_intersection_observer.html); [use_scroll](https://leptos-use.rs/sensors/use_scroll.html); [leptos-use 0.18.2 README](https://docs.rs/crate/leptos-use/0.18.2/source/README.md)
- **dioxus-motion** (0.3.x, by wheregmis) provides spring and tween animations, color interpolation, transforms and sequences through a `use_motion` hook. Custom types work via the `Animatable` trait. A `MotionTransitions` derive gives route/page transitions behind a `transitions` feature. It targets web, desktop and mobile. The README notes that nested routers are not fully tested. — [docs.rs dioxus-motion](https://docs.rs/dioxus-motion); [rust-digger dioxus-motion](https://rust-digger.code-maven.com/crates/dioxus-motion); [lib.rs features](https://lib.rs/crates/dioxus-motion/features)
- **dioxus-spring** also exists for spring animations in Dioxus. — [rust-digger dioxus-spring](https://rust-digger.code-maven.com/crates/dioxus-spring)

### Inferences
- Version numbers like leptos-motion's "1.1.0" do not signal maturity. Download counts are tiny and one crate was yanked. Several of these crates show signs of rapid, possibly AI-assisted generation, such as inflated README claims. Vet each one by reading its source.
- The pragmatic Leptos stack: leptos-use for triggers (scroll, intersection, rAF), CSS/WAAPI for the actual motion, and leptos_animate or animato for FLIP list animations.
- On Dioxus, dioxus-motion is the reasonable default for spring physics and route transitions.

### Gaps
- I did not verify GitHub stars, last-commit dates or open issue counts for these crates.
- I found no "leptos-animate" with a hyphen distinct from leptos_animate.

## 2. Browser-native features usable from Rust, and browser support in 2026

### Takeaway
By Nov 2026 the big native features are cross-browser. CSS scroll-driven animations became Baseline in Oct 2026 with Firefox 159. Same-document View Transitions have been Baseline since Oct 2025. Cross-document View Transitions are full in Chrome and Safari and partial in Firefox. All of them are plain CSS or a single JS call, so they work identically from Rust.

### Cited Findings
- **Scroll-driven animations** (`animation-timeline: scroll()/view()`): web.dev says "Firefox 159 enables scroll-driven animations by default" and that, with Chrome, Safari and Firefox 159 shipping, the feature is "Baseline Newly available" (October 2026). — [web.dev, New to the web platform in October 2026](https://web.dev/blog/web-platform-10-2026)
- Safari 26 shipped scroll-driven animations in Sept 2025, and 26.4 added threaded (off-main-thread) scroll-driven animations. — [lilting.ch article](https://lilting.ch/en/articles/chrome-146-scroll-animation-safari-grid-lanes)
- **Outdated info flag:** many early-to-mid 2026 articles still say Firefox has the feature behind a flag; one cites Firefox 152 in June 2026. That is superseded by Firefox 159. — [buildmvpfast](https://www.buildmvpfast.com/blog/css-scroll-driven-animations-replace-js-2026); [web-features explorer](https://web-platform-dx.github.io/web-features-explorer/features/scroll-driven-animations)
- **Same-document View Transitions** (`document.startViewTransition`) became Baseline Newly available in Oct 2025. Firefox 144 shipped them. — [web.dev Baseline post](https://web.dev/blog/same-document-view-transitions-are-now-baseline-newly-available?hl=de); [web.dev Oct 2025](https://web.dev/blog/web-platform-10-2025)
- **Cross-document View Transitions** (`@view-transition { navigation: auto }`, MPA/SSR page navigations), per caniuse: Chrome 126+ yes, Edge 126+ yes, Safari 18.2+ yes (desktop and iOS), Opera 112+ yes. Firefox 143 has it disabled by default and 144 to 163 have only *partial* support. Global support is about 89.6%. — [caniuse cross-document view transitions](https://caniuse.com/cross-document-view-transitions)
- Cross-document View Transitions are part of the Interop 2026 focus areas. — [search summary citing Interop 2026; see Chrome docs](https://developer.chrome.com/docs/web-platform/view-transitions/cross-document)
- **Web Animations API from Rust:** web-sys generates `Animation` (and related KeyframeEffect) bindings from WebIDL, so `element.animate()` is callable directly from Rust. — [web-sys gen_Animation.rs](https://docs.rs/crate/web-sys/0.3.99/source/src/features/gen_Animation.rs)
- **Leptos router** contains an internal `view_transition` module with `start_view_transition`. It is `pub(crate)`, so it is not a documented public API in the 0.8.x source. — [leptos_router 0.8.7 lib.rs](https://docs.rs/crate/leptos_router/0.8.7/source/src/lib.rs)

### Inferences
- **Yandex Browser** is Chromium-based, so it should track Chrome's support with some version lag. caniuse does not list it, so confirm it in testing.
- Leptos SSR with real page navigations could use cross-document View Transitions purely in CSS. For client-side routing, call `document.startViewTransition` via web-sys or js-sys around the route change. In Firefox, treat cross-document transitions as progressive enhancement.
- Scroll-driven CSS animations run on the compositor (threaded in Safari 26.4+), so they avoid WASM/JS per-frame cost entirely. This is the single biggest enabler of "Apple-style" scroll effects in a Rust site.

### Gaps
- The exact scope of Firefox's "partial" cross-document view transition support is not stated on caniuse.
- No public, documented Leptos API for view transitions was confirmed. Check the leptos_router docs and GitHub issues.
- Yandex Browser support was not directly verified.

## 3. Styling: Tailwind v4, scoped CSS, design tokens

### Takeaway
Tailwind v4 is first-class in Dioxus 0.7: the dx CLI auto-runs it. With Leptos, cargo-leptos has v4 handling. Scoped CSS is available via stylance, stylers, leptos-scoped-style, Dioxus's built-in `css_module`, and dioxus_style. Design tokens map naturally onto Tailwind v4's CSS-first `@theme` and CSS custom properties.

### Cited Findings
- Dioxus 0.7 has "zero-setup tailwind support built-in". `dx` runs the Tailwind CLI on serve and build when `tailwind.css` exists at the app root. The CLI picks v4 when there is a `tailwind.css` and v3 when there is a `tailwind.config.js/ts`, and it downloads a pinned Tailwind v4.1.5 binary. — [Dioxus 0.7 docs, new app](https://dioxuslabs.com/learn/0.7/guide/new_app); [dioxus-cli 0.7.8 tailwind.rs](https://docs.rs/crate/dioxus-cli/0.7.8/source/src/tailwind.rs)
- cargo-leptos has v4-specific config detection (JS config no longer required in v4). Many older Leptos examples still show the v3 pattern with `tailwind.config.js` and a separate `npx tailwindcss --watch`. — [cargo-leptos 0.3.4 tailwind.rs](https://docs.rs/crate/cargo-leptos/0.3.4/source/src/config/tailwind.rs); [leptos tailwind examples](https://cdn05042023.gitlink.org.cn/dnrops/leptos/src/branch/main/examples/tailwind_axum/README.md)
- **stylance** (0.8.0) is a library plus CLI for scoped CSS modules in Rust. It has an import macro and generates one CSS file with hashed class names. It is framework-agnostic. — [docs.rs stylance 0.8.0](https://docs.rs/crate/stylance/0.8.0)
- **stylers** is a scoped-CSS style macro for component frameworks like Leptos. **leptos-scoped-style** uses Angular-style emulated encapsulation. — [docs.rs stylers](https://docs.rs/stylers); [docs.rs leptos-scoped-style](https://docs.rs/leptos-scoped-style)
- Dioxus has a `css_module` attribute macro for globally unique scoped classes. **dioxus_style** (0.5.x) offers scoped CSS/SCSS with compile-time processing. — [dioxus css_module](https://docs.rs/dioxus-native/0.7.10/dioxus_native/prelude/attr.css_module.html); [dioxus_style](https://docs.rs/dioxus_style)

### Inferences
- Since visual "wow" is mostly CSS, the styling layer is framework-neutral. Tailwind v4 plus hand-written CSS for @keyframes, view-timeline and view-transition pseudo-elements is the lowest-risk path for both Leptos and Dioxus.
- Design tokens can live as CSS variables in Tailwind v4's `@theme`. No Rust-specific token tooling appeared necessary or prominent.

### Gaps
- I found no source confirming a "leptos-style" crate.
- I did not confirm whether cargo-leptos bundles a pinned Tailwind v4 version.

## 4. UI component libraries in Rust

### Takeaway
They are usable but well behind React's shadcn/Radix ecosystem. For Leptos: Thaw (Fluent, the most established), rust-ui (shadcn-style copy-paste, Tailwind), biji-ui (headless, small), plus newer singlestage and dui-leptos. For Dioxus: first-party Dioxus Primitives (a Radix port), Lumen Blocks (shadcn-style on top of it), dioxus_components and dioxus-tw-components. None of them gives designer-grade animation out of the box.

### Cited Findings
- **Thaw** follows Fluent Design. 0.4 targets Leptos 0.7, and a 0.5-beta on main targets 0.8. An aggregator listed about 495 stars and a last push in Aug 2025. — [ecosyste.ms thaw](https://awesome.ecosyste.ms/projects/github.com%2Fthaw-ui%2Fthaw); [thaw README](https://docs.rs/crate/thaw/latest/source/README.md)
- **rust-ui** offers Leptos + Tailwind copy-paste components (accordion, dialog, dropdown, toast, …). Some component docs have mismatched descriptions, a sign of rough docs. — [rust-ui.com components](https://rust-ui.com/docs/components)
- **singlestage** (0.4.1) is based on Basecoat, shadcn/ui and Radix. **dui-leptos** (0.5.0) advertises 29 accessible signal-driven components with a dark-first design system. — [singlestage](https://docs.rs/crate/singlestage/latest); [dui-leptos](https://docs.rs/crate/dui-leptos/latest)
- **biji-ui** (0.3.2, targets Leptos 0.8, depends on leptos-use 0.16) is headless and unstyled, inspired by HeadlessUI and Melt UI. Its components are Accordion, Dialog, Menu, Menubar and Tooltip only. — [docs.rs biji-ui 0.3.2](https://docs.rs/crate/biji-ui/0.3.2)
- An opinion post argues nothing with shadcn's maturity exists for Leptos yet and recommends owning thin Tailwind-styled components. — [vp0.com blog](https://vp0.com/blogs/rust-leptos-ui-component-library)
- **Dioxus Primitives** (DioxusLabs/components) is first-party, ARIA-accessible, unstyled and based on Radix Primitives. It was announced with Dioxus 0.7, and many components are still being stabilized. **Lumen Blocks** is shadcn-inspired and built on the primitives. **dioxus_components** 0.1.2 uses Tailwind v4 with Radix patterns. **dioxus-tw-components** targets Dioxus 0.7. — [ecosyste.ms DioxusLabs/components](https://awesome.ecosyste.ms/projects/github.com%2Fdioxuslabs%2Fcomponents); [lumen-blocks](https://docs.rs/crate/lumen-blocks/0.0.0/source/README.md); [dioxus_components](https://docs.rs/crate/dioxus_components/0.1.2/source/README.md); [dioxus-tw-components](https://lib.rs/crates/dioxus-tw-components)

### Inferences
- For a bespoke, high-design storefront, you will likely hand-build visual components with Tailwind, and use headless crates (biji-ui or Dioxus Primitives) only for accessibility-heavy widgets like dialogs, menus and tooltips.

### Gaps
- No current data on **Leptonic**. It did not appear in 2026 results and may be stale or unmaintained, so verify on GitHub.
- No data on a separate "leptos-shadcn" crate's activity.

## 5. Calling JS animation libraries (GSAP, Lenis, Motion) from Rust

### Takeaway
Technically straightforward. Declare the library with a `#[wasm_bindgen] extern "C"` block, or call it through `js_sys::Reflect` / an inline JS snippet module, then load the library as a script or ES module. It breaks "100% Rust" only in that a JS dependency ships. Glue code stays small, and the per-frame work runs in JS, so performance is fine.

### Cited Findings
- I found no published guide specifically for GSAP + Leptos via wasm-bindgen. The standard pattern is a wasm-bindgen extern block importing `gsap` with `js_namespace`, but no search result confirmed it with a worked example. — [search result summary; see wasm-bindgen/leptos_animate refs](https://docs.rs/crate/leptos_animate/latest/source/Cargo.toml)
- leptos-motion-core explicitly lists "GSAP, SVG, Lottie exports not implemented". This is evidence that Rust-native crates do not replace GSAP features. — [lib.rs leptos-motion-core](https://lib.rs/crates/leptos-motion-core)

### Inferences
- With native scroll-driven animations Baseline in 2026, the need for GSAP ScrollTrigger is reduced. Smooth scrolling (Lenis) and complex timelines/SplitText-style text effects remain reasons to pull in JS.
- GSAP has been free (including plugins) since Webflow acquired it in 2024–2025. That comes from training knowledge and was not re-verified here.
- Mixing a few KB of JS glue does not undermine a Rust codebase in practice. Both Leptos and Dioxus already ship a JS loader shim for the WASM.

### Gaps
- No verified code example or benchmark of GSAP/Lenis/Motion driven from Leptos or Dioxus.

## 6. WASM performance for animation, and bundle size

### Takeaway
Every DOM or style write from WASM crosses the JS boundary. That is cheap per call, but per-frame loops over many elements add up. Best practice is to drive motion with CSS/WAAPI (compositor-threaded) and use Rust only to set targets or classes. Bundle size is the bigger cost: typical Leptos/Dioxus apps ship roughly 100 to several hundred KB of compressed WASM, which matters most for time-to-interactive.

### Cited Findings
- WASM cannot touch the DOM directly, so every DOM operation is a JS call. One (unverified) estimate puts boundary cost at about 10–100 ns per call, mattering in tight loops. — [webassembly integration, Steve Kinney](https://stevekinney.com/courses/react-performance/webassembly-integration)
- A 2026 write-up found Rust/WASM was ~25% slower than TypeScript in its case because crossing cost (especially serde_json marshalling) dominated when inputs and outputs live in JS. — [urandom.io 2026](https://urandom.io/blog/2026-03-21-rust-wasm-slower-than-typescript); [kunalganglani.com](https://www.kunalganglani.com/blog/rust-wasm-vs-typescript-performance.md)
- Sledgehammer, a Dioxus-ecosystem project, batches DOM calls to JS to cut boundary overhead. — [sledgehammer crate](https://crates-io.ganymede-static.progval.net/crates/sledgehammer/0.1.0)
- The Leptos book recommends a `wasm-release` profile with `opt-level='z'`, LTO and `codegen-units=1`, and notes that in SSR apps WASM size mainly affects time-to-interactive. — [Leptos book: binary size](https://book.leptos.dev/deployment/binary_size.html)
- Example: flux.dev (Zola + Leptos islands) ships zero WASM on blog posts and about 180 KB of WASM on the homepage playground. — [building flux website in rust](https://flux-web-357.pages.dev/blog/building-flux-website-in-rust)
- wasm-opt -Oz plus a size profile typically cuts size by 40–55%. Gzip brings WASM to roughly 35% of raw, and Brotli is 15–30% better than gzip. — [dwasm](https://dirmacs.github.io/dwasm); [kimg binary size](https://www.mintlify.com/iamkaf/kimg/advanced/wasm-binary-size); [salivity](https://salivity.github.io/wasm/article/webassembly-impact-on-web-app-bundle-size)

### Inferences
- Animating ~10–50 values per frame via `use_raf_fn` or dioxus-motion is fine. Hundreds of elements per frame (particle effects, large staggered grids) should go to CSS, WAAPI, canvas or WebGL instead.
- Leptos islands mode (SSR + minimal hydration) is a good fit for a content- and visual-heavy shop: most pages ship little or no WASM, and CSS does the animation.

### Gaps
- No rigorous rAF-from-WASM per-frame benchmark found.
- No measured 2026 baseline WASM size for a minimal Leptos 0.8 or Dioxus 0.7 app. A "5 KB gzipped" Leptos claim on DEV Community is unsourced and implausible, so ignore it.

## 7. Examples of visually impressive Rust-frontend sites

### Takeaway
I found no Awwwards-recognized site built with Leptos or Dioxus. Existing showcases are competent developer sites, not design-award showpieces.

### Cited Findings
- flux.dev is built entirely in Rust (Zola + Leptos WASM islands). — [flux blog](https://flux-web-357.pages.dev/blog/building-flux-website-in-rust)
- leptos.dev is the official site, built with Leptos. moturbo.com is a motorcycle-parts shop built with Leptos, a rare e-commerce example. benw.is is a Leptos SSR blog. — [awesome-leptos](https://de.github.com/leptos-rs/awesome-leptos); [leptos-website](https://awesome.ecosyste.ms/projects/github.com%2Fleptos-rs%2Fleptos-website)

### Inferences
- The lack of award-winning showcases reflects ecosystem size and designer adoption more than technical limits: the same CSS and browser APIs apply. Expect to be a pioneer, and budget for custom work.

### Gaps
- I did not search Awwwards directly, and found no confirmed Rust-frontend award entries.
- moturbo.com's visual quality was not inspected.
