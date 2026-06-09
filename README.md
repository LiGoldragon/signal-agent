# signal-agent

Ordinary Signal contract for the `agent` LLM-call component.

`agent` makes provider HTTP API calls in an OpenAI-compatible chat-completions
style (psyche scope: an LLM-API caller, not an agent harness). This crate is the
schema-derived wire vocabulary a peer (the gated Spirit guardian) uses to ask a
configured provider to complete a prompt: `Call` (single-shot), `StreamCall`
(token deltas on `CompletionStream`), and `CancelStream`.

`schema/lib.schema` is the source of truth; `schema-rust-next` emits the
freshness-checked `src/schema/lib.rs`. Read `ARCHITECTURE.md` for the channel
shape and `INTENT.md` for the psyche-stated scope. Adding a provider is
configuration in `meta-signal-agent`, never a change here.
