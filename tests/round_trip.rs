use dotos::{DotosEncode, DotosSource};
use signal_agent::*;
fn exchange() -> signal_frame::ExchangeIdentifier {
    signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(1),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::first(),
    )
}
#[test]
fn authority_projected_request_round_trips_through_dotos_and_bound_frame() {
    let request = z2VNAv::z2VRrf(z2VVej::new(z2VMMe {
        field_0: Some(z2VahV::new("fixture".to_owned())),
        field_1: z2VLbX::new(vec![z2Vav4 {
            field_0: z2VRdN::z2Va7M,
            field_1: z2VV18::new("fixture".to_owned()),
        }]),
        field_2: z2VLFu {
            field_0: Some(z2VUMb::new("fixture".to_owned())),
            field_1: Some(z2VRqc::new("fixture".to_owned())),
            field_2: Some(z2VRQS::new(7)),
            field_3: Some(z2VYSf::new(7)),
            field_4: z2VSHd::z2VPna,
            field_5: Some(z2VKsS::z2VVD9),
            field_6: Some(z2VXxS::z2VbC1),
        },
    }));
    let text = request.to_dotos();
    assert_eq!(
        DotosSource::new(&text)
            .parse::<z2VNAv>()
            .expect("Dotos decodes"),
        request
    );
    let bytes = request
        .clone()
        .encode_request_frame(exchange())
        .expect("frame encodes");
    let (found_exchange, decoded) =
        ContractMarker::decode_single_request(&bytes).expect("frame decodes");
    assert_eq!(found_exchange, exchange());
    assert_eq!(decoded, request);
}
#[test]
fn authority_projected_reply_round_trips_through_dotos_and_archive() {
    let reply = z2Ve6M::z2VR63(z2VQ3h {
        field_0: z2VUN4::z2VZ8M,
        field_1: z2VRg8::new("fixture".to_owned()),
    });
    let text = reply.to_dotos();
    assert_eq!(
        DotosSource::new(&text)
            .parse::<z2Ve6M>()
            .expect("Dotos decodes"),
        reply
    );
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).expect("archives");
    let recovered = rkyv::from_bytes::<z2Ve6M, rkyv::rancor::Error>(&bytes).expect("recovers");
    assert_eq!(recovered, reply);
}
