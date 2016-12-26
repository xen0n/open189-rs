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

        WrongSmsCodeLength(length_actual: usize, length_expected: usize) {
            description("wrong SMS verification code length")
            display("wrong SMS verification code length: expected {}, got {}",
                    length_expected,
                    length_actual)
        }

        NonDigitInSmsCode(code: String) {
            description("non-digit character found in SMS verification code")
            display("non-digit char in SMS verification code: {:?}", code)
        }
    }

    foreign_links {
        IoError(::std::io::Error);
        JsonDecodeError(::serde_json::Error);
        HyperError(::hyper::Error);
        HyperParseError(::hyper::error::ParseError);
    }
}
