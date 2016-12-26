use std::collections::HashMap;
use std::io::Read;

use hyper::client::Client;

use super::errors::*;
use super::msg;
use super::resp;
use super::resp::IntoResult;
use super::net::Open189Client;

const URL_SMS_TOKEN: &'static str = "http://api.189.cn/v2/dm/randcode/token";


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
        let mut params = HashMap::new();
        self.client.get_sync::<_, _, resp::SmsTokenResponse>(self.app_id(),
                                                             self.secret(),
                                                             access_token.as_ref(),
                                                             URL_SMS_TOKEN,
                                                             params)
    }
}
