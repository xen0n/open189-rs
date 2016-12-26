use hyper::status::StatusCode;

use super::errors::*;
use super::msg;


pub trait IntoResult {
    type Item;

    fn into_result(self, StatusCode) -> Result<Self::Item>;
}


#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessTokenResponse {
    #[serde(rename="res_code")]
    retcode: u64,
    #[serde(rename="res_message")]
    message: Option<String>,
    state: String,
    access_token: Option<String>,
    expires_in: Option<u64>,
}


impl IntoResult for AccessTokenResponse {
    type Item = msg::AccessToken;

    fn into_result(self, http_status: StatusCode) -> Result<Self::Item> {
        if self.retcode == 0 {
            Ok(msg::AccessToken {
                state: self.state,
                token: self.access_token.unwrap(),
                expires_in: self.expires_in.unwrap(),
            })
        } else {
            Err(ErrorKind::ApiError(http_status, self.retcode, Some(self.state), self.message)
                .into())
        }
    }
}


#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SmsTokenResponse {
    #[serde(rename="res_code")]
    retcode: u64,
    #[serde(rename="res_message")]
    message: Option<String>,
    token: Option<String>,
}


impl IntoResult for SmsTokenResponse {
    type Item = String;

    fn into_result(self, http_status: StatusCode) -> Result<Self::Item> {
        if self.retcode == 0 && self.token.is_some() {
            Ok(self.token.unwrap())
        } else {
            Err(ErrorKind::ApiError(http_status, self.retcode, None, self.message).into())
        }
    }
}
