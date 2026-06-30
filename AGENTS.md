# signal-agent agent notes

You MUST read `/home/li/primary/AGENTS.md` first, then this repo's
`ARCHITECTURE.md` — the durable direction read on entry before code.

This repository is the ordinary schema-derived wire contract for the `agent`
LLM-call component — the single-shot + streaming provider-call surface. `agent`
makes OpenAI-compatible provider HTTP API calls; it is NOT an agent harness
(psyche Spirit `iucr`, `f8k7`).

Keep daemon behaviour, actors, storage, the provider registry, the HTTPS call,
and CLI surface policy out of this crate. This crate owns only the wire
vocabulary and its NOTA/rkyv round-trip witnesses. Edit `schema/lib.schema` and
regenerate (`SIGNAL_AGENT_UPDATE_SCHEMA_ARTIFACTS=1 cargo build`); never
hand-edit `src/schema/lib.rs`.
