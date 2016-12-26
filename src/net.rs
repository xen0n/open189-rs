use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use hyper::client::Client;
use hyper::client::Response;
use hyper::client::IntoUrl;
use hyper::header::ContentType;
use hyper::header::Headers;
use hyper::mime;

use url::form_urlencoded;
use url::form_urlencoded::Target;

use super::errors::*;
use super::sig;
use super::util;

const URL_ACCESS_TOKEN_REQ: &'static str = "https://oauth.api.189.cn/emp/oauth2/v3/access_token";


pub struct Open189Client<'a> {
    http: Arc<Client>,
    app_id: &'a str,
    secret: &'a str,
    access_token: Option<&'a str>,
}


fn prepare_request_params(params: &mut HashMap<&'static str, String>,
                          app_id: &str,
                          secret: &str,
                          access_token: &str) {
    params.insert("app_id", app_id.to_string());
    params.insert("access_token", access_token.to_string());
    params.insert("timestamp", util::get_api_timestamp());
    let sign = sig::sign(params, secret);
    params.insert("sign", sign);
}


impl<'a> Open189Client<'a> {
    pub fn new(http_client: Client,
               app_id: &'a str,
               secret: &'a str,
               access_token: Option<&'a str>)
               -> Open189Client<'a> {
        Open189Client {
            http: Arc::new(http_client),
            app_id: app_id,
            secret: secret,
            access_token: access_token,
        }
    }

    fn require_access_token(&self) -> Result<()> {
        if self.access_token.is_none() {
            Err(ErrorKind::AccessTokenRequired.into())
        } else {
            Ok(())
        }
    }

    pub fn get_sync<U: IntoUrl>(&self,
                                url: U,
                                mut params: HashMap<&'static str, String>)
                                -> Result<Response> {
        self.require_access_token()?;

        let mut url = url.into_url()?;
        prepare_request_params(&mut params,
                               self.app_id,
                               self.secret,
                               self.access_token.unwrap());
        {
            let mut qs = url.query_pairs_mut();
            qs.clear();
            for (k, v) in params.iter() {
                qs.append_pair(k.as_ref(), v.as_ref());
            }
        }

        let response = self.http.get(url).send()?;

        Ok(response)
    }

    pub fn post_sync<U: IntoUrl>(&self,
                                 url: U,
                                 mut params: HashMap<&'static str, String>)
                                 -> Result<Response> {
        self.require_access_token()?;
        prepare_request_params(&mut params,
                               self.app_id,
                               self.secret,
                               self.access_token.unwrap());
        self.post_sync_prepared(url, params)
    }

    pub fn perform_access_token_req(&self,
                                    mut params: HashMap<&'static str, String>)
                                    -> Result<Response> {
        params.insert("app_id", self.app_id.to_string());
        params.insert("app_secret", self.secret.to_string());
        params.insert("state", util::get_random_state_str());
        self.post_sync_prepared(URL_ACCESS_TOKEN_REQ, params)
    }

    fn post_sync_prepared<U: IntoUrl>(&self,
                                      url: U,
                                      params: HashMap<&'static str, String>)
                                      -> Result<Response> {
        let url = url.into_url()?;
        let body = {
            let mut serializer = form_urlencoded::Serializer::new(String::new());
            for (k, v) in params.iter() {
                serializer.append_pair(k, v);
            }
            serializer.finish()
        };

        let headers = {
            let mut tmp = Headers::new();
            tmp.set(ContentType(mime::Mime(mime::TopLevel::Application,
                                           mime::SubLevel::WwwFormUrlEncoded,
                                           vec![(mime::Attr::Charset, mime::Value::Utf8)])));
            tmp
        };

        let response = self.http.post(url).headers(headers).body(&body).send()?;
        Ok(response)
    }
}
