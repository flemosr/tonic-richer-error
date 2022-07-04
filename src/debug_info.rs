use prost::{DecodeError, EncodeError, Message};
use prost_types::Any;

use super::{pb, FromAny, IntoAny};

#[derive(Clone, Debug)]
pub struct DebugInfo {
    pub stack_entries: Vec<String>,
    pub detail: String,
}

impl DebugInfo {
    pub const TYPE_URL: &'static str = "type.googleapis.com/google.rpc.DebugInfo";

    pub fn empty() -> Self {
        DebugInfo {
            stack_entries: Vec::new(),
            detail: String::from(""),
        }
    }

    pub fn with_stack(stack_entries: Vec<impl Into<String>>, detail: impl Into<String>) -> Self {
        let stack_entries = stack_entries
            .into_iter()
            .map(|e| Into::<String>::into(e))
            .collect();

        DebugInfo {
            stack_entries: stack_entries,
            detail: detail.into(),
        }
    }
}

impl IntoAny for DebugInfo {
    fn into_any(&self) -> Result<Any, EncodeError> {
        let detail_data = pb::DebugInfo {
            stack_entries: self.stack_entries.clone(),
            detail: self.detail.clone(),
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
    fn from_any(any: &Any) -> Result<Self, DecodeError> {
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

    use crate::{FromAny, IntoAny};

    use super::DebugInfo;

    #[test]
    fn gen_debug_info() {
        let debug_info = DebugInfo::empty();
        let formatted = format!("{:?}", debug_info);

        println!("empty DebugInfo -> {formatted}");

        let expected = "DebugInfo { stack_entries: [], detail: \"\" }";

        assert!(
            formatted.eq(expected),
            "empty DebugInfo differs from expected result"
        );

        let debug_info = DebugInfo::with_stack(
            vec!["trace 3", "trace 2", "trace 1"],
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

        let br_details = match DebugInfo::from_any(&gen_any) {
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
