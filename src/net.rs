use std::collections::HashMap;
use std::hash::Hash;
use std::io::Read;
use std::sync::Arc;

use hyper::client::Client;
use hyper::client::Response;
use hyper::client::IntoUrl;
use hyper::header::ContentType;
use hyper::header::Headers;
use hyper::mime;

use serde::Deserialize;

use url::form_urlencoded;
use url::form_urlencoded::Target;

use super::errors::*;
use super::resp::IntoResult;
use super::sig;
use super::util;

const URL_ACCESS_TOKEN_REQ: &'static str = "https://oauth.api.189.cn/emp/oauth2/v3/access_token";


pub struct Open189Client {
    http: Arc<Client>,
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


fn process_response<T>(mut response: Response) -> Result<T::Item>
    where T: Deserialize + IntoResult
{
    let mut response_str = String::new();
    response.read_to_string(&mut response_str)?;
    let obj: T = ::serde_json::from_str(&response_str)?;
    obj.into_result(response.status)
}


impl Open189Client {
    pub fn new(http_client: Client) -> Open189Client {
        Open189Client { http: Arc::new(http_client) }
    }

    pub fn get_sync<U, S, T>(&self,
                             app_id: S,
                             secret: S,
                             access_token: S,
                             url: U,
                             mut params: HashMap<&'static str, String>)
                             -> Result<T::Item>
        where U: IntoUrl,
              S: AsRef<str>,
              T: Deserialize + IntoResult
    {
        let mut url = url.into_url()?;
        prepare_request_params(&mut params,
                               app_id.as_ref(),
                               secret.as_ref(),
                               access_token.as_ref());
        {
            let mut qs = url.query_pairs_mut();
            qs.clear();
            for (k, v) in params.iter() {
                qs.append_pair(k.as_ref(), v.as_ref());
            }
        }

        let response = self.http.get(url).send()?;
        process_response::<T>(response)
    }

    pub fn post_sync<U, S, T>(&self,
                              app_id: S,
                              secret: S,
                              access_token: S,
                              url: U,
                              mut params: HashMap<&'static str, String>)
                              -> Result<T::Item>
        where U: IntoUrl,
              S: AsRef<str>,
              T: Deserialize + IntoResult
    {
        prepare_request_params(&mut params,
                               app_id.as_ref(),
                               secret.as_ref(),
                               access_token.as_ref());
        self.post_sync_prepared::<U, T>(url, params)
    }

    pub fn perform_access_token_req<S, T>(&self,
                                          app_id: S,
                                          secret: S,
                                          mut params: HashMap<&'static str, String>)
                                          -> Result<T::Item>
        where S: AsRef<str>,
              T: Deserialize + IntoResult
    {
        params.insert("app_id", app_id.as_ref().to_string());
        params.insert("app_secret", secret.as_ref().to_string());
        params.insert("state", util::get_random_state_str());
        self.post_sync_prepared::<_, T>(URL_ACCESS_TOKEN_REQ, params)
    }

    fn post_sync_prepared<U, T>(&self,
                                url: U,
                                params: HashMap<&'static str, String>)
                                -> Result<T::Item>
        where U: IntoUrl,
              T: Deserialize + IntoResult
    {
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
        process_response::<T>(response)
    }
}
