use sled::IVec;
use std::fmt;

#[derive(Clone)]
pub struct BangUrl<'a>(Vec<&'a [u8]>);

// get default just appends to url
const DEFAULT_URL: &[u8] = b"https://www.google.com/search?q=";

#[derive(Debug)]
pub struct InvalidBangUrl;

impl fmt::Display for InvalidBangUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Null character found")
    }
}

impl std::error::Error for InvalidBangUrl {}

impl<'a> BangUrl<'a> {
    pub fn decode(bytes: &'a [u8]) -> Self {
        Self(bytes.split(|&b| b == 0).collect())
    }

    pub fn encode(&self) -> Vec<u8> {
        self.0.join(&0)
    }

    pub fn parse(s: &'a str) -> Result<Self, InvalidBangUrl> {
        if s.as_bytes().contains(&0) {
            return Err(InvalidBangUrl);
        }
        Ok(Self(
            s.split("{{{s}}}").map(|part| part.as_bytes()).collect(),
        ))
    }

    fn strs(&self) -> &[&'a str] {
        unsafe { std::mem::transmute(self.0.as_slice()) }
    }
}

impl<'a> From<&'a IVec> for BangUrl<'a> {
    fn from(ivec: &'a IVec) -> Self {
        Self::decode(ivec.as_ref())
    }
}

impl<'a> From<BangUrl<'a>> for IVec {
    fn from(url: BangUrl<'a>) -> IVec {
        url.encode().into()
    }
}

impl<'a> fmt::Display for BangUrl<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.strs().join("{{{s}}}"))
    }
}

impl<'a> fmt::Debug for BangUrl<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BangUrl").field(&self.to_string()).finish()
    }
}
