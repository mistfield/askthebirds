# 3D bird cage configurator in Rust/WASM (Leptos/Dioxus), state as of Nov 2026

Context: there is already a working prototype in three.js r128 (~140 KB HTML, Box/Cylinder geometry, Lambert materials). The question is which Rust approach to use for a configurator with dimension controls, placement of perches, feeders and toys, orbit camera, and PBR wood and metal.

## 1. Bevy on WASM: bundle size, startup, embedding, host communication, PBR, picking/gizmos, mobile

### Takeaway
Bevy is the most capable Rust option and the only one with a ready Leptos bridge (leptos-bevy-canvas 0.6 for Bevy 0.19 / Leptos 0.8). Its web payload is still measured in megabytes, though: about 7.7 MB raw / 2.0 MB gzip in older minimal 3D builds, 12–15 MB after wasm-opt for typical builds, and up to 44 MB for a full app. There is no published production configurator built on Bevy, and size and startup are the main risks for a shop.

### Cited Findings
**Versions and pace (current info)**
- Bevy release history: 0.20 on 2026-10-08, 0.19 on 2026-06-19, 0.18 on 2026-01-13, 0.17 on 2025-09-30, 0.16 on 2025-04-24. Releases come roughly every 3–4 months and each one breaks the API. — [Bevy News](https://bevy.org/news/)
- Bevy 0.20: work on Solari (ray tracing, not relevant to the web), a new pan-orbit camera (`DefaultPanOrbitCameraPlugins`, which needs `MeshPickingPlugin` added alongside it), and new Feathers widgets (Dropdown, Number Input, etc.). Basis Universal/UASTC is described as "the best choice for cross-platform distribution (including WebGPU)", and mesh shaders are unavailable on the web. — [Bevy 0.20 release notes](https://bevy.org/news/bevy-0-20/)
- Bevy 0.19: contact shadows, rectangular area lights (feature `area_light_luts`), physically based SSR, parallax-corrected cubemaps, and white furnace test fixes (more correct IBL). The release adds a built-in transform gizmo (`TransformGizmoSettings`/`TransformGizmoMode`; "deliberately not connected to user input"), an infinite grid, and cancellable tasks on WASM. — [Bevy 0.19 release notes](https://bevy.org/news/bevy-0-19/)
- The picking events `Pointer<Press>` appear in the 0.19 BSN examples, which shows picking is built into the engine. — [Bevy 0.19 release notes](https://bevy.org/news/bevy-0-19/)

**Bundle size (most data is dated, see Gaps)**
- In 2023 (Bevy ~0.10), Reddit reported that unoptimized examples were ~22 MB and opt-level="z" + LTO + wasm-opt could bring them down to ~12 MB or less. — [r/rust thread (via search summary)](https://axebps-redlib.hf.space/r/rust/comments/13lb0h8/bevy_webgpu/jkr35ga/?context=3)
- Crowded Plaza (Bevy 0.7, a 3D io game): ~7.7 MB wasm uncompressed, ~2.0 MB after gzip -9. — [maajor/crowded-plaza](https://www.github.com/maajor/crowded-plaza)
- Another figure: "upwards of 30MB (reduced down to 15MB with wasm-opt)". The page is marked outdated (Bevy 0.12). — [Bevy Cheatbook: WASM](https://bevy-cheatbook.github.io/platforms/wasm.html), [Cheatbook size-opt](https://bevy-cheatbook.github.io/platforms/wasm/size-opt.html)
- A Bevy 0.16 app embedded in a Leptos page weighs about 44 MB. This gave a 1-second delay to first contentful paint even on a high-end desktop, which the author called "unacceptable". For comparison, an app using only Bevy ECS + glow (no Bevy renderer) is ~450 kB compressed. "The hurdles to bundle splitting Wasm are high." — [nickb.dev: A Bevy app entirely off the main thread](https://nickb-dev.pages.dev/blog/a-bevy-app-entirely-off-the-main-thread)

**Embedding and host communication**
- A Bevy app can be built with `default-features=false` and only the needed features (that author used `bevy_sprite`, `bevy_window`, `webgpu`, without winit). It can be moved into a Web Worker via `OffscreenCanvas` (`transferControlToOffscreen`) and driven by `requestAnimationFrame`, with control through Comlink plus a `ResizeObserver`. The author noted that the `finish`/`cleanup` calls are critical and hard to find. — [nickb.dev](https://nickb-dev.pages.dev/blog/a-bevy-app-entirely-off-the-main-thread)
- Bevy CLI: `bevy run web` / `bevy build web --bundle` compile to Wasm, generate JS bindings, serve the app locally, and produce a deployable bundle. — [Bevy CLI web docs](https://thebevyflock.github.io/bevy_cli/cli/web.html)
- The Wavedash guide recommends Trunk (`trunk build --release`) and mounting the canvas into a container. — [Wavedash: Bevy](https://docs.wavedash.com/engines/bevy)

**Backends and limitations**
- The 2023 WebGPU announcement says WebGL2 is still the default web backend for Bevy apps "until WebGPU support is more universal". — [Bevy + WebGPU](https://bevy.org/news/bevy-webgpu/)
- The `webgl2` and `webgpu` cargo features select the backend, and webgpu takes precedence if both are enabled (from a Reddit comment, not confirmed in official docs). — [r/bevy thread](https://axebps-redlib.hf.space/r/bevy/comments/1k99chb/bevy_using_webgl2_backend)
- Features that use compute shaders (e.g. bevy_hanabi particles) require WebGPU and do not work on WebGL2. — [bevy_hanabi wasm.md](https://rust-digger.code-maven.com/src/bevy_hanabi-0.14.0/docs/wasm.md)
- Known WASM problems: very poor performance on low-end hardware (issue #6363); Bevy's 3D renderer stopped working on WebGL2 in Firefox 128 on Linux/AMD Mesa; shadows in the wasm lighting example needed cascade tuning. — [bevy#6363](https://github.com/bevyengine/bevy/issues/6363), [Mozilla bug 1912404](https://bugzilla.mozilla.org/show_bug.cgi?id=1912404), [bevy#7397](https://github.hscsec.cn/bevyengine/bevy/issues/7397)
- WebAssembly in Bevy does not support multithreading (the source is marked outdated, Bevy 0.12). — [Bevy Cheatbook WASM](https://bevy-cheatbook.github.io/platforms/wasm.html)

### Inferences
- A realistic estimate for a 3D configurator on Bevy 0.19/0.20 with a trimmed feature set (bevy_pbr, bevy_gltf, picking, one backend) is several MB of wasm after wasm-opt and roughly 2–5 MB after brotli/gzip. That is more than 10× the three.js prototype. This is an extrapolation from the old figures; it must be verified with a test build.
- WebGL2 and WebGPU are chosen at compile time. Supporting both means two wasm files plus JS detection (`navigator.gpu`) to pick one, which doubles CI and cache cost.
- On mid-range mobile devices, expect download time plus compiling a multi-megabyte wasm before the first frame. Reasonable assumption: seconds over 4G. A placeholder (image or skeleton) and a "Launch 3D" button are mandatory.
- Bevy's pace of 3–4 breaking releases a year combined with leptos-bevy-canvas means upgrade costs are paid regularly.

### Gaps
- No current (Bevy 0.19/0.20) measurements of wasm size or startup time were found for a minimal 3D PBR scene. A test build is needed: `bevy build web --release`, then wasm-opt -Oz and brotli.
- Mobile performance of Bevy WASM in 2026: no measurements found.
- Whether 0.20 includes a pan-orbit camera controller usable without third-party crates is only confirmed by the release notes mention; the API was not checked.

## 2. Alternatives: three-d, kiss3d, rend3, Fyrox, macroquad, raw wgpu

### Takeaway
three-d is the most realistic lightweight alternative (WebGL2, PBR, Rust-only API, much smaller than Bevy), but it is pre-1.0 and has no editor or picking infrastructure. kiss3d is for prototypes and visualization. rend3 is archived. Fyrox 1.0 is a game engine with a heavy web build. Raw wgpu means writing your own renderer, which is overkill. macroquad: no current data was found.

### Cited Findings
- **three-d**: WebGL2 via WASM in Chrome/Firefox/Edge/Safari; WebGPU is not mentioned. It ships a PBR example, and assets (models, textures) load through the separate `three-d-asset` crate. The README says "do expect regular breaking changes until a 1.0.0 release". Activity: 4,477 commits, 23 open issues. — [asny/three-d](https://github.com/asny/three-d)
- The old `three-d-gltf-import` is marked "still under development" and was last published in 2021; do not use it. — [three-d-gltf-import](https://docs.rs/crate/three-d-gltf-import/0.0.3)
- New Rust scene libraries found in search: `scena` (glTF/GLB, WebGPU/WebGL2 in the browser, version 1.10.x) and `myth-engine` 0.2 (PBR, WASM+WebGPU). Both are young with little history. — [scena](https://docs.rs/crate/scena/1.10.6), [myth-engine](https://docs.rs/crate/myth-engine/latest/source/docs/UserGuide.md)
- **kiss3d** (dimforge): actively developed, version 0.45.1. It runs on native and WASM with the same code (async API, `#[kiss3d::main]`). — [dimforge/kiss3d](https://github.com/dimforge/kiss3d), [docs.rs kiss3d 0.45.1](https://docs.rs/crate/kiss3d/0.45.1)
- **rend3**: the BVE-Reborn/rend3 repository is archived ("MAINTENCE MODE"); the latest release, 0.3.0, is more than 4 years old. A community fork, rend3-hp, exists. — [ecosyste.ms summary](https://summary.ecosyste.ms/projects/369860), [rend3-hp](https://relatedrepos.com/gh/John-Nagle/rend3-hp)
- **Fyrox**: 1.0 has been released (after ~7 years of development). It has its own editor, FyroxEd, and the export CLI supports pc/android/wasm. — [Fyrox 1.0.0](https://fyrox.rs/blog/post/fyrox-game-engine-1-0-0/), [secondary source](https://ai-news-site-1ih.pages.dev/en/news/fyrox-1-0-lands-after-seven-years-in-rust-en)
- **Bevy ECS + glow** (no Bevy renderer) comes to ~450 kB compressed in one real app. This shows the "light renderer on raw GL" path gives a payload an order of magnitude smaller. — [nickb.dev](https://nickb-dev.pages.dev/blog/a-bevy-app-entirely-off-the-main-thread)

### Inferences
- For "boxes, cylinders, wood, metal, orbit camera, picking", three-d covers rendering (PBR, lights; shadows to be checked), while picking (raycast against AABBs/cylinders) and drag-and-drop would have to be written by hand. That is acceptable for a simple scene of parametric primitives.
- Fyrox and Bevy are game engines. A configurator uses perhaps 10% of their capabilities but pays the full size of the runtime.
- Raw wgpu: a WebGPU+WebGL2 backend through one API, but PBR, shadows, IBL and glTF would all have to be written from scratch. That is weeks of graphics programming.

### Gaps
- macroquad: no current information on 3D/PBR capabilities in 2026 was found. From general knowledge (unverified), it is a simple 2D-first framework with basic 3D and no PBR, so it does not fit.
- Current three-d version and release date, and whether it supports shadows and environment maps: not confirmed by the sources read.
- No measured three-d wasm sizes were found.

## 3. WebGPU support in browsers (2026) and the WebGL2 fallback

### Takeaway
WebGPU works in Chrome/Edge/Opera/Samsung on desktop and Android, and in Safari 26+ including iOS. Firefox is the weak spot: Windows only since 141 and Apple Silicon macOS since 145/147, while caniuse still marks it "disabled by default", and Android Firefox has no support. Overall coverage is ~88.6%. A WebGL2 fallback is still needed for the shop.

### Cited Findings
- caniuse (Nov 2026): Chrome 113–160 supported; Edge 113–156; Safari 26.0–27.3 TP "partial"; Safari iOS 26.0–27.2 supported; Firefox 63–163 "disabled by default"; Firefox Android 160 "disabled by default"; Chrome Android 157 supported; Opera 99–139; Samsung Internet 24–31. Global: 88.63% (86.83% full + 1.8% partial). Yandex Browser is not listed separately. — [caniuse WebGPU](https://caniuse.com/webgpu)
- Late 2025: web.dev announced WebGPU support in all major browsers. Safari 26 (Sept 2025) brought WebGPU to macOS/iOS/iPadOS/visionOS; Firefox 141 shipped it on Windows. — [web.dev](https://web.dev/blog/webgpu-supported-major-browsers), [AppDeveloperMagazine](https://appdevelopermagazine.com/WebGPU-in-iOS-26/)
- Firefox: sources disagree on macOS (145 vs 147 for Apple Silicon). Linux and Intel Macs have no WebGPU by default; Android is "targeted for late 2026" (aggregator, unverified). — [webo360solutions](https://webo360solutions.com/blog/webgpu-browser-support), [youngju.dev](https://www.youngju.dev/blog/culture/2026-05-16-webgpu-wgsl-ecosystem-2026-chrome-safari-firefox-wgpu-naga-tint-three-js-babylon-webllm-transformers-deep-dive.en)
- Chrome Android: WebGPU since Chrome 121 on Android 12+. — [web.dev](https://web.dev/blog/webgpu-supported-major-browsers)
- Conflict: caniuse marks Firefox as "disabled by default", while web.dev/news report that 141+ ships it on Windows. caniuse probably marks partial platform support this way. Worth checking on MDN.

### Inferences
- Yandex Browser is built on Chromium, so WebGPU most likely follows Chromium versions, but this is unconfirmed (it is not in caniuse) and needs real-device testing. This matters for a Russian-speaking audience.
- Practical rule for 2026: WebGL2 is the baseline that works everywhere (including Firefox Linux/Android and old iPhones below iOS 26), and WebGPU is an optional improvement. A configurator of simple primitives gains nothing visible from WebGPU, so a WebGL2-only build is the simplest choice.

### Gaps
- Exact Yandex Browser numbers (desktop and Android) were not found.
- The share of iOS devices still on versions below 26 (no WebGPU) in Nov 2026 is unknown.

## 4. glTF, PBR, shadows, environment maps in each option

### Takeaway
Bevy has the most complete ready-made stack (glTF loader, PBR, cascaded shadows, IBL/environment maps, KTX2/Basis). three-d has PBR and asset loading through three-d-asset, but details are unconfirmed. kiss3d and macroquad are basic. Fyrox is a full engine but heavy.

### Cited Findings
- Bevy: improved IBL correctness (passes the white furnace test), parallax-corrected cubemaps, contact shadows, SSR, and the glTF loader moving to the new scene system (0.19). — [Bevy 0.19](https://bevy.org/news/bevy-0-19/)
- Bevy 0.20: the new `Ready` event for logic on top of glTF scenes; Basis Universal/UASTC for cross-platform textures. — [Bevy 0.20](https://bevy.org/news/bevy-0-20/)
- Shadows in Bevy WASM need cascade tuning (2023 issue). — [bevy#7397](https://github.hscsec.cn/bevyengine/bevy/issues/7397)
- three-d: PBR example; models/textures through three-d-asset; WebGL2. — [asny/three-d](https://github.com/asny/three-d)
- scena: glTF/GLB, and falls back to neutral lighting when a glTF has none. — [scena](https://docs.rs/crate/scena/1.10.6)

### Inferences
- For a cage, most geometry is parametric (bars, frame, tray) and generated in code, while glTF is needed only for decorative items (feeders, toys). Any option with a solid glTF loader and PBR (metal = metallic 1/roughness ~0.3; wood = albedo texture + roughness ~0.7) plus one environment map gives a "nice" result. The jump from Lambert to PBR+IBL matters more than the choice of engine.

### Gaps
- Shadows and environment maps in three-d: not confirmed by sources.

## 5. Examples of Rust/WASM configurators and 3D web apps in production

### Takeaway
No public production product configurator built on Bevy or another Rust engine was found. The configurator market is built almost entirely on three.js / React Three Fiber / Babylon.js.

### Cited Findings
- Search results for "Bevy product configurator web wasm" return only commercial three.js/R3F configurators and freelance offers; no Bevy configurator case was found. — [Kickflip: web-based configurator software](https://gokickflip.com/blog/web-based-product-configurator-software), [BeeGraphy](https://beegraphy.com/blog/3d-configurators-made-easy/)
- One real Bevy+Leptos app (a 2D/3D viewer for a game save editor) moved Bevy into a worker because of a 44 MB payload; its author built a lighter path (ECS + glow, ~450 kB). — [nickb.dev](https://nickb-dev.pages.dev/blog/a-bevy-app-entirely-off-the-main-thread)
- Bevy's examples run in the browser on WebGL2 and WebGPU (a showcase, not a product). — [Bevy WebGPU examples](https://bevyengine.org/examples-webgpu)

### Inferences
- Choosing Rust for the 3D part of a shop is an ideological or stack decision, not a proven industry path. The risk falls on the team.

### Gaps
- No public metrics (conversion, load time) for Rust-based configurators.

## 6. Integration with Leptos/Dioxus and lazy-loading

### Takeaway
Leptos has a mature bridge: leptos-bevy-canvas 0.6 (Leptos 0.8 + Bevy 0.19), with messages both ways and sync of Resource/Query/State into RwSignal. Dioxus has only experimental crates (bevy_dioxus_sync 0.1, bevy_dioxus for desktop). The robust pattern is a separate wasm bundle for the 3D part, loaded on a separate route or on click, communicating through a thin JS/CustomEvent/postMessage API.

### Cited Findings
- leptos-bevy-canvas 0.6.0, compatibility: 0.6 = Leptos 0.8 + Bevy 0.19; 0.5 = Bevy 0.18; 0.4 = Bevy 0.17; 0.3 = Bevy 0.16; 0.1–0.2 = Leptos 0.7 + Bevy 0.15. Features: messages Leptos→Bevy, Bevy→Leptos and both ways; sync of a Bevy `Resource`/`Query`/`State` with an `RwSignal`; the `<BevyCanvas>` component (default canvas `#bevy_canvas`). Docs are 61% complete. There is no version for Bevy 0.20 yet. — [docs.rs leptos-bevy-canvas](https://docs.rs/leptos-bevy-canvas/), [lib.rs](https://lib.rs/crates/leptos-bevy-canvas)
- The Leptos repo has had a bevy3d_ui example (Bevy 3D + Leptos UI). — [leptos projects/bevy3d_ui](https://cdn05042023.gitlink.org.cn/dnrops/leptos/src/branch/main/projects/bevy3d_ui)
- Dioxus: `bevy_dioxus_sync` 0.1.0 (Nov 2025) provides hooks for passing resources/components/assets between Dioxus and Bevy, served with `dx serve --hotpatch`; wasm support is unclear. `bevy_dioxus` is desktop-only and "very early stages". Dioxus PR #4360 renders headless Bevy into a wgpu texture for dioxus_native (not the browser). — [bevy_dioxus_sync](https://docs.rs/crate/bevy_dioxus_sync/0.1.0), [bevy_dioxus](https://docs.rs/crate/bevy_dioxus/0.1.1), [Dioxus PR #4360](https://github.bileizhen.workers.dev/DioxusLabs/dioxus/pull/4360)
- Code splitting a Wasm bundle is hard and uncommon ("hurdles... are high"). — [nickb.dev](https://nickb-dev.pages.dev/blog/a-bevy-app-entirely-off-the-main-thread)
- Pattern: a separate worker module, `instantiateStreaming` (compilation is async and cacheable), a content-hashed wasm URL for long-lived caching, and OffscreenCanvas to keep the main thread free. — [nickb.dev](https://nickb-dev.pages.dev/blog/a-bevy-app-entirely-off-the-main-thread)
- Leptos book: a dedicated section on optimizing WASM size (opt-level z, LTO, wasm-opt). — [Leptos book: binary size](https://book.leptos.dev/deployment/binary_size.html)

### Inferences
- Important: leptos-bevy-canvas places Bevy in the same wasm module as the Leptos app. Without splitting, all shop pages would load several MB of Bevy. Lazy-loading options:
  1. A separate route/page `/configurator` with its own Leptos (or plain) wasm build containing Bevy. The main shop stays light. This is the simplest and most reliable.
  2. A separate 3D wasm module (Bevy or three-d) loaded with dynamic `import()` on a "Launch 3D" click and mounted into a `<canvas>` from the Leptos component. Communication goes through `wasm_bindgen`-exported functions or CustomEvent/postMessage with a JSON configuration (cage dimensions, list of items with positions). This works the same for Leptos and for Dioxus, and avoids depending on the bridge crate's versions.
  3. Run it in a Web Worker + OffscreenCanvas (as in the nickb.dev article) for UI responsiveness. Note that OffscreenCanvas+WebGL2 on older iOS is a risk to verify.
- For Dioxus, option 2 is the only mature path.
- Configuration state (dimensions, accessories) should live in the host framework (Leptos signals or the Dioxus store) and be passed to 3D as a declarative snapshot. That keeps the cart and price logic independent of the renderer and lets you replace the renderer (Bevy, three-d, or even the three.js prototype) without rewriting the shop.

### Gaps
- No working public example of Dioxus web + Bevy in a browser canvas was found.
- When leptos-bevy-canvas will support Bevy 0.20 is unknown.

## Overall recommendation (inference for the report writer)
- **Default for "Rust everywhere"**: Bevy 0.19 + leptos-bevy-canvas 0.6, compiled as a **separate wasm bundle for the configurator page**, WebGL2 backend (maximum coverage, including Firefox and older iOS), trimmed feature set, wasm-opt -Oz + brotli, content-hashed URL, and a placeholder or "Launch 3D" button. Strengths: PBR/IBL/shadows, glTF, picking, transform gizmo, a pan-orbit camera (0.20) are available out of the box. Costs: payload of several MB, slow startup on mobile, upgrades every 3–4 months.
- **Lightweight alternative**: three-d (WebGL2, PBR) with your own simple picking. The payload is substantially smaller, but drag-and-drop, gizmos and the camera need more hand-written code, and the library is pre-1.0.
- **Pragmatic hybrid** (not Rust, but realistic): keep three.js (upgrade from r128 to a current version, switch to MeshStandardMaterial + an HDR environment) as an island inside the Leptos/Dioxus page, with all shop logic in Rust. This has the smallest payload and the shortest time to a good result. It is mentioned because no Rust configurators in production were found.
- Do not use: rend3 (archived), macroquad (no PBR, per general knowledge), raw wgpu (too much work), Fyrox (a game engine with a heavy web build, no data on configurators), kiss3d (fine for prototypes, but limited materials and UI).
