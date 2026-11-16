# LLM-powered parrot-care assistant in Rust (state as of Nov 2026)

Scope: Rust stack for LLM calls, RAG retrieval, browser streaming, and safety/escalation for a parrot-care Q&A assistant embedded in an online shop. Research date: 2026-11-16. About 15 tool calls; several areas rely on docs.rs/lib.rs listings and search snippets, not full repo reads. Those spots are flagged.

## 1. Rust clients for the Claude API and multi-provider crates

### Takeaway
Anthropic still has **no official Rust SDK** as of Nov 2026. Its official SDKs cover Python, TypeScript, Java (also Kotlin/Scala), Go, Ruby, C# and PHP; for any other language Anthropic recommends raw HTTP. A Rust shop therefore has two options: a community crate, or a thin `reqwest` + SSE client written against the Messages API. The most defensible choice is a multi-provider layer (`rig-core` or `genai`) behind an internal trait. That keeps a non-Anthropic fallback possible, which matters given the Russia availability issue in section 6.

### Cited Findings
- **No official Rust SDK.** A search turned up only community crates, each self-described as "unofficial": `anthropic-sdk-rust` 0.1.0, `anthropic-ai-sdk` 0.2.27, `anthropic` 0.0.8 (anthropic-rs), `anthropic-rs` 0.1.7, and `anthropic-rs-sdk` 0.1.0. The last is an early port of the Go SDK that covers only the non-streaming Messages API — [docs.rs anthropic-sdk-rust](https://docs.rs/crate/anthropic-sdk-rust/0.1.0/source/README.md), [docs.rs anthropic-ai-sdk](https://docs.rs/crate/anthropic-ai-sdk), [docs.rs anthropic](https://docs.rs/crate/anthropic/latest), [docs.rs anthropic-rs-sdk](https://docs.rs/crate/anthropic-rs-sdk/latest)
- Anthropic's own API reference (Claude API skill bundled with Claude Code, cached 2026-11-03) lists official SDKs for Python, TypeScript, Java, Go, Ruby, C# and PHP, and says to use raw HTTP/cURL for unsupported languages such as Rust — [Anthropic client SDKs](https://platform.claude.com/docs/en/api/client-sdks)
- One search hit is `anthropic_rust`, whose rust-digger listing links to `github.com/anthropics/anthropic-sdk-rust` and advertises full tool calling. The anthropics org path makes it look official, but I could not confirm this and treat it as community. Verify before use — [lib.rs anthropic_rust](https://lib.rs/crates/anthropic_rust), [rust-digger](https://rust-digger.code-maven.com/crates/anthropic_rust)
- `misanthropic` is an unofficial, simple, ergonomic client for the Messages API. It supports tool use and streaming, and it is async without depending directly on tokio — [lib.rs misanthropic](https://lib.rs/crates/misanthropic)
- `misanthropy` (with the `misan` CLI) is a separate crate. It supports streaming and tools via `Tool::new::<T>()` with a derived JSON schema — [docs.rs misanthropy](https://docs.rs/misanthropy)
- `anthropic-sdk` 0.0.9 documents both streaming and tool use; the 0.1.0 docs show only non-streaming examples — [docs.rs anthropic-sdk 0.0.9](https://docs.rs/crate/anthropic-sdk/0.0.9)
- `anthropic-async` 0.5.x is "an Anthropic API client with prompt caching support" — [docs.rs anthropic-async](https://docs.rs/crate/anthropic-async/0.5.2/source/README.md)
- `clust` turned up nothing in my search (it may be abandoned or renamed). Status is unverified.
- **genai** (jeremychone/rust-genai) gives one native-protocol API across multiple providers. Version 0.7.0 added chat-level `CacheControl` with provider-specific prompt-caching options. Version 0.7.1 added Anthropic prompt caching on tools (`Tool::with_cache_control`) and SSE heartbeat events (`ChatStreamEvent::Heartbeat`) — [docs.rs genai 0.7.0](https://docs.rs/crate/genai/0.7.0), [docs.rs genai 0.7.1](https://docs.rs/crate/genai/0.7.1)
- **rig-core** (0xPlaygrounds) is at 0.46.0 on docs.rs, with "Full WASM compatibility (core library only)". Other listings report older versions (0.44.0, 0.39.0 from 2026-06-19), so release cadence is high. Upstream commits from Aug 2026 split `rig-core` and `rig-agent` behind a `rig` facade crate (0.42.0). Rig claims one API over 20+ model providers and 10+ vector stores. MCP support is native-only (no WASI) — [docs.rs rig-core latest](https://docs.rs/crate/rig-core/latest), [docs.rs rig](https://docs.rs/crate/rig/latest), [everydev.ai](https://www.everydev.ai/tools/rig)
- `async-openai` was not researched in this pass. It is relevant only for OpenAI-compatible endpoints such as Yandex AI Studio (section 6).

### Inferences
- The API surface changed a lot in 2025–2026: adaptive thinking replaced `budget_tokens`, assistant prefill was removed, forced `tool_choice` returns a 400 on Opus 5.5 and Sonnet 5.5, and there is a new `refusal` stop reason plus server-side `fallbacks`. Small community crates probably lag behind these changes; this is a key risk. Two mitigations:
  - Own a thin, internally maintained Anthropic client. The Messages API plus SSE fits in a few hundred lines with `reqwest` + `eventsource-stream`.
  - Use rig or genai, which have the most maintainers, and confirm they can pass new fields (`output_config.effort`, `fallbacks`, `cache_control`) through as raw JSON.
- **Recommendation:** use `rig-core` for the agent/RAG layer if its Anthropic provider supports `cache_control` and streaming tool calls. Otherwise use `genai` for chat and streaming and write retrieval yourself. Wrap either one in a project trait `LlmBackend` so you can swap in YandexGPT or GigaChat.

### Gaps
- I did not verify GitHub stars, last-commit dates, or whether rig's Anthropic provider supports prompt caching, `fallbacks`, or adaptive thinking. Check rig's `providers/anthropic` source.
- I could not confirm the status or ownership of `clust` or of the `anthropic_rust` → `anthropics/anthropic-sdk-rust` link.

## 2. Agent/RAG frameworks in Rust: rig, swiftide, langchain-rust, kalosm

### Takeaway
**rig** is the most active general LLM/agent framework, with very frequent releases. **swiftide** (bosun-ai) is the strongest choice for streaming *indexing pipelines*: chunking, embedding, and storing to Qdrant or LanceDB. I found no 2026 evidence of activity for langchain-rust or kalosm, so treat them as lower priority. Suggested split: swiftide (or a hand-written loader) for ingestion, rig or plain code for query-time RAG.

### Cited Findings
- Swiftide is a Rust library for fast, streaming indexing and query pipelines for RAG — [GitHub bosun-ai/swiftide](https://github.com/bosun-ai/swiftide)
- A review site reports swiftide is actively maintained (v0.32.1 cited). It notes a steep onboarding curve for teams without Rust experience — [dev.co swiftide](https://dev.co/ai/rag/swiftide)
- Swiftide 0.9 added Fluvio, LanceDB and Ragas support. This is old (2024), but it shows evaluation tooling exists — [dev.to swiftide 0.9](https://dev.to/timonv/swiftide-09-a-rust-native-library-for-building-llm-applications-with-rag-brings-fluvio-lancedb-and-ragas-support-3ai6)
- Rig integrates vector stores including MongoDB, LanceDB, Neo4j and Qdrant, and has an extensible trait for custom backends. Its ecosystem is smaller than Python's — [aitoolsatlas rig](https://aitoolsatlas.ai/tools/rig-rust)
- Framework overhead is under 10% of total RAG latency; retrieval strategy matters more. This comes from a 2026 SEO-style guide, so it is low-confidence — [techsy.io](https://techsy.io/en/blog/best-rag-framework-2026)
- kalosm and langchain-rust: my search found no 2026 sources on either.

### Inferences
- For a corpus of a few thousand curated docs, a framework adds little value. Retrieval is "embed query → top-k from the vector DB → rerank → stuff into the prompt." Writing it by hand with `qdrant-client` or `sqlx`/pgvector keeps control over safety logic. rig is worth it mainly for tool-calling agent loops, for example a `find_products` tool that queries the shop catalog or a `recommend_vet` tool.
- kalosm (local models via candle) could matter for fully on-prem inference if you cannot reach cloud APIs from Russia, but its maintenance status is unverified.

### Gaps
- No verified 2026 activity data (stars, commits, releases) for langchain-rust or kalosm.
- No head-to-head quality comparison of the Rust frameworks exists in the sources I found.

## 3. Vector search and Russian-language embeddings

### Takeaway
For a few thousand documents (realistically 10k–100k chunks), **pgvector via sqlx** is enough and the simplest to run, if the shop already uses Postgres. **Qdrant** (written in Rust, `qdrant-client`) is the better choice if you want native dense+sparse/BM25 hybrid search with fusion out of the box. Russian care texts contain species names, drug names and Latin terms where lexical matching helps, which argues for hybrid. For embeddings, use a Russian-strong model:
- **Self-hosted:** Giga-Embeddings or FRIDA (Sber/ai-forever), or `multilingual-e5-large(-instruct)` / `bge-m3` via fastembed-rs/ort.
- **API:** Yandex or GigaChat embeddings (not researched in depth).

### Cited Findings
- Qdrant has native BM25 via sparse vectors. Qdrant's own blog says the Postgres ecosystem "still lacks a high-quality BM25 implementation" (vendor source) — [Qdrant pgvector tradeoffs](https://qdrant.tech/blog/pgvector-tradeoffs/)
- The same post says pgvector is the right call when you want vector search without operational overhead, but structured content such as product names or SKUs benefits from hybrid search — [Qdrant blog](https://qdrant.tech/blog/pgvector-tradeoffs/)
- Hybrid search adds storage, indexing and query cost; measure whether the gain justifies it — [Qdrant hybrid search docs](https://qdrant.tech/documentation/search-tuning/hybrid-search/)
- BM25 may perform poorly on small chunks and may need parameter tuning — [Qdrant skills index](https://openskillindex.com/skills/qdrant-skills-qdrant-hybrid-search-prefetches)
- **ruMTEB** has 23 datasets in 7 task categories (17 new). It is on the HF MTEB leaderboard. The paper also introduced ru-en-RoSBERTa — [arXiv 2408.12503](https://arxiv.org/html/2408.12503v1)
- 2024 snapshot from X5 Tech: e5-mistral-7b-instruct was best but heavy; `sergeyzh/rubert-tiny-turbo` was best for minimal resources (outdated) — [prog.world](https://prog.world/updated-rumteb-benchmark-and-leaderboard)
- MMTEB found `multilingual-e5-large-instruct` (560M params) the best publicly available multilingual model — [HF papers 2408.12503](https://huggingface.co/papers/2408.12503)
- GigaEmbeddings scored a 69.1 ruMTEB average across 23 tasks and held the top spot as of Dec 2024 — [arXiv 2510.22369](https://arxiv.org/pdf/2510.22369)
- Giga-Embeddings (paper listed as submitted 24 Aug 2026) is a 480M model with 70.98 on Russian MTEB. It reportedly beats FRIDA with 42% fewer parameters, and 3B dense and 10B-A1.8B MoE variants were also released. This is a recent preprint; verify the leaderboard — [alphaxiv 2608.23806](https://www.alphaxiv.org/abs/2608.23806)
- FRIDA (ai-forever) produces 1536-dim embeddings with 24 layers. BERTA (sergeyzh) is distilled from FRIDA into 768 dims and 12 layers. Classification accuracy on ruMTEB: FRIDA 0.707, BERTA 0.698, multilingual-e5-large-instruct 0.654 — [HF sergeyzh/BERTA](https://huggingface.co/sergeyzh/BERTA)
- **fastembed-rs** (5.17.x) supports `intfloat/multilingual-e5-small/base/large`, which need "query: " / "passage: " prefixes. `BAAI/bge-m3` appears in the 5.13.4 README, and the 5.16.2 README lists it under *sparse* embeddings — [docs.rs fastembed 5.17.1](https://docs.rs/crate/fastembed/5.17.1/source/README.md), [fastembed 5.16.2](https://docs.rs/crate/fastembed/5.16.2/source/README.md), [fastembed 5.13.4](https://docs.rs/crate/fastembed/5.13.4/source/README.md)

### Inferences
- **Recommended default:**
  - Qdrant (single node in Docker) with a dense Russian embedding plus a sparse BM25/bge-m3 vector, fused with RRF.
  - Or Postgres + pgvector (HNSW) plus Postgres full-text search with the `russian` config, fused in SQL, if you want one database.
  - Both are fine at this scale. Pick on ops preference, not performance.
- **Embeddings:**
  - Easiest in Rust: `multilingual-e5-large` via fastembed-rs, which runs ONNX through `ort` on CPU.
  - Best Russian quality: Giga-Embeddings or FRIDA. These likely need ONNX export to run under `ort`/candle, or a small Python/TEI sidecar.
  - Re-embedding a few thousand docs is cheap, so switching models later is low-risk.
- Anthropic has no embeddings endpoint (it recommends third parties), so embeddings must come from local models or another provider either way. This is an inference from the Claude API reference, which lists no embeddings endpoint.

### Gaps
- Current live ruMTEB leaderboard standings for Nov 2026 were not fetched.
- I did not verify whether FRIDA or Giga-Embeddings ship ONNX weights usable from fastembed/ort/candle.
- Yandex and GigaChat embedding API quality and pricing were not researched.
- LanceDB and Tantivy were not researched in this pass. Tantivy could provide BM25 in-process alongside pgvector.

## 4. Streaming to the browser: Axum (SSE/WebSockets) + Leptos

### Takeaway
Use an **Axum SSE endpoint** (`axum::response::sse::Sse` with `KeepAlive`) that re-emits Claude's token deltas. On the Leptos client, either consume it with `EventSource`/fetch-streaming, or use Leptos 0.8 server functions with the `StreamingText` codec or the new WebSocket protocol. SSE is the simplest choice for one-directional token streams. Plan for cancellation and reconnect.

### Cited Findings
- Pattern: convert an upstream LLM stream into SSE events and return an axum `Sse` response — [UMich llmchat-rs](https://reactive.eecs.umich.edu/asns/llmchat-rs)
- Production code wraps `Sse` with `.keep_alive(KeepAlive::default())` to avoid idle timeouts — [docs.rs embacle-server streaming.rs](https://docs.rs/embacle-server/0.6.0/src/embacle_server/streaming.rs.html)
- `EventSource` auto-reconnects; fetch-based streaming does not. Resumable or multi-device streams need a server-side token buffer — [zknill.io](https://zknill.io/posts/everyone-said-sse-token-streaming-was-easy/), [zknill.io SSE vs WS](https://zknill.io/posts/ai-token-streaming-isnt-about-sse-vs-websockets/)
- Leptos 0.8.0 added WebSocket support for server functions (`#[server(protocol = Websocket<JsonEncoding, JsonEncoding>)]`, PR #3656). A beta note says it does not integrate directly with Resources/SSR — [Leptos v0.8.0 release](https://github.com/leptos-rs/leptos/releases/tag/v0.8.0)
- The `server_fn` `StreamingText` codec (`TextStream`, `text/plain`) lets a server function stream text output. Streaming *requests* need HTTP/2 or HTTP/3, and browsers do not support full duplex fetch — [docs.rs server_fn stream codec](https://docs.rs/crate/server_fn/0.8.2/source/src/codec/stream.rs)
- Later releases fixed `StreamingText` content-type and WebSocket back-pressure, so use the latest 0.8.x `server_fn`. `leptos_axum` is at 0.8.9 and `leptos` at 0.8.19 — [Leptos releases](https://github.com/leptos-rs/leptos/releases), [docs.rs leptos 0.8.19](https://docs.rs/crate/leptos/0.8.19/source/src/lib.rs)
- `leptos_sse` / `leptos_ws` sync server *signals* over SSE or WS. They are not token-streaming APIs. `leptos_ws` 0.9.x is compatible with Leptos 0.8 — [lib.rs leptos_ws](https://lib.rs/crates/leptos_ws)

### Inferences
- **Recommended flow:**
  1. `POST /api/assistant/chat` returns `Sse<impl Stream<Item=Event>>`.
  2. Event types: `meta` (retrieved sources and the escalation flag), `delta` (text), `escalation` (vet card), `done`, `error`.
  3. Send the safety classification *before* the first delta, so the UI can render the "see an avian vet" banner immediately.
  4. In Leptos, a `#[server]` function with `StreamingText` output is the most idiomatic path. A plain SSE route plus `web_sys::EventSource` (or `gloo-net` EventSource) also works.
  - Choose WebSockets only if you need bidirectional features such as mid-stream cancel plus typing indicators.
- Note that `EventSource` supports only GET. Because the chat uses POST, either use fetch-streaming (and handle reconnect yourself) or create a session first and then stream via a GET with a session id.

### Gaps
- The Leptos book section on streaming server functions was not fetched; there is no end-to-end example of `StreamingText` with an LLM.
- There is no source on Leptos 0.9 or 1.0 status (if released in late 2026). Check crates.io.

## 5. Safety design and Claude 5-family pricing for cost estimates

### Takeaway
Use a layered design:
1. A deterministic, vet-curated red-flag keyword and symptom list (Russian plus synonyms), checked before and after the LLM.
2. A cheap LLM triage classifier (Claude Haiku 5.5) using structured outputs that return `{urgency, matched_signs, needs_vet}`.
3. A main answer model (Sonnet 5.5) with a stable, cached system prompt that requires vet escalation and fixed disclaimers, grounded only in retrieved KB passages.
4. UI-level enforcement, where the server injects the escalation card regardless of model text.

The pricing below comes from Anthropic's model table (cached 2026-11-03). Sonnet 5.5 at about $0.01–0.015 per typical RAG turn is affordable; Haiku 5.5 triage costs under $0.001 per turn.

### Cited Findings — Claude models and pricing (first-party API, per 1M tokens)
Source for all rows: the Anthropic Claude API reference bundled with Claude Code (models table cached 2026-11-03); live page [Anthropic pricing](https://platform.claude.com/docs/en/about-claude/pricing).

| Model | ID | Context | Input | Output | Cache read |
|---|---|---|---|---|---|
| Claude Opus 5.5 | `claude-opus-5-5` | 1M | $4.00 | $20.00 | $0.20 |
| Claude Sonnet 5.5 | `claude-sonnet-5-5` | 1M | $2.00 | $10.00 | $0.10 |
| Claude Haiku 5.5 | `claude-haiku-5-5` | 1M | $0.10 | $0.50 (≤100K-token prompts; $0.50/$2.50 beyond) | n/a |
| Claude Fable 5.2 (top tier) | `claude-fable-5-2` | 1M | $8.00 | $40.00 | $0.20 |

Relevant API behaviour, from the same source:
- Opus 5.5 and Sonnet 5.5 return `stop_reason: "refusal"` from safety classifiers. A server-side `fallbacks` parameter exists (beta `server-side-fallback-2026-07-01`, `fallbacks: "default"`).
- Forced `tool_choice` (`any`/`tool`) returns a 400 on Opus 5.5 and Sonnet 5.5. Use `auto` + `strict: true` or structured outputs (`output_config.format`).
- Assistant prefill returns a 400.
- Thinking cannot be disabled on Opus 5.5. Sonnet 5.5 uses `between_tools` to switch it off. Effort is set via `output_config.effort`; Opus 5.5 and Haiku 5.5 default to `medium`.
- Prompt caching is a prefix match over the order tools → system → messages, with a model-dependent minimum cacheable prefix of 512–4096 tokens. Mid-conversation `role: "system"` messages are supported on Sonnet 5.5, Opus 5.5 and Haiku 5.5.
- Message Batches cost 50% less, which helps with offline KB enrichment.

### Cited Findings — safety
- I found no Anthropic-published guidance specific to *veterinary* or animal-health assistants in this pass.
- Anthropic's usage policies treat medical advice as a context needing human-in-the-loop and disclosure. This comes from training knowledge and was not re-verified in 2026, so it is moved to Gaps.

### Inferences (design recommendations; not sourced claims)
- **Red-flag layer (deterministic, runs first).** Maintain a vet-reviewed list of avian emergency signs, with Russian synonyms, mapped to an urgency tier. The list should be built *by an avian vet*, not by the LLM. Plausible candidate categories to validate with that vet:
  - Bleeding, including a broken blood feather.
  - Labored or open-mouth or tail-bobbing breathing.
  - Fluffed up and sitting on the cage floor.
  - Not eating for more than ~24h.
  - Changes in droppings: blood, undigested seeds.
  - Egg binding.
  - Seizures, trauma or falls.
  - Toxin exposure: PTFE/Teflon fumes, heavy metals, avocado, chocolate.
  - Regurgitation or crop problems.
  - Discharge from eyes or nostrils.
  A match forces `needs_vet = true` and streams an escalation event before any LLM text.
- **Triage classifier.** Haiku 5.5 with `output_config.format` JSON schema `{urgency: emergency|soon|routine|not_medical, signs:[...], ask_followup: bool}`. Cost ≈ $0.0006 for 5K input + 100 output tokens. Take the max of the rule and classifier verdicts; never downgrade a rule match.
- **System prompt (cached, frozen bytes).**
  - Role: shop care assistant, not a vet.
  - Answer only from the provided KB passages, and cite them.
  - For any symptom: no diagnosis, no drug dosages, no home medication. Recommend an avian (not general) vet and give the urgency tier.
  - Ask the species when relevant.
  - Russian-language answers.
  - Put the volatile parts (retrieved chunks, user question) after the cache breakpoint.
  - Use mid-conversation `role:"system"` messages to inject the triage verdict without breaking the cache.
- **Output guardrails.**
  - Post-check the answer for dosage patterns (mg/kg, drug names) and strip or block them.
  - Always append a server-rendered disclaimer and vet-finder card when `needs_vet`. Do not depend on the model to say it.
  - Handle `stop_reason == "refusal"` and `max_tokens` gracefully.
  - Log all escalations for review.
- **Product recommendations.** Keep commerce (tool `search_products`) separate from medical answers. Do not upsell supplements or medications in response to symptom questions; this is both an ethics and a liability concern.
- **Cost estimate per turn** (assumes a 3K cached system prompt, 4K retrieved chunks, a 300-token question, 600 output tokens; cache-write premium ignored):

  | Configuration | Calculation | Cost per turn |
  |---|---|---|
  | Sonnet 5.5 | 3K × $0.10/M + 4.3K × $2/M + 0.6K × $10/M ≈ $0.0003 + $0.0086 + $0.006 | **≈ $0.015** |
  | Haiku 5.5 alone | | **≈ $0.001** |
  | Opus 5.5 | | **≈ $0.03** |

  At 10k questions per month, Sonnet 5.5 plus Haiku triage costs about $150–160 per month.

### Gaps
- I did not verify the cache *write* multiplier for the 5.5 family or the Haiku 5.5 cache-read price.
- There is no authoritative, sourced avian red-flag symptom list here. It needs input from an avian vet or literature such as AAV (Association of Avian Veterinarians) resources.
- I did not re-verify Anthropic usage-policy wording on health advice for 2026, or any Russian legal requirements for veterinary advice disclaimers.

## 6. Availability: Anthropic API from Russia, and YandexGPT/GigaChat alternatives

### Takeaway
**Russia (and Belarus) are not supported.** The exclusion covers users located there and entities incorporated in, headquartered in, or majority-owned from unsupported regions, whatever their location. A Russian online shop therefore cannot lawfully use the Anthropic API directly. Bedrock, Vertex and Foundry resell under their own terms but are also generally unavailable to Russian entities; I did not verify this. For a Russia-based shop, the realistic primary LLMs are YandexGPT (Yandex AI Studio, with an OpenAI-compatible API, so `async-openai`, rig's OpenAI provider or genai work) or GigaChat (Sber, not natively OpenAI-compatible, so it needs a custom `reqwest` client or a proxy). Claude is only an option if the operating entity is outside Russia and not Russian-owned.

### Cited Findings
- Anthropic's supported-countries page (effective 12 Nov 2026) does not list Russia, Belarus or China. "Any country or region not listed is unsupported." The page also excludes:
  - entities incorporated or headquartered in unsupported regions, even when their users are elsewhere;
  - entities majority-owned or controlled from unsupported regions;
  - Crimea, Donetsk, Kherson, Luhansk and Zaporizhzhia (Ukraine is otherwise listed).
  Source: [Anthropic supported countries](https://www.anthropic.com/supported-countries)
- Yandex AI Studio's API is compatible with the OpenAI Responses, Realtime and Vector Store APIs, and partially compatible with Completions — [Yandex AI Studio OpenAI-compatible API](https://aistudio.yandex.ru/en/docs/ai-studio/api/)
- To use OpenAI libraries with Yandex, change the base endpoint and supply a service-account API key plus the folder ID. Model URIs look like `gpt://<folder_ID>/yandexgpt-5.1` (YandexGPT Pro 5.1, YandexGPT Lite 5) — [Yandex Cloud OpenAI compatibility](https://yandex.cloud/en/docs/ai-studio/concepts/openai-compatibility), [AI Studio models](https://aistudio.yandex.ru/docs/en/ai-studio/concepts/generation/models.html)
- A community config uses base URL `https://ai.api.cloud.yandex.net/v1` with an `OpenAI-Project` header carrying the folder ID (third-party; verify) — [Habr 996966](https://habr.com/ru/articles/996966/)
- GigaChat is not OpenAI-compatible out of the box. The community `gpt2giga` Python proxy translates OpenAI-format requests — [Habr 996966](https://habr.com/ru/articles/996966/)
- An older Habr article calls YandexGPT not OpenAI-compatible. That predates the current AI Studio compatibility layer and is outdated — [Habr 996966](https://habr.com/en/articles/996966)

### Inferences
- **Architecture recommendation.** Define an `LlmBackend` trait with two implementations:
  - `AnthropicBackend` (Claude Sonnet 5.5 / Haiku 5.5), used only if the legal entity and users are in supported regions.
  - `OpenAiCompatBackend` (YandexGPT via AI Studio).
  - GigaChat optional, via its native REST API with OAuth token exchange, implemented directly with `reqwest`. This avoids a Python proxy in a Rust stack.
- Keep the safety layer (rule red flags + classifier + server-side escalation card) provider-independent, so switching models does not weaken escalation.
- If you use Russian providers, use their embeddings too (Yandex text embeddings, GigaChat Embeddings), or local Giga-Embeddings/FRIDA. This keeps data inside the Russian Federation, which may matter for 152-FZ personal-data localization (not researched).
- Proxying Anthropic through a foreign intermediary to serve Russian users would violate Anthropic's terms. Do not recommend it.

### Gaps
- I did not research GigaChat's current API (endpoints, OAuth flow, models such as GigaChat 2 Max, pricing) or YandexGPT pricing.
- No Rust crates for GigaChat or YandexGPT were found or checked (for example, `gigachat-rs`).
- Not checked: whether Bedrock, Vertex or Foundry Claude access is blocked for Russian entities (very likely, given sanctions and provider terms), and the applicability of 152-FZ.
