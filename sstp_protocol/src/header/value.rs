use crate::decode::Decoder;
use crate::encode::Encoder;
use crate::{decode, encode, Charset};

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
    /// # use sstp_protocol::{HeaderValue};
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// assert_eq!(HeaderValue::from_static("sakura")?.text()?, "sakura");
    /// #     Ok(())
    /// # }
    /// ```
    pub fn text(&self) -> Result<String, decode::Error> {
        self.text_with_charset(Charset::ASCII)
    }

    /// Extract HeaderValue as a string with Charset
    /// ```rust
    /// # use sstp_protocol::{HeaderValue, Charset};
    /// # use anyhow::Result;
    /// # fn main() -> Result<()> {
    /// let input = [130, 179, 130, 173, 130, 231].to_vec();
    /// assert_eq!(HeaderValue::from(input).text_with_charset(Charset::SHIFT_JIS)?, "さくら");
    /// #     Ok(())
    /// # }
    /// ```
    pub fn text_with_charset(&self, charset: Charset) -> Result<String, decode::Error> {
        match charset {
            Charset::ASCII => Decoder::decode_ascii(&self.0),
            Charset::SHIFT_JIS => Decoder::decode_sjis(&self.0),
            Charset::ISO2022JP => Decoder::decode_iso_2022_jp(&self.0),
            Charset::EUC_JP => Decoder::decode_euc_jp(&self.0),
            Charset::UTF8 => Decoder::decode_utf8(&self.0),
        }
    }

    ///　Convert string to HeaderValue with ASCII code bytes.
    ///
    /// ```rust
    /// # use sstp_protocol::{HeaderValue};
    /// assert_eq!(HeaderValue::from_static("sakura").unwrap().as_bytes(), b"sakura");
    /// ```
    pub fn from_static(s: &str) -> Result<Self, encode::Error> {
        Self::from_static_with_charset(s, Charset::ASCII)
    }

    ///　Convert string to HeaderValue with Charset.
    ///
    ///```rust
    /// # use sstp_protocol::{Charset, HeaderValue};
    /// assert_eq!(
    ///    HeaderValue::from_static_with_charset("さくら", Charset::SHIFT_JIS).unwrap().as_bytes(),
    ///    [130, 179, 130, 173, 130, 231]);
    /// ```
    pub fn from_static_with_charset(s: &str, charset: Charset) -> Result<Self, encode::Error> {
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
    // use super::*;
    // use anyhow::Result;
    // use rstest::rstest;
    //
    // #[rstest]
    // #[case::nul("\0")]
    // #[case::soh("\x01")]
    // #[case::stx("\x02")]
    // #[case::etx("\x03")]
    // #[case::eot("\x04")]
    // #[case::enq("\x05")]
    // #[case::ack("\x06")]
    // #[case::bel("\x07")]
    // #[case::bs("\x08")]
    // #[case::ht("\x09")]
    // #[case::lf("\n")]
    // #[case::vt("\x0b")]
    // #[case::ff("\x0c")]
    // #[case::cr("\r")]
    // #[case::so("\x0e")]
    // #[case::si("\x0f")]
    // #[case::dle("\x10")]
    // #[case::dc1("\x11")]
    // #[case::dc2("\x12")]
    // #[case::dc3("\x13")]
    // #[case::dc4("\x14")]
    // #[case::nak("\x15")]
    // #[case::syn("\x16")]
    // #[case::etb("\x17")]
    // #[case::can("\x18")]
    // #[case::em("\x19")]
    // #[case::sub("\x1a")]
    // #[case::esc("\x1b")]
    // #[case::fs("\x1c")]
    // #[case::gs("\x1d")]
    // #[case::rs("\x1e")]
    // #[case::us("\x1f")]
    // #[case::delete("\x7f")]
    // fn test_header_value_from_static_failed_control_character(#[case] input: String) -> Result<()> {
    //     let res = HeaderValue::from_static(&input);
    //     assert!(res.is_err());
    //     matches!(res, Err(encode::Error::AsciiEncodeFailure { .. }));
    //
    //     Ok(())
    // }
}
