use prost::{DecodeError, EncodeError, Message};
use prost_types::Any;

use super::super::pb;
use super::super::{FromAny, IntoAny};

#[derive(Clone, Debug)]
pub struct LocalizedMessage {
    pub locale: String,
    pub message: String,
}

impl LocalizedMessage {
    pub const TYPE_URL: &'static str = "type.googleapis.com/google.rpc.LocalizedMessage";

    pub fn empty() -> Self {
        LocalizedMessage {
            locale: String::from(""),
            message: String::from(""),
        }
    }

    pub fn with_data(locale: impl Into<String>, message: impl Into<String>) -> Self {
        LocalizedMessage {
            locale: locale.into(),
            message: message.into(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.locale.is_empty() && self.message.is_empty()
    }
}

impl IntoAny for LocalizedMessage {
    fn into_any(self) -> Result<Any, EncodeError> {
        let detail_data = pb::LocalizedMessage {
            locale: self.locale,
            message: self.message,
        };

        let mut buf: Vec<u8> = Vec::new();
        buf.reserve(detail_data.encoded_len());
        detail_data.encode(&mut buf)?;

        Ok(Any {
            type_url: LocalizedMessage::TYPE_URL.to_string(),
            value: buf,
        })
    }
}

impl FromAny for LocalizedMessage {
    fn from_any(any: Any) -> Result<Self, DecodeError> {
        let buf: &[u8] = &any.value;
        let req_info = pb::LocalizedMessage::decode(buf)?;

        let debug_info = LocalizedMessage {
            locale: req_info.locale,
            message: req_info.message,
        };

        Ok(debug_info)
    }
}

#[cfg(test)]
mod tests {

    use super::super::super::{FromAny, IntoAny};
    use super::LocalizedMessage;

    #[test]
    fn gen_error_info() {
        let error_info = LocalizedMessage::empty();
        let formatted = format!("{:?}", error_info);

        println!("empty LocalizedMessage -> {formatted}");

        let expected = "LocalizedMessage { locale: \"\", message: \"\" }";

        assert!(
            formatted.eq(expected),
            "empty LocalizedMessage differs from expected result"
        );

        let error_info = LocalizedMessage::with_data("en-US", "message for the user");

        let formatted = format!("{:?}", error_info);

        println!("filled LocalizedMessage -> {formatted}");

        let expected_filled =
            "LocalizedMessage { locale: \"en-US\", message: \"message for the user\" }";

        assert!(
            formatted.eq(expected_filled),
            "filled LocalizedMessage differs from expected result"
        );

        let gen_any = match error_info.into_any() {
            Err(error) => panic!("Error generating Any from LocalizedMessage: {:?}", error),
            Ok(gen_any) => gen_any,
        };
        let formatted = format!("{:?}", gen_any);

        println!("Any generated from LocalizedMessage -> {formatted}");

        let expected =
            "Any { type_url: \"type.googleapis.com/google.rpc.LocalizedMessage\", value: [10, 5, 101, 110, 45, 85, 83, 18, 20, 109, 101, 115, 115, 97, 103, 101, 32, 102, 111, 114, 32, 116, 104, 101, 32, 117, 115, 101, 114] }";

        assert!(
            formatted.eq(expected),
            "Any from filled LocalizedMessage differs from expected result"
        );

        let br_details = match LocalizedMessage::from_any(gen_any) {
            Err(error) => panic!("Error generating LocalizedMessage from Any: {:?}", error),
            Ok(from_any) => from_any,
        };

        let formatted = format!("{:?}", br_details);

        println!("LocalizedMessage generated from Any -> {formatted}");

        assert!(
            formatted.eq(expected_filled),
            "LocalizedMessage from Any differs from expected result"
        );
    }
}
