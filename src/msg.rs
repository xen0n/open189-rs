//! Types for the `open.189.cn` API responses.

/// An access token for the `open.189.cn` API.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct AccessToken {
    /// Nonce for distinguishing between access token requests.
    pub state: String,
    /// The access token returned.
    pub token: String,
    /// TTL of the access token, in seconds.
    pub expires_in: u64,
}


/// A summary of a successfully queued SMS verification code.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SentSmsCode {
    /// The time the SMS is queued for sending, as recorded by the API.
    pub send_time: u64,
    /// API-generated unique identifier for the SMS.
    pub sms_id: String,
}
