use chrono::*;

const UTC_8_OFFSET: i32 = 8 * 3600;

lazy_static! {
    static ref ASIA_SHANGHAI: FixedOffset = FixedOffset::east(UTC_8_OFFSET);
}


/// Generates a timestamp string suitable for the `open.189.cn` API.
pub fn get_api_timestamp() -> String {
    let utc_time = UTC::now();
    let time = utc_time.with_timezone(&*ASIA_SHANGHAI);
    time.format("%Y-%m-%d %H:%M:%S").to_string()
}
