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


/// Client for the `open.189.cn` API.
pub struct Open189App {
    app_id: String,
    secret: String,
    client: Open189Client,
}


impl Open189App {
    /// Construct a client instance given the `open.189.cn` app ID and secret.
    ///
    /// A Hyper client is created with default parameters for the underlying
    /// HTTP transport. You can provide your own Hyper client instance instead
    /// with the [`with_client`] method.
    ///
    /// [`with_client`]: #method.with_client
    ///
    /// # Examples
    ///
    /// ```
    /// use open189::Open189App;
    ///
    /// let app_id = "your app id here";
    /// let secret = "your app secret here";
    /// let client = Open189App::new(app_id, secret);
    /// ```
    pub fn new<S: AsRef<str>>(app_id: S, secret: S) -> Open189App {
        Open189App::with_client(app_id, secret, Client::new())
    }

    /// Construct a client instance with the provided Hyper client instance.
    ///
    /// Consumes the `Client` passed in; useful if you want to configure your
    /// HTTP client before using, for example setting up proxy or connection
    /// pooling.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate hyper;
    /// # extern crate open189;
    /// use open189::Open189App;
    ///
    /// # fn main() {
    /// let app_id = "your app id here";
    /// let secret = "your app secret here";
    /// let http_client = hyper::client::Client::new();
    /// // configure your HTTP client
    /// let client = Open189App::with_client(app_id, secret, http_client);
    /// # }
    /// ```
    pub fn with_client<S: AsRef<str>>(app_id: S, secret: S, client: Client) -> Open189App {
        Open189App {
            app_id: app_id.as_ref().to_string(),
            secret: secret.as_ref().to_string(),
            client: Open189Client::new(client),
        }
    }

    /// Get the app ID the client is created with.
    ///
    /// # Examples
    ///
    /// ```
    /// use open189::Open189App;
    ///
    /// let app_id = "your app id here";
    /// let secret = "your app secret here";
    /// let client = Open189App::new(app_id, secret);
    ///
    /// assert_eq!(client.app_id(), app_id);
    /// ```
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Get the app secret the client is created with.
    ///
    /// # Examples
    ///
    /// ```
    /// use open189::Open189App;
    ///
    /// let app_id = "your app id here";
    /// let secret = "your app secret here";
    /// let client = Open189App::new(app_id, secret);
    ///
    /// assert_eq!(client.secret(), secret);
    /// ```
    pub fn secret(&self) -> &str {
        &self.secret
    }

    /// Request a user-independent access token with the Client Credentials flow.
    ///
    /// As a best practice, you should utilize the API considerately and avoid
    /// repeatedly hammering it with access token requests. It's recommended to
    /// cache the access token yourself, in any way you'd like to, and periodically
    /// refresh it before expiry.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn foo(client: &::open189::Open189App) -> ::open189::errors::Result<()> {
    /// let access_token = client.get_access_token_cc()?;
    /// // store the access token somewhere!
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_access_token_cc(&self) -> Result<msg::AccessToken> {
        let mut params = HashMap::new();
        params.insert("grant_type", "client_credentials".to_string());
        self.client
            .perform_access_token_req::<_, resp::AccessTokenResponse>(self.app_id(),
                                                                      self.secret(),
                                                                      params)
    }

    /// Request a token for use in the SMS sending API.
    ///
    /// An access token is required; you can get one with the [`get_access_token_cc`]
    /// method.
    ///
    /// [`get_access_token_cc`]: #method.get_access_token_cc
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn fetch_cached_access_token() -> &'static str { "dummy" }
    /// # fn foo(client: &::open189::Open189App) -> ::open189::errors::Result<()> {
    /// let access_token = fetch_cached_access_token();
    /// let sms_token = client.sms_get_token(access_token)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn sms_get_token<S: AsRef<str>>(&self, access_token: S) -> Result<String> {
        let params = HashMap::new();
        self.client.get_sync::<_, _, resp::SmsTokenResponse>(self.app_id(),
                                                             self.secret(),
                                                             access_token.as_ref(),
                                                             URL_SMS_TOKEN,
                                                             params)
    }
}


/// Configuration for sending of locally generated SMS verification code.
///
/// This struct is not meant to be used directly; see [`SmsCodeConfig::prepared`]
/// instead.
///
/// [`SmsCodeConfig::prepared`]: enum.SmsCodeConfig.html#method.prepared
pub struct PreparedSmsCode<'a> {
    phone: &'a str,
    code: &'a str,
    expire_time: Option<usize>,
}


/// Configuration for sending of API-generated SMS verification code.
///
/// This struct is not meant to be used directly; see [`SmsCodeConfig::callback`]
/// instead.
///
/// [`SmsCodeConfig::callback`]: enum.SmsCodeConfig.html#method.callback
pub struct CallbackSmsCode<'a> {
    phone: &'a str,
    url: Url,
    expire_time: Option<usize>,
}


/// Configuration for SMS verification code.
pub enum SmsCodeConfig<'a> {
    /// Code is generated locally, ready to be sent.
    Prepared(PreparedSmsCode<'a>),
    /// Code is to be generated remotely by the API server, and sent back to the
    /// callback URL provided.
    Callback(CallbackSmsCode<'a>),
}


impl<'a> SmsCodeConfig<'a> {
    /// Construct the parameters for sending pre-generated verification code.
    ///
    /// The code should consist of 6 digits only. Everything else would be rejected
    /// by the API anyway, so the validation is done locally before firing the
    /// actual request.
    ///
    /// Expiry time is optional and seems purely informative, given it's just
    /// another integer formatted into the fixed SMS template. The value is
    /// expected to be in minutes, as suggested by the wording of the template.
    /// Defaults to 2 minutes if not specified.
    ///
    /// # Examples
    ///
    /// ```
    /// use open189::SmsCodeConfig;
    ///
    /// // use default expiry time
    /// let config = SmsCodeConfig::prepared("12345678901", "234567", None);
    ///
    /// // manually specify an expiry time of 5 min
    /// let config = SmsCodeConfig::prepared("12345678901", "234567", Some(5));
    /// ```
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

    /// Construct the parameters for sending remotely-generated verification code.
    ///
    /// Instead of providing the code yourself, the API would choose one for you,
    /// and notify you by `POST`-ing to the callback URL you provided. The URL is
    /// validated on construction and may fail, in which case an `Err` would be
    /// returned.
    ///
    /// Expiry time is interpreted the same way as [above].
    ///
    /// [above]: #method.prepared
    ///
    /// # Examples
    ///
    /// ```
    /// use open189::SmsCodeConfig;
    ///
    /// let url = "https://api.example.com/v1/callback/sms";
    /// let config = SmsCodeConfig::callback("12345678901", url, None).unwrap();
    /// ```
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
    /// Send a verification code with SMS.
    ///
    /// An access token is required; you can get one with the [`get_access_token_cc`]
    /// method. This token is reusable; just remember to refresh it periodically.
    /// You also need to request a separate token for every SMS you're going
    /// to send, with the [`sms_get_token`] method.
    ///
    /// There are two supported ways to send a SMS verification code. One is to
    /// generate the verification code yourself; the other is to have the API
    /// pick one for you, then notifying you with a POST to a callback URL provided
    /// alongside the request. You're expected to manage the expiry time yourself
    /// anyway, so probably you're also generating your own codes, but the choice
    /// is there; use one of [the `SmsCodeConfig` constructors][ctors] to pass the
    /// parameters.
    ///
    /// [`get_access_token_cc`]: #method.get_access_token_cc
    /// [`sms_get_token`]: #method.sms_get_token
    /// [ctors]: enum.SmsCodeConfig.html#methods
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use open189::SmsCodeConfig;
    ///
    /// # fn fetch_cached_access_token() -> &'static str { "dummy" }
    /// # fn foo(client: &::open189::Open189App) -> ::open189::errors::Result<()> {
    /// let access_token = fetch_cached_access_token();
    /// let sms_token = client.sms_get_token(access_token)?;
    ///
    /// let config = SmsCodeConfig::prepared("12345678901", "234567", Some(5));
    /// let result = client.sms_send_verification_code(access_token, &sms_token, config)?;
    /// # Ok(())
    /// # }
    /// ```
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
