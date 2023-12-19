#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug, Copy, Clone)]
pub struct ContentType {}
impl ContentType {

    pub const CONTENT_TYPE_TEXT_PLAIN: &'static str = "text/plain";
    pub const CONTENT_TYPE_TEXT_HTML: &'static str = "text/html";
    pub const CONTENT_TYPE_TEXT_CSS: &'static str = "text/css";
    pub const CONTENT_TYPE_APPLICATION_JAVASCRIPT: &'static str = "application/javascript";
    pub const CONTENT_TYPE_APPLICATION_JSON: &'static str = "application/json";
    pub const CONTENT_TYPE_APPLICATION_XML: &'static str = "application/xml";
    pub const CONTENT_TYPE_APPLICATION_PDF: &'static str = "application/pdf";
    pub const CONTENT_TYPE_APPLICATION_ZIP: &'static str = "application/zip";
    pub const CONTENT_TYPE_IMAGE_JPEG: &'static str = "image/jpeg";
    pub const CONTENT_TYPE_IMAGE_PNG: &'static str = "image/png";
    pub const CONTENT_TYPE_IMAGE_GIF: &'static str = "image/gif";
    pub const CONTENT_TYPE_AUDIO_MPEG: &'static str = "audio/mpeg";
    pub const CONTENT_TYPE_AUDIO_WAV: &'static str = "audio/wav";
    pub const CONTENT_TYPE_VIDEO_MP4: &'static str = "video/mp4";
    pub const CONTENT_TYPE_MULTIPART_FORM_DATA: &'static str = "multipart/form-data";
    pub const CONTENT_TYPE_FORM_URLENCODED: &'static str = "application/x-www-form-urlencoded";
    pub const CONTENT_TYPE_APPLICATION_OCTET_STREAM: &'static str = "application/octet-stream";
    pub const CONTENT_TYPE_APPLICATION_XLSX: &'static str = "application/xlsx";
    pub const CONTENT_TYPE_TEXT_CSV: &'static str = "text/csv";
    pub const CONTENT_TYPE_TEXT_XML: &'static str = "text/xml";

}