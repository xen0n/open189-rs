use std::collections::HashMap;

use hyper::client::Client;

use super::errors::*;
use super::msg;
use super::resp;
use super::net::Open189Client;

const URL_SMS_TOKEN: &'static str = "http://api.189.cn/v2/dm/randcode/token";
const URL_SMS_SEND_WITH_CODE: &'static str = "http://api.189.cn/v2/dm/randcode/sendSms";


pub struct Open189App {
    app_id: String,
    secret: String,
    client: Open189Client,
}


impl Open189App {
    pub fn new<S: AsRef<str>>(app_id: S, secret: S) -> Open189App {
        Open189App::with_client(app_id, secret, Client::new())
    }

    pub fn with_client<S: AsRef<str>>(app_id: S, secret: S, client: Client) -> Open189App {
        Open189App {
            app_id: app_id.as_ref().to_string(),
            secret: secret.as_ref().to_string(),
            client: Open189Client::new(client),
        }
    }

    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    pub fn secret(&self) -> &str {
        &self.secret
    }

    pub fn get_access_token_cc(&self) -> Result<msg::AccessToken> {
        let mut params = HashMap::new();
        params.insert("grant_type", "client_credentials".to_string());
        self.client
            .perform_access_token_req::<_, resp::AccessTokenResponse>(self.app_id(),
                                                                      self.secret(),
                                                                      params)
    }

    pub fn sms_get_token<S: AsRef<str>>(&self, access_token: S) -> Result<String> {
        let params = HashMap::new();
        self.client.get_sync::<_, _, resp::SmsTokenResponse>(self.app_id(),
                                                             self.secret(),
                                                             access_token.as_ref(),
                                                             URL_SMS_TOKEN,
                                                             params)
    }

    pub fn sms_send_verification_code<S: AsRef<str>>(&self,
                                                     access_token: S,
                                                     sms_token: S,
                                                     phone: S,
                                                     code: S,
                                                     expire_time: Option<usize>)
                                                     -> Result<msg::SentSmsCode> {
        let code = code.as_ref().to_string();
        if code.len() != 6 {
            return Err(ErrorKind::WrongSmsCodeLength(code.len(), 6).into());
        }
        if !code.chars().all(|ch| ch.is_digit(10)) {
            return Err(ErrorKind::NonDigitInSmsCode(code).into());
        }

        let mut params = HashMap::new();
        params.insert("token", sms_token.as_ref().to_string());
        params.insert("phone", phone.as_ref().to_string());
        if expire_time.is_some() {
            params.insert("exp_time", format!("{}", expire_time.unwrap()));
        }
        params.insert("randcode", code);
        self.client.post_sync::<_, _, resp::SmsCodeResponse>(self.app_id(),
                                                             self.secret(),
                                                             access_token.as_ref(),
                                                             URL_SMS_SEND_WITH_CODE,
                                                             params)
    }
}
