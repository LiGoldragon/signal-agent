# INTENT — signal-agent

*The ordinary peer-callable wire contract for the `agent` LLM-call component.
Companion to `ARCHITECTURE.md` and `Cargo.toml`.
Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-agent` contract.
Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
Component daemon intent stays in `agent/INTENT.md`. Meta policy intent stays
in `meta-signal-agent/INTENT.md`.

## Why this repo exists

`signal-agent` is the **ordinary peer-callable wire contract** for the `agent`
component — the working signal a peer (principally the gated Spirit guardian)
uses to ask a configured LLM provider to complete a prompt. It carries a
single-shot `Call`, a streaming `StreamCall` with token deltas, and a
`CancelStream`. Provider configuration and lifecycle stay in
`meta-signal-agent`; the runtime, the provider registry, the HTTPS call,
sockets, and storage live in `agent`.

## Stated psyche intent

The psyche authorized this build and made two shaping decisions, captured in
Spirit:

- The `agent` component is built as an LLM-API-call component that makes
  provider HTTP API calls in an API style, *not* as an agent-harness front door;
  harness backends are deferred. (Spirit `iucr`, Decision.)
- The `agent` component models LLM providers as a generic OpenAI-compatible API
  with endpoint, model, and key as configuration, so adding a provider is
  configuration rather than a contract change. (Spirit `f8k7`, Decision.)

The second decision is the load-bearing constraint on this crate: the contract
names no concrete provider. A `ProviderName` is a string the daemon's registry
resolves; adding DeepSeek, MiMo, Kimi, GLM, or MiniMax is configuration in
`meta-signal-agent`, never a new variant here.

## The channel shape

- **Requests:** `Call(Call)`, `StreamCall(StreamCall)` (opens
  `CompletionStream`), `CancelStream(CancelStream)`.
- **Replies:** `Completed(Completion)`, `CallRejected(CallRejection)`,
  `StreamOpened(StreamOpening)`, `StreamCancelled(StreamCancellation)`,
  `RequestUnimplemented(RequestUnimplemented)` (skeleton honesty).
- **Events:** `TokenStreamDelta` and a terminal `CompletionStreamDelta` on the
  open `CompletionStream`.

A `Prompt` carries an optional system message, a chat transcript, and call
options (model, provider, temperature, max output tokens, output mode). The
guardian wants a structured verdict back, so `OutputMode::Nota` asks the
daemon to request JSON output from the provider.

## Constraints

- This crate carries only typed wire vocabulary, NOTA codecs, and round-trip
  witnesses. No runtime: no actors, sockets, tokio, redb, or HTTP client.
- Wire enums are closed. No `Unknown` escape hatch. No concrete provider name
  appears as a variant.
- Identifiers are full English words.
- Every operation, reply, and event has an rkyv and NOTA round-trip witness in
  `tests/round_trip.rs`.

## Non-ownership

This crate does not own:

- `agent` daemon actor topology, the provider registry, redb tables, the HTTPS
  provider call, or sockets;
- provider configuration (endpoint / model / key handle) or lifecycle (those
  live in `meta-signal-agent`);
- per-provider private contracts — providers are a generic OpenAI-compatible
  API resolved at the daemon.

## What this is NOT

- Not an agent-harness delivery contract. The earlier message-delivery /
  transcript-from-a-running-agent / backend-spawn framing is discarded; this
  contract is the LLM-call surface only.

## See also

- `ARCHITECTURE.md` — detailed channel shape, owned vocabulary, closed enums.
- `../agent/INTENT.md` — daemon-side intent (provider registry, the HTTPS call).
- `../meta-signal-agent/INTENT.md` — provider configuration + lifecycle.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and wire layers.
