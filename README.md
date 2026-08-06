# signal-agent

Ordinary Signal contract for the `agent` LLM-call component.

`agent` makes provider HTTP API calls in an OpenAI-compatible chat-completions
style (psyche scope: an LLM-API caller, not an agent harness). This crate is the
schema-derived wire vocabulary a peer (the gated Spirit guardian) uses to ask a
configured provider to complete a prompt: `Call` (single-shot), `StreamCall`
(token deltas on `CompletionStream`), and `CancelStream`.

`ethos/interface.ethos` is the authority-verified Interface. Core Nomos
revalidates it and Rust Logos projects only encoded Rust coordinates. Read
`ARCHITECTURE.md` for the channel shape. Adding a provider is configuration in
`meta-signal-agent`, never a change here.
