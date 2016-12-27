use std::collections::HashMap;
use std::hash::Hash;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;

use super::util;


fn transform_payload<K, V>(d: &HashMap<K, V>) -> String
    where K: AsRef<str> + Eq + Hash,
          V: AsRef<str> + Eq + Hash
{
    let mut kv_list: Vec<_> = d.iter().map(|(k, v)| (k.as_ref(), v.as_ref())).collect();
    kv_list.sort_by(|a, b| a.0.cmp(b.0));
    let mut result = String::new();
    for (i, (k, v)) in kv_list.into_iter().enumerate() {
        if i > 0 {
            result.push('&');
        }
        result.push_str(k);
        result.push('=');
        result.push_str(v);
    }
    result
}


pub fn sign<K, V, S>(d: &HashMap<K, V>, secret: S) -> String
    where K: AsRef<str> + Eq + Hash,
          V: AsRef<str> + Eq + Hash,
          S: AsRef<str>
{
    let payload = transform_payload(d);

    let mut hmac = Hmac::new(Sha1::new(), secret.as_ref().as_bytes());
    hmac.input(payload.as_bytes());

    util::b64encode(hmac.result().code())
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_transform_payload() {
        let x = {
            let mut tmp: HashMap<&str, String> = HashMap::new();
            tmp.insert("k2", "v2".to_string());
            tmp.insert("k3", "v3".to_string());
            tmp.insert("k1", "v1".to_string());
            tmp
        };
        assert_eq!(transform_payload(&x), "k1=v1&k2=v2&k3=v3");
    }


    #[test]
    fn test_sign() {
        let x = {
            let mut tmp: HashMap<&str, String> = HashMap::new();
            tmp.insert("k2", "v2".to_string());
            tmp.insert("k3", "v3".to_string());
            tmp.insert("k1", "v1".to_string());
            tmp
        };
        assert_eq!(sign(&x, "012345"), "iAKpGb9i8EKY8q4HPfiMdfb27OM=");
    }
}
