use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use hyper::client::Client;
use hyper::client::Response;
use hyper::client::IntoUrl;

use super::errors::*;
use super::sig;
use super::util;


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
}
