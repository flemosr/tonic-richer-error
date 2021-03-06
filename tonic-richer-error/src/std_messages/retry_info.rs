use std::{ops::Add, time};

use prost::{DecodeError, EncodeError, Message};
use prost_types::Any;

use super::super::pb;
use super::super::{FromAny, IntoAny};

/// Used to encode/decode the `RetryInfo` standard error message.
#[derive(Clone, Debug)]
pub struct RetryInfo {
    pub retry_delay: Option<time::Duration>,
}

impl RetryInfo {
    pub const TYPE_URL: &'static str = "type.googleapis.com/google.rpc.RetryInfo";

    pub fn new(retry_delay: Option<time::Duration>) -> Self {
        RetryInfo {
            retry_delay: retry_delay,
        }
    }
}

impl RetryInfo {
    pub fn is_empty(&self) -> bool {
        self.retry_delay.is_none()
    }
}

impl IntoAny for RetryInfo {
    fn into_any(self) -> Result<Any, EncodeError> {
        let retry_delay = match self.retry_delay {
            Some(duration) => Some(prost_types::Duration::from(duration)),
            None => None,
        };

        let detail_data = pb::RetryInfo {
            retry_delay: retry_delay,
        };

        let mut buf: Vec<u8> = Vec::new();
        buf.reserve(detail_data.encoded_len());
        detail_data.encode(&mut buf)?;

        Ok(Any {
            type_url: RetryInfo::TYPE_URL.to_string(),
            value: buf,
        })
    }
}

impl FromAny for RetryInfo {
    // Negative retry_delays become 0
    fn from_any(any: Any) -> Result<Self, DecodeError> {
        let buf: &[u8] = &any.value;
        let retry_info = pb::RetryInfo::decode(buf)?;

        let retry_delay = match retry_info.retry_delay {
            Some(duration) => {
                let secs: u64 = duration.seconds.try_into().unwrap_or(0);

                let mut conv_duration = time::Duration::from_secs(secs);

                let nanos: u64 = duration.nanos.try_into().unwrap_or(0);

                conv_duration = conv_duration.add(time::Duration::from_nanos(nanos));

                Some(conv_duration)
            }
            None => None,
        };

        let retry_info = RetryInfo {
            retry_delay: retry_delay,
        };

        Ok(retry_info)
    }
}

#[cfg(test)]
mod tests {

    use core::time::Duration;

    use super::super::super::{FromAny, IntoAny};
    use super::RetryInfo;

    #[test]
    fn gen_retry_info() {
        let ri_details = RetryInfo::new(Some(Duration::from_secs(5)));

        let formatted = format!("{:?}", ri_details);

        println!("filled RetryInfo -> {formatted}");

        let expected_filled = "RetryInfo { retry_delay: Some(5s) }";

        assert!(
            formatted.eq(expected_filled),
            "filled RetryInfo differs from expected result"
        );

        assert!(
            ri_details.is_empty() == false,
            "filled RetryInfo returns 'false' from .has_retry_delay()"
        );

        let gen_any = match ri_details.into_any() {
            Err(error) => panic!("Error generating Any from RetryInfo: {:?}", error),
            Ok(gen_any) => gen_any,
        };
        let formatted = format!("{:?}", gen_any);

        println!("Any generated from RetryInfo -> {formatted}");

        let expected =
            "Any { type_url: \"type.googleapis.com/google.rpc.RetryInfo\", value: [10, 2, 8, 5] }";

        assert!(
            formatted.eq(expected),
            "Any from filled RetryInfo differs from expected result"
        );

        let br_details = match RetryInfo::from_any(gen_any) {
            Err(error) => panic!("Error generating RetryInfo from Any: {:?}", error),
            Ok(from_any) => from_any,
        };

        let formatted = format!("{:?}", br_details);

        println!("RetryInfo generated from Any -> {formatted}");

        assert!(
            formatted.eq(expected_filled),
            "RetryInfo from Any differs from expected result"
        );
    }
}
