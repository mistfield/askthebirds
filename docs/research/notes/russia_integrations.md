# Russia-specific integrations and hosting for a Rust online-shop backend (state as of Nov 2026)

Research date: 2026-11-16. Crate metadata pulled live from the crates.io API and GitHub API on that date. Many web sources are commercial blogs/aggregators; flagged where relevant.

## CDEK API v2 (+ widget, alternatives) — API surface, Rust crates, effort via reqwest

### Takeaway
CDEK API v2 is a plain REST/JSON API with OAuth2 client-credentials auth, a separate test host (api.edu.cdek.ru), and endpoints for tariff calculation, pickup points, orders and webhooks. There is **no Rust crate for CDEK on crates.io** (exact name "cdek" not found, search for "cdek" returned nothing), but the official JS map widget (v4.x, Oct 2026) only needs a tiny backend proxy with three actions that is trivial to reimplement in Rust (axum + reqwest), so the widget can be used as-is with a Rust backend.

### Cited Findings
- Auth: OAuth 2.0 client credentials on `/v2/oauth/token?grant_type=client_credentials` — [Infostart](https://infostart.ru/1c/tools/1542796/); confirmed by CDEK's own widget proxy code, which POSTs form data `grant_type=client_credentials, client_id, client_secret` to `oauth/token` and reads `access_token` — [cdek-it/widget service.php](https://github.com/cdek-it/widget)
- Base URLs: production `https://api.cdek.ru/v2/`, test environment `https://api.edu.cdek.ru/v2/` (used by PHP and JS SDKs) — [packagist antistress-store/cdek-sdk-v2](https://packagist.org/packages/antistress-store/cdek-sdk-v2); [github shevernitskiy/cdek (TS/Deno)](https://github.com/shevernitskiy/cdek). Community results also contain public test credentials for the edu environment (not reproduced here).
- Endpoints used by the official widget backend: `deliverypoints` (list of ПВЗ/постаматов, filterable by type PVZ/POSTAMAT/ALL, city code, cashless/COD flags), `deliverypoints/byPolygons` (points by map coordinates), `calculator/tarifflist` (JSON POST, list of tariffs with price/terms) — [cdek-it/widget dist/service.php](https://github.com/cdek-it/widget); filters per [packagist cdek-sdk-v2](https://packagist.org/packages/antistress-store/cdek-sdk-v2)
- Webhooks: SDKs expose subscribing/listing/deleting webhooks (order status etc.) — [packagist cdek-sdk-v2](https://packagist.org/packages/antistress-store/cdek-sdk-v2); [rubydoc cdek_api_client](https://www.rubydoc.info/gems/cdek_api_client)
- Official widget "Виджет ПВЗ от СДЭК" (github.com/cdek-it/widget, npm `@cdek-it/widget` via jsDelivr): shows ПВЗ on a map, filter/search, delivery calculation for given dimensions, passes the chosen point to a JS callback. Latest releases v4.0.0 (2026-09-07) and v4.1.0 (2026-10-07); repo actively pushed Oct 2026. It requires a server-side "service" script (reference implementation: PHP 7.4 + cURL) — [cdek-it/widget README + GitHub API](https://github.com/cdek-it/widget)
- The reference PHP service is a thin proxy: takes `action` = `offices` | `calculate` | `byCoordinate`, obtains a token and forwards to `deliverypoints`, `calculator/tarifflist`, `deliverypoints/byPolygons` respectively; credentials stay on the server — [service.php](https://github.com/cdek-it/widget)
- User complaint (2024, single forum opinion) that widget v3 was very buggy and v4 was promised — [oborot.ru](https://oborot.ru/forum/otzyvy-o-rabote-kompanii-sdek-i27459-614.html) (outdated; v4 has since shipped)
- Existing SDKs in other languages: PHP (antistress-store/cdek-sdk-v2, ensi/cdek-sdk2-php), TS/Deno (@shevernitskiy/cdek on JSR/npm), Ruby (cdek_api_client), an npm MCP server — [packagist ensi](https://packagist.org/packages/ensi/cdek-sdk2-php); [jsr](https://jsr.io/@shevernitskiy/cdek@3.0.0); [socket.dev cdek-mcp](https://socket.dev/npm/package/@theyahia/cdek-mcp). Useful as reference for request/response models.
- Common pain point: 401 on `oauth/token` — [qna.habr.com](https://qna.habr.com/user/Bread09/comments)
- Yandex Delivery has its own ПВЗ widget (`widget-pvz.dostavka.yandex.net/widget.js`) and ready-made modules — [Яндекс Доставка: виджет ПВЗ](https://yandex.ru/support/delivery-profile/ru/modules/widgets)

### Inferences
- Implementation effort in Rust: low-to-moderate. Token cache (expires_in, refresh on 401) + ~5 endpoints (tarifflist, deliverypoints, orders POST/GET, webhooks) with serde structs ≈ a few hundred lines. The widget proxy is ~50 lines in axum. Biggest cost is modelling the large order/tariff JSON schemas — can be generated from the PHP/TS SDK models.
- No need to replace the JS widget; if you want no JS widget, you can build your own map with Yandex Maps JS API + `deliverypoints` data, but that's more frontend work.
- Alternatives (Boxberry, Яндекс Доставка, Почта России, Ozon/WB ПВЗ) — likewise no Rust crates expected; all REST/JSON (Почта России via otpravka API). Consider aggregators (ApiShip, Shiptor-like) to cover several carriers with one API.

### Gaps
- Official api-docs.cdek.ru pages were not fetched (search returned only third-party results); exact webhook payload schema, signature (CDEK webhooks historically have no HMAC — verify) and rate limits unconfirmed.
- Boxberry, Почта России, Ozon/WB pickup-point APIs not researched in depth (Ozon/WB do not offer a public "use our ПВЗ for your own shop" API as far as found — unverified). Yandex Delivery API details beyond the widget not checked.

## Payments: ЮKassa, CloudPayments, Т-Касса, Робокасса, СБП

### Takeaway
All major providers are REST/JSON (or form-redirect for Робокасса) with simple auth (HTTP Basic or SHA-256 tokens) and support 54-ФЗ receipts and СБП through the provider, so implementing directly via reqwest is easy. Rust crates exist only as tiny, very-low-adoption community projects (tens to hundreds of downloads) — do not depend on them; write your own thin client. ЮKassa is the most common default choice for a small shop (no monthly fee, built-in receipts via "Чеки от ЮKassa", big documentation), Т-Касса is attractive if you already bank with Т-Банк.

### Cited Findings
- ЮKassa API base `https://api.yookassa.ru/v3` — [docs.rs yookassa-rs source](https://docs.rs/yookassa-rs/1.0.1/src/yookassa_rs/lib.rs.html)
- ЮKassa webhooks: no HMAC/signature. Recommended verification = re-fetch object status via API and/or check source IP against allowlist: 185.71.76.0/27, 185.71.77.0/27, 77.75.153.0/25, 77.75.156.11, 77.75.156.35, 77.75.154.128/25, 2a02:5180::/32. Endpoint must be HTTPS on 443/8443, TLS ≥1.2; must answer HTTP 200, otherwise retries for 24 h. Webhooks are set in the cabinet for Basic Auth shops, or via API for OAuth (partners); requests use an `Idempotence-Key` header — [ЮKassa: входящие уведомления](https://yookassa.ru/developers/using-api/webhooks)
- ЮKassa fees: from 0.4% + НДС for successful payments, no monthly fee; tariff «Фиксированный» switches to 2.6% at turnover > 2 млн ₽/мес (aggregator data); >322 000 shops as of Feb 2026 — [toolfox ЮKassa](https://toolfox.ru/services/s/yookassa) (aggregator, verify on yookassa.ru)
- Rust crates for ЮKassa (crates.io, checked 2026-11-16): `yookassa-rs` 1.0.1 (2025-04-22, 671 total downloads, "docs not ready"), `yookassa` 0.1.1 (2026-09-02, 34 downloads), `yookassa-sdk-rs` 0.1.1 (2026-04-27, 74 downloads) — [crates.io API](https://crates.io/crates/yookassa-rs); [lib.rs](https://lib.rs/crates/yookassa-rs)
- CloudPayments: base `https://api.cloudpayments.ru`, HTTP Basic Auth (login = Public ID, password = API Secret), 401 on bad credentials; notifications Check, Pay, Fail, Confirm, Refund, Recurrent, Cancel with a "Проверка уведомлений" section; 54-ФЗ online receipts supported (CloudKassir; `receipt` object in widget, `JsonData.cloudpayments` in API); СБП supported (link/QR, refunds, notifications) — [developers.cloudpayments.ru](https://developers.cloudpayments.ru/). Fees are individual — [kokoc](https://kokoc.com/blog/kak-prinimat-oplatu-na-sajte/)
- Т-Касса (Т-Банк internet acquiring): request `Token` = SHA-256 of concatenated values of request params + `Password`, sorted by key (nested objects like Receipt/Data excluded); notifications verified the same way (all params except Receipt, Data, Token; add Password; sort; concat; SHA-256; compare). Number formatting must match exactly or signature fails — [qna.habr.com](https://qna.habr.com/q/1390540); [habr](https://habr.com/ru/post/453174) (community sources; official developer.tbank.ru page failed to load due to TLS chain issue). Requires a current account (расчётный счёт) at Т-Банк; commission individual — [dtf.ru overview](https://dtf.ru/id2419219/3999013-internet-ekvayring-2025-top-10-servisov)
- Rust crate `tinkoff-acquiring` 0.3.0 (2026-10-10, 839 downloads); `tinkoff` 0.1.0 is from 2021 and abandoned — [crates.io](https://crates.io/crates/tinkoff-acquiring)
- Робокасса: can process payment, generate the receipt and send it to ФНС and the buyer; onboarding 3–15 days depending on partner bank — [cleverence](https://www.cleverence.ru/articles/elektronnaya-kommertsiya/-robokassa-ili-yukassa-chto-luchshe-vybrat-biznesu-dlya-priema-platezhey/); [toolfox](https://toolfox.ru/services/s/robokassa/alternatives). No Robokassa or CloudPayments crate on crates.io (searched 2026-11-16).
- СБП: base commission is set by the Bank of Russia and is the same for all banks; providers differ by markups/conditions — [Timeweb community](https://timeweb.com/ru/community/articles/platyozhnye-sistemy-dlya-obrazovaniya-v-2026-sravnenie-i-podvodnye-kamni)
- Possible VAT change from 2026 for acquiring operations (plan to apply standard VAT, raised to 22%) — reported as a plan in a 2025 article, not confirmed in results — [dtf.ru](https://dtf.ru/id2419219/3999025-luchshie-servisy-internet-ekvajringa-dlya-biznesa-2025)

### Inferences
- Effort per provider via reqwest: ЮKassa ~1–2 days (create payment w/ `confirmation.type=redirect`, `receipt` object for 54-ФЗ, webhook handler with IP check + GET re-verify, Idempotence-Key = UUID). CloudPayments similar (widget-based; verify Content-HMAC on notifications). Т-Касса: careful token canonicalisation (`BTreeMap` + `sha2`). Робокасса: form redirect with MD5/SHA signature.
- For a small parrot shop: ЮKassa (no subscription, receipts included, СБП) or Т-Касса if the business account is at Т-Банк (often lower fees bundled with РКО). Using the provider's receipts service avoids buying your own ККТ/ОФД integration.

### Gaps
- Exact current tariff % for each provider (card vs СБП) not confirmed from official pages; aggregators conflict and tariffs are often individual.
- CloudPayments Content-HMAC algorithm details not read in fetched portion (commonly HMAC-SHA256 base64 with API Secret — unverified here).
- Robokassa signature algorithm not verified from official docs.

## Telegram bot and email in Rust

### Takeaway
All three Telegram options are alive in Nov 2026: teloxide is the most popular framework but its last crates.io release is 0.17.0 (July 2025) with low-volume maintenance commits in 2026; frankenstein is a thin, very actively released Bot API client; grammers is an MTProto (user/bot client) library. For email, lettre is the de-facto standard and actively maintained.

### Cited Findings
- teloxide: 0.17.0 released 2025-07-11, 2.34M total / 546k recent downloads, 4.27k GitHub stars, last repo push 2026-09-15, recent commits Aug 2026 (merged PRs, dependabot) — [crates.io](https://crates.io/crates/teloxide); [GitHub teloxide](https://github.com/teloxide/teloxide)
- frankenstein: 0.53.0 released 2026-11-14, 223k total downloads, 380 stars — [crates.io](https://crates.io/crates/frankenstein); [GitHub](https://github.com/ayrat555/frankenstein)
- grammers-client: 0.12.0 released 2026-11-08, 274k downloads, hosted on Codeberg — [crates.io](https://crates.io/crates/grammers-client); [Codeberg](https://codeberg.org/Lonami/grammers)
- lettre: 0.11.23 released 2026-08-03, 24.4M total / 9.2M recent downloads — [crates.io](https://crates.io/crates/lettre)

### Inferences
- For a shop notifications/admin bot: teloxide (dialogues, dispatching) is fine; if newest Bot API features matter, frankenstein tracks them faster. grammers is unnecessary unless you need MTProto.
- Telegram is reachable from Russian servers for Bot API in general, but note periodic Roskomnadzor slowdowns of Telegram features in 2025–2026 (not verified in this research) — keep email as a fallback channel.
- Russian SMTP: Yandex 360 / Mail.ru SMTP via lettre for low volume; Unisender Go / Mailganer-type transactional services for volume (they also store PD in RF).

### Gaps
- No sourced data gathered on Unisender Go / Yandex 360 SMTP limits and pricing in 2026; teloxide supported Bot API version not checked.

## Hosting in Russia under 152-ФЗ, RKN registration, privacy/cookies, 2025 changes and fines

### Takeaway
Since 1 July 2025 (23-ФЗ of 28.02.2025, amending ч.5 ст.18 152-ФЗ) recording, systematization, accumulation, storage and modification of Russian citizens' PD using foreign databases is prohibited (narrow exceptions), so the DB must be in RF. Every operator, including ИП/self-employed, must notify РКН before processing (ст. 22). Fines rose sharply from 30 May 2025, and since 1 Sep 2025 PD consent must be a separate document (156-ФЗ). Russian clouds (Selectel, Timeweb Cloud, Yandex Cloud, VK Cloud, Beget) all host in RF; small VPS start from a few hundred rubles/month.

### Cited Findings
- From 1 July 2025 ч.5 ст.18 152-ФЗ amended: recording, systematization, accumulation, storage, modification using foreign DBs not allowed except listed exceptions; cross-border transfer (ст. 12) requires prior РКН notification — [habr](https://habr.com/ru/articles/925642/); [www1.ru](https://www1.ru/news/2025/06/28/zapret-na-xranenie-personalnyx-dannyx-za-granicei-novye-pravila-vstupaiut-v-silu-v-rossii.html); law 23-ФЗ of 28.02.2025 — [vc.ru](https://vc.ru/marketing/2153541-zapret-na-google-analytics-v-rossii)
- Notification of РКН (ст. 22) applies to all operators incl. ИП and самозанятые; exceptions only per ч.2 ст.22 — [saby.ru](https://saby.ru/articles/contragents/uvedomlenie_o_personalnyh_dannyh_v_roskomnadzor)
- From 30 May 2025 fines for PD violations increased (e.g. 300–700k ₽ vs. 5k ₽ before for some offences; turnover-based fines for repeated leaks) — [uibcom](https://uibcom.ru/articles/informatsionnaya-bezopasnost/uzhestochenie-otvetstvennosti-za-narusheniya-pri-obrabotke-pdn-2025); [harant.ru](https://harant.ru/blog/yuridicheskie-lica/usilenie-otvetstvennosti-za-narushenie-152-fz-kak-izbezhat-shtrafov-i-ugolovnogo-presledovaniya/); localization breach fines up to 6 млн ₽ first / 18 млн ₽ repeat (as reported) — [mosdigitals](https://mosdigitals.ru/blog/regulirovanie-raboty-google-analytics-v-rossii-v-2025-godu). Blog sources; verify in КоАП ст. 13.11 on consultant.ru.
- From 1 Sep 2025 (156-ФЗ of 24.06.2025) consent to PD processing must be a separate document (not buried in offer/policy) — [tproger](https://tproger.ru/articles/it-zakonodatelstvo-2025--razbor-izmenenij-dlya-biznesa-i-specialistov)
- Claims of a 15 млн ₽ fine for cookies without consent appear in blogs without court references — unconfirmed — [habr](https://habr.com/en/articles/959816)
- Selectel: planned price increase from 15 Jan 2026 (≤8% for most services); S3 cheapest class 0.79 ₽/GB/month, free ingress; 3 TB/mo egress free — [Selectel PDF](https://files.selectel.ru/docs/ru/New-prices-January-15-2026.pdf); [hostinghub](https://hostinghub.ru/top/vds)
- Timeweb Cloud: aggregator lists cheapest VPS from 477 ₽/mo, unmetered traffic, managed PostgreSQL/MySQL/Redis and S3 available — [toolfox](https://toolfox.ru/services/s/timeweb-cloud); official VPS page shows "MSK Ultra" line from 1 920 ₽/mo (1 vCPU 3.3 GHz/1 GB/15 GB NVMe) with more tariffs hidden — [timeweb.cloud](https://timeweb.cloud/services/vds-vps) (conflict: cheaper standard lines likely exist; verify). Timeweb S3 cold class "from ~1 ₽/month"; managed PG replication priced as single-node cost ×3 or ×5 — [habr Timeweb](https://habr.com/ru/companies/timeweb/posts/853138)
- Minimum spend: Yandex Cloud 200 ₽, Timeweb Cloud 189 ₽ (aggregator) — [startpack](https://startpack.ru/compare/yandex-cloud-vs-timeweb-cloud)

### Inferences
- Practical checklist: (1) host app + PostgreSQL + backups in RF (any RU provider); (2) submit РКН notification via pd.rkn.gov.ru before launch; (3) publish privacy policy (ч.2 ст.18.1) and collect separate consent at checkout/registration (checkbox linking separate consent doc); (4) cookie banner if analytics cookies are considered PD; (5) do not send PD to foreign services (incl. LLM APIs) without cross-border notification.
- Rust is fine with any RU VPS; a 2 vCPU/2–4 GB VPS + managed PG is likely in the ~1–3k ₽/mo range total, but exact 2026 managed PG prices were not confirmed.

### Gaps
- Official 2026 prices for managed PostgreSQL at Selectel/Timeweb/Yandex/VK Cloud and Beget VPS not retrieved.
- Primary texts (consultant.ru/garant.ru) of 23-ФЗ, 156-ФЗ and КоАП 13.11 amendments not fetched; amounts from blogs.

## Analytics: Яндекс Метрика vs Google Analytics

### Takeaway
There is no explicit ban on Google Analytics as a product, but after 1 July 2025 its use is legally risky (treated as collection of PD into foreign DBs / cross-border transfer); Russian businesses massively moved to Яндекс Метрика. For the shop: use Яндекс Метрика (plain JS counter, no Rust integration needed; optional server-side offline conversions API).

### Cited Findings
- 23-ФЗ (in force 1 July 2025) restricts primary collection/storage of PD on foreign servers; GA is not banned outright but risky; РКН position reported as: GA use = cross-border transfer, allowed only with notification — [vc.ru](https://vc.ru/marketing/2153541-zapret-na-google-analytics-v-rossii); [ppc.world](https://ppc.world/articles/google-analytics-mozhno-budet-ispolzovat-posle-1-iyulya-2025-goda-nepopulyarnoe-mnenie/); [Robokassa blog](https://robokassa.com/blog/articles/zapret-google-analytics-v-rossii-s-1-iyulya-2025-goda-chto-nuzhno-znat-biznesu/)
- Reported fines up to 6 млн ₽ (first) / 18 млн ₽ (repeat); a fitness chain reportedly fined 25 млн ₽ for GA (unverified anecdote) — [mosdigitals](https://mosdigitals.ru/blog/regulirovanie-raboty-google-analytics-v-rossii-v-2025-godu); [sostav](https://www.sostav.ru/blogs/269166/62563)
- GA share among Russian companies fell from 90–95% (2021) to ~45% (2025) per Kokoc — [ComNews](https://www.comnews.ru/content/240597/2025-08-08/2025-w32/1008/prisutstvie-google-analytics-rossiyskikh-kompaniyakh-sokratilos-vdvoe)
- Google Tag Manager alternatives are being adopted — [kokoc](https://kokoc.com/blog/google-tag-manager-vse-8-rabochikh-alternativ-i-5-sovetov-po-pereezdu/)

### Inferences
- Метрика (Webvisor) itself collects data and needs cookie consent/privacy-policy mention; it is a Russian service so no cross-border issue. Self-hosted alternatives (Plausible/Umami on RU VPS) are also compliant.

### Gaps
- A March 2026 gazeta.ru news item on the topic could not be read (page rendered empty) — possible 2026 developments missed. Метрика API (offline conversions, ecommerce dataLayer) not researched.

## Foreign services accessibility: Anthropic API for an LLM assistant feature

### Takeaway
Russia is not a supported region for Anthropic. Under the policy effective 12 Nov 2026, access is restricted by physical location, by incorporation/HQ, and by majority ownership/control from unsupported regions — so a Russian ИП/ООО serving from a Russian server cannot legitimately use the Claude API (directly or via cloud resellers), and payment from Russian cards is impossible anyway. Also, sending customer PD to a foreign LLM would be cross-border transfer under 152-ФЗ.

### Cited Findings
- Russia not listed; "any country or region not listed is unsupported"; covers apps, API/developer platform, and Claude via cloud platforms and authorized resellers; restricted by physical location, by incorporation/HQ, and by majority ownership/control by entities in unsupported regions; effective date Nov 12, 2026 — [Anthropic supported countries](https://www.anthropic.com/supported-countries)
- Earlier (2025) Anthropic barred companies majority-controlled by China, Russia, Iran, North Korea — [The Decoder](https://the-decoder.com/anthropic-bans-companies-majority-controlled-by-china-russia-iran-and-north-korea-from-claude/); [expert.ru](https://expert.ru/news/anthropic-uzhestochit-pravila-dostupa-k-claude-dlya-kompaniy-iz-rossii)
- Reports of mass account blocks of Russian users in 2026 and OpenRouter stopping Russian-billed accounts mid-2026; ruble resellers exist but reselling violates ToS — reseller/blog sources, low reliability — [techora](https://techora.ru/news/v-rossii-narodnyy-metod-dlya-claude-2026-10-02); [ssdnodes](https://www.ssdnodes.com/learn/lang/ru/claude-api-from-russia-what-works)

### Inferences
- For an LLM assistant in a Russia-based shop, use Russian providers (YandexGPT via Yandex Cloud Foundation Models, GigaChat by Sber) or self-hosted open-weight models on RU GPU clouds — these also avoid cross-border PD issues. Design the assistant behind a provider-agnostic trait in Rust.

### Gaps
- YandexGPT / GigaChat API pricing and Rust crates not researched (out of fetched scope).
