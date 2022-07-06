use prost::{DecodeError, EncodeError, Message};
use prost_types::Any;

use super::super::pb;
use super::super::{FromAny, IntoAny};

#[derive(Clone, Debug)]
pub struct Link {
    pub description: String,
    pub url: String,
}
#[derive(Clone, Debug)]
pub struct Help {
    pub links: Vec<Link>,
}

impl Help {
    pub const TYPE_URL: &'static str = "type.googleapis.com/google.rpc.Help";

    pub fn empty() -> Self {
        Help { links: Vec::new() }
    }

    pub fn add_link(
        &mut self,
        description: impl Into<String>,
        url: impl Into<String>,
    ) -> &mut Self {
        self.links.append(&mut vec![Link {
            description: description.into(),
            url: url.into(),
        }]);
        self
    }

    pub fn with_link(description: impl Into<String>, url: impl Into<String>) -> Self {
        Help {
            links: vec![Link {
                description: description.into(),
                url: url.into(),
            }],
        }
    }

    pub fn has_links(&self) -> bool {
        self.links.is_empty() == false
    }
}

impl IntoAny for Help {
    fn into_any(self) -> Result<Any, EncodeError> {
        let detail_data = pb::Help {
            links: self
                .links
                .into_iter()
                .map(|v| pb::help::Link {
                    description: v.description,
                    url: v.url,
                })
                .collect(),
        };

        let mut buf: Vec<u8> = Vec::new();
        buf.reserve(detail_data.encoded_len());
        detail_data.encode(&mut buf)?;

        Ok(Any {
            type_url: Help::TYPE_URL.to_string(),
            value: buf,
        })
    }
}

impl FromAny for Help {
    fn from_any(any: Any) -> Result<Self, DecodeError> {
        let buf: &[u8] = &any.value;
        let help = pb::Help::decode(buf)?;

        let quota_failure = Help {
            links: help
                .links
                .into_iter()
                .map(|v| Link {
                    description: v.description,
                    url: v.url,
                })
                .collect(),
        };

        Ok(quota_failure)
    }
}

#[cfg(test)]
mod tests {

    use super::super::super::{FromAny, IntoAny};
    use super::Help;

    #[test]
    fn gen_quota_failure() {
        let mut help = Help::empty();
        let formatted = format!("{:?}", help);

        println!("empty Help -> {formatted}");

        let expected = "Help { links: [] }";

        assert!(
            formatted.eq(expected),
            "empty Help differs from expected result"
        );

        assert!(
            help.has_links() == false,
            "empty Help returns 'true' from .has_violations()"
        );

        help.add_link("link to resource a", "resource-a.example.local")
            .add_link("link to resource b", "resource-b.example.local");

        let formatted = format!("{:?}", help);

        println!("filled Help -> {formatted}");

        let expected_filled = "Help { links: [Link { description: \"link to resource a\", url: \"resource-a.example.local\" }, Link { description: \"link to resource b\", url: \"resource-b.example.local\" }] }";

        assert!(
            formatted.eq(expected_filled),
            "filled Help differs from expected result"
        );

        assert!(
            help.has_links() == true,
            "filled Help returns 'false' from .has_violations()"
        );

        let gen_any = match help.into_any() {
            Err(error) => panic!("Error generating Any from Help: {:?}", error),
            Ok(gen_any) => gen_any,
        };
        let formatted = format!("{:?}", gen_any);

        println!("Any generated from Help -> {formatted}");

        let expected = "Any { type_url: \"type.googleapis.com/google.rpc.Help\", value: [10, 46, 10, 18, 108, 105, 110, 107, 32, 116, 111, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 97, 18, 24, 114, 101, 115, 111, 117, 114, 99, 101, 45, 97, 46, 101, 120, 97, 109, 112, 108, 101, 46, 108, 111, 99, 97, 108, 10, 46, 10, 18, 108, 105, 110, 107, 32, 116, 111, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 18, 24, 114, 101, 115, 111, 117, 114, 99, 101, 45, 98, 46, 101, 120, 97, 109, 112, 108, 101, 46, 108, 111, 99, 97, 108] }";

        assert!(
            formatted.eq(expected),
            "Any from filled Help differs from expected result"
        );

        let br_details = match Help::from_any(gen_any) {
            Err(error) => panic!("Error generating Help from Any: {:?}", error),
            Ok(from_any) => from_any,
        };

        let formatted = format!("{:?}", br_details);

        println!("Help generated from Any -> {formatted}");

        assert!(
            formatted.eq(expected_filled),
            "Help from Any differs from expected result"
        );
    }
}
