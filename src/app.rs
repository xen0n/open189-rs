use std::collections::HashMap;

use hyper::client::Client;
use hyper::client::IntoUrl;

use url::Url;

use super::errors::*;
use super::msg;
use super::resp;
use super::net::Open189Client;

const URL_SMS_TOKEN: &'static str = "http://api.189.cn/v2/dm/randcode/token";
const URL_SMS_SEND_WITH_CODE: &'static str = "http://api.189.cn/v2/dm/randcode/sendSms";
const URL_SMS_SEND_WITH_CALLBACK: &'static str = "http://api.189.cn/v2/dm/randcode/send";


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
}


pub struct PreparedSmsCode<'a> {
    phone: &'a str,
    code: &'a str,
    expire_time: Option<usize>,
}


pub struct CallbackSmsCode<'a> {
    phone: &'a str,
    url: Url,
    expire_time: Option<usize>,
}


pub enum SmsCodeConfig<'a> {
    Prepared(PreparedSmsCode<'a>),
    Callback(CallbackSmsCode<'a>),
}


impl<'a> SmsCodeConfig<'a> {
    pub fn prepared(phone: &'a str,
                    code: &'a str,
                    expire_time: Option<usize>)
                    -> SmsCodeConfig<'a> {
        SmsCodeConfig::Prepared(PreparedSmsCode {
            phone: phone,
            code: code,
            expire_time: expire_time,
        })
    }

    pub fn callback<U: IntoUrl>(phone: &'a str,
                                callback_url: U,
                                expire_time: Option<usize>)
                                -> Result<SmsCodeConfig<'a>> {
        Ok(SmsCodeConfig::Callback(CallbackSmsCode {
            phone: phone,
            url: callback_url.into_url()?,
            expire_time: expire_time,
        }))
    }
}


impl Open189App {
    pub fn sms_send_verification_code<S: AsRef<str>>(&self,
                                                     access_token: S,
                                                     sms_token: S,
                                                     config: SmsCodeConfig)
                                                     -> Result<msg::SentSmsCode> {
        let mut params = HashMap::new();
        params.insert("token", sms_token.as_ref().to_string());

        let url;
        match config {
            SmsCodeConfig::Prepared(config) => {
                url = URL_SMS_SEND_WITH_CODE;

                let code = config.code;
                if code.len() != 6 {
                    return Err(ErrorKind::WrongSmsCodeLength(code.len(), 6).into());
                }
                if !code.chars().all(|ch| ch.is_digit(10)) {
                    return Err(ErrorKind::NonDigitInSmsCode(code.to_string()).into());
                }
                params.insert("randcode", code.to_string());

                params.insert("phone", config.phone.to_string());
                if let Some(expire_time) = config.expire_time {
                    params.insert("exp_time", format!("{}", expire_time));
                }
            }
            SmsCodeConfig::Callback(config) => {
                url = URL_SMS_SEND_WITH_CALLBACK;

                params.insert("phone", config.phone.to_string());
                params.insert("url", config.url.into_string());
                if let Some(expire_time) = config.expire_time {
                    params.insert("exp_time", format!("{}", expire_time));
                }
            }
        }

        self.client.post_sync::<_, _, resp::SmsCodeResponse>(self.app_id(),
                                                             self.secret(),
                                                             access_token.as_ref(),
                                                             url,
                                                             params)
    }
}
