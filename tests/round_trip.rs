use std::fmt::Debug;

use nota_next::{NotaDecode, NotaEncode, NotaSource};
use signal_agent::{
    AgentBackend, AgentIdentifier, AgentLifecycle, AgentObservation, DeliveryAcknowledgement,
    DeliveryCancellation, DeliveryCancellationAcknowledgement, DeliveryFailure,
    DeliveryFailureReason, DeliveryToken, EffectEmitted, EffectOutcome, Event, Frame, FrameBody,
    MessageBody, MessageDelivery, MessageSender, MessageSlot, Observation, ObservationSelection,
    Operation, OperationKind, OperationReceived, Reply, RequestUnimplemented, StreamKind,
    TranscriptDelta, TranscriptLine, TranscriptSequence, TranscriptSnapshot,
    TranscriptSubscription, TranscriptSubscriptionRetracted, TranscriptToken, UnimplementedReason,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply as FrameReply, RequestPayload,
    SessionEpoch, StreamEventIdentifier, SubReply, SubscriptionTokenInner,
};
use signal_persona_origin::{ConnectionClass, IngressContext};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn event_identifier() -> StreamEventIdentifier {
    StreamEventIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Acceptor,
        LaneSequence::first(),
    )
}

fn completed_reply(payload: Reply) -> FrameReply<Reply> {
    FrameReply::committed(NonEmpty::single(SubReply::Ok(payload)))
}

fn agent() -> AgentIdentifier {
    AgentIdentifier::new("agent-alpha")
}

fn delivery_token() -> DeliveryToken {
    DeliveryToken::new("delivery-0001")
}

fn transcript_token() -> TranscriptToken {
    TranscriptToken::new(SubscriptionTokenInner::new(7))
}

fn message_delivery() -> MessageDelivery {
    MessageDelivery {
        agent: agent(),
        delivery_token: delivery_token(),
        message_slot: MessageSlot::new(42),
        sender: MessageSender::new("router"),
        body: MessageBody::new("hello agent"),
        ingress_context: IngressContext::external(ConnectionClass::Owner),
        connection_class: ConnectionClass::Owner,
    }
}

fn transcript_snapshot() -> TranscriptSnapshot {
    TranscriptSnapshot {
        agent: agent(),
        token: transcript_token(),
        current_sequence: TranscriptSequence::new(2),
        lines: vec![
            TranscriptLine::new("first line"),
            TranscriptLine::new("second line"),
        ],
    }
}

fn observation() -> Observation {
    Observation::Agent(AgentObservation {
        agent: agent(),
        backend: AgentBackend::Claude,
        lifecycle: AgentLifecycle::Running,
    })
}

fn effect_emitted() -> EffectEmitted {
    EffectEmitted {
        operation: OperationKind::Send,
        outcome: EffectOutcome::Delivered,
    }
}

fn round_trip_operation(operation: Operation) -> Operation {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: operation.clone().into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode operation");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode operation");

    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request, got {other:?}"),
    }
}

fn round_trip_reply(reply: Reply) -> Reply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: completed_reply(reply.clone()),
    });
    let bytes = frame.encode_length_prefixed().expect("encode reply");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode reply");

    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            FrameReply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply, got {other:?}"),
    }
}

fn round_trip_event(event: Event) -> Event {
    let frame = Frame::new(FrameBody::SubscriptionEvent {
        event_identifier: event_identifier(),
        token: SubscriptionTokenInner::new(11),
        event: event.clone(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode event");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode event");

    match decoded.into_body() {
        FrameBody::SubscriptionEvent { event, .. } => event,
        other => panic!("expected subscription event, got {other:?}"),
    }
}

fn round_trip_nota<T>(value: T) -> String
where
    T: NotaEncode + NotaDecode + PartialEq + Debug,
{
    let text = value.to_nota();
    let recovered = NotaSource::new(&text).parse::<T>().expect("decode nota");
    assert_eq!(recovered, value);
    text
}

#[test]
fn operations_round_trip_through_length_prefixed_frames() {
    let operations = [
        Operation::Send(message_delivery()),
        Operation::Cancel(DeliveryCancellation {
            agent: agent(),
            token: delivery_token(),
        }),
        Operation::SubscribeTranscript(TranscriptSubscription { agent: agent() }),
        Operation::TranscriptRetraction(transcript_token()),
        Operation::Observe(ObservationSelection::Agent(agent())),
        Operation::Tap(signal_agent::ObserverFilter::All),
        Operation::Untap(signal_agent::ObserverSubscriptionToken::new(
            SubscriptionTokenInner::new(3),
        )),
    ];

    for operation in operations {
        assert_eq!(round_trip_operation(operation.clone()), operation);
    }
}

#[test]
fn replies_round_trip_through_length_prefixed_frames() {
    let replies = [
        Reply::DeliveryAcknowledged(DeliveryAcknowledgement {
            agent: agent(),
            token: delivery_token(),
            message_slot: MessageSlot::new(42),
        }),
        Reply::DeliveryFailed(DeliveryFailure {
            agent: agent(),
            token: delivery_token(),
            message_slot: MessageSlot::new(42),
            reason: DeliveryFailureReason::BackendUnavailable(AgentBackend::Claude),
        }),
        Reply::Cancelled(DeliveryCancellationAcknowledgement {
            agent: agent(),
            token: delivery_token(),
        }),
        Reply::TranscriptSnapshot(transcript_snapshot()),
        Reply::TranscriptSubscriptionRetracted(TranscriptSubscriptionRetracted {
            token: transcript_token(),
        }),
        Reply::Observed(observation()),
        Reply::RequestUnimplemented(RequestUnimplemented {
            reason: UnimplementedReason::NotBuiltYet,
        }),
        Reply::ObserverSubscriptionOpened(signal_agent::ObserverSubscriptionOpened::new(
            signal_agent::ObserverSubscriptionToken::new(SubscriptionTokenInner::new(5)),
        )),
    ];

    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn events_round_trip_through_length_prefixed_frames() {
    let events = [
        Event::TranscriptDelta(TranscriptDelta {
            agent: agent(),
            token: transcript_token(),
            sequence: TranscriptSequence::new(3),
            line: TranscriptLine::new("third line"),
        }),
        Event::OperationReceived(OperationReceived {
            operation: OperationKind::Send,
        }),
        Event::EffectEmitted(effect_emitted()),
    ];

    for event in events {
        assert_eq!(round_trip_event(event.clone()), event);
    }
}

#[test]
fn public_payloads_round_trip_through_nota_text() {
    assert_eq!(
        round_trip_nota(AgentIdentifier::new("agent-alpha")),
        "[agent-alpha]"
    );
    assert_eq!(round_trip_nota(AgentBackend::Claude), "Claude");
    assert_eq!(round_trip_nota(AgentBackend::OpenCode), "OpenCode");
    assert_eq!(
        round_trip_nota(DeliveryToken::new("delivery-0001")),
        "[delivery-0001]"
    );
    assert_eq!(round_trip_nota(transcript_token()), "(TranscriptToken 7)");

    round_trip_nota(message_delivery());
    round_trip_nota(transcript_snapshot());
    round_trip_nota(TranscriptDelta {
        agent: agent(),
        token: transcript_token(),
        sequence: TranscriptSequence::new(3),
        line: TranscriptLine::new("third line"),
    });
    round_trip_nota(DeliveryAcknowledgement {
        agent: agent(),
        token: delivery_token(),
        message_slot: MessageSlot::new(42),
    });
    round_trip_nota(DeliveryFailure {
        agent: agent(),
        token: delivery_token(),
        message_slot: MessageSlot::new(42),
        reason: DeliveryFailureReason::AcknowledgementChainBroken(
            signal_agent::AcknowledgementHop::BackendDaemon,
        ),
    });
    round_trip_nota(RequestUnimplemented {
        reason: UnimplementedReason::NotBuiltYet,
    });
}

#[test]
fn public_payloads_round_trip_through_rkyv_archives() {
    let backend = AgentBackend::Fixture;
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&backend).expect("archive backend");
    let recovered =
        rkyv::from_bytes::<AgentBackend, rkyv::rancor::Error>(&bytes).expect("decode backend");
    assert_eq!(recovered, backend);

    let delivery = message_delivery();
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&delivery).expect("archive delivery");
    let recovered =
        rkyv::from_bytes::<MessageDelivery, rkyv::rancor::Error>(&bytes).expect("decode delivery");
    assert_eq!(recovered, delivery);

    let snapshot = transcript_snapshot();
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&snapshot).expect("archive snapshot");
    let recovered = rkyv::from_bytes::<TranscriptSnapshot, rkyv::rancor::Error>(&bytes)
        .expect("decode snapshot");
    assert_eq!(recovered, snapshot);
}

#[test]
fn operation_kind_and_stream_witnesses_are_generated_by_macro() {
    let send = Operation::Send(message_delivery());
    assert_eq!(send.kind(), OperationKind::Send);
    assert_eq!(send.opened_stream(), None);
    assert_eq!(send.closed_stream(), None);

    let subscribe = Operation::SubscribeTranscript(TranscriptSubscription { agent: agent() });
    assert_eq!(subscribe.kind(), OperationKind::SubscribeTranscript);
    assert_eq!(
        subscribe.opened_stream(),
        Some(StreamKind::TranscriptStream)
    );

    let close = Operation::TranscriptRetraction(transcript_token());
    assert_eq!(close.kind(), OperationKind::TranscriptRetraction);
    assert_eq!(close.closed_stream(), Some(StreamKind::TranscriptStream));

    let tap = Operation::Tap(signal_agent::ObserverFilter::OperationsOnly);
    assert_eq!(tap.kind(), OperationKind::Tap);
    assert_eq!(tap.opened_stream(), Some(StreamKind::ObserverStream));

    let event = Event::TranscriptDelta(TranscriptDelta {
        agent: agent(),
        token: transcript_token(),
        sequence: TranscriptSequence::new(3),
        line: TranscriptLine::new("third line"),
    });
    assert_eq!(event.stream_kind(), StreamKind::TranscriptStream);
}

#[test]
fn operation_and_reply_variants_round_trip_through_nota_text() {
    let request_text = round_trip_nota(Operation::Observe(ObservationSelection::AllAgents));
    assert_eq!(request_text, "(Observe AllAgents)");

    let reply_text = round_trip_nota(Reply::RequestUnimplemented(RequestUnimplemented {
        reason: UnimplementedReason::NotBuiltYet,
    }));
    assert_eq!(reply_text, "(RequestUnimplemented (NotBuiltYet))");

    let tap_text = round_trip_nota(Operation::Tap(signal_agent::ObserverFilter::EffectsOnly));
    assert_eq!(tap_text, "(Tap EffectsOnly)");
}
