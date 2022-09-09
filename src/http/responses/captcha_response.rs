use actix_web::{
    http::{
        header::{self, HeaderValue},
        StatusCode,
    },
    web::Bytes,
    HttpRequest, HttpResponse, Responder,
};

///验证码响应
pub struct CaptchaResponse(Bytes);
impl CaptchaResponse {
    ///构造函数
    pub fn new(data: &[u8]) -> Self {
        Self(Bytes::copy_from_slice(data))
    }
}
impl Responder for CaptchaResponse {
    type Body = Bytes;
    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut resp = HttpResponse::with_body(StatusCode::OK, self.0);
        let header = resp.headers_mut();
        header.insert(header::CONTENT_TYPE, HeaderValue::from_static("image/png"));
        header.insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("private, max-age=0, no-store, no-cache, must-revalidate"),
        );
        resp
    }
}
