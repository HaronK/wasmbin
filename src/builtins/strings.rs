use super::blob::RawBlob;
use crate::{DecodeError, WasmbinDecode, WasmbinEncode};

impl WasmbinEncode for str {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        RawBlob { contents: self }.encode(w)
    }
}

impl WasmbinEncode for String {
    fn encode(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
        self.as_str().encode(w)
    }
}

impl WasmbinDecode for String {
    fn decode(r: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        Ok(String::from_utf8(RawBlob::decode(r)?.contents)?)
    }
}