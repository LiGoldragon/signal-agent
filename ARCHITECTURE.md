# signal-agent — Architecture

`signal-agent` is the ordinary working Signal contract for the `agent`
LLM-call component. It is the peer-callable working signal in the `agent`
triad: `agent` runtime, `signal-agent` ordinary contract, and
`meta-signal-agent` meta policy contract.

It is a schema-derived `WireContract` crate: `schema/lib.schema` is the source
of truth; `schema-rust-next`'s `ContractCrateBuild` emits the freshness-checked
`src/schema/lib.rs` (wire types + rkyv + NOTA codecs over the
`signal_frame::StreamingFrame` envelope). The crate carries no engine traits,
runtime, actors, or `tokio`.

## Scope — LLM API calls, not a harness

Per psyche intent (Spirit `f8k7`, `iucr`), `agent` makes provider HTTP API
calls in an OpenAI-compatible chat-completions style. It is the LLM-call
substrate the gated Spirit guardian uses to judge intent. It is NOT an
agent-harness front door — harness backends (Claude Code / Codex / Pi sessions)
are deferred and absent from this contract. The earlier message-delivery /
backend-spawn framing is discarded.

Adding a provider (DeepSeek, MiMo, Kimi, GLM, MiniMax) is configuration in
`meta-signal-agent` — endpoint + model + key handle — never a change to this
contract.

## Component direction (archived intent)

These records frame the `agent` component the contract serves. Daemon-side
behavior they name lives in `agent` / `meta-signal-agent`, not in this
contract crate.

- **Workspace-native `agent` triad (`7yth`).** `agent` is the workspace-native
  successor to `persona-llm-client` (the `persona-` prefix is dropped), shaped
  as its own triad — `agent` CLI, `agent-daemon`, `signal-agent`, and an
  owner-side `owner-signal-agent` — a supervised component others call for
  pre-configured API agent calls (not a thin wrapper, not subsumed into Pi). It
  hooks to `mind` for skills/context (a model call plus the agent is a thinking
  process) and takes typed NOTA directly from the model via prompt examples
  plus validate-and-retry, grammar-constrained when self-hosted. (The earlier
  `gdbf` framing of `agent` as an abstraction over harness backends — Claude
  Code / Codex / Pi sessions — is superseded: see **Scope** above, where
  harness backends are deferred and the backend-spawn framing is discarded.
  `gdbf`'s still-true core, providers as an HTTP-API-call component and Router
  talking to `agent` rather than harness directly, is preserved by this triad.)
- **Fallback chain and Criome escalation (`l0w8`).** LLM calls are computation
  units configured as a default provider plus an ordered fallback-provider
  chain tried in sequence, with per-provider config tweaks evaluated at the
  typed daemon boundary. The daemon library is the reducer plus checker —
  including authorization that can escalate to Criome — giving failure
  tolerance and provider portability.
- **Capability observation (`f8k7`).** Provider support may be build-time
  opt-in; capability observation distinguishes built-but-unconfigured from
  not-built, and an unsupported request returns a typed unsupported-capability
  reply (this contract's `RequestUnimplemented` skeleton-honesty reply is the
  wire surface for that case).

## Boundary

This crate carries one peer-callable relation: a caller (the gated Spirit
guardian) asks a configured provider to complete a prompt.

- `Call(Call)` — one single-shot completion. Reply: `Completed(Completion)` or
  `CallRejected(CallRejection)`.
- `StreamCall(StreamCall)` — the same call, streamed; opens `CompletionStream`.
  Reply: `StreamOpened(StreamOpening)`; then `TokenStreamDelta` events, then one
  terminal `CompletionStreamDelta` event carrying stop reason and usage.
- `CancelStream(CancelStream)` — cancel an open stream by `StreamToken`. Reply:
  `StreamCancelled(StreamCancellation)`.

`RequestUnimplemented` is the skeleton-honesty reply for a valid request the
daemon does not yet serve.

## Owned vocabulary

- prompt vocabulary: `Prompt`, `ChatTranscript`, `ChatMessage`, `ChatRole`
  (closed: `System` / `User` / `Assistant`), `SystemText`, `UserText`,
  `PromptOptions`, `OutputMode` (closed: `FreeText` / `Nota`), and the
  call-tuning newtypes `ModelName`, `ProviderName`, `TemperatureMilli`,
  `MaximumOutputTokens`;
- completion vocabulary: `Completion`, `CompletionText`, `StopReasonText`,
  `TokenUsage`, `PromptTokenCount`, `CompletionTokenCount`;
- streaming vocabulary: `StreamToken`, `StreamOpening`, `StreamCancellation`,
  `TokenStreamDelta`, `CompletionStreamDelta`, `DeltaSequence`, `TokenDelta`;
- rejection vocabulary: `CallRejection`, `CallRejectionReason` (closed),
  `RejectionDetail`;
- skeleton honesty: `RequestUnimplemented`, `OperationKind`,
  `UnimplementedReason` (all closed).

The output-mode design is load-bearing: the guardian asks for `Nota` to
get a typed structured verdict back; the daemon asks the provider to emit JSON.

## Not owned

- `agent` daemon actor topology, the provider registry, redb tables, the HTTPS
  provider call, or sockets;
- provider configuration (endpoint / model / key handle) and lifecycle — those
  live in `meta-signal-agent`;
- per-provider private wire shapes — providers are a generic
  OpenAI-compatible API resolved at the daemon, not contract variants.

## Invariants

- Wire enums are closed: no `Unknown` escape hatch. `ChatRole`, `OutputMode`,
  `CallRejectionReason`, `OperationKind`, `UnimplementedReason` are all closed.
- The contract names NO concrete provider. A provider is a `ProviderName`
  string resolved by the daemon's registry, so adding one is configuration.
- Streaming is push-shaped: `StreamOpened` first, then token deltas, then one
  terminal completion delta.
- Request payloads carry no minted identity, timestamps, or durable facts — the
  daemon mints the `StreamToken` and stamps usage.
- Runtime code stays out of the contract crate: no actors, sockets, tokio, redb,
  or HTTP client.
- Every operation, reply, and event variant has a NOTA and rkyv round-trip
  witness in `tests/round_trip.rs`.

## Code map

```text
schema/lib.schema       the source of truth (schema-rust-next grammar)
src/schema/lib.rs        freshness-checked schema-rust-next artifact (generated)
src/lib.rs               module entry + hand-written methods on emitted nouns
build.rs                 ContractCrateBuild -> WireContract emission
examples/canonical.nota  one canonical NOTA example per operation/reply/event
tests/round_trip.rs      rkyv frame and NOTA round-trip witnesses
```

## See also

- `/home/li/primary/skills/component-triad.md`
- `/home/li/primary/skills/contract-repo.md`
- `../agent/ARCHITECTURE.md` — daemon-side runtime triad and provider call.
- `../meta-signal-agent/ARCHITECTURE.md` — provider configuration + lifecycle.
