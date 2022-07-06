use prost::{DecodeError, EncodeError, Message};
use prost_types::Any;

use super::super::pb;
use super::super::{FromAny, IntoAny};

#[derive(Clone, Debug)]
pub struct ResourceInfo {
    pub resource_type: String,
    pub resource_name: String,
    pub owner: String,
    pub description: String,
}

impl ResourceInfo {
    pub const TYPE_URL: &'static str = "type.googleapis.com/google.rpc.ResourceInfo";

    pub fn empty() -> Self {
        ResourceInfo {
            resource_type: String::from(""),
            resource_name: String::from(""),
            owner: String::from(""),
            description: String::from(""),
        }
    }

    pub fn with_data(
        resource_type: impl Into<String>,
        resource_name: impl Into<String>,
        owner: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        ResourceInfo {
            resource_type: resource_type.into(),
            resource_name: resource_name.into(),
            owner: owner.into(),
            description: description.into(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.resource_type.is_empty()
            && self.resource_name.is_empty()
            && self.owner.is_empty()
            && self.description.is_empty()
    }
}

impl IntoAny for ResourceInfo {
    fn into_any(self) -> Result<Any, EncodeError> {
        let detail_data = pb::ResourceInfo {
            resource_type: self.resource_type,
            resource_name: self.resource_name,
            owner: self.owner,
            description: self.description,
        };

        let mut buf: Vec<u8> = Vec::new();
        buf.reserve(detail_data.encoded_len());
        detail_data.encode(&mut buf)?;

        Ok(Any {
            type_url: ResourceInfo::TYPE_URL.to_string(),
            value: buf,
        })
    }
}

impl FromAny for ResourceInfo {
    fn from_any(any: Any) -> Result<Self, DecodeError> {
        let buf: &[u8] = &any.value;
        let res_info = pb::ResourceInfo::decode(buf)?;

        let debug_info = ResourceInfo {
            resource_type: res_info.resource_type,
            resource_name: res_info.resource_name,
            owner: res_info.owner,
            description: res_info.description,
        };

        Ok(debug_info)
    }
}

#[cfg(test)]
mod tests {

    use super::super::super::{FromAny, IntoAny};
    use super::ResourceInfo;

    #[test]
    fn gen_error_info() {
        let error_info = ResourceInfo::empty();
        let formatted = format!("{:?}", error_info);

        println!("empty ResourceInfo -> {formatted}");

        let expected = "ResourceInfo { resource_type: \"\", resource_name: \"\", owner: \"\", description: \"\" }";

        assert!(
            formatted.eq(expected),
            "empty ResourceInfo differs from expected result"
        );

        let error_info =
            ResourceInfo::with_data("resource-type", "resource-name", "owner", "description");

        let formatted = format!("{:?}", error_info);

        println!("filled ResourceInfo -> {formatted}");

        let expected_filled = "ResourceInfo { resource_type: \"resource-type\", resource_name: \"resource-name\", owner: \"owner\", description: \"description\" }";

        assert!(
            formatted.eq(expected_filled),
            "filled ResourceInfo differs from expected result"
        );

        let gen_any = match error_info.into_any() {
            Err(error) => panic!("Error generating Any from ResourceInfo: {:?}", error),
            Ok(gen_any) => gen_any,
        };
        let formatted = format!("{:?}", gen_any);

        println!("Any generated from ResourceInfo -> {formatted}");

        let expected =
            "Any { type_url: \"type.googleapis.com/google.rpc.ResourceInfo\", value: [10, 13, 114, 101, 115, 111, 117, 114, 99, 101, 45, 116, 121, 112, 101, 18, 13, 114, 101, 115, 111, 117, 114, 99, 101, 45, 110, 97, 109, 101, 26, 5, 111, 119, 110, 101, 114, 34, 11, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110] }";

        assert!(
            formatted.eq(expected),
            "Any from filled ResourceInfo differs from expected result"
        );

        let br_details = match ResourceInfo::from_any(gen_any) {
            Err(error) => panic!("Error generating ResourceInfo from Any: {:?}", error),
            Ok(from_any) => from_any,
        };

        let formatted = format!("{:?}", br_details);

        println!("ResourceInfo generated from Any -> {formatted}");

        assert!(
            formatted.eq(expected_filled),
            "ResourceInfo from Any differs from expected result"
        );
    }
}
