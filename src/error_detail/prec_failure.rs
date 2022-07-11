use prost::{DecodeError, EncodeError, Message};
use prost_types::Any;

use super::super::pb;
use super::super::{FromAny, IntoAny};

/// Used to setup the `violations` field of the `PreconditionFailure` struct.
#[derive(Clone, Debug)]
pub struct PreconditionViolation {
    pub r#type: String,
    pub subject: String,
    pub description: String,
}

impl PreconditionViolation {
    pub fn new(
        r#type: impl Into<String>,
        subject: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        PreconditionViolation {
            r#type: r#type.into(),
            subject: subject.into(),
            description: description.into(),
        }
    }
}

/// Used to encode/decode the `PreconditionFailure` standard error message.
#[derive(Clone, Debug)]
pub struct PreconditionFailure {
    pub violations: Vec<PreconditionViolation>,
}

impl PreconditionFailure {
    pub const TYPE_URL: &'static str = "type.googleapis.com/google.rpc.PreconditionFailure";

    pub fn new(violations: Vec<PreconditionViolation>) -> Self {
        PreconditionFailure {
            violations: violations,
        }
    }

    pub fn with_violation(
        violation_type: impl Into<String>,
        subject: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        PreconditionFailure {
            violations: vec![PreconditionViolation {
                r#type: violation_type.into(),
                subject: subject.into(),
                description: description.into(),
            }],
        }
    }
}

impl PreconditionFailure {
    pub fn add_violation(
        &mut self,
        r#type: impl Into<String>,
        subject: impl Into<String>,
        description: impl Into<String>,
    ) -> &mut Self {
        self.violations.append(&mut vec![PreconditionViolation {
            r#type: r#type.into(),
            subject: subject.into(),
            description: description.into(),
        }]);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }
}

impl IntoAny for PreconditionFailure {
    fn into_any(self) -> Result<Any, EncodeError> {
        let detail_data = pb::PreconditionFailure {
            violations: self
                .violations
                .into_iter()
                .map(|v| pb::precondition_failure::Violation {
                    r#type: v.r#type,
                    subject: v.subject,
                    description: v.description,
                })
                .collect(),
        };

        let mut buf: Vec<u8> = Vec::new();
        buf.reserve(detail_data.encoded_len());
        detail_data.encode(&mut buf)?;

        Ok(Any {
            type_url: PreconditionFailure::TYPE_URL.to_string(),
            value: buf,
        })
    }
}

impl FromAny for PreconditionFailure {
    fn from_any(any: Any) -> Result<Self, DecodeError> {
        let buf: &[u8] = &any.value;
        let precondition_failure = pb::PreconditionFailure::decode(buf)?;

        let precondition_failure = PreconditionFailure {
            violations: precondition_failure
                .violations
                .into_iter()
                .map(|v| PreconditionViolation {
                    r#type: v.r#type,
                    subject: v.subject,
                    description: v.description,
                })
                .collect(),
        };

        Ok(precondition_failure)
    }
}

#[cfg(test)]
mod tests {

    use super::super::super::{FromAny, IntoAny};
    use super::PreconditionFailure;

    #[test]
    fn gen_prec_failure() {
        let mut prec_failure = PreconditionFailure::new(Vec::new());
        let formatted = format!("{:?}", prec_failure);

        println!("empty PreconditionFailure -> {formatted}");

        let expected = "PreconditionFailure { violations: [] }";

        assert!(
            formatted.eq(expected),
            "empty PreconditionFailure differs from expected result"
        );

        assert!(
            prec_failure.is_empty(),
            "empty PreconditionFailure returns 'false' from .is_empty()"
        );

        prec_failure
            .add_violation("TOS", "example.local", "Terms of service not accepted")
            .add_violation("FNF", "example.local", "File not found");

        let formatted = format!("{:?}", prec_failure);

        println!("filled PreconditionFailure -> {formatted}");

        let expected_filled = "PreconditionFailure { violations: [PreconditionViolation { type: \"TOS\", subject: \"example.local\", description: \"Terms of service not accepted\" }, PreconditionViolation { type: \"FNF\", subject: \"example.local\", description: \"File not found\" }] }";

        assert!(
            formatted.eq(expected_filled),
            "filled PreconditionFailure differs from expected result"
        );

        assert!(
            prec_failure.is_empty() == false,
            "filled PreconditionFailure returns 'true' from .is_empty()"
        );

        let gen_any = match prec_failure.into_any() {
            Err(error) => panic!("Error generating Any from PreconditionFailure: {:?}", error),
            Ok(gen_any) => gen_any,
        };
        let formatted = format!("{:?}", gen_any);

        println!("Any generated from PreconditionFailure -> {formatted}");

        let expected = "Any { type_url: \"type.googleapis.com/google.rpc.PreconditionFailure\", value: [10, 51, 10, 3, 84, 79, 83, 18, 13, 101, 120, 97, 109, 112, 108, 101, 46, 108, 111, 99, 97, 108, 26, 29, 84, 101, 114, 109, 115, 32, 111, 102, 32, 115, 101, 114, 118, 105, 99, 101, 32, 110, 111, 116, 32, 97, 99, 99, 101, 112, 116, 101, 100, 10, 36, 10, 3, 70, 78, 70, 18, 13, 101, 120, 97, 109, 112, 108, 101, 46, 108, 111, 99, 97, 108, 26, 14, 70, 105, 108, 101, 32, 110, 111, 116, 32, 102, 111, 117, 110, 100] }";

        assert!(
            formatted.eq(expected),
            "Any from filled PreconditionFailure differs from expected result"
        );

        let br_details = match PreconditionFailure::from_any(gen_any) {
            Err(error) => panic!("Error generating PreconditionFailure from Any: {:?}", error),
            Ok(from_any) => from_any,
        };

        let formatted = format!("{:?}", br_details);

        println!("PreconditionFailure generated from Any -> {formatted}");

        assert!(
            formatted.eq(expected_filled),
            "PreconditionFailure from Any differs from expected result"
        );
    }
}
