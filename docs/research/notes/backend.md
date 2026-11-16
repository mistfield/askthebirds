# Rust Backend Stack for a Small-to-Medium E-commerce Shop (late 2026)

Research done 2026-11-16. Many version and date facts come from docs.rs, GitHub release pages and official blogs. Some GitHub release pages leave out the year, so where a year is inferred, the notes say so.

**Recommended stack at a glance (synthesis):** Axum 0.8 + Leptos 0.8 (or Dioxus 0.7) SSR · PostgreSQL + sqlx 0.8/0.9 (or SeaORM 2.0 if you want an ORM plus the free SeaORM Pro admin) · tower-sessions + axum-login + argon2 + oauth2 crate (Yandex/VK) + webauthn-rs 0.5 · apalis 1.x (apalis-postgres + apalis-cron) or underway · aws-sdk-s3 / object_store + image + fast_image_resize · Postgres FTS ('russian' config) first, Meilisearch later · tracing + tracing-opentelemetry · Docker multi-stage with cargo-chef onto a VPS. Do not use Shuttle, which has shut down. Loco.rs 1.x is a valid "batteries-included" alternative that bundles Axum + SeaORM 2 + jobs.

## Web frameworks: Axum vs Actix Web vs Loco.rs vs Poem/Salvo/Rocket

### Takeaway
Axum (tokio-rs, 0.8.x line) is the default choice. It is actively patched in 2026 and both Leptos and Dioxus fullstack build on it. Actix Web 4.x is also very active (several releases in Nov 2026). Loco.rs reached 1.0 in July 2026 and is now a credible Rails-like layer on top of Axum + SeaORM.

### Cited Findings
- Axum 0.8.0 was announced Jan 1, 2025 by the Tokio team. — [Tokio blog](https://tokio.rs/blog/2025-01-01-announcing-axum-0-8-0)
- The latest Axum is a 0.8.x patch: 0.8.9, reportedly published Apr 2026, adds a WebSocketUpgrade protocol method. No 0.9 had shipped as of the search. — [docs.rs axum 0.8.9](https://docs.rs/crate/axum/0.8.9/source/README.md); [releasealert](https://releasealert.dev/github/tokio-rs/axum)
- Actix Web latest is v4.19.0 (09 Nov), preceded by v4.18.2 (06 Nov). actix-http 3.21.0 and actix-multipart 0.8.8 were released the same week. The page shows no year; it is presumably 2026. — [GitHub actix-web releases](https://github.com/actix/actix-web/releases)
- Loco.rs v1.0.0 was the first stable release, with notes dated 2026-07-25. It moved to SeaORM 2.0 and sqlx 0.9, with MSRV 1.94. — [GitHub loco-rs releases](https://github.com/loco-rs/loco/releases)
- Loco releases since 1.0:
  - 1.1.0 (Aug): Tera 2, AWS Lambda deploy, `jobs retry`
  - 1.2.0 (Sep): presigned storage URLs, row-level multi-tenancy, Postgres-only builds
  - 1.3.0 / 1.3.1 (Nov 5–6): notifications, DB-backed cache, server-side sessions
  - Background workers support Postgres, SQLite and Redis queues. Sidekiq-compat was replaced in 0.16.

  The releases page describes no built-in admin panel, only CLI tasks. — [GitHub loco-rs releases](https://github.com/loco-rs/loco/releases)
- Dioxus 0.7 has "full Axum integration" for server functions and upgraded to Axum 0.8. A `DioxusRouterExt` trait mounts into an existing Axum router. dioxus-server is at 0.7.10, and 0.8.0-alpha.1 exists. — [Dioxus v0.7.0 release](https://github.bileizhen.workers.dev/DioxusLabs/dioxus/releases/tag/v0.7.0); [Dioxus Axum guide](https://dioxuslabs.com/learn/0.7/guides/fullstack/axum); [docs.rs dioxus-server](https://docs.rs/crate/dioxus-server/0.8.0-alpha.1)
- Leptos latest is 0.8.20. Its official starters are cargo-leptos templates for Axum (`start-axum`) or Actix. — [docs.rs leptos](https://docs.rs/crate/leptos/latest); [leptos_axum docs](https://docs.rs/leptos_axum/0.8.9/src/leptos_axum/lib.rs.html)
- Dioxus fullstack reuses Leptos' server_fn crate, but lacks Leptos' Suspense-based streaming HTML SSR. — [search summary of GitHub comparison](https://github.com/Buzzec/leptos) (secondary; treat as indicative)

### Inferences
- For an SSR shop frontend in Rust, Axum is the natural backbone. Leptos (best SSR streaming, SEO-friendly) and Dioxus both mount onto an Axum router, so the API, webhooks, admin and SSR can live in a single binary.
- Loco is attractive if the team wants generators, a mailer, a jobs/queue system, storage and auth out of the box. The cost is SeaORM lock-in and Loco's conventions. It is now post-1.0 with monthly releases, so it is no longer "experimental".
- Actix Web stays a solid choice with excellent performance, but has less synergy with the tower ecosystem (tower-sessions, axum-login, tower-http).

### Gaps
- I did not verify the 2026 maintenance status of Poem, Salvo or Rocket. Rocket in particular has historically had long gaps between releases (0.5 in late 2023), so verify before choosing it. I found no 2026 data on these frameworks.
- I found no crates.io download numbers or Rust Survey 2025 framework-share data in this session.

## Database: sqlx vs SeaORM vs Diesel/diesel-async vs Toasty; migrations; compile-time checking

### Takeaway
sqlx (plain SQL, `query!` macros checked against a live DB or an offline cache) is the mainstream async default. SeaORM 2.0 went stable in July 2026 and is the strongest async ORM; it also gives you the free SeaORM Pro admin panel. Diesel 2.x + diesel-async remains the most compile-time-type-safe choice.

### Cited Findings
- SeaORM 2.0.0 stable was released 2026-07-20, with the announcement on Jul 27, 2026, after 43 release candidates. Features:
  - denser entity format with typed relations
  - entity-first workflow that creates tables and FKs
  - nested ActiveModel saves in one transaction
  - strongly typed columns
  - a sync API
  - RBAC and an updated SeaORM Pro admin

  — [SeaQL blog: Announcing SeaORM 2.0](https://www.sea-ql.org/blog/2026-07-27-sea-orm-2.0/)
- Loco 1.0 uses SeaORM 2.0 together with sqlx 0.9, which shows sqlx 0.9 exists by mid-2026. — [GitHub loco-rs releases](https://github.com/loco-rs/loco/releases); Rustfinity also lists sqlx 0.9.0 — [Rustfinity Rust ORMs](https://www.rustfinity.com/blog/rust-orms)
- The sqlx `query!` macro checks SQL against a live DB, and builds need either a connection or cached offline metadata. Its migrations come via a basic sqlx-cli (some teams use Flyway/Liquibase). Diesel validates queries at compile time through generated schema types, but compiles slowly. Diesel's diesel_cli supports up/down migrations, and sea-orm-cli generates migrations and entities. — [Reintech comparison 2026](https://reintech.io/blog/diesel-vs-sqlx-vs-seaorm-rust-database-library-comparison-2026)
- Rustfinity lists diesel-async 0.9.2 and recommends Diesel + diesel-async on Tokio when you want compile-time-checked queries without SQL strings. — [Rustfinity Rust ORMs](https://www.rustfinity.com/blog/rust-orms)
- In SeaORM, schema/code mismatches surface at runtime rather than compile time. — [Rustfinity Rust ORMs](https://www.rustfinity.com/blog/rust-orms)

### Inferences
- For a shop with transactional logic (orders, payment state machine, stock reservations), sqlx + hand-written SQL with offline `.sqlx` metadata in CI is the lowest-risk option. SeaORM 2.0 is the better fit if you want CRUD speed and the ready admin panel.
- Avoid mixing ORMs. If you pick Loco, you get SeaORM by definition.

### Gaps
- Toasty (Tokio's ORM): no 2026 status found. It was experimental/pre-release when I last knew of it, so flag it as not production-ready unless verified.
- The exact current sqlx version (0.8.6 vs 0.9.x) and its release date were not confirmed from a primary source. Sources conflict (Rustfinity says 0.9.0, another guide cites 0.8.6).

## Auth/sessions: tower-sessions, axum-login, argon2, OAuth (Yandex ID / VK ID / Telegram), passkeys

### Takeaway
The standard Axum combo is tower-sessions (with a Postgres store) + axum-login (both by Max Countryman), argon2 (RustCrypto) for password hashes, the `oauth2` crate for Yandex ID / VK ID, and webauthn-rs 0.5 for passkeys. axum-login has had no tagged release since mid-2025, but its repo is still active.

### Cited Findings
- axum-login's latest crates.io version is 0.18.0, updated 2025-07-20. The GitHub repo was "updated last week" as of a May 2026 snapshot. — [docs.rs axum-login](https://docs.rs/crate/axum-login/latest); [relatedrepos](https://relatedrepos.com/gh/maxcountryman/axum-login)
- tower-sessions is at v0.15.0. Its session stores were split into separate crates (the tower-sessions-stores repo). — [openapps tower-sessions](https://openapps.pro/packages/tower-sessions)
- webauthn-rs 0.5 is still the line in use. A 2026 crate (yauth 0.2.4) depends on webauthn-rs ^0.5. — [docs.rs yauth](https://docs.rs/crate/yauth/0.2.4)
- There is a reference implementation of discoverable passkeys with webauthn-rs + tower-sessions on Axum. — [patte/axum-solid-playground](https://github.com/patte/axum-solid-playground)
- Loco 1.3.0 added server-side sessions. — [GitHub loco-rs releases](https://github.com/loco-rs/loco/releases)

### Inferences
- Yandex ID and VK ID are standard OAuth 2.0 / OIDC-like providers. The generic `oauth2` (or `openidconnect`) crate should work with custom endpoints. Telegram Login Widget uses an HMAC-SHA256 signature check over the data fields, which is easy to hand-roll with `hmac` + `sha2`.
- For a parrot shop, guest checkout + email magic link or OAuth is likely enough. Passkeys matter most for the admin panel.

### Gaps
- I found no maintained Rust crates specifically for Yandex ID, VK ID or Telegram login in this session. This needs verification on crates.io.
- I did not confirm the argon2 crate version or a 2026 webauthn-rs release date.

## Background jobs / queues / scheduling

### Takeaway
apalis reached 1.0 in October 2026 and released 1.2.0 on 2026-11-14. It is now the most complete option (Postgres/Redis/SQLite backends, cron, workflows, a board UI). underway (Postgres-only durable step functions) is a lighter alternative that is still pre-1.0. For a Postgres-only shop, both avoid Redis.

### Cited Findings
- apalis 1.2.0 was released 2026-11-14, after 1.0.0 in Oct 2026 and 1.1.0/1.1.1 in Nov 2026.
  - Backends are separate crates: apalis-postgres, Redis, SQLite, MySQL.
  - apalis-cron handles scheduling.
  - apalis-workflow provides sequential, DAG and conditional workflows.

  — [docs.rs apalis](https://docs.rs/crate/apalis/latest)
- apalis-postgres offers polling and trigger-based (LISTEN/NOTIFY) storage. `PostgresStorage::setup(&pool)` creates the tables. apalis-board provides a monitoring UI. — [docs.rs apalis-postgres](https://docs.rs/crate/apalis-postgres/latest/source/README.md)
- underway (Max Countryman) provides durable background jobs and workflows on Postgres: steps, scheduled jobs, and recovery across retries, restarts and deploys. The latest is 0.2.0, with ~158 GitHub stars by Feb 2026. — [docs.rs underway](https://docs.rs/crate/underway/latest); [awesome.ecosyste.ms](https://awesome.ecosyste.ms/projects/github.com%2Fmaxcountryman%2Funderway)
- Loco includes its own job queue with Postgres/SQLite/Redis backends, retry policies, `perform_all_later`, and `cargo loco jobs retry/purge`. — [GitHub loco-rs releases](https://github.com/loco-rs/loco/releases)

### Inferences
- Typical shop jobs are: payment-webhook post-processing, email/SMS notifications, image derivative generation, search-index sync, and abandoned-cart or unpaid-order expiry (cron). All of these fit apalis-postgres + apalis-cron.
- Process payment webhooks idempotently. Store the event ID with a unique constraint and enqueue the work in the same transaction (transactional outbox). Postgres-backed queues make this trivial.

### Gaps
- fang, graphile-worker-rs and tokio-cron-scheduler: 2026 maintenance status not checked. Flag them as "verify before use". apalis-cron or underway's scheduling covers cron needs.

## Admin panel in Rust

### Takeaway
There is no Django-admin equivalent built natively in Rust. The closest is SeaORM Pro, which is free and MIT. It pairs a React/Ant Design-based admin UI with SeaORM + Seaography GraphQL, and 2.0 added RBAC. Loco has no admin UI. Otherwise you build the admin in Leptos/Dioxus or htmx + templates.

### Cited Findings
- SeaORM Pro is "free to use", MIT-licensed on GitHub, with the stated aim of becoming fully open source. 2.0 adds role-based access control. Pro Plus has a GUI RBAC editor. A live demo exists. — [SeaORM Pro site](https://www.sea-ql.org/sea-orm-pro/); [What's new in SeaORM Pro 2.0](https://www.sea-ql.org/blog/2025-11-21-whats-new-in-seaormpro-2.0); [GitHub sea-orm-pro](https://github.com/SeaQL/sea-orm-pro)
- The SeaORM 2.0 announcement lists "an updated SeaORM Pro admin panel" and Seaography 2.0. — [SeaQL blog](https://www.sea-ql.org/blog/2026-07-27-sea-orm-2.0/)
- Loco releases describe only CLI admin tasks (`cargo loco task user:delete`, `jobs purge`), not a web admin. — [GitHub loco-rs releases](https://github.com/loco-rs/loco/releases)
- Other crates exist, such as `adminx` 2.0.1 on docs.rs, but no maturity data was found. — [docs.rs adminx](https://docs.rs/crate/adminx/2.0.1)

### Inferences
- **Effort estimate** (my judgement, not sourced):
  - SeaORM Pro gives basic CRUD for products, categories and orders within days, but custom workflows (order state transitions, refunds, image upload with drag-drop ordering) need custom work.
  - A hand-built Leptos or htmx admin with ~10 screens (products with variants and images, categories, orders list/detail with status actions, customers, promo codes, settings) is roughly 3–6 developer-weeks.
- **Image management flow:** upload to S3 via presigned URL → background job generates WebP/AVIF derivatives → store keys in a `product_images` table with sort order. Loco 1.2 added presigned storage URLs.

### Gaps
- No 2026 data was found on other Rust admin crates (axum-admin-style projects) beyond the existence of adminx. The date of the SeaORM Pro 2.0 post (2025-11 in its URL) conflicts slightly with SeaORM 2.0 going stable in 2026-07; Pro 2.0 probably shipped against the RCs.

## Image handling and storage

### Takeaway
Use the `image` 0.25.x crate for decoding and encoding (WebP via image-webp; AVIF encode via ravif). Use fast_image_resize (SIMD, much faster than image's resize) for thumbnails. Use aws-sdk-s3 or object_store for S3-compatible storage (Yandex Object Storage, Selectel, MinIO alternatives).

### Cited Findings
- The latest image crate is 0.25.10.
  - AVIF encoding is an optional feature using `ravif`, while AVIF decoding uses the `avif-native` feature (dav1d + mp4parse).
  - WebP comes via `image-webp`.
  - The default formats include AVIF and WebP.

  — [docs.rs image](https://docs.rs/crate/image/latest); [image Cargo.toml](https://docs.rs/crate/image/0.25.10/source/Cargo.toml)
- fast_image_resize is at 5.2.x and is SIMD-based. Benchmarks cited in an image-rs issue show image's resize is much slower than fast_image_resize. — [docs.rs fast_image_resize](https://docs.rs/crate/fast_image_resize/5.2.2/source/benchmarks-wasm32.md); [image-rs issue #2223](https://www.github.com/image-rs/image/issues/2223)
- S3 client options:
  - aws-sdk-s3: the official SDK, full S3 feature coverage, safest with provider quirks.
  - object_store (Apache Arrow): one API over S3/GCS/Azure/local, with in-memory and local backends useful for tests.
  - rust-s3 0.37.2: small footprint and its own SigV4 implementation, but it may hit provider quirks.
  - rusty-s3: a sans-IO signer.

  — [Encore: object storage with Axum](https://encore.dev/articles/add-object-storage-to-axum); [docs.rs rust-s3](https://docs.rs/crate/rust-s3/latest); [obstore alternatives](https://developmentseed.org/obstore/latest/alternatives/); [engineered.at B2 article](https://engineered.at/articles/accessing-public-and-private-b2-s3-buckets-in-rust)

### Inferences
- AVIF encoding through ravif is CPU-heavy. Run it in a background job (apalis) via `spawn_blocking`, not in the request path. WebP alone is probably sufficient for a small shop.
- An alternative is to offload resizing to an imgproxy sidecar or a CDN and skip in-process processing entirely.

### Gaps
- I did not verify S3 compatibility specifics for Russian providers (Yandex Object Storage, Selectel, VK Cloud) with each crate.

## Catalog search with Russian morphology

### Takeaway
PostgreSQL FTS with the built-in `russian` Snowball configuration (plus pg_trgm for typo tolerance) is the simplest choice and is enough for a small catalog. Meilisearch (written in Rust) is excellent for typo tolerance and instant search, but its docs list only decomposition and lowercasing for Cyrillic, with no stemming documented. Tantivy can stem Russian via Snowball (tantivy-stemmers). None of the three do true lemmatization.

### Cited Findings
- Meilisearch (Charabia tokenizer) lists "Cyrillic / Georgian (Russian, Ukrainian, Bulgarian, etc.)" with default segmentation and normalization by decomposition + lowercase. Stemming is not mentioned. `localizedAttributes` sets per-language tokenization. — [Meilisearch language docs](https://www.meilisearch.com/docs/resources/help/language)
- A Charabia PR adds a Russian normalizer treating ё/е as equivalent; its merge status is unconfirmed. — [charabia PR #296 mirror](https://github.junw03049948.workers.dev/meilisearch/charabia/pull/296)
- FOSDEM 2026 had a talk "Language support in Meilisearch". — [FOSDEM 2026](https://fosdem.org/2026/schedule/event/meilisearch-laguage-support)
- tantivy-stemmers provides Snowball tokenizers including Russian (0.4.0, June 2024), to be applied after lowercasing. — [tantivy-stemmers GitHub](https://github.com/testuj-to/tantivy-stemmers)
- The Snowball Russian stemmer algorithm is the reference stemmer used by Lucene-style systems. — [Snowball Russian](https://snowballstem.org/algorithms/russian/stemmer.html)

### Inferences
- PostgreSQL ships a `russian` text-search configuration (Snowball stemmer), plus `unaccent` and `pg_trgm` extensions. This is standard Postgres knowledge, not re-verified this session. For a parrot shop with hundreds to low thousands of SKUs, `tsvector` + GIN index + trigram fallback avoids running another service.
- Meilisearch is still worth it for typo-tolerant instant search and facets. Its prefix matching partly compensates for the lack of Russian stemming, but word-form matching (e.g. «попугай/попугаи/попугаев») may be weaker than in Postgres. Test it with real queries.
- For lemmatization, external morphology (pymorphy-style or the Rust `rsmorphy` port) would need a custom pipeline. Its maturity is unverified.

### Gaps
- I could not confirm whether Meilisearch 1.x applies any Russian stemming in 2026. Tantivy's built-in `Stemmer` (rust-stemmers with Language::Russian) was not re-verified.

## Observability, error handling, config, testing

### Takeaway
This area has a stable consensus with little churn: `tracing` + `tracing-subscriber` (JSON) + `tower-http::TraceLayer`; `tracing-opentelemetry` + `opentelemetry-otlp` for traces and metrics; `thiserror` for domain errors and `anyhow` (or `eyre`) at the edges; `figment` or `config` + `dotenvy` for config; `sqlx::test` / testcontainers for DB tests.

### Cited Findings
- Loco 1.0 rebuilt its `remote_ip` middleware on `axum-client-ip` and moved to tower 0.5. — [GitHub loco-rs releases](https://github.com/loco-rs/loco/releases)

### Inferences
- These recommendations come from general ecosystem knowledge and were not freshly verified with 2026 sources this session (see Gaps).
- For a small shop, Sentry (the `sentry` crate with a tracing integration), or OTLP export to Grafana Cloud or a self-hosted SigNoz/Uptrace, is plenty.

### Gaps
- No 2026 primary sources were fetched for tracing/opentelemetry crate versions. The opentelemetry-rust API has historically churned (0.2x versions), so pin versions together.

## Deployment

### Takeaway
Build a Docker multi-stage image with cargo-chef for dependency caching. The final image is debian-slim or distroless (or a musl static binary on scratch/alpine), deployed to a VPS or PaaS (Fly.io, Railway, a Russian VPS). Shuttle.dev has shut down, so do not plan around it. Loco 1.1 added AWS Lambda deployment.

### Cited Findings
- Shuttle is ceasing operations. Pro users had until Jan 16, 2026 to migrate, and community projects were deleted at the start of 2026. — [Shuttle shutdown docs](https://docs.shuttle.dev/docs/shuttle-shutdown.md); [LavX News](https://news.lavx.hu/article/shuttle-cloud-development-platform-announces-shutdown-urges-developer-migration)
- Loco 1.1.0 added AWS Lambda deployment. — [GitHub loco-rs releases](https://github.com/loco-rs/loco/releases)
- PandaStack's 2026 hosting comparison notes Shuttle's Rust-native provisioning was unique. — [PandaStack best Rust hosting 2026](https://pandastack.io/blog/best-rust-hosting-2026)

### Inferences
- These points are general knowledge, not re-verified this session:
  - A typical Axum + sqlx binary is 10–30 MB.
  - Idle RSS is in the tens of MB.
  - A 1 vCPU / 1–2 GB VPS comfortably runs the app plus Postgres for a small shop.
  - Compile time, not runtime, is the main resource cost. Use cargo-chef, sccache, and the `mold` linker in CI.
- With musl static builds (`x86_64-unknown-linux-musl`, or cross-compiling via `cargo-zigbuild`), swap the allocator to mimalloc or jemalloc, because musl malloc is slow.

### Gaps
- I did not fetch current cargo-chef or cargo-zigbuild docs, and found no 2026 benchmark of Rust shop resource usage.
