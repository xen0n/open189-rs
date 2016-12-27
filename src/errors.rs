//! [`Result`] and [`Error`] types for this crate, powered by `error-chain`.
//!
//! [`Result`]: type.Result.html
//! [`Error`]: struct.Error.html


error_chain! {
    errors {
        /// API returned error.
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

        /// Validation error: the verification code provided is of the wrong length.
        WrongSmsCodeLength(length_actual: usize, length_expected: usize) {
            description("wrong SMS verification code length")
            display("wrong SMS verification code length: expected {}, got {}",
                    length_expected,
                    length_actual)
        }

        /// Validation error: the verification code provided has non-digit characters
        /// in it.
        NonDigitInSmsCode(code: String) {
            description("non-digit character found in SMS verification code")
            display("non-digit char in SMS verification code: {:?}", code)
        }
    }

    foreign_links {
        IoError(::std::io::Error) #[doc="I/O error."];
        JsonDecodeError(::serde_json::Error) #[doc="JSON decoding error."];
        HyperError(::hyper::Error) #[doc="Hyper library error."];
        HyperParseError(::hyper::error::ParseError) #[doc="Hyper parsing error."];
    }
}
