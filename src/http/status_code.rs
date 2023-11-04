#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug, Copy, Clone)]
pub struct StatusCode {}
impl StatusCode {

    pub const HTTP_OK: &str = "HTTP/1.1 200 Ok";
    pub const HTTP_CREATED: &str = "HTTP/1.1 201 Created";
    pub const HTTP_ACCEPTED: &str = "HTTP/1.1 202 Accepted";
    pub const HTTP_NON_AUTH_INFORMATION: &str = "HTTP/1.1 203 Non-Authoritative Information";
    pub const HTTP_NO_CONTENT: &str = "HTTP/1.1 204 No Content";
    pub const HTTP_RESET_CONTENT: &str = "HTTP/1.1 205 Reset Content";
    pub const HTTP_PARTIAL_CONTENT: &str = "HTTP/1.1 206 Partial Content";
    pub const HTTP_MULTI_STATUS: &str = "HTTP/1.1 207 Multi Status";
    pub const HTTP_ALREADY_REPORTED: &str = "HTTP/1.1 208 Already Reported";
    pub const HTTP_IM_USED: &str = "HTTP/1.1 226 IM Used";

    pub const HTTP_MULTIPLE_CHOICES: &str = "HTTP/1.1 300 Multiple Choices";
    pub const HTTP_MOVED_PERMANENTLY: &str = "HTTP/1.1 301 Moved Permanently";
    pub const HTTP_FOUND: &str = "HTTP/1.1 302 Found";
    pub const HTTP_SEE_OTHER: &str = "HTTP/1.1 303 See Other";
    pub const HTTP_NOT_MODIFIED: &str = "HTTP/1.1 304 Not Modified";
    pub const HTTP_USE_PROXY: &str = "HTTP/1.1 305 Use Proxy";
    pub const HTTP_UNUSED: &str = "HTTP/1.1 306 Unused";
    pub const HTTP_TEMPORARY_REDIRECT: &str = "HTTP/1.1 307 Temporary Redirect";
    pub const HTTP_PERMANENT_REDIRECT: &str = "HTTP/1.1 308 Permanent Redirect";

    pub const HTTP_BAD_REQUEST: &str = "HTTP/1.1 400 Bad Request";
    pub const HTTP_UNAUTHORIZED: &str = "HTTP/1.1 401 Unauthorized";
    pub const HTTP_PAYMENT_REQUIRED: &str = "HTTP/1.1 402 Payment Required";
    pub const HTTP_FORBIDDEN: &str = "HTTP/1.1 403 Forbidden";
    pub const HTTP_NOT_FOUND: &str = "HTTP/1.1 404 Not Found";
    pub const HTTP_METHOD_NOT_ALLOWED: &str = "HTTP/1.1 405 Method Not Allowed";
    pub const HTTP_NOT_ACCEPTABLE: &str = "HTTP/1.1 406 Not Acceptable";
    pub const HTTP_PROXY_AUTH_REQUIRED: &str = "HTTP/1.1 407 Proxy Authentication Required";
    pub const HTTP_REQUEST_TIMEOUT: &str = "HTTP/1.1 408 Request Timeout";
    pub const HTTP_CONFLICT: &str = "HTTP/1.1 409 Conflict";
    pub const HTTP_GONE: &str = "HTTP/1.1 410 Gone";
    pub const HTTP_LENGTH_REQUIRED: &str = "HTTP/1.1 411 Length Required";
    pub const HTTP_PRECONDITION_FAILED: &str = "HTTP/1.1 412 Precondition Failed";
    pub const HTTP_PAYLOAD_TOO_LARGE: &str = "HTTP/1.1 413 Payload Too Large";
    pub const HTTP_URI_TOO_LONG: &str = "HTTP/1.1 414 URI Too Long";
    pub const HTTP_UNSUPPORTED_MEDIA_TYPE: &str = "HTTP/1.1 415 Unsupported Media Type";
    pub const HTTP_RANGE_NOT_SATISFIABLE: &str = "HTTP/1.1 416 Range Not Satisfiable";
    pub const HTTP_EXPECTATION_FAILED: &str = "HTTP/1.1 417 Expectation Failed";
    pub const HTTP_IM_A_TEAPOT: &str = "HTTP/1.1 418 I'm a teapot";
    pub const HTTP_MISDIRECTED_REQUEST: &str = "HTTP/1.1 421 Misdirected Request";
    pub const HTTP_UNPROCESSABLE_CONTENT: &str = "HTTP/1.1 422 Unprocessable Content";
    pub const HTTP_LOCKED: &str = "HTTP/1.1 423 Locked";
    pub const HTTP_FAILED_DEPENDENCY: &str = "HTTP/1.1 424 Failed Dependency";
    pub const HTTP_TOO_EARLY: &str = "HTTP/1.1 425 Too Early";
    pub const HTTP_UPGRADE_REQUIRED: &str = "HTTP/1.1 426 Upgrade Required";
    pub const HTTP_PRECONDITION_REQUIRED: &str = "HTTP/1.1 428 Precondition Required";
    pub const HTTP_TOO_MANY_REQUESTS: &str = "HTTP/1.1 429 Too Many Requests";
    pub const HTTP_HEADER_FIELD_TOO_LARGE: &str = "HTTP/1.1 431 Request Header Fields Too Large";
    pub const HTTP_UNAVAILABLE_FOR_LEGAL_REASONS: &str =
        "HTTP/1.1 451 Unavailable For Legal Reasons";

    pub const HTTP_INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 Internal Server Error";
    pub const HTTP_NOT_IMPLEMENTED: &str = "HTTP/1.1 501 Not Implemented";
    pub const HTTP_BAD_GATEWAY: &str = "HTTP/1.1 502 Bad Gateway";
    pub const HTTP_SERVICE_UNAVAILABLE: &str = "HTTP/1.1 503 Service Unavailable";
    pub const HTTP_GATEWAY_TIMEOUT: &str = "HTTP/1.1 504 Gateway Timeout";
    pub const HTTP_VERSION_NOT_SUPPORTED: &str = "HTTP/1.1 505 HTTP Version Not Supported";
    pub const HTTP_VARIANT_ALSO_NEGOTIATES: &str = "HTTP/1.1 506 Variant Also Negotiates";
    pub const HTTP_INSUFFICENT_STORAGE: &str = "HTTP/1.1 507 Insufficient Storage";
    pub const HTTP_LOOP_DETECTED: &str = "HTTP/1.1 508 Loop Detected";
    pub const HTTP_NOT_EXTENDED: &str = "HTTP/1.1 510 Not Extended";
    pub const HTTP_NETWORK_AUTH_REQUIRED: &str = "HTTP/1.1 511 Network Authentication Required";

}