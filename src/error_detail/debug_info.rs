use prost::{DecodeError, EncodeError, Message};
use prost_types::Any;

use super::super::pb;
use super::super::{FromAny, IntoAny};

/// Used to encode/decode the `DebugInfo` standard error message.
#[derive(Clone, Debug)]
pub struct DebugInfo {
    pub stack_entries: Vec<String>,
    pub detail: String,
}

impl DebugInfo {
    pub const TYPE_URL: &'static str = "type.googleapis.com/google.rpc.DebugInfo";

    pub fn new(stack_entries: Vec<String>, detail: impl Into<String>) -> Self {
        DebugInfo {
            stack_entries: stack_entries,
            detail: detail.into(),
        }
    }
}

impl DebugInfo {
    pub fn is_empty(&self) -> bool {
        self.stack_entries.is_empty() && self.detail.is_empty()
    }
}

impl IntoAny for DebugInfo {
    fn into_any(self) -> Result<Any, EncodeError> {
        let detail_data = pb::DebugInfo {
            stack_entries: self.stack_entries,
            detail: self.detail,
        };

        let mut buf: Vec<u8> = Vec::new();
        buf.reserve(detail_data.encoded_len());
        detail_data.encode(&mut buf)?;

        Ok(Any {
            type_url: DebugInfo::TYPE_URL.to_string(),
            value: buf,
        })
    }
}

impl FromAny for DebugInfo {
    fn from_any(any: Any) -> Result<Self, DecodeError> {
        let buf: &[u8] = &any.value;
        let debug_info = pb::DebugInfo::decode(buf)?;

        let debug_info = DebugInfo {
            stack_entries: debug_info.stack_entries,
            detail: debug_info.detail,
        };

        Ok(debug_info)
    }
}

#[cfg(test)]
mod tests {

    use super::super::super::{FromAny, IntoAny};
    use super::DebugInfo;

    #[test]
    fn gen_debug_info() {
        let debug_info = DebugInfo::new(
            vec![
                "trace 3".to_string(),
                "trace 2".to_string(),
                "trace 1".to_string(),
            ],
            "details about the error",
        );

        let formatted = format!("{:?}", debug_info);

        println!("filled DebugInfo -> {formatted}");

        let expected_filled = "DebugInfo { stack_entries: [\"trace 3\", \"trace 2\", \"trace 1\"], detail: \"details about the error\" }";

        assert!(
            formatted.eq(expected_filled),
            "filled DebugInfo differs from expected result"
        );

        let gen_any = match debug_info.into_any() {
            Err(error) => panic!("Error generating Any from DebugInfo: {:?}", error),
            Ok(gen_any) => gen_any,
        };
        let formatted = format!("{:?}", gen_any);

        println!("Any generated from DebugInfo -> {formatted}");

        let expected =
            "Any { type_url: \"type.googleapis.com/google.rpc.DebugInfo\", value: [10, 7, 116, 114, 97, 99, 101, 32, 51, 10, 7, 116, 114, 97, 99, 101, 32, 50, 10, 7, 116, 114, 97, 99, 101, 32, 49, 18, 23, 100, 101, 116, 97, 105, 108, 115, 32, 97, 98, 111, 117, 116, 32, 116, 104, 101, 32, 101, 114, 114, 111, 114] }";

        assert!(
            formatted.eq(expected),
            "Any from filled DebugInfo differs from expected result"
        );

        let br_details = match DebugInfo::from_any(gen_any) {
            Err(error) => panic!("Error generating DebugInfo from Any: {:?}", error),
            Ok(from_any) => from_any,
        };

        let formatted = format!("{:?}", br_details);

        println!("DebugInfo generated from Any -> {formatted}");

        assert!(
            formatted.eq(expected_filled),
            "DebugInfo from Any differs from expected result"
        );
    }
}
