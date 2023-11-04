#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug, Copy, Clone)]
pub struct ContentType {}
impl ContentType {

    pub const CONTENT_TYPE_TEXT_PLAIN: &str = "text/plain";
    pub const CONTENT_TYPE_TEXT_HTML: &str = "text/html";
    pub const CONTENT_TYPE_TEXT_CSS: &str = "text/css";
    pub const CONTENT_TYPE_APPLICATION_JAVASCRIPT: &str = "application/javascript";
    pub const CONTENT_TYPE_APPLICATION_JSON: &str = "application/json";
    pub const CONTENT_TYPE_APPLICATION_XML: &str = "application/xml";
    pub const CONTENT_TYPE_APPLICATION_PDF: &str = "application/pdf";
    pub const CONTENT_TYPE_APPLICATION_ZIP: &str = "application/zip";
    pub const CONTENT_TYPE_IMAGE_JPEG: &str = "image/jpeg";
    pub const CONTENT_TYPE_IMAGE_PNG: &str = "image/png";
    pub const CONTENT_TYPE_IMAGE_GIF: &str = "image/gif";
    pub const CONTENT_TYPE_AUDIO_MPEG: &str = "audio/mpeg";
    pub const CONTENT_TYPE_AUDIO_WAV: &str = "audio/wav";
    pub const CONTENT_TYPE_VIDEO_MP4: &str = "video/mp4";
    pub const CONTENT_TYPE_MULTIPART_FORM_DATA: &str = "multipart/form-data";
    pub const CONTENT_TYPE_FORM_URLENCODED: &str = "application/x-www-form-urlencoded";
    pub const CONTENT_TYPE_APPLICATION_OCTET_STREAM: &str = "application/octet-stream";
    pub const CONTENT_TYPE_APPLICATION_XLSX: &str = "application/xlsx";
    pub const CONTENT_TYPE_TEXT_CSV: &str = "text/csv";
    pub const CONTENT_TYPE_TEXT_XML: &str = "text/xml";

}