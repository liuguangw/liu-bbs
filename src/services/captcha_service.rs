use captcha_a::{Captcha, CaptchaBuilder, Font, ImageError};
///验证码服务
pub struct CaptchaService {
    fonts: Vec<Font<'static>>,
}
impl CaptchaService {
    ///生成验证码
    pub fn build(&self) -> Result<Captcha, ImageError> {
        let captcha = CaptchaBuilder {
            width: 250,
            height: 68,
            fonts: &self.fonts,
            //default attribute
            ..Default::default()
        };
        captcha.build()
    }
}

impl Default for CaptchaService {
    fn default() -> Self {
        let fonts = vec![
            Font::try_from_bytes(include_bytes!("../../fonts/captcha0.ttf")).unwrap(),
            Font::try_from_bytes(include_bytes!("../../fonts/captcha1.ttf")).unwrap(),
            Font::try_from_bytes(include_bytes!("../../fonts/captcha2.ttf")).unwrap(),
            Font::try_from_bytes(include_bytes!("../../fonts/captcha3.ttf")).unwrap(),
            Font::try_from_bytes(include_bytes!("../../fonts/captcha4.ttf")).unwrap(),
            Font::try_from_bytes(include_bytes!("../../fonts/captcha5.ttf")).unwrap(),
        ];
        Self { fonts }
    }
}
