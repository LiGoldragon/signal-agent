//! Ordinary Signal contract for the agent front door.
//!
//! `persona-router` sends deliveries to one `agent` socket. The agent
//! daemon multiplexes by [`AgentIdentifier`] and forwards to backend daemons
//! according to owner-managed policy. This crate carries only the typed wire
//! vocabulary; runtime routing, storage, backend spawning, and delivery
//! reducers live in `agent`.

use nota_next::{Block, Delimiter, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::{SubscriptionTokenInner, signal_channel};

pub use signal_frame::{
    ExchangeFrameBody as FrameExchangeFrameBody, HandshakeReply, HandshakeRequest, ProtocolVersion,
    Request as FrameRequest, SIGNAL_FRAME_PROTOCOL_VERSION,
};
pub use signal_message::{MessageBody, MessageSender, MessageSlot};
pub use signal_persona_origin::{ConnectionClass, IngressContext};

/// Stable identity for one logical agent run.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct AgentIdentifier(String);

impl AgentIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Backend family assigned to an agent run by owner policy.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum AgentBackend {
    Claude,
    Codex,
    Gemini,
    Pi,
    OpenCode,
    Fixture,
}

/// Opaque delivery identity minted by the router for one in-flight delivery.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct DeliveryToken(String);

impl DeliveryToken {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Message bytes and provenance routed to a single logical agent.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct MessageDelivery {
    pub agent: AgentIdentifier,
    pub delivery_token: DeliveryToken,
    pub message_slot: MessageSlot,
    pub sender: MessageSender,
    pub body: MessageBody,
    pub ingress_context: IngressContext,
    pub connection_class: ConnectionClass,
}

/// Retracts one in-flight delivery.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DeliveryCancellation {
    pub agent: AgentIdentifier,
    pub token: DeliveryToken,
}

/// Backend or harness accepted the delivery bytes.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DeliveryAcknowledgement {
    pub agent: AgentIdentifier,
    pub token: DeliveryToken,
    pub message_slot: MessageSlot,
}

/// Cancellation reached the component that owned the in-flight delivery.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DeliveryCancellationAcknowledgement {
    pub agent: AgentIdentifier,
    pub token: DeliveryToken,
}

/// Delivery failure reported back through the agent front door.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DeliveryFailure {
    pub agent: AgentIdentifier,
    pub token: DeliveryToken,
    pub message_slot: MessageSlot,
    pub reason: DeliveryFailureReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum DeliveryFailureReason {
    BackendUnavailable(AgentBackend),
    TransportRejected,
    HumanInputIntervened,
    AgentStoppedBeforeDelivery,
    AcknowledgementChainBroken(AcknowledgementHop),
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum AcknowledgementHop {
    BackendDaemon,
    AgentDaemon,
    Router,
}

/// Opaque stream token for one transcript subscription.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TranscriptToken(SubscriptionTokenInner);

impl TranscriptToken {
    pub const fn new(inner: SubscriptionTokenInner) -> Self {
        Self(inner)
    }

    pub const fn inner(self) -> SubscriptionTokenInner {
        self.0
    }
}

impl NotaEncode for TranscriptToken {
    fn to_nota(&self) -> String {
        Delimiter::Parenthesis.wrap(["TranscriptToken".to_owned(), self.0.value().to_string()])
    }
}

impl NotaDecode for TranscriptToken {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let fields =
            NotaBlock::new(block).expect_children(Delimiter::Parenthesis, "TranscriptToken", 2)?;
        let head = fields[0]
            .demote_to_string()
            .ok_or(NotaDecodeError::ExpectedAtom {
                type_name: "TranscriptToken",
            })?;
        if head != "TranscriptToken" {
            return Err(NotaDecodeError::UnknownVariant {
                enum_name: "TranscriptToken",
                variant: head.to_owned(),
            });
        }
        let value = NotaBlock::new(&fields[1]).parse_integer()?;
        Ok(Self(SubscriptionTokenInner::new(value)))
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct TranscriptSequence(u64);

impl TranscriptSequence {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TranscriptLine(String);

impl TranscriptLine {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Opens the push transcript stream for one agent.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptSubscription {
    pub agent: AgentIdentifier,
}

/// Initial transcript state returned when a subscription opens.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptSnapshot {
    pub agent: AgentIdentifier,
    pub token: TranscriptToken,
    pub current_sequence: TranscriptSequence,
    pub lines: Vec<TranscriptLine>,
}

/// Transcript delta pushed after the opening snapshot.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptDelta {
    pub agent: AgentIdentifier,
    pub token: TranscriptToken,
    pub sequence: TranscriptSequence,
    pub line: TranscriptLine,
}

/// Acknowledges transcript stream retraction.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TranscriptSubscriptionRetracted {
    pub token: TranscriptToken,
}

/// Selects the ordinary observation returned by `Observe`.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum ObservationSelection {
    Agent(AgentIdentifier),
    AllAgents,
}

/// Current state visible through the ordinary observation surface.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum Observation {
    Agent(AgentObservation),
    Agents(Vec<AgentObservation>),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AgentObservation {
    pub agent: AgentIdentifier,
    pub backend: AgentBackend,
    pub lifecycle: AgentLifecycle,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum AgentLifecycle {
    Starting,
    Running,
    Draining,
    Stopped,
    Failed,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    BackendUnavailable(AgentBackend),
    DependencyNotReady,
}

signal_channel! {
    channel Agent {
        operation Send(MessageDelivery),
        operation Cancel(DeliveryCancellation),
        operation SubscribeTranscript(TranscriptSubscription) opens TranscriptStream,
        operation TranscriptRetraction(TranscriptToken),
        operation Observe(ObservationSelection),
    }
    reply Reply {
        DeliveryAcknowledged(DeliveryAcknowledgement),
        DeliveryFailed(DeliveryFailure),
        Cancelled(DeliveryCancellationAcknowledgement),
        TranscriptSnapshot(TranscriptSnapshot),
        TranscriptSubscriptionRetracted(TranscriptSubscriptionRetracted),
        Observed(Observation),
        RequestUnimplemented(RequestUnimplemented),
    }
    event Event {
        TranscriptDelta(TranscriptDelta) belongs TranscriptStream,
    }
    stream TranscriptStream {
        token TranscriptToken;
        opened TranscriptSnapshot;
        event TranscriptDelta;
        close TranscriptRetraction;
    }
    observable {
        filter default;
        operation_event OperationReceived;
        effect_event EffectEmitted;
    }
}

/// A valid request reached the daemon but is not implemented in the current
/// skeleton.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RequestUnimplemented {
    pub reason: UnimplementedReason,
}

/// Observer event emitted before an inbound operation lowers into daemon work.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OperationReceived {
    pub operation: OperationKind,
}

/// Contract-owned outcome for work the daemon emits to observers after
/// processing an operation.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum EffectOutcome {
    Delivered,
    Cancelled,
    Observed,
    StreamOpened,
    StreamClosed,
    Failed,
    NoChange,
}

/// Observer event emitted after daemon work produces an externally visible effect.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct EffectEmitted {
    pub operation: OperationKind,
    pub outcome: EffectOutcome,
}
