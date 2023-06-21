use crate::decode::Decoder;
use crate::encode::Encoder;
use crate::{decode, encode, Charset};

/// Error occurs when serializing/deserializing HeaderValue.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HeaderValue cannot handle strings that contain unprintable characters: {0}")]
    UnprintableCharacters(String),

    #[error("{0}")]
    FailedEncode(#[from] encode::Error),

    #[error("{0}")]
    FailedDecode(#[from] decode::Error),
}

type Result<T> = std::result::Result<T, Error>;

/// HeaderValue is the value of the SSTP header field.
///
/// The value is held in a byte string of the character set and encoding that can be specified in Charset.
/// Since it is not possible to determine which SSTP header fields are allowed to contain multibyte characters
/// and which actually contain multibyte characters,
/// users should specify them explicitly when retrieving them.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct HeaderValue(Vec<u8>);
impl HeaderValue {
    /// Extract HeaderValue as an ASCII code string.
    ///
    /// ```rust
    /// # use sstp::{HeaderValue};
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// assert_eq!(HeaderValue::from_static("sakura")?.text()?, "sakura");
    /// #     Ok(())
    /// # }
    /// ```
    pub fn text(&self) -> Result<String> {
        self.text_with_charset(Charset::ASCII)
    }

    /// Extract HeaderValue as a string with Charset
    /// ```rust
    /// # use sstp::{HeaderValue, Charset};
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// let input = [130, 179, 130, 173, 130, 231].to_vec();
    /// assert_eq!(HeaderValue::from(input).text_with_charset(Charset::SHIFT_JIS)?, "さくら");
    /// #     Ok(())
    /// # }
    /// ```
    pub fn text_with_charset(&self, charset: Charset) -> Result<String> {
        let text = match charset {
            Charset::ASCII => Decoder::decode_ascii(&self.0)?,
            Charset::SHIFT_JIS => Decoder::decode_sjis(&self.0)?,
            Charset::ISO2022JP => Decoder::decode_iso_2022_jp(&self.0)?,
            Charset::EUC_JP => Decoder::decode_euc_jp(&self.0)?,
            Charset::UTF8 => Decoder::decode_utf8(&self.0)?,
        };

        if text.chars().any(|c| c.is_ascii_control()) {
            return Err(Error::UnprintableCharacters(text));
        }

        Ok(text)
    }

    ///　Convert string to HeaderValue with ASCII code bytes.
    ///
    /// ```rust
    /// # use sstp::{HeaderValue};
    /// assert_eq!(HeaderValue::from_static("sakura").unwrap().as_bytes(), b"sakura");
    /// ```
    pub fn from_static(s: &str) -> Result<Self> {
        Self::from_static_with_charset(s, Charset::ASCII)
    }

    ///　Convert string to HeaderValue with Charset.
    ///
    ///```rust
    /// # use sstp::{Charset, HeaderValue};
    /// assert_eq!(
    ///    HeaderValue::from_static_with_charset("さくら", Charset::SHIFT_JIS).unwrap().as_bytes(),
    ///    [130, 179, 130, 173, 130, 231]);
    /// ```
    pub fn from_static_with_charset(s: &str, charset: Charset) -> Result<Self> {
        if s.chars().any(|c| c.is_ascii_control()) {
            return Err(Error::UnprintableCharacters(s.to_string()));
        }

        let bytes = match charset {
            Charset::ASCII => Encoder::encode_ascii(s)?,
            Charset::SHIFT_JIS => Encoder::encode_sjis(s)?,
            Charset::ISO2022JP => Encoder::encode_iso_2022_jp(s)?,
            Charset::EUC_JP => Encoder::encode_euc_jp(s)?,
            Charset::UTF8 => Encoder::encode_utf8(s)?,
        };
        Ok(Self(bytes))
    }

    /// Convert HeaderValue to bytes.
    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl From<&[u8]> for HeaderValue {
    fn from(s: &[u8]) -> Self {
        Self(s.to_vec())
    }
}

impl From<Vec<u8>> for HeaderValue {
    fn from(s: Vec<u8>) -> Self {
        Self(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rstest::rstest;

    #[test]
    pub fn test_from_static_with_charset_pass_ascii() -> Result<()> {
        let header_value = HeaderValue::from_static_with_charset("sakura", Charset::ASCII)?;
        assert_eq!(header_value.as_bytes(), b"sakura");
        Ok(())
    }

    #[test]
    pub fn test_from_static_with_charset_pass_sjis() -> Result<()> {
        let header_value = HeaderValue::from_static_with_charset("さくら", Charset::SHIFT_JIS)?;
        assert_eq!(header_value.as_bytes(), [130, 179, 130, 173, 130, 231]);
        Ok(())
    }

    #[test]
    pub fn test_from_static_with_charset_pass_iso2022jp() -> Result<()> {
        let header_value = HeaderValue::from_static_with_charset("さくら", Charset::ISO2022JP)?;
        assert_eq!(
            header_value.as_bytes(),
            [27, 36, 66, 36, 53, 36, 47, 36, 105, 27, 40, 66]
        );
        Ok(())
    }

    #[test]
    pub fn test_from_static_with_charset_pass_eucjp() -> Result<()> {
        let header_value = HeaderValue::from_static_with_charset("さくら", Charset::EUC_JP)?;
        assert_eq!(header_value.as_bytes(), [164, 181, 164, 175, 164, 233]);
        Ok(())
    }

    #[test]
    pub fn test_from_static_with_charset_pass_utf8() -> Result<()> {
        let header_value = HeaderValue::from_static_with_charset("さくら", Charset::UTF8)?;
        assert_eq!(
            header_value.as_bytes(),
            [227, 129, 149, 227, 129, 143, 227, 130, 137]
        );
        Ok(())
    }

    #[rstest]
    #[case::nul("\0")]
    #[case::soh("\x01")]
    #[case::stx("\x02")]
    #[case::etx("\x03")]
    #[case::eot("\x04")]
    #[case::enq("\x05")]
    #[case::ack("\x06")]
    #[case::bel("\x07")]
    #[case::bs("\x08")]
    #[case::ht("\x09")]
    #[case::lf("\n")]
    #[case::vt("\x0b")]
    #[case::ff("\x0c")]
    #[case::cr("\r")]
    #[case::so("\x0e")]
    #[case::si("\x0f")]
    #[case::dle("\x10")]
    #[case::dc1("\x11")]
    #[case::dc2("\x12")]
    #[case::dc3("\x13")]
    #[case::dc4("\x14")]
    #[case::nak("\x15")]
    #[case::syn("\x16")]
    #[case::etb("\x17")]
    #[case::can("\x18")]
    #[case::em("\x19")]
    #[case::sub("\x1a")]
    #[case::esc("\x1b")]
    #[case::fs("\x1c")]
    #[case::gs("\x1d")]
    #[case::rs("\x1e")]
    #[case::us("\x1f")]
    #[case::delete("\x7f")]
    fn test_from_static_with_charset_failed_control_character(#[case] input: String) -> Result<()> {
        let res = HeaderValue::from_static(&input);
        assert!(res.is_err());
        matches!(res, Err(Error::UnprintableCharacters(s)) if s == input);

        Ok(())
    }

    #[test]
    fn test_text_with_charset_pass_ascii() -> Result<()> {
        let value = HeaderValue::from(b"sakura".to_vec());
        assert_eq!(value.text_with_charset(Charset::ASCII)?, "sakura");
        Ok(())
    }

    #[test]
    fn test_text_with_charset_pass_sjis() -> Result<()> {
        let value = HeaderValue::from([130, 179, 130, 173, 130, 231].to_vec());
        assert_eq!(value.text_with_charset(Charset::SHIFT_JIS)?, "さくら");
        Ok(())
    }

    #[test]
    fn test_text_with_charset_pass_iso2022jp() -> Result<()> {
        let value = HeaderValue::from([27, 36, 66, 36, 53, 36, 47, 36, 105, 27, 40, 66].to_vec());
        assert_eq!(value.text_with_charset(Charset::ISO2022JP)?, "さくら");
        Ok(())
    }

    #[test]
    fn test_text_with_charset_pass_eucjp() -> Result<()> {
        let value = HeaderValue::from([164, 181, 164, 175, 164, 233].to_vec());
        assert_eq!(value.text_with_charset(Charset::EUC_JP)?, "さくら");
        Ok(())
    }

    #[test]
    fn test_text_with_charset_pass_utf8() -> Result<()> {
        let value = HeaderValue::from([227, 129, 149, 227, 129, 143, 227, 130, 137].to_vec());
        assert_eq!(value.text_with_charset(Charset::UTF8)?, "さくら");
        Ok(())
    }

    #[rstest]
    #[case::nul(b"\0")]
    #[case::soh(b"\x01")]
    #[case::stx(b"\x02")]
    #[case::etx(b"\x03")]
    #[case::eot(b"\x04")]
    #[case::enq(b"\x05")]
    #[case::ack(b"\x06")]
    #[case::bel(b"\x07")]
    #[case::bs(b"\x08")]
    #[case::ht(b"\x09")]
    #[case::lf(b"\n")]
    #[case::vt(b"\x0b")]
    #[case::ff(b"\x0c")]
    #[case::cr(b"\r")]
    #[case::so(b"\x0e")]
    #[case::si(b"\x0f")]
    #[case::dle(b"\x10")]
    #[case::dc1(b"\x11")]
    #[case::dc2(b"\x12")]
    #[case::dc3(b"\x13")]
    #[case::dc4(b"\x14")]
    #[case::nak(b"\x15")]
    #[case::syn(b"\x16")]
    #[case::etb(b"\x17")]
    #[case::can(b"\x18")]
    #[case::em(b"\x19")]
    #[case::sub(b"\x1a")]
    #[case::esc(b"\x1b")]
    #[case::fs(b"\x1c")]
    #[case::gs(b"\x1d")]
    #[case::rs(b"\x1e")]
    #[case::us(b"\x1f")]
    #[case::delete(b"\x7f")]
    fn test_text_with_charset_failed_control_character(#[case] input: &[u8]) -> Result<()> {
        let value = HeaderValue::from(input);
        let res = value.text();

        assert!(res.is_err());
        matches!(res, Err(Error::UnprintableCharacters(_)));

        Ok(())
    }
}
