use dapi_grpc::platform::v0 as platform_proto;
use rs_dapi_client::{AddressList, DapiClient, DapiRequest, RequestSettings};

pub const OWNER_ID_BYTES: [u8; 32] = [
    65, 63, 57, 243, 204, 9, 106, 71, 187, 2, 94, 221, 190, 127, 141, 114, 137, 209, 243, 50, 60,
    215, 90, 101, 229, 15, 115, 5, 44, 117, 182, 217,
];

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut address_list = AddressList::new();
    address_list.add_uri(rs_dapi_client::Uri::from_static(
        "https://54.213.204.85:1443",
    ));

    let mut client = DapiClient::new(address_list, RequestSettings::default());
    let request = platform_proto::GetIdentityRequest {
        id: OWNER_ID_BYTES.to_vec(),
        prove: false,
    };
    dbg!(
        request
            .execute(&mut client, RequestSettings::default())
            .await
    );
}
