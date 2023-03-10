use ascii::AsciiString;
use encoding_rs::{EUC_JP, ISO_2022_JP, SHIFT_JIS};

/// Error that can occur when decode from bytes.
#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{source}")]
    AsciiDecodeFailure {
        #[from]
        source: ascii::FromAsciiError<Vec<u8>>,
    },
    #[error("cannot decode byte to string in Shift_JIS")]
    ShiftJisDecodeFailure,
    #[error("cannot decode byte to string in ISO-2022-JP")]
    Iso2022JpDecodeFailure,
    #[error("cannot decode byte to string in EUC-JP")]
    EucJpDecodeFailure,
    #[error("{source}")]
    Utf8DecodeFailure {
        #[from]
        source: std::string::FromUtf8Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

/// Decoder decodes bytes of a specific character set and encoding into a string.
pub struct Decoder;
impl Decoder {
    /// Decode bytes of ascii code to string.
    pub fn decode_ascii(input: &[u8]) -> Result<String> {
        AsciiString::from_ascii(input.to_vec())
            .map_err(Error::from)
            .map(|v| v.to_string())
    }

    /// Decode bytes of shift jis encoding to string.
    pub fn decode_sjis(input: &[u8]) -> Result<String> {
        let (cow, _, is_error) = SHIFT_JIS.decode(input.as_ref());
        if is_error {
            Err(Error::ShiftJisDecodeFailure)
        } else {
            Ok(cow.to_string())
        }
    }

    /// Decode bytes of iso-20220-jp encoding to string.
    pub fn decode_iso_2022_jp(input: &[u8]) -> Result<String> {
        let (cow, _, is_error) = ISO_2022_JP.decode(input.as_ref());
        if is_error {
            Err(Error::Iso2022JpDecodeFailure)
        } else {
            Ok(cow.to_string())
        }
    }

    /// Decode bytes of euc-jp encoding to string.
    pub fn decode_euc_jp(input: &[u8]) -> Result<String> {
        let (cow, _, is_error) = EUC_JP.decode(input.as_ref());
        if is_error {
            Err(Error::EucJpDecodeFailure)
        } else {
            Ok(cow.to_string())
        }
    }

    /// Decode bytes of utf-8 encoding to string.
    pub fn decode_utf8(input: &[u8]) -> Result<String> {
        String::from_utf8(input.into()).map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use crate::decode::{Decoder, Error};
    use anyhow::Result;

    const SHIFT_JIS_BYTES: [u8; 6] = [130, 179, 130, 173, 130, 231];
    const ISO_2022_JP_BYTES: [u8; 12] = [27, 36, 66, 36, 53, 36, 47, 36, 105, 27, 40, 66];
    const EUC_JP_BYTES: [u8; 6] = [164, 181, 164, 175, 164, 233];
    const UTF8_BYTES: [u8; 9] = [227, 129, 149, 227, 129, 143, 227, 130, 137];

    #[test]
    fn test_decode_ascii() -> Result<()> {
        let input = b"sakura";
        let str = Decoder::decode_ascii(input).unwrap();

        assert_eq!(str, "sakura");

        Ok(())
    }

    #[test]
    fn test_decode_ascii_failed_pass_non_ascii() -> Result<()> {
        let input = UTF8_BYTES;
        let result = Decoder::decode_ascii(&input);

        assert!(result.is_err());
        matches!(result.unwrap_err(), Error::AsciiDecodeFailure { .. });

        Ok(())
    }

    #[test]
    fn test_decode_sjis() -> Result<()> {
        let input = SHIFT_JIS_BYTES;
        let str = Decoder::decode_sjis(&input).unwrap();

        assert_eq!(str, "さくら");

        Ok(())
    }

    #[test]
    fn test_decode_sjis_failed_pass_non_sjis() -> Result<()> {
        let input = UTF8_BYTES;
        let result = Decoder::decode_sjis(&input);

        assert!(result.is_err());
        matches!(result.unwrap_err(), Error::ShiftJisDecodeFailure);

        Ok(())
    }

    #[test]
    fn test_decode_iso_2022_jp() -> Result<()> {
        let input = ISO_2022_JP_BYTES;
        let str = Decoder::decode_iso_2022_jp(&input).unwrap();

        assert_eq!(str, "さくら");

        Ok(())
    }

    #[test]
    fn test_decode_iso_2022_jp_failed_pass_non_iso2022jp() -> Result<()> {
        let input = UTF8_BYTES;
        let result = Decoder::decode_iso_2022_jp(&input);

        assert!(result.is_err());
        matches!(result.unwrap_err(), Error::Iso2022JpDecodeFailure);

        Ok(())
    }

    #[test]
    fn test_decode_euc_jp() -> Result<()> {
        let input = EUC_JP_BYTES;
        let str = Decoder::decode_euc_jp(&input).unwrap();

        assert_eq!(str, "さくら");

        Ok(())
    }

    #[test]
    fn test_decode_euc_jp_failed_pass_non_eucjp() -> Result<()> {
        let input = UTF8_BYTES;
        let result = Decoder::decode_euc_jp(&input);

        assert!(result.is_err());
        matches!(result.unwrap_err(), Error::EucJpDecodeFailure);

        Ok(())
    }

    #[test]
    fn test_decode_utf8() -> Result<()> {
        let input = UTF8_BYTES;
        let str = Decoder::decode_utf8(&input).unwrap();

        assert_eq!(str, "さくら");

        Ok(())
    }

    #[test]
    fn test_decode_utf8_failed_pass_non_utf8() -> Result<()> {
        let input = SHIFT_JIS_BYTES;
        let result = Decoder::decode_utf8(&input);

        assert!(result.is_err());
        matches!(result.unwrap_err(), Error::Utf8DecodeFailure { .. });

        Ok(())
    }
}
