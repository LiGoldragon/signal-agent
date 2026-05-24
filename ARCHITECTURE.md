# signal-agent — Architecture

`signal-agent` is the ordinary Signal contract for the `agent`
front door. It is the router-facing working signal in the `agent` triad:
`agent` runtime, `signal-agent` ordinary contract, and
`owner-signal-agent` owner-only policy contract.

This shape follows
`/home/li/primary/reports/designer/309-design-agent-component-abstraction.md`
and the Wave 3 booking in
`/home/li/primary/reports/designer/310-meta-overhaul-booking-roadmap.md`.

## Boundary

This crate carries four peer-callable relations:

- `Send(MessageDelivery)` — router submits one message to one agent, keyed by
  `AgentIdentifier` and `DeliveryToken`.
- `Cancel(DeliveryCancellation)` — router retracts an in-flight delivery by
  token.
- `SubscribeTranscript(TranscriptSubscription)` — caller opens a push transcript
  stream for one agent and receives `TranscriptSnapshot` followed by
  `TranscriptDelta` events.
- `Observe(ObservationSelection)` — caller asks for current agent-front-door
  state, such as one agent's backend and lifecycle.

The macro-injected `Tap` / `Untap` observer stream exists because this is an
observable component contract. `Tap` observes operations and Sema effects; it is
not the transcript stream.

## Owned vocabulary

This crate owns:

- agent identity and backend vocabulary: `AgentIdentifier`, `AgentBackend`;
- delivery vocabulary: `DeliveryToken`, `MessageDelivery`,
  `DeliveryAcknowledgement`, `DeliveryFailure`, and cancellation records;
- transcript vocabulary: `TranscriptToken`, `TranscriptSequence`,
  `TranscriptSnapshot`, `TranscriptDelta`, and transcript line records;
- observation vocabulary: `ObservationSelection`, `Observation`,
  `AgentObservation`, and `AgentLifecycle`;
- skeleton honesty: `RequestUnimplemented` with a closed
  `UnimplementedReason`.

It imports `IngressContext` and `ConnectionClass` from `signal-persona-origin`
so the front door can carry already-classified ingress provenance without
inventing a parallel origin shape. It imports message sender, body, and slot
records from `signal-message` so delivery acknowledgement stays keyed to
the router/message durable slot vocabulary.

## Not owned

This crate does not own:

- `agent-daemon` actor topology, redb tables, backend registry, or
  delivery reducers;
- owner-only spawn, retire, backend policy, or route-toggle verbs;
- backend-private contracts for Claude, Codex, Gemini, Pi, OpenCode, or the
  fixture backend;
- router durability state. The router writes the durable `Delivered` fact after
  receiving the agent acknowledgement.

## Invariants

- A single agent socket is multiplexed by `AgentIdentifier`.
- `AgentBackend` is closed: adding a backend is a coordinated contract bump.
- Backend assignment is observed here; owner policy assigns it elsewhere.
- Transcript observation is push-shaped: snapshot first, then deltas, then typed
  retraction acknowledgement.
- Wire enums contain no `Unknown` escape hatch.
- Runtime code stays out of the contract crate: no actors, sockets, tokio, redb,
  or backend spawning.
- Every operation, reply, event, and public payload shape has a NOTA and rkyv
  round-trip witness in `tests/round_trip.rs`.

## Code map

```text
src/lib.rs              payloads plus the signal_channel! declaration
examples/canonical.nota one canonical NOTA example per operation/reply/event
tests/round_trip.rs     rkyv frame and NOTA round-trip witnesses
```

## See also

- `/home/li/primary/skills/component-triad.md`
- `/home/li/primary/skills/contract-repo.md`
- `/home/li/primary/reports/designer/309-design-agent-component-abstraction.md`
- `/home/li/primary/reports/designer/310-meta-overhaul-booking-roadmap.md`
