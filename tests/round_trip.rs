//! Architectural-truth round-trip tests for the schema-derived `signal-agent`
//! contract. Each request, reply, and stream-event variant round-trips through
//! the `signal_frame::StreamingFrame` envelope (rkyv) and through NOTA text.

use nota_next::{NotaDecode, NotaEncode, NotaSource};
use signal_agent::{
    AgentEvent, Call, CallRejection, CallRejectionReason, CancelStream, ChatMessage, ChatRole,
    ChatTranscript, Completion, CompletionStreamDelta, CompletionText, DeltaSequence, Frame,
    FrameBody, Input, MaximumOutputTokens, ModelName, OperationKind, Output, OutputMode, Prompt,
    PromptOptions, ProviderName, ReasoningEffort, RejectionDetail, RequestUnimplemented,
    StopReasonText, StreamCall, StreamCancellation, StreamOpening, StreamToken, SystemText,
    TemperatureMilli, ThinkingMode, TokenDelta, TokenStreamDelta, TokenUsage, UnimplementedReason,
    UserText,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, SessionEpoch,
    StreamEventIdentifier, SubReply, SubscriptionTokenInner,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn stream_event() -> StreamEventIdentifier {
    StreamEventIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Acceptor,
        LaneSequence::first(),
    )
}

fn guardian_prompt() -> Prompt {
    Prompt::new(
        Some(SystemText::new("You judge intent.".to_owned())),
        ChatTranscript::new(vec![
            ChatMessage::user("Is this a durable decision?"),
            ChatMessage::assistant("Considering."),
        ]),
        PromptOptions::new(
            Some(ModelName::new("deepseek-v4-flash".to_owned())),
            Some(ProviderName::new("deepseek".to_owned())),
            Some(TemperatureMilli::new(200)),
            Some(MaximumOutputTokens::new(512)),
            OutputMode::Nota,
            Some(ReasoningEffort::High),
            Some(ThinkingMode::Enabled),
        ),
    )
}

fn usage() -> TokenUsage {
    TokenUsage::new(None, None)
}

fn round_trip_request(request: Input) -> Input {
    let expected = request.clone();
    let frame = request.into_frame(exchange());
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => {
            assert_eq!(request.payloads().head(), &expected);
            request.payloads().head().clone()
        }
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_reply(reply: Output) -> Output {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply operation, got {other:?}"),
    }
}

fn round_trip_event(event: AgentEvent) -> AgentEvent {
    let frame = event.into_subscription_frame(stream_event(), SubscriptionTokenInner::new(1));
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::SubscriptionEvent { event, .. } => event,
        other => panic!("expected subscription event, got {other:?}"),
    }
}

fn round_trip_nota<T>(value: T, expected: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_nota();
    assert_eq!(encoded, expected);
    let recovered = NotaSource::new(&encoded)
        .parse::<T>()
        .expect("decode nota text");
    assert_eq!(recovered, value);
}

#[test]
fn every_request_round_trips_through_streaming_frame() {
    let requests = [
        Input::Call(Call::new(guardian_prompt())),
        Input::StreamCall(StreamCall::new(guardian_prompt())),
        Input::CancelStream(CancelStream::new(StreamToken::new(7))),
    ];
    for request in requests {
        assert_eq!(round_trip_request(request.clone()), request);
    }
}

#[test]
fn every_reply_round_trips_through_streaming_frame() {
    let replies = [
        Output::Completed(Completion {
            completion_text: CompletionText::new("Yes, durable.".to_owned()),
            stop_reason: StopReasonText::new("stop".to_owned()),
            token_usage: usage(),
        }),
        Output::CallRejected(CallRejection {
            reason: CallRejectionReason::NoProviderConfigured,
            detail: RejectionDetail::new("no provider in registry".to_owned()),
        }),
        Output::StreamOpened(StreamOpening::new(StreamToken::new(7))),
        Output::StreamCancelled(StreamCancellation::new(StreamToken::new(7))),
        Output::RequestUnimplemented(RequestUnimplemented {
            operation: OperationKind::StreamCall,
            reason: UnimplementedReason::NotInPrototypeScope,
        }),
    ];
    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn stream_events_round_trip_through_subscription_frame() {
    let events = [
        AgentEvent::TokenStreamDelta(TokenStreamDelta {
            token: StreamToken::new(7),
            sequence: DeltaSequence::new(1),
            delta: TokenDelta::new("Yes".to_owned()),
        }),
        AgentEvent::CompletionStreamDelta(CompletionStreamDelta {
            token: StreamToken::new(7),
            stop_reason: StopReasonText::new("stop".to_owned()),
            token_usage: usage(),
        }),
    ];
    for event in events {
        assert_eq!(round_trip_event(event.clone()), event);
    }
}

#[test]
fn input_exposes_contract_owned_operation_kind() {
    assert_eq!(
        Input::Call(Call::new(guardian_prompt())).operation_kind(),
        OperationKind::Call
    );
    assert_eq!(
        Input::CancelStream(CancelStream::new(StreamToken::new(1))).operation_kind(),
        OperationKind::CancelStream
    );
}

#[test]
fn chat_role_and_output_mode_round_trip_through_nota_text() {
    round_trip_nota(ChatRole::Assistant, "Assistant");
    round_trip_nota(OutputMode::Nota, "Nota");
    round_trip_nota(
        ChatMessage {
            role: ChatRole::User,
            text: UserText::new("hello".to_owned()),
        },
        "(User hello)",
    );
}

#[test]
fn call_rejection_round_trips_through_nota_text() {
    round_trip_nota(
        Output::CallRejected(CallRejection {
            reason: CallRejectionReason::ProviderUnreachable,
            detail: RejectionDetail::new("connection refused".to_owned()),
        }),
        "(CallRejected (ProviderUnreachable [connection refused]))",
    );
}
