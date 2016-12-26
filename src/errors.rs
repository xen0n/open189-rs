error_chain! {
    errors {
        ApiError(http_status: ::hyper::status::StatusCode,
                 retcode: u64,
                 state: Option<String>,
                 message: Option<String>) {
            description("open.189.cn API response error")
            display("[HTTP {} res_code {} state {:?}] {:?}",
                    http_status,
                    retcode,
                    state,
                    message)
        }
    }

    foreign_links {
        IoError(::std::io::Error);
        JsonDecodeError(::serde_json::Error);
        HyperError(::hyper::Error);
        HyperParseError(::hyper::error::ParseError);
    }
}
