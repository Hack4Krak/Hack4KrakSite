use lettre::message::header::{Header, HeaderName, HeaderValue};

// Can be removed when lettre supports raw headers
// https://github.com/lettre/lettre/issues/661
#[derive(Clone, Debug)]
pub struct RawToHeader<'a>(pub &'a str);

impl<'a> Header for RawToHeader<'a> {
    fn name() -> HeaderName {
        HeaderName::new_from_ascii_str("To")
    }

    fn parse(_: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        unimplemented!()
    }

    fn display(&self) -> HeaderValue {
        HeaderValue::new(Self::name(), self.0.into())
    }
}
