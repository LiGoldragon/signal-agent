# signal-agent agent notes

You MUST read `/home/li/primary/AGENTS.md` first.

This repository is the ordinary typed contract for the agent front door
from `/home/li/primary/reports/designer/309-design-agent-component-abstraction.md`
and the Wave 3 booking in
`/home/li/primary/reports/designer/310-meta-overhaul-booking-roadmap.md`.
Keep daemon behavior, actors, storage, backend spawning, and CLI surface policy
out of this crate. This crate owns only the wire vocabulary and its NOTA/rkyv
round-trip witnesses.
