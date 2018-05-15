pub use chrono::{DateTime, FixedOffset, Duration};

#[derive(Deserialize, Debug)]
pub struct FcmResponse {
    pub message_id: Option<u64>,
    pub error: Option<String>,
    pub multicast_id: Option<i64>,
    pub success: Option<u64>,
    pub failure: Option<u64>,
    pub canonical_ids: Option<u64>,
    pub results: Option<Vec<MessageResult>>,
}

#[derive(Deserialize, Debug)]
pub struct MessageResult {
    pub message_id: Option<String>,
    pub registration_id: Option<String>,
    pub error: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum FcmError {
    Unauthorized,
    InvalidMessage(String),
    ServerError(Option<RetryAfter>),
}

#[derive(PartialEq, Debug)]
pub enum RetryAfter {
    Delay(Duration),
    DateTime(DateTime<FixedOffset>),
}

impl RetryAfter {
    pub fn from_str(header_value: &str) -> Option<RetryAfter> {
        if let Ok(seconds) = header_value.parse::<i64>() {
            Some(RetryAfter::Delay(Duration::seconds(seconds)))
        } else {
            DateTime::parse_from_rfc2822(header_value)
                .map(|date_time| {
                    RetryAfter::DateTime(date_time)
                })
                .ok()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Duration};

    #[test]
    fn test_retry_after_from_seconds() {
        assert_eq!(
            Some(RetryAfter::Delay(Duration::seconds(420))),
            RetryAfter::from_str("420")
        );
    }

    #[test]
    fn test_retry_after_from_date() {
        let date = "Sun, 06 Nov 1994 08:49:37 GMT";
        let retry_after = RetryAfter::from_str(date);

        assert_eq!(
            Some(RetryAfter::DateTime(
                DateTime::parse_from_rfc2822(date).unwrap()
            )),
            retry_after,
        );
    }
}
