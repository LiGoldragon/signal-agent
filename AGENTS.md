# signal-agent agent notes

Read this repo's `ARCHITECTURE.md` before editing — the durable direction read
on entry before code.

This repository is the ordinary schema-derived wire contract for the `agent`
LLM-call component — the single-shot + streaming provider-call surface. `agent`
makes OpenAI-compatible provider HTTP API calls; it is NOT an agent harness
(psyche Spirit `iucr`, `f8k7`).

Keep daemon behaviour, actors, storage, the provider registry, the HTTPS call,
and CLI surface policy out of this crate. This crate owns only the wire
vocabulary and its Dotos/rkyv round-trip witnesses. Edit
`ethos/interface.ethos` and the producer-owned authority manifest together.
Regenerate with `SIGNAL_AGENT_UPDATE_INTERFACE_ARTIFACTS=1 cargo build`; never
hand-edit `src/schema/lib/generated.rs`.

## Protos estate status

Stack: correct-new destination
Status: active component contract, current checkout legacy-wired
This checkout is not proof of correct-new adoption.
