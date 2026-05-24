# signal-agent

Ordinary Signal contract for the agent front door.

Read `src/lib.rs` for the public interface. The `signal_channel!` declaration
emits `Operation`, `Reply`, and `Event` for `Send`, `Cancel`,
`SubscribeTranscript`, `Observe`, the transcript stream, and the macro-mandated
`Tap`/`Untap` observer stream.
