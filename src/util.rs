use chrono::*;
use rand;
use rand::Rng;
use rustc_serialize::base64;
use rustc_serialize::base64::ToBase64;

const UTC_8_OFFSET: i32 = 8 * 3600;
const RANDOM_STATE_LENGTH: usize = 30;

lazy_static! {
    static ref ASIA_SHANGHAI: FixedOffset = FixedOffset::east(UTC_8_OFFSET);
}


/// Generates a timestamp string suitable for the `open.189.cn` API.
pub fn get_api_timestamp() -> String {
    let utc_time = UTC::now();
    let time = utc_time.with_timezone(&*ASIA_SHANGHAI);
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}


/// Generates a sufficiently long random string for tracking OAuth requests.
pub fn get_random_state_str() -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<_> = rng.gen_iter::<u8>().take(RANDOM_STATE_LENGTH).collect();
    b64encode(&bytes)
}


/// Base64-encodes the given buffer.
pub fn b64encode(input: &[u8]) -> String {
    input.to_base64(base64::Config {
        char_set: base64::CharacterSet::Standard,
        newline: base64::Newline::LF,
        pad: true,
        line_length: None,
    })
}
