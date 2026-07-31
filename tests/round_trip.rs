//! Architectural-truth round-trip tests for the schema-derived `signal-agent`
//! contract. Each request, reply, and stream-event variant round-trips through
//! the `signal_frame::Frame` envelope (rkyv) and through DOTOS text.

use dotos::{DotosDecode, DotosEncode, DotosSource};
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
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, SessionEpoch, SubReply,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
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
            OutputMode::Dotos,
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
    let route = reply.wire_route();
    let frame = Frame::new(
        route,
        FrameBody::Reply {
            exchange: exchange(),
            reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
        },
    );
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
    match round_trip_reply(Output::Event(event)) {
        Output::Event(event) => event,
        other => panic!("expected event reply, got {other:?}"),
    }
}

fn round_trip_dotos<T>(value: T, expected: &str)
where
    T: DotosEncode + DotosDecode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_dotos();
    assert_eq!(encoded, expected);
    let recovered = DotosSource::new(&encoded)
        .parse::<T>()
        .expect("decode dotos text");
    assert_eq!(recovered, value);
}

#[test]
fn every_request_round_trips_through_length_prefixed_frame() {
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
fn every_reply_round_trips_through_length_prefixed_frame() {
    let replies = [
        Output::Completed(Completion {
            completion_text: CompletionText::new("Yes, durable.".to_owned()),
            stop_reason_text: StopReasonText::new("stop".to_owned()),
            token_usage: usage(),
        }),
        Output::CallRejected(CallRejection {
            call_rejection_reason: CallRejectionReason::NoProviderConfigured,
            rejection_detail: RejectionDetail::new("no provider in registry".to_owned()),
        }),
        Output::StreamOpened(StreamOpening::new(StreamToken::new(7))),
        Output::StreamCancelled(StreamCancellation::new(StreamToken::new(7))),
        Output::RequestUnimplemented(RequestUnimplemented {
            operation_kind: OperationKind::StreamCall,
            unimplemented_reason: UnimplementedReason::NotInPrototypeScope,
        }),
    ];
    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn stream_events_round_trip_through_routed_reply_frames() {
    let events = [
        AgentEvent::TokenStreamDelta(TokenStreamDelta {
            stream_token: StreamToken::new(7),
            delta_sequence: DeltaSequence::new(1),
            token_delta: TokenDelta::new("Yes".to_owned()),
        }),
        AgentEvent::CompletionStreamDelta(CompletionStreamDelta {
            stream_token: StreamToken::new(7),
            stop_reason_text: StopReasonText::new("stop".to_owned()),
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
fn chat_role_and_output_mode_round_trip_through_dotos_text() {
    round_trip_dotos(ChatRole::Assistant, "Assistant");
    round_trip_dotos(OutputMode::Dotos, "Dotos");
    round_trip_dotos(
        ChatMessage {
            chat_role: ChatRole::User,
            user_text: UserText::new("hello".to_owned()),
        },
        "{User hello}",
    );
}

#[test]
fn call_rejection_round_trips_through_dotos_text() {
    round_trip_dotos(
        Output::CallRejected(CallRejection {
            call_rejection_reason: CallRejectionReason::ProviderUnreachable,
            rejection_detail: RejectionDetail::new("connection refused".to_owned()),
        }),
        "CallRejected.{ProviderUnreachable (connection refused)}",
    );
}
