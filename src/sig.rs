use std::collections::HashMap;
use std::hash::Hash;


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
}
