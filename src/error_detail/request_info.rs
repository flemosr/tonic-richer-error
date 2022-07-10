use prost::{DecodeError, EncodeError, Message};
use prost_types::Any;

use super::super::pb;
use super::super::{FromAny, IntoAny};

#[derive(Clone, Debug)]
pub struct RequestInfo {
    pub request_id: String,
    pub serving_data: String,
}

impl RequestInfo {
    pub const TYPE_URL: &'static str = "type.googleapis.com/google.rpc.RequestInfo";

    pub fn new(request_id: impl Into<String>, serving_data: impl Into<String>) -> Self {
        RequestInfo {
            request_id: request_id.into(),
            serving_data: serving_data.into(),
        }
    }
}

impl RequestInfo {
    pub fn is_empty(&self) -> bool {
        self.request_id.is_empty() && self.serving_data.is_empty()
    }
}

impl IntoAny for RequestInfo {
    fn into_any(self) -> Result<Any, EncodeError> {
        let detail_data = pb::RequestInfo {
            request_id: self.request_id,
            serving_data: self.serving_data,
        };

        let mut buf: Vec<u8> = Vec::new();
        buf.reserve(detail_data.encoded_len());
        detail_data.encode(&mut buf)?;

        Ok(Any {
            type_url: RequestInfo::TYPE_URL.to_string(),
            value: buf,
        })
    }
}

impl FromAny for RequestInfo {
    fn from_any(any: Any) -> Result<Self, DecodeError> {
        let buf: &[u8] = &any.value;
        let req_info = pb::RequestInfo::decode(buf)?;

        let debug_info = RequestInfo {
            request_id: req_info.request_id,
            serving_data: req_info.serving_data,
        };

        Ok(debug_info)
    }
}

#[cfg(test)]
mod tests {

    use super::super::super::{FromAny, IntoAny};
    use super::RequestInfo;

    #[test]
    fn gen_error_info() {
        let error_info = RequestInfo::new("some-id", "some-data");

        let formatted = format!("{:?}", error_info);

        println!("filled RequestInfo -> {formatted}");

        let expected_filled =
            "RequestInfo { request_id: \"some-id\", serving_data: \"some-data\" }";

        assert!(
            formatted.eq(expected_filled),
            "filled RequestInfo differs from expected result"
        );

        let gen_any = match error_info.into_any() {
            Err(error) => panic!("Error generating Any from RequestInfo: {:?}", error),
            Ok(gen_any) => gen_any,
        };
        let formatted = format!("{:?}", gen_any);

        println!("Any generated from RequestInfo -> {formatted}");

        let expected =
            "Any { type_url: \"type.googleapis.com/google.rpc.RequestInfo\", value: [10, 7, 115, 111, 109, 101, 45, 105, 100, 18, 9, 115, 111, 109, 101, 45, 100, 97, 116, 97] }";

        assert!(
            formatted.eq(expected),
            "Any from filled RequestInfo differs from expected result"
        );

        let br_details = match RequestInfo::from_any(gen_any) {
            Err(error) => panic!("Error generating RequestInfo from Any: {:?}", error),
            Ok(from_any) => from_any,
        };

        let formatted = format!("{:?}", br_details);

        println!("RequestInfo generated from Any -> {formatted}");

        assert!(
            formatted.eq(expected_filled),
            "RequestInfo from Any differs from expected result"
        );
    }
}
