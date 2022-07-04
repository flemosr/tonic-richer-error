use std::collections::HashMap;

use prost::{DecodeError, EncodeError, Message};
use prost_types::Any;

use super::{pb, FromAny, IntoAny};

#[derive(Clone, Debug)]
pub struct ErrorInfo {
    pub reason: String,
    pub domain: String,
    pub metadata: HashMap<String, String>,
}

impl ErrorInfo {
    pub const TYPE_URL: &'static str = "type.googleapis.com/google.rpc.ErrorInfo";

    pub fn empty() -> Self {
        ErrorInfo {
            reason: String::from(""),
            domain: String::from(""),
            metadata: HashMap::new(),
        }
    }

    pub fn with_data(
        reason: impl Into<String>,
        domain: impl Into<String>,
        metadata: HashMap<impl Into<String>, impl Into<String>>,
    ) -> Self {
        let mut proc_metadata: HashMap<String, String> = HashMap::new();
        for (key, value) in metadata.into_iter() {
            proc_metadata.insert(key.into(), value.into());
        }

        ErrorInfo {
            reason: reason.into(),
            domain: domain.into(),
            metadata: proc_metadata,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.reason.is_empty() && self.domain.is_empty() && self.metadata.is_empty()
    }
}

impl IntoAny for ErrorInfo {
    fn into_any(self) -> Result<Any, EncodeError> {
        let detail_data = pb::ErrorInfo {
            reason: self.reason,
            domain: self.domain,
            metadata: self.metadata,
        };

        let mut buf: Vec<u8> = Vec::new();
        buf.reserve(detail_data.encoded_len());
        detail_data.encode(&mut buf)?;

        Ok(Any {
            type_url: ErrorInfo::TYPE_URL.to_string(),
            value: buf,
        })
    }
}

impl FromAny for ErrorInfo {
    fn from_any(any: Any) -> Result<Self, DecodeError> {
        let buf: &[u8] = &any.value;
        let debug_info = pb::ErrorInfo::decode(buf)?;

        let debug_info = ErrorInfo {
            reason: debug_info.reason,
            domain: debug_info.domain,
            metadata: debug_info.metadata,
        };

        Ok(debug_info)
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::{FromAny, IntoAny};

    use super::ErrorInfo;

    #[test]
    fn gen_error_info() {
        let error_info = ErrorInfo::empty();
        let formatted = format!("{:?}", error_info);

        println!("empty ErrorInfo -> {formatted}");

        let expected = "ErrorInfo { reason: \"\", domain: \"\", metadata: {} }";

        assert!(
            formatted.eq(expected),
            "empty ErrorInfo differs from expected result"
        );

        let mut metadata = HashMap::new();
        metadata.insert("instanceLimitPerRequest", "100");

        let error_info = ErrorInfo::with_data("SOME_INFO", "mydomain.com", metadata);

        let formatted = format!("{:?}", error_info);

        println!("filled ErrorInfo -> {formatted}");

        let expected_filled = "ErrorInfo { reason: \"SOME_INFO\", domain: \"mydomain.com\", metadata: {\"instanceLimitPerRequest\": \"100\"} }";

        assert!(
            formatted.eq(expected_filled),
            "filled ErrorInfo differs from expected result"
        );

        let gen_any = match error_info.into_any() {
            Err(error) => panic!("Error generating Any from ErrorInfo: {:?}", error),
            Ok(gen_any) => gen_any,
        };
        let formatted = format!("{:?}", gen_any);

        println!("Any generated from ErrorInfo -> {formatted}");

        let expected =
            "Any { type_url: \"type.googleapis.com/google.rpc.ErrorInfo\", value: [10, 9, 83, 79, 77, 69, 95, 73, 78, 70, 79, 18, 12, 109, 121, 100, 111, 109, 97, 105, 110, 46, 99, 111, 109, 26, 30, 10, 23, 105, 110, 115, 116, 97, 110, 99, 101, 76, 105, 109, 105, 116, 80, 101, 114, 82, 101, 113, 117, 101, 115, 116, 18, 3, 49, 48, 48] }";

        assert!(
            formatted.eq(expected),
            "Any from filled ErrorInfo differs from expected result"
        );

        let br_details = match ErrorInfo::from_any(gen_any) {
            Err(error) => panic!("Error generating ErrorInfo from Any: {:?}", error),
            Ok(from_any) => from_any,
        };

        let formatted = format!("{:?}", br_details);

        println!("ErrorInfo generated from Any -> {formatted}");

        assert!(
            formatted.eq(expected_filled),
            "ErrorInfo from Any differs from expected result"
        );
    }
}
