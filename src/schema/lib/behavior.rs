// Handwritten operational behavior for the authority-verified ordinary Mirror Interface.
//
// The strict bootstrap projection owns every structural type below. This file
// supplies only current-stage behavior: structural traits over the ordinary
// producer's shared representation, readable Dotos roles, and the allocated
// Signal frame boundary.

use rkyv::{
    Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
    rancor::Source as _,
};
use signal_standard::schema::lib::{WireShape, WireShapeError, WireValue};

fn one_field(mut fields: Vec<WireValue>) -> Result<WireValue, WireShapeError> {
    if fields.len() != 1 {
        return Err(WireShapeError);
    }
    Ok(fields.pop().expect("one field checked"))
}

macro_rules! wire_traits {
    ($name:ident) => {
        impl Clone for $name { fn clone(&self) -> Self { Self::from_wire(self.to_wire()).expect("a projected value revalidates") } }
        impl std::fmt::Debug for $name { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_wire().fmt(formatter) } }
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.to_wire() == other.to_wire() } }
        impl Eq for $name {}
    };
}
macro_rules! wire_external_newtype {
    ($name:ident, $inner:ty) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.payload().to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self::new(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(self.payload())
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self::new)
            }
        }
    };
}
macro_rules! wire_newtype {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub fn new(payload: $inner) -> Self {
                Self(payload)
            }
            pub fn payload(&self) -> &$inner {
                &self.0
            }
            pub fn into_payload(self) -> $inner {
                self.0
            }
        }
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                self.0.to_wire()
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                Ok(Self(<$inner as WireShape>::from_wire(value)?))
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(&self.0)
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self)
            }
        }
    };
}
macro_rules! wire_struct {
    ($name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { WireValue::Product(vec![$(self.$field.to_wire()),*]) }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Product(fields) = value else { return Err(WireShapeError) };
                let mut fields = fields.into_iter();
                let result = Self { $($field: <$field_type as WireShape>::from_wire(fields.next().ok_or(WireShapeError)?)?),* };
                if fields.next().is_some() { return Err(WireShapeError); }
                Ok(result)
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::Delimiter::Parenthesis.wrap([
                    $(dotos::DotosEncode::to_dotos(&self.$field)),*
                ])
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                let body = dotos::DotosBody::from_delimited(
                    block,
                    dotos::Delimiter::Parenthesis,
                    stringify!($name),
                )?;
                let expected = 0usize $(+ {
                    let _ = stringify!($field);
                    1usize
                })*;
                #[allow(unused_mut, unused_variables)]
                let mut fields = body.expect_fields(stringify!($name), expected)?.iter();
                Ok(Self {
                    $($field: <$field_type as dotos::DotosDecode>::from_dotos_block(
                        fields.next().expect("field count checked"),
                    )?),*
                })
            }
        }
    };
}
macro_rules! wire_enum {
    ($name:ident {
        unit { $($unit_ordinal:literal => $unit:ident : $unit_visible:literal),* $(,)? }
        unary { $($unary_ordinal:literal => $unary:ident($payload:ty) : $unary_visible:literal),* $(,)? }
    }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                match self {
                    $(Self::$unit => WireValue::Variant { ordinal: $unit_ordinal, fields: Vec::new() },)*
                    $(Self::$unary(payload) => WireValue::Variant { ordinal: $unary_ordinal, fields: vec![payload.to_wire()] },)*
                }
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Variant { ordinal, fields } = value else { return Err(WireShapeError) };
                match ordinal {
                    $($unit_ordinal if fields.is_empty() => Ok(Self::$unit),)*
                    $($unary_ordinal => Ok(Self::$unary(<$payload as WireShape>::from_wire(one_field(fields)?)?)),)*
                    _ => Err(WireShapeError),
                }
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                match self {
                    $(Self::$unit => $unit_visible.to_owned(),)*
                    $(Self::$unary(payload) => format!(
                        "{}.{}",
                        $unary_visible,
                        dotos::DotosEncode::to_dotos(payload),
                    ),)*
                }
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                if let Some(variant) = block.demote_to_string() {
                    return match variant {
                        $($unit_visible => Ok(Self::$unit),)*
                        _ => Err(dotos::DotosDecodeError::UnknownVariant {
                            enum_name: stringify!($name),
                            variant: variant.to_owned(),
                        }),
                    };
                }
                let (head, payload) = block.as_application().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                let _ = &payload;
                let variant = head.demote_to_string().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                match variant {
                    $($unary_visible => Ok(Self::$unary(
                        <$payload as dotos::DotosDecode>::from_dotos_block(payload)?,
                    )),)*
                    _ => Err(dotos::DotosDecodeError::UnknownVariant {
                        enum_name: stringify!($name),
                        variant: variant.to_owned(),
                    }),
                }
            }
        }
    };
}
wire_enum!(z2VNAv { unit {  } unary { 0 => z2VRrf(z2VVej) : "Call", 1 => z2VPoN(z2VLFK) : "CancelStream", 2 => z2VTYr(z2VaUr) : "StreamCall" } });
wire_enum!(z2Ve6M { unit {  } unary { 0 => z2VR63(z2VQ3h) : "CallRejected", 1 => z2VTjq(z2VNBf) : "StreamCancelled", 2 => z2VRXc(z2VXAa) : "Completed", 3 => z2Vc1N(z2VQBL) : "RequestUnimplemented", 4 => z2VRTb(z2VT5d) : "StreamOpened", 5 => z2VaCt(z2VR3K) : "Event" } });
wire_external_newtype!(z2VahV, std::string::String);
wire_external_newtype!(z2VV18, std::string::String);
wire_external_newtype!(z2VQbv, std::string::String);
wire_external_newtype!(z2VUMb, std::string::String);
wire_external_newtype!(z2VRqc, std::string::String);
wire_external_newtype!(z2VYGd, u64);
wire_external_newtype!(z2VTct, std::string::String);
wire_external_newtype!(z2VaBD, u64);
wire_external_newtype!(z2VRQS, u64);
wire_external_newtype!(z2VYSf, u64);
wire_external_newtype!(z2VTYs, std::string::String);
wire_external_newtype!(z2VbCg, std::string::String);
wire_external_newtype!(z2VecB, u64);
wire_external_newtype!(z2VTNt, u64);
wire_external_newtype!(z2VRg8, std::string::String);
wire_enum!(z2VRdN { unit { 0 => z2Va7M : "User", 1 => z2VfEc : "Assistant", 2 => z2VNn5 : "System" } unary {  } });
wire_struct!(z2Vav4 { field_0: z2VRdN, field_1: z2VV18 });
wire_external_newtype!(z2VLbX, Vec<z2Vav4>);
wire_enum!(z2VSHd { unit { 0 => z2VPna : "Dotos", 1 => z2Va7b : "FreeText" } unary {  } });
wire_enum!(z2VKsS { unit { 0 => z2VVD9 : "Medium", 1 => z2VVUc : "High", 2 => z2VMyX : "Low" } unary {  } });
wire_enum!(z2VXxS { unit { 0 => z2VbC1 : "Disabled", 1 => z2VeVv : "Enabled" } unary {  } });
wire_struct!(z2VLFu { field_0: Option<z2VUMb>, field_1: Option<z2VRqc>, field_2: Option<z2VRQS>, field_3: Option<z2VYSf>, field_4: z2VSHd, field_5: Option<z2VKsS>, field_6: Option<z2VXxS> });
wire_struct!(z2VMMe { field_0: Option<z2VahV>, field_1: z2VLbX, field_2: z2VLFu });
wire_struct!(z2Vb6p { field_0: Option<z2VecB>, field_1: Option<z2VTNt> });
wire_newtype!(z2VVej, z2VMMe);
wire_newtype!(z2VaUr, z2VMMe);
wire_newtype!(z2VLFK, z2VYGd);
wire_struct!(z2VXAa { field_0: z2VTYs, field_1: z2VbCg, field_2: z2Vb6p });
wire_enum!(z2VUN4 { unit { 0 => z2VZ8M : "DaemonUnconfigured", 1 => z2VQdg : "InvalidDotosOutput", 2 => z2VdLV : "ProviderUnreachable", 3 => z2VYdH : "ProviderRejected", 4 => z2VTxt : "OutputModeUnsupported", 5 => z2VS9d : "NoProviderConfigured" } unary {  } });
wire_struct!(z2VQ3h { field_0: z2VUN4, field_1: z2VRg8 });
wire_newtype!(z2VT5d, z2VYGd);
wire_newtype!(z2VNBf, z2VYGd);
wire_enum!(z2VZf9 { unit { 0 => z2Vdwh : "StreamCall", 1 => z2VKzq : "Call", 2 => z2VRkZ : "CancelStream" } unary {  } });
wire_enum!(z2VSnT { unit { 0 => z2VVPv : "NotInPrototypeScope" } unary {  } });
wire_struct!(z2VQBL { field_0: z2VZf9, field_1: z2VSnT });
wire_struct!(z2VUeX { field_0: z2VYGd, field_1: z2VaBD, field_2: z2VTct });
wire_struct!(z2VUBQ { field_0: z2VYGd, field_1: z2VbCg, field_2: z2Vb6p });
wire_enum!(z2VR3K { unit {  } unary { 0 => z2VQhq(z2VUeX) : "TokenStreamDelta", 1 => z2VQU1(z2VUBQ) : "CompletionStreamDelta" } });
wire_struct!(z2VL3r { field_0: z2VYGd, field_1: z2VT5d, field_2: z2VR3K, field_3: z2VYGd });

macro_rules! archive_root {
    ($root:ident) => {
        impl Archive for $root {
            type Archived = <WireValue as Archive>::Archived;
            type Resolver = <WireValue as Archive>::Resolver;
            fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
                self.to_wire().resolve(resolver, out);
            }
        }
        impl<Serializer> RkyvSerialize<Serializer> for $root
        where
            Serializer: rkyv::rancor::Fallible + ?Sized,
            WireValue: RkyvSerialize<Serializer>,
        {
            fn serialize(
                &self,
                serializer: &mut Serializer,
            ) -> Result<Self::Resolver, Serializer::Error> {
                self.to_wire().serialize(serializer)
            }
        }
        impl<Deserializer> RkyvDeserialize<$root, Deserializer>
            for signal_standard::schema::lib::ArchivedWireValue
        where
            Deserializer: rkyv::rancor::Fallible + ?Sized,
            Deserializer::Error: rkyv::rancor::Source,
            signal_standard::schema::lib::ArchivedWireValue:
                RkyvDeserialize<WireValue, Deserializer>,
        {
            fn deserialize(
                &self,
                deserializer: &mut Deserializer,
            ) -> Result<$root, Deserializer::Error> {
                let wire = <signal_standard::schema::lib::ArchivedWireValue as RkyvDeserialize<
                    WireValue,
                    Deserializer,
                >>::deserialize(self, deserializer)?;
                <$root as WireShape>::from_wire(wire).map_err(Deserializer::Error::new)
            }
        }
    };
}
archive_root!(z2VNAv);
archive_root!(z2Ve6M);
archive_root!(z2VahV);
archive_root!(z2VV18);
archive_root!(z2VQbv);
archive_root!(z2VUMb);
archive_root!(z2VRqc);
archive_root!(z2VYGd);
archive_root!(z2VTct);
archive_root!(z2VaBD);
archive_root!(z2VRQS);
archive_root!(z2VYSf);
archive_root!(z2VTYs);
archive_root!(z2VbCg);
archive_root!(z2VecB);
archive_root!(z2VTNt);
archive_root!(z2VRg8);
archive_root!(z2VRdN);
archive_root!(z2Vav4);
archive_root!(z2VLbX);
archive_root!(z2VSHd);
archive_root!(z2VKsS);
archive_root!(z2VXxS);
archive_root!(z2VLFu);
archive_root!(z2VMMe);
archive_root!(z2Vb6p);
archive_root!(z2VVej);
archive_root!(z2VaUr);
archive_root!(z2VLFK);
archive_root!(z2VXAa);
archive_root!(z2VUN4);
archive_root!(z2VQ3h);
archive_root!(z2VT5d);
archive_root!(z2VNBf);
archive_root!(z2VZf9);
archive_root!(z2VSnT);
archive_root!(z2VQBL);
archive_root!(z2VUeX);
archive_root!(z2VUBQ);
archive_root!(z2VR3K);
archive_root!(z2VL3r);


pub enum ContractMarker {}
impl signal_frame::WireContract for ContractMarker {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        match signal_frame::ContractId::try_new(15) { Ok(value) => value, Err(_) => panic!("contract ID is allocated") },
        match signal_frame::WireRevision::try_new(2) { Ok(value) => value, Err(_) => panic!("wire revision is allocated") },
    );
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SignalFrameError {
    #[error("failed to encode bound signal frame")] FrameEncode,
    #[error("failed to decode bound signal frame")] ArchiveDecode,
    #[error("unexpected signal frame body")] UnexpectedFrameBody,
    #[error("expected one request operation, found {found}")] OperationCount { found: usize },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)] #[repr(u8)] pub enum InputRoute { Call, CancelStream, StreamCall }
#[derive(Clone, Copy, Debug, PartialEq, Eq)] #[repr(u8)] pub enum OutputRoute { CallRejected, StreamCancelled, Completed, RequestUnimplemented, StreamOpened, Event }
impl z2VNAv {
 pub fn route(&self)->InputRoute{match self{Self::z2VRrf(_)=>InputRoute::Call,Self::z2VPoN(_)=>InputRoute::CancelStream,Self::z2VTYr(_)=>InputRoute::StreamCall,}}
 pub fn wire_route(&self)->signal_frame::WireRoute{signal_frame::WireRoute::new(signal_frame::RootCode::new(0),signal_frame::VariantCode::new(self.route() as u8))}
 pub fn into_frame(self,exchange:signal_frame::ExchangeIdentifier)->Frame{let route=self.wire_route();Frame::new(route,FrameBody::Request{exchange,request:signal_frame::Request::from_payload(self)})}
 pub fn encode_request_frame(self,exchange:signal_frame::ExchangeIdentifier)->Result<Vec<u8>,SignalFrameError>{self.into_frame(exchange).encode().map_err(|_|SignalFrameError::FrameEncode)}
}
impl z2Ve6M {
 pub fn route(&self)->OutputRoute{match self{Self::z2VR63(_)=>OutputRoute::CallRejected,Self::z2VTjq(_)=>OutputRoute::StreamCancelled,Self::z2VRXc(_)=>OutputRoute::Completed,Self::z2Vc1N(_)=>OutputRoute::RequestUnimplemented,Self::z2VRTb(_)=>OutputRoute::StreamOpened,Self::z2VaCt(_)=>OutputRoute::Event,}}
 pub fn wire_route(&self)->signal_frame::WireRoute{signal_frame::WireRoute::new(signal_frame::RootCode::new(1),signal_frame::VariantCode::new(self.route() as u8))}
 pub fn into_reply_frame(self,exchange:signal_frame::ExchangeIdentifier)->Frame{let route=self.wire_route();let reply=signal_frame::Reply::committed(signal_frame::NonEmpty::single(signal_frame::SubReply::Ok(self)));Frame::new(route,FrameBody::Reply{exchange,reply})}
 pub fn encode_reply_frame(self,exchange:signal_frame::ExchangeIdentifier)->Result<Vec<u8>,SignalFrameError>{self.into_reply_frame(exchange).encode().map_err(|_|SignalFrameError::FrameEncode)}
}
impl signal_frame::RequestPayload for z2VNAv {}
impl signal_frame::SignalOperationHeads for z2VNAv{const HEADS:&'static[&'static str]=&["Call","CancelStream","StreamCall"];}
impl signal_frame::LogVariant for z2VNAv{fn log_variant(&self)->u64{let route=self.wire_route();u64::from(route.root().value())|(u64::from(route.variant().value())<<8)}}
pub type Frame=signal_frame::BoundExchangeFrame<ContractMarker,z2VNAv,z2Ve6M>;
pub type FrameBody=signal_frame::ExchangeFrameBody<z2VNAv,z2Ve6M>;
pub type Request=signal_frame::Request<z2VNAv>;
pub type ReplyEnvelope=signal_frame::Reply<z2Ve6M>;
pub type RequestBuilder=signal_frame::RequestBuilder<z2VNAv>;
impl ContractMarker{
 pub fn decode_frame(bytes:&[u8])->Result<Frame,SignalFrameError>{Frame::decode(bytes).map_err(|_|SignalFrameError::ArchiveDecode)}
 pub fn decode_single_request(bytes:&[u8])->Result<(signal_frame::ExchangeIdentifier,z2VNAv),SignalFrameError>{match Self::decode_frame(bytes)?.into_body(){FrameBody::Request{exchange,request}=>{let found=request.payloads().len();if found!=1{return Err(SignalFrameError::OperationCount{found});}Ok((exchange,request.payloads.into_head()))},_=>Err(SignalFrameError::UnexpectedFrameBody)}}
}

