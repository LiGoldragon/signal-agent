//! Producer-owned authority state for the ordinary Agent Interface.
//!
//! Every identity and canonical-order value is an allocated opaque seat.
//! None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}
impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}
impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    226, 0, 230, 1, 17, 150, 84, 54, 4, 230, 148, 101, 174, 9, 24, 205, 119, 30, 193, 102, 93, 159,
    144, 88, 177, 55, 95, 32, 184, 71, 136, 199,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 34984;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 50029;

pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 39690, 0x26eeef2c3a0260eb);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 33966, 0x5551c6f0faa87215);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 14804, 0x969e2e9d4bdd734f);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 47747, 0x1d95c5a4f055c319);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 48081, 0xda602b174b97e3f3);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 27551, 0x56a13ddd3ec11c5d);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 60044, 0x02fbb9ad579b16d7);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 27938, 0xaa47b7d0a86a81e1);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 52355, 0x75c0832d351eaffb);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 30768, 0xee690ec6e2ca37a5);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 34199, 0x61db6fef508c935f);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 55172, 0xa41bbd888554c1a9);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 22205, 0xd67b6578fc24e503);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 50751, 0x86ebbcfae0bee3ed);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 12537, 0x2d65b5c850e208e7);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 5120, 0x04b59a85c281a271);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    36748, 21916, 57913, 26782, 36203, 20591, 36544, 10658, 27122, 26204,
];

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "AgentRequest", 8099, 0x0955947f94eb6f7d),
    DeclarationSeat::new(None, "AgentReply", 61658, 0x0fb22ddfed2d5ef7),
    DeclarationSeat::new(None, "SystemText", 50240, 0xe8f96c2d0f2bc701),
    DeclarationSeat::new(None, "UserText", 31079, 0x269d0232139d3a1b),
    DeclarationSeat::new(None, "AssistantText", 16277, 0x117c240a2f2d8ec5),
    DeclarationSeat::new(None, "ModelName", 28902, 0xfef475e3c3b77f7f),
    DeclarationSeat::new(None, "ProviderName", 20435, 0xce2446ff5ed74ac9),
    DeclarationSeat::new(None, "StreamToken", 42070, 0x26489c55e37e5323),
    DeclarationSeat::new(None, "TokenDelta", 26425, 0xb6ed3903846fbf0d),
    DeclarationSeat::new(None, "DeltaSequence", 48484, 0x19ee3b2957ce1907),
    DeclarationSeat::new(None, "TemperatureMilli", 18975, 0xf8288fafb821ef91),
    DeclarationSeat::new(None, "MaximumOutputTokens", 42652, 0xcd7423156d81752b),
    DeclarationSeat::new(None, "CompletionText", 26192, 0x5f4ab18e49c12055),
    DeclarationSeat::new(None, "StopReasonText", 51933, 0xeaba92358cd54b8f),
    DeclarationSeat::new(None, "PromptTokenCount", 63388, 0x9a0469e99accd559),
    DeclarationSeat::new(None, "CompletionTokenCount", 25613, 0x3f0b50de5b0cc033),
    DeclarationSeat::new(None, "RejectionDetail", 19885, 0x66a89d1f90b4d29d),
    DeclarationSeat::new(None, "ChatRole", 19725, 0x3f9e4dc4ae553717),
    DeclarationSeat::new(None, "ChatMessage", 50969, 0xe562db42a35d1c21),
    DeclarationSeat::new(None, "ChatTranscript", 2798, 0xede9891ef7ea543b),
    DeclarationSeat::new(None, "OutputMode", 21944, 0x6ab66459cbe1f5e5),
    DeclarationSeat::new(None, "ReasoningEffort", 357, 0x5ac84aac0879fb9f),
    DeclarationSeat::new(None, "ThinkingMode", 41015, 0x3984ff1cd79be3e9),
    DeclarationSeat::new(None, "PromptOptions", 1660, 0x2ddf3a7052c85143),
    DeclarationSeat::new(None, "Prompt", 5357, 0x7ca440dae363aa2d),
    DeclarationSeat::new(None, "TokenUsage", 51593, 0x2153b1c51093b927),
    DeclarationSeat::new(None, "CallPayload", 33260, 0xd1600f8d03164cb1),
    DeclarationSeat::new(None, "StreamCallPayload", 49507, 0x6115d042edb8d74b),
    DeclarationSeat::new(None, "CancelStreamPayload", 1626, 0x5892fe2f59590f75),
    DeclarationSeat::new(None, "Completion", 38355, 0xe958c8641d968faf),
    DeclarationSeat::new(None, "CallRejectionReason", 28929, 0x57d4eb97239d7679),
    DeclarationSeat::new(None, "CallRejection", 14408, 0x457cd0affeb20653),
    DeclarationSeat::new(None, "StreamOpening", 24612, 0xe52e0d5c7e6545bd),
    DeclarationSeat::new(None, "StreamCancellation", 8142, 0x69c4f21cb09a9f37),
    DeclarationSeat::new(None, "OperationKind", 46740, 0xce685752e5c68141),
    DeclarationSeat::new(None, "UnimplementedReason", 23616, 0x37b2f151400dfe5b),
    DeclarationSeat::new(
        None,
        "RequestUnimplementedPayload",
        14851,
        0x0a434cb3b62f6d05,
    ),
    DeclarationSeat::new(None, "TokenStreamDeltaPayload", 29884, 0x67d118d54d5c07bf),
    DeclarationSeat::new(
        None,
        "CompletionStreamDeltaPayload",
        28311,
        0xaa4b8eca316a8d09,
    ),
    DeclarationSeat::new(None, "AgentEvent", 17750, 0x3cf12441510adf63),
    DeclarationSeat::new(None, "CompletionStream", 961, 0xe41185e0cbe2a54d),
    DeclarationSeat::new(Some(8099), "Call", 20496, 0x65f7ca6ce2bae947),
    DeclarationSeat::new(Some(8099), "StreamCall", 26191, 0xba33e8a3c626b9d1),
    DeclarationSeat::new(Some(8099), "CancelStream", 13577, 0x7e24dca6a64ac96b),
    DeclarationSeat::new(Some(61658), "Completed", 19391, 0x940f8c6556ae0e95),
    DeclarationSeat::new(Some(61658), "CallRejected", 17908, 0x541afd73c33b63cf),
    DeclarationSeat::new(Some(61658), "StreamOpened", 19158, 0xdbf731ebe7dc2799),
    DeclarationSeat::new(Some(61658), "StreamCancelled", 26828, 0x740b106b0a53dc73),
    DeclarationSeat::new(
        Some(61658),
        "RequestUnimplemented",
        54641,
        0xa3ab16f32e44c8dd,
    ),
    DeclarationSeat::new(Some(61658), "Event", 48581, 0xe51598ba2d859757),
    DeclarationSeat::new(Some(19725), "System", 10138, 0xdec2208d1b2ff661),
    DeclarationSeat::new(Some(19725), "User", 48260, 0x4947cb01ae10387b),
    DeclarationSeat::new(Some(19725), "Assistant", 65501, 0x6da0e657ef5df425),
    DeclarationSeat::new(Some(21944), "FreeText", 48274, 0x84578bb664e5a3df),
    DeclarationSeat::new(Some(21944), "Dotos", 13531, 0x12c7b64af20b4629),
    DeclarationSeat::new(Some(357), "Low", 7438, 0xd4cf7b5dc94dfd83),
    DeclarationSeat::new(Some(357), "Medium", 31776, 0x45e88ceb9034b06d),
    DeclarationSeat::new(Some(357), "High", 32673, 0xcf9d800459cba967),
    DeclarationSeat::new(Some(41015), "Enabled", 63025, 0x7b5cb250e81b36f1),
    DeclarationSeat::new(Some(41015), "Disabled", 51894, 0x4748857bcb3f4b8b),
    DeclarationSeat::new(
        Some(28929),
        "NoProviderConfigured",
        21480,
        0xbcae31b405081db5,
    ),
    DeclarationSeat::new(
        Some(28929),
        "ProviderUnreachable",
        59114,
        0x92afee80e24bc7ef,
    ),
    DeclarationSeat::new(Some(28929), "ProviderRejected", 43268, 0xb7923ba74f50e8b9),
    DeclarationSeat::new(
        Some(28929),
        "OutputModeUnsupported",
        27585,
        0xba3e061f1afa4293,
    ),
    DeclarationSeat::new(Some(28929), "InvalidDotosOutput", 16379, 0x278cfaaff49b5bfd),
    DeclarationSeat::new(Some(28929), "DaemonUnconfigured", 44954, 0x0ad1acc2829e1f77),
    DeclarationSeat::new(Some(46740), "Call", 786, 0x77afc3204c617b81),
    DeclarationSeat::new(Some(46740), "StreamCall", 61156, 0x631925a667f9029b),
    DeclarationSeat::new(Some(46740), "CancelStream", 20142, 0x8d5e61fa7cb58b45),
    DeclarationSeat::new(
        Some(23616),
        "NotInPrototypeScope",
        32401,
        0xaebcd9f1c59ecfff,
    ),
    DeclarationSeat::new(Some(17750), "TokenStreamDelta", 16620, 0x94a6b78ee3460f49),
    DeclarationSeat::new(
        Some(17750),
        "CompletionStreamDelta",
        15818,
        0xe705cd188a99aba3,
    ),
];
