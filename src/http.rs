/// HTTP request method for scanning
#[derive(Clone)]
pub enum RequestMethod {
    Get,
    Post,
    Head,
}

/// HTTP response status
#[derive(Clone, PartialEq)]
pub enum ResponseStatus {
    /// 100 Continue
    Continue,
    /// 101 Switching Protocols
    SwitchingProtocols,
    /// 102 Processing
    Processing,
    /// 200 OK 
    Ok,
    /// 201 Created
    Created,
    /// 202 Accepted
    Accepted,
    /// 203 Non-Authoritative Information
    NonAuthoritativeInformation,
    /// 204 No Content
    NoContent,
    /// 205 Reset Content
    ResetContent,
    /// 206 Partial Content
    PartialContent,
    /// 207 Multi-Status
    MultiStatus,
    /// 208 Already Reported
    AlreadyReported,
    /// 226 IM Used
    ImUsed,
    /// 300 Multiple Choices
    MultipleChoices,
    /// 301 Moved Permanently
    MovedPermanently,
    /// 302 Found
    Found,
    /// 303 See Other
    SeeOther,
    /// 304 Not Modified
    NotModified,
    /// 305 Use Proxy
    UseProxy,
    /// 307 Temporary Redirect
    TemporaryRedirect,
    /// 308 Permanent Redirect
    PermanentRedirect,
    /// 400 Bad Request
    BadRequest,
    /// 401 Unauthorized
    Unauthorized,
    /// 402 Payment Required
    PaymentRequired,
    /// 403 Forbidden
    Forbidden,
    /// 404 Not Found
    NotFound,
    /// 405 Method Not Allowed
    MethodNotAllowed,
    /// 406 Not Acceptable
    NotAcceptable,
    /// 407 Proxy Authentication Required
    ProxyAuthenticationRequired,
    /// 408 Request Timeout
    RequestTimeout,
    /// 409 Conflict
    Conflict,
    /// 410 Gone
    Gone,
    /// 411 Length Required
    LengthRequired,
    /// 412 Precondition Failed
    PreconditionFailed,
    /// 413 Payload Too Large
    PayloadTooLarge,
    /// 414 URI Too Long
    UriTooLong,
    /// 415 Unsupported Media Type
    UnsupportedMediaType,
    /// 416 Range Not Satisfiable
    RangeNotSatisfiable,
    /// 417 Expectation Failed
    ExpectationFailed,
    /// 418 I’m a teapot
    ImaTeapot,
    /// 421 Misdirected Request
    MisdirectedRequest,
    /// 422 Unprocessable Entity
    UnprocessableEntity,
    /// 423 Locked
    Locked,
    /// 424 Failed Dependency
    FailedDependency,
    /// 426 Upgrade Required
    UpgradeRequired,
    /// 428 Precondition Required
    PreconditionRequired,
    /// 429 Too Many Requests
    TooManyRequests,
    /// 431 Request Header Fields Too Large
    RequestHeaderFieldsTooLarge,
    /// 451 Unavailable For Legal Reasons
    UnavailableForLegalReasons,
    /// 500 Internal Server Error
    InternalServerError,
    /// 501 Not Implemented
    NotImplemented,
    /// 502 Bad Gateway
    BadGateway,
    /// 503 Service Unavailable
    ServiceUnavailable,
    /// 504 Gateway Timeout
    GatewayTimeout,
    /// 505 HTTP Version Not Supported
    HttpVersionNotSupported,
    /// 506 Variant Also Negotiates
    VariantAlsoNegotiates,
    /// 507 Insufficient Storage
    InsufficientStorage,
    /// 508 Loop Detected
    LoopDetected,
    /// 510 Not Extended
    NotExtended,
    /// 511 Network Authentication Required
    NetworkAuthenticationRequired,
}

impl ResponseStatus {
    pub fn code(&self) -> u16 {
        match self {
            ResponseStatus::Continue => 100,
            ResponseStatus::SwitchingProtocols => 101,
            ResponseStatus::Processing => 102,
            ResponseStatus::Ok => 200,
            ResponseStatus::Created => 201,
            ResponseStatus::Accepted => 202,
            ResponseStatus::NonAuthoritativeInformation => 203,
            ResponseStatus::NoContent => 204,
            ResponseStatus::ResetContent => 205,
            ResponseStatus::PartialContent => 206,
            ResponseStatus::MultiStatus => 207,
            ResponseStatus::AlreadyReported => 208,
            ResponseStatus::ImUsed => 226,
            ResponseStatus::MultipleChoices => 300,
            ResponseStatus::MovedPermanently => 301,
            ResponseStatus::Found => 302,
            ResponseStatus::SeeOther => 303,
            ResponseStatus::NotModified => 304,
            ResponseStatus::UseProxy => 305,
            ResponseStatus::TemporaryRedirect => 307,
            ResponseStatus::PermanentRedirect => 308,
            ResponseStatus::BadRequest => 400,
            ResponseStatus::Unauthorized => 401,
            ResponseStatus::PaymentRequired => 402,
            ResponseStatus::Forbidden => 403,
            ResponseStatus::NotFound => 404,
            ResponseStatus::MethodNotAllowed => 405,
            ResponseStatus::NotAcceptable => 406,
            ResponseStatus::ProxyAuthenticationRequired => 407,
            ResponseStatus::RequestTimeout => 408,
            ResponseStatus::Conflict => 409,
            ResponseStatus::Gone => 410,
            ResponseStatus::LengthRequired => 411,
            ResponseStatus::PreconditionFailed => 412,
            ResponseStatus::PayloadTooLarge => 413,
            ResponseStatus::UriTooLong => 414,
            ResponseStatus::UnsupportedMediaType => 415,
            ResponseStatus::RangeNotSatisfiable => 416,
            ResponseStatus::ExpectationFailed => 417,
            ResponseStatus::ImaTeapot => 418,
            ResponseStatus::MisdirectedRequest => 421,
            ResponseStatus::UnprocessableEntity => 422,
            ResponseStatus::Locked => 423,
            ResponseStatus::FailedDependency => 424,
            ResponseStatus::UpgradeRequired => 426,
            ResponseStatus::PreconditionRequired => 428,
            ResponseStatus::TooManyRequests => 429,
            ResponseStatus::RequestHeaderFieldsTooLarge => 431,
            ResponseStatus::UnavailableForLegalReasons => 451,
            ResponseStatus::InternalServerError => 500,
            ResponseStatus::NotImplemented => 501,
            ResponseStatus::BadGateway => 502,
            ResponseStatus::ServiceUnavailable => 503,
            ResponseStatus::GatewayTimeout => 504,
            ResponseStatus::HttpVersionNotSupported => 505,
            ResponseStatus::VariantAlsoNegotiates => 506,
            ResponseStatus::InsufficientStorage => 507,
            ResponseStatus::LoopDetected => 508,
            ResponseStatus::NotExtended => 510,
            ResponseStatus::NetworkAuthenticationRequired => 511,
        }
    }
    pub fn reason(&self) -> String {
        match self {
            ResponseStatus::Continue => String::from("Continue"),
            ResponseStatus::SwitchingProtocols => String::from("Switching Protocols"),
            ResponseStatus::Processing => String::from("Processing"),
            ResponseStatus::Ok => String::from("OK"),
            ResponseStatus::Created => String::from("Created"),
            ResponseStatus::Accepted => String::from("Accepted"),
            ResponseStatus::NonAuthoritativeInformation => String::from("Non-Authoritative Information"),
            ResponseStatus::NoContent => String::from("No Content"),
            ResponseStatus::ResetContent => String::from("Reset Content"),
            ResponseStatus::PartialContent => String::from("Partial Content"),
            ResponseStatus::MultiStatus => String::from("Multi-Status"),
            ResponseStatus::AlreadyReported => String::from("Already Reported"),
            ResponseStatus::ImUsed => String::from("IM Used"),
            ResponseStatus::MultipleChoices => String::from("Multiple Choices"),
            ResponseStatus::MovedPermanently => String::from("Moved Permanently"),
            ResponseStatus::Found => String::from("Found"),
            ResponseStatus::SeeOther => String::from("See Other"),
            ResponseStatus::NotModified => String::from("Not Modified"),
            ResponseStatus::UseProxy => String::from("Use Proxy"),
            ResponseStatus::TemporaryRedirect => String::from("Temporary Redirect"),
            ResponseStatus::PermanentRedirect => String::from("Permanent Redirect"),
            ResponseStatus::BadRequest => String::from("Bad Request"),
            ResponseStatus::Unauthorized => String::from("Unauthorized"),
            ResponseStatus::PaymentRequired => String::from("Payment Required"),
            ResponseStatus::Forbidden => String::from("Forbidden"),
            ResponseStatus::NotFound => String::from("Not Found"),
            ResponseStatus::MethodNotAllowed => String::from("Method Not Allowed"),
            ResponseStatus::NotAcceptable => String::from("Not Acceptable"),
            ResponseStatus::ProxyAuthenticationRequired => String::from("Proxy Authentication Required"),
            ResponseStatus::RequestTimeout => String::from("Request Timeout"),
            ResponseStatus::Conflict => String::from("Conflict"),
            ResponseStatus::Gone => String::from("Gone"),
            ResponseStatus::LengthRequired => String::from("Length Required"),
            ResponseStatus::PreconditionFailed => String::from("Precondition Failed"),
            ResponseStatus::PayloadTooLarge => String::from("Payload Too Large"),
            ResponseStatus::UriTooLong => String::from("URI Too Long"),
            ResponseStatus::UnsupportedMediaType => String::from("Unsupported Media Type"),
            ResponseStatus::RangeNotSatisfiable => String::from("Range Not Satisfiable"),
            ResponseStatus::ExpectationFailed => String::from("Expectation Failed"),
            ResponseStatus::ImaTeapot => String::from("I'm a teapot"),
            ResponseStatus::MisdirectedRequest => String::from("Misdirected Request"),
            ResponseStatus::UnprocessableEntity => String::from("Unprocessable Entity"),
            ResponseStatus::Locked => String::from("Locked"),
            ResponseStatus::FailedDependency => String::from("Failed Dependency"),
            ResponseStatus::UpgradeRequired => String::from("Upgrade Required"),
            ResponseStatus::PreconditionRequired => String::from("Precondition Required"),
            ResponseStatus::TooManyRequests => String::from("Too Many Requests"),
            ResponseStatus::RequestHeaderFieldsTooLarge => String::from("Request Header Fields Too Large"),
            ResponseStatus::UnavailableForLegalReasons => String::from("Unavailable For Legal Reasons"),
            ResponseStatus::InternalServerError => String::from("Internal Server Error "),
            ResponseStatus::NotImplemented => String::from("Not Implemented"),
            ResponseStatus::BadGateway => String::from("Bad Gateway"),
            ResponseStatus::ServiceUnavailable => String::from("Service Unavailable"),
            ResponseStatus::GatewayTimeout => String::from("Gateway Timeout"),
            ResponseStatus::HttpVersionNotSupported => String::from("HTTP Version Not Supported"),
            ResponseStatus::VariantAlsoNegotiates => String::from("Variant Also Negotiates"),
            ResponseStatus::InsufficientStorage => String::from("Insufficient Storage"),
            ResponseStatus::LoopDetected => String::from("Loop Detected"),
            ResponseStatus::NotExtended => String::from("Not Extended"),
            ResponseStatus::NetworkAuthenticationRequired => String::from("Network Authentication Required"),
        }
    }
    pub fn from_code(code: u16) -> ResponseStatus {
        match code {
            100 => ResponseStatus::Continue,
            101 => ResponseStatus::SwitchingProtocols,
            102 => ResponseStatus::Processing,
            200 => ResponseStatus::Ok,
            201 => ResponseStatus::Created,
            202 => ResponseStatus::Accepted,
            203 => ResponseStatus::NonAuthoritativeInformation,
            204 => ResponseStatus::NoContent,
            205 => ResponseStatus::ResetContent,
            206 => ResponseStatus::PartialContent,
            207 => ResponseStatus::MultiStatus,
            208 => ResponseStatus::AlreadyReported,
            226 => ResponseStatus::ImUsed,
            300 => ResponseStatus::MultipleChoices,
            301 => ResponseStatus::MovedPermanently,
            302 => ResponseStatus::Found,
            303 => ResponseStatus::SeeOther,
            304 => ResponseStatus::NotModified,
            305 => ResponseStatus::UseProxy,
            307 => ResponseStatus::TemporaryRedirect,
            308 => ResponseStatus::PermanentRedirect,
            400 => ResponseStatus::BadRequest,
            401 => ResponseStatus::Unauthorized,
            402 => ResponseStatus::PaymentRequired,
            403 => ResponseStatus::Forbidden,
            404 => ResponseStatus::NotFound,
            405 => ResponseStatus::MethodNotAllowed,
            406 => ResponseStatus::NotAcceptable,
            407 => ResponseStatus::ProxyAuthenticationRequired,
            408 => ResponseStatus::RequestTimeout,
            409 => ResponseStatus::Conflict,
            410 => ResponseStatus::Gone,
            411 => ResponseStatus::LengthRequired,
            412 => ResponseStatus::PreconditionFailed,
            413 => ResponseStatus::PayloadTooLarge,
            414 => ResponseStatus::UriTooLong,
            415 => ResponseStatus::UnsupportedMediaType,
            416 => ResponseStatus::RangeNotSatisfiable,
            417 => ResponseStatus::ExpectationFailed,
            418 => ResponseStatus::ImaTeapot,
            421 => ResponseStatus::MisdirectedRequest,
            422 => ResponseStatus::UnprocessableEntity,
            423 => ResponseStatus::Locked,
            424 => ResponseStatus::FailedDependency,
            426 => ResponseStatus::UpgradeRequired,
            428 => ResponseStatus::PreconditionRequired,
            429 => ResponseStatus::TooManyRequests,
            431 => ResponseStatus::RequestHeaderFieldsTooLarge,
            451 => ResponseStatus::UnavailableForLegalReasons,
            500 => ResponseStatus::InternalServerError,
            501 => ResponseStatus::NotImplemented,
            502 => ResponseStatus::BadGateway,
            503 => ResponseStatus::ServiceUnavailable,
            504 => ResponseStatus::GatewayTimeout,
            505 => ResponseStatus::HttpVersionNotSupported,
            506 => ResponseStatus::VariantAlsoNegotiates,
            507 => ResponseStatus::InsufficientStorage,
            508 => ResponseStatus::LoopDetected,
            510 => ResponseStatus::NotExtended,
            511 => ResponseStatus::NetworkAuthenticationRequired,
            _ => ResponseStatus::ImaTeapot,
        }
    }
    pub fn is_information(&self) -> bool {
        if self.code() >= 100 && self.code() <= 199 {
            true
        }else{
            false
        }
    }
    pub fn is_success(&self) -> bool {
        if self.code() >= 200 && self.code() <= 299 {
            true
        }else{
            false
        }
    }
    pub fn is_redirection(&self) -> bool {
        if self.code() >= 300 && self.code() <= 399 {
            true
        }else{
            false
        }
    }
    pub fn is_client_error(&self) -> bool {
        if self.code() >= 400 && self.code() <= 499 {
            true
        }else{
            false
        }
    }
    pub fn is_server_error(&self) -> bool {
        if self.code() >= 500 && self.code() <= 599 {
            true
        }else{
            false
        }
    }
}
