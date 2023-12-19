#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug, Copy, Clone)]
pub struct StatusCode {}
impl StatusCode {

    pub const HTTP_OK: &'static str = "HTTP/1.1 200 Ok";
    pub const HTTP_CREATED: &'static str = "HTTP/1.1 201 Created";
    pub const HTTP_ACCEPTED: &'static str = "HTTP/1.1 202 Accepted";
    pub const HTTP_NON_AUTH_INFORMATION: &'static str = "HTTP/1.1 203 Non-Authoritative Information";
    pub const HTTP_NO_CONTENT: &'static str = "HTTP/1.1 204 No Content";
    pub const HTTP_RESET_CONTENT: &'static str = "HTTP/1.1 205 Reset Content";
    pub const HTTP_PARTIAL_CONTENT: &'static str = "HTTP/1.1 206 Partial Content";
    pub const HTTP_MULTI_STATUS: &'static str = "HTTP/1.1 207 Multi Status";
    pub const HTTP_ALREADY_REPORTED: &'static str = "HTTP/1.1 208 Already Reported";
    pub const HTTP_IM_USED: &'static str = "HTTP/1.1 226 IM Used";

    pub const HTTP_MULTIPLE_CHOICES: &'static str = "HTTP/1.1 300 Multiple Choices";
    pub const HTTP_MOVED_PERMANENTLY: &'static str = "HTTP/1.1 301 Moved Permanently";
    pub const HTTP_FOUND: &'static str = "HTTP/1.1 302 Found";
    pub const HTTP_SEE_OTHER: &'static str = "HTTP/1.1 303 See Other";
    pub const HTTP_NOT_MODIFIED: &'static str = "HTTP/1.1 304 Not Modified";
    pub const HTTP_USE_PROXY: &'static str = "HTTP/1.1 305 Use Proxy";
    pub const HTTP_UNUSED: &'static str = "HTTP/1.1 306 Unused";
    pub const HTTP_TEMPORARY_REDIRECT: &'static str = "HTTP/1.1 307 Temporary Redirect";
    pub const HTTP_PERMANENT_REDIRECT: &'static str = "HTTP/1.1 308 Permanent Redirect";

    pub const HTTP_BAD_REQUEST: &'static str = "HTTP/1.1 400 Bad Request";
    pub const HTTP_UNAUTHORIZED: &'static str = "HTTP/1.1 401 Unauthorized";
    pub const HTTP_PAYMENT_REQUIRED: &'static str = "HTTP/1.1 402 Payment Required";
    pub const HTTP_FORBIDDEN: &'static str = "HTTP/1.1 403 Forbidden";
    pub const HTTP_NOT_FOUND: &'static str = "HTTP/1.1 404 Not Found";
    pub const HTTP_METHOD_NOT_ALLOWED: &'static str = "HTTP/1.1 405 Method Not Allowed";
    pub const HTTP_NOT_ACCEPTABLE: &'static str = "HTTP/1.1 406 Not Acceptable";
    pub const HTTP_PROXY_AUTH_REQUIRED: &'static str = "HTTP/1.1 407 Proxy Authentication Required";
    pub const HTTP_REQUEST_TIMEOUT: &'static str = "HTTP/1.1 408 Request Timeout";
    pub const HTTP_CONFLICT: &'static str = "HTTP/1.1 409 Conflict";
    pub const HTTP_GONE: &'static str = "HTTP/1.1 410 Gone";
    pub const HTTP_LENGTH_REQUIRED: &'static str = "HTTP/1.1 411 Length Required";
    pub const HTTP_PRECONDITION_FAILED: &'static str = "HTTP/1.1 412 Precondition Failed";
    pub const HTTP_PAYLOAD_TOO_LARGE: &'static str = "HTTP/1.1 413 Payload Too Large";
    pub const HTTP_URI_TOO_LONG: &'static str = "HTTP/1.1 414 URI Too Long";
    pub const HTTP_UNSUPPORTED_MEDIA_TYPE: &'static str = "HTTP/1.1 415 Unsupported Media Type";
    pub const HTTP_RANGE_NOT_SATISFIABLE: &'static str = "HTTP/1.1 416 Range Not Satisfiable";
    pub const HTTP_EXPECTATION_FAILED: &'static str = "HTTP/1.1 417 Expectation Failed";
    pub const HTTP_IM_A_TEAPOT: &'static str = "HTTP/1.1 418 I'm a teapot";
    pub const HTTP_MISDIRECTED_REQUEST: &'static str = "HTTP/1.1 421 Misdirected Request";
    pub const HTTP_UNPROCESSABLE_CONTENT: &'static str = "HTTP/1.1 422 Unprocessable Content";
    pub const HTTP_LOCKED: &'static str = "HTTP/1.1 423 Locked";
    pub const HTTP_FAILED_DEPENDENCY: &'static str = "HTTP/1.1 424 Failed Dependency";
    pub const HTTP_TOO_EARLY: &'static str = "HTTP/1.1 425 Too Early";
    pub const HTTP_UPGRADE_REQUIRED: &'static str = "HTTP/1.1 426 Upgrade Required";
    pub const HTTP_PRECONDITION_REQUIRED: &'static str = "HTTP/1.1 428 Precondition Required";
    pub const HTTP_TOO_MANY_REQUESTS: &'static str = "HTTP/1.1 429 Too Many Requests";
    pub const HTTP_HEADER_FIELD_TOO_LARGE: &'static str = "HTTP/1.1 431 Request Header Fields Too Large";
    pub const HTTP_UNAVAILABLE_FOR_LEGAL_REASONS: &'static str =
        "HTTP/1.1 451 Unavailable For Legal Reasons";

    pub const HTTP_INTERNAL_SERVER_ERROR: &'static str = "HTTP/1.1 500 Internal Server Error";
    pub const HTTP_NOT_IMPLEMENTED: &'static str = "HTTP/1.1 501 Not Implemented";
    pub const HTTP_BAD_GATEWAY: &'static str = "HTTP/1.1 502 Bad Gateway";
    pub const HTTP_SERVICE_UNAVAILABLE: &'static str = "HTTP/1.1 503 Service Unavailable";
    pub const HTTP_GATEWAY_TIMEOUT: &'static str = "HTTP/1.1 504 Gateway Timeout";
    pub const HTTP_VERSION_NOT_SUPPORTED: &'static str = "HTTP/1.1 505 HTTP Version Not Supported";
    pub const HTTP_VARIANT_ALSO_NEGOTIATES: &'static str = "HTTP/1.1 506 Variant Also Negotiates";
    pub const HTTP_INSUFFICENT_STORAGE: &'static str = "HTTP/1.1 507 Insufficient Storage";
    pub const HTTP_LOOP_DETECTED: &'static str = "HTTP/1.1 508 Loop Detected";
    pub const HTTP_NOT_EXTENDED: &'static str = "HTTP/1.1 510 Not Extended";
    pub const HTTP_NETWORK_AUTH_REQUIRED: &'static str = "HTTP/1.1 511 Network Authentication Required";

}
