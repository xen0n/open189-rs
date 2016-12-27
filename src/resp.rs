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
    res_code: u64,
    res_message: Option<String>,
    state: String,
    access_token: Option<String>,
    expires_in: Option<u64>,
}


impl IntoResult for AccessTokenResponse {
    type Item = msg::AccessToken;

    fn into_result(self, http_status: StatusCode) -> Result<Self::Item> {
        if self.res_code == 0 {
            Ok(msg::AccessToken {
                state: self.state,
                token: self.access_token.unwrap(),
                expires_in: self.expires_in.unwrap(),
            })
        } else {
            Err(ErrorKind::ApiError(http_status,
                                    self.res_code,
                                    Some(self.state),
                                    self.res_message)
                .into())
        }
    }
}


#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SmsTokenResponse {
    res_code: u64,
    res_message: Option<String>,
    token: Option<String>,
}


impl IntoResult for SmsTokenResponse {
    type Item = String;

    fn into_result(self, http_status: StatusCode) -> Result<Self::Item> {
        if self.res_code == 0 && self.token.is_some() {
            Ok(self.token.unwrap())
        } else {
            Err(ErrorKind::ApiError(http_status, self.res_code, None, self.res_message).into())
        }
    }
}


#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SmsCodeResponse {
    res_code: u64,
    res_message: Option<String>,
    create_at: Option<u64>,
    identifier: Option<String>,
}


impl IntoResult for SmsCodeResponse {
    type Item = msg::SentSmsCode;

    fn into_result(self, http_status: StatusCode) -> Result<Self::Item> {
        if self.res_code == 0 && self.create_at.is_some() && self.identifier.is_some() {
            Ok(msg::SentSmsCode {
                send_time: self.create_at.unwrap(),
                sms_id: self.identifier.unwrap(),
            })
        } else {
            Err(ErrorKind::ApiError(http_status, self.res_code, None, self.res_message).into())
        }
    }
}
