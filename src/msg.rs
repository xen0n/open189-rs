#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct AccessToken {
    pub state: String,
    pub token: String,
    pub expires_in: u64,
}


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct SentSmsCode {
    pub send_time: u64,
    pub sms_id: String,
}
