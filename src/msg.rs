#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct AccessToken {
    pub state: String,
    pub token: String,
    pub expires_in: u64,
}
