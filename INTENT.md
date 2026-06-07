# INTENT — signal-agent

*The ordinary peer-callable wire contract for the `agent` front door.
Companion to `ARCHITECTURE.md` and `Cargo.toml`.
Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-agent`
contract. Workspace-shape intent stays in the primary workspace
`primary/INTENT.md`. Component daemon intent stays in `agent/INTENT.md`.
Owner-only policy intent stays in `owner-signal-agent/INTENT.md`.

## Why this repo exists

`signal-agent` is the **ordinary peer-callable wire contract** for the
`agent` component — the router-facing working signal in the agent triad
(`agent` runtime, `signal-agent` ordinary contract, `owner-signal-agent`
owner-only policy contract). It carries the vocabulary for delivering
one message to one agent, cancelling in-flight deliveries, subscribing
to a push transcript stream, and observing agent-front-door state.
Owner-only spawn, retire, backend assignment, and route-toggle policy
stay in `owner-signal-agent`; runtime actors, the backend registry,
sockets, storage, and delivery reducers live in `agent`.

## The channel shape

The `Agent` channel carries:

- **Requests:** `Send(MessageDelivery)`, `Cancel(DeliveryCancellation)`,
  `SubscribeTranscript(TranscriptSubscription)` (opens the transcript
  stream), `TranscriptRetraction`, `Observe(ObservationSelection)`.
- **Replies:** `DeliveryAcknowledged`, `DeliveryFailed`, `Cancelled`,
  `TranscriptSnapshot`, `TranscriptSubscriptionRetracted`, `Observed`,
  and `RequestUnimplemented` (skeleton honesty).
- **Events:** `TranscriptDelta` on the open transcript stream; the
  macro-injected `Tap`/`Untap` observer stream taps operations and
  contract-owned effects, distinct from the transcript stream.

A single agent socket is multiplexed by `AgentIdentifier`.

## Channels are closed, boundaries are named

- Wire enums are closed. No `Unknown` escape hatch.
- `AgentBackend` is closed: adding a backend is a coordinated contract
  bump.
- Backend assignment is *observed* here; owner policy *assigns* it in
  `owner-signal-agent`.
- Transcript observation is push-shaped: snapshot first, then deltas,
  then typed retraction acknowledgement.
- Request payloads do not mint delivery identity, timestamps, or the
  durable `Delivered` fact; the router writes the durable fact after the
  agent acknowledges.

## Wire vocabulary discipline

Per `primary/skills/contract-repo.md` §"Public contracts use
contract-local operation verbs":

- Operation roots are domain verbs in verb form: `Send`, `Cancel`,
  `SubscribeTranscript`, `Observe` — never Sema class words.
- Reply success variants name the outcome: `Send` →
  `DeliveryAcknowledged`/`DeliveryFailed`; `Observe` → `Observed`.
- Payload records are domain nouns: `MessageDelivery`,
  `DeliveryCancellation`, `TranscriptSubscription`, `ObservationSelection`.

## Constraints

- This crate carries only typed wire vocabulary, NOTA codecs, and
  round-trip witnesses.
- No runtime code: no actors, sockets, tokio, redb, or backend spawning.
- Contract types derive NOTA in this crate. Clients do not carry shadow
  types that re-derive the text surface.
- Every operation, reply, event, and public payload shape has an rkyv
  and NOTA round-trip witness in `tests/round_trip.rs`.
- `IngressContext` / `ConnectionClass` are imported from
  `signal-persona-origin`; message sender/body/slot records are imported
  from `signal-message` — this contract does not invent a parallel origin
  or slot shape.

## Non-ownership

This crate does not own:

- `agent-daemon` actor topology, redb tables, the backend registry, or
  delivery reducers;
- owner-only spawn, retire, backend policy, or route-toggle verbs (those
  live in `owner-signal-agent`);
- backend-private contracts for the individual agent runtimes;
- router durability state — the router writes the durable `Delivered`
  fact after receiving the agent acknowledgement.

## See also

- `ARCHITECTURE.md` — detailed channel shape, owned vocabulary, and
  closed-enum discipline.
- `../agent/INTENT.md` — daemon-side intent (schema-driven planes,
  actors, state) when it lands.
- `../owner-signal-agent/INTENT.md` — owner-only policy signal.
- `primary/skills/contract-repo.md` — contract repo discipline and
  naming rules.
- `primary/skills/component-triad.md` — repo triad structure and wire
  layers.
