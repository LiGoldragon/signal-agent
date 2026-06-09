//! Schema-derived ordinary Signal contract for the `agent` LLM-call component.
//!
//! Read this crate as the public call surface of the `agent` daemon. A peer —
//! principally the gated Spirit guardian — sends a `Call(Prompt)` and receives a
//! `Completed(Completion)`, or opens a streaming call (`StreamCall`) and consumes
//! `TokenStreamDelta` / `CompletionStreamDelta` events on the opened
//! `CompletionStream`, cancelling with `CancelStream`.
//!
//! Scope (psyche f8k7, iucr): `agent` makes provider HTTP API calls in an
//! OpenAI-compatible chat-completions style; it is NOT an agent-harness front
//! door. Adding a provider (DeepSeek, MiMo, Kimi, GLM, MiniMax) is configuration
//! in `meta-signal-agent`, not a change to this contract.
//!
//! `schema/lib.schema` is the source of truth. The checked-in `src/schema/lib.rs`
//! is a freshness-checked schema-rust-next artifact, not handwritten vocabulary.
//! See `ARCHITECTURE.md` for the channel's role and boundaries.

#[rustfmt::skip]
pub mod schema;

pub use schema::lib::*;

impl ChatMessage {
    pub fn system(text: impl Into<String>) -> Self {
        Self {
            role: ChatRole::System,
            text: UserText::new(text.into()),
        }
    }

    pub fn user(text: impl Into<String>) -> Self {
        Self {
            role: ChatRole::User,
            text: UserText::new(text.into()),
        }
    }

    pub fn assistant(text: impl Into<String>) -> Self {
        Self {
            role: ChatRole::Assistant,
            text: UserText::new(text.into()),
        }
    }
}

impl Input {
    pub fn operation_kind(&self) -> OperationKind {
        match self {
            Self::Call(_) => OperationKind::Call,
            Self::StreamCall(_) => OperationKind::StreamCall,
            Self::CancelStream(_) => OperationKind::CancelStream,
        }
    }
}
