use ascii::AsAsciiStr;
use encoding_rs::{EUC_JP, ISO_2022_JP, SHIFT_JIS};

/// Error that can occur when encode to bytes.
#[allow(clippy::enum_variant_names)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{source}")]
    AsciiEncodeFailure {
        #[from]
        source: ascii::AsAsciiStrError,
    },
    #[error("cannot encode from string to Shift_JIS")]
    ShiftJisEncodeFailure,
    #[error("cannot encode from string to ISO-2022-JP")]
    Iso2022JpEncodeFailure,
    #[error("cannot encode from string to EUC-JP")]
    EucJpEncodeFailure,
}

pub type Result<T> = std::result::Result<T, Error>;

/// Encoder encodes string into byte units of a specific character set and encoding.
pub struct Encoder;
impl Encoder {
    /// Encode string to bytes of ascii code.
    pub fn encode_ascii(input: &str) -> Result<Vec<u8>> {
        input
            .as_ascii_str()
            .map_err(Error::from)
            .map(|v| v.as_bytes().to_vec())
    }

    /// Encode string to bytes of shift jis encoding.
    pub fn encode_sjis(input: &str) -> Result<Vec<u8>> {
        let (cow, _, is_error) = SHIFT_JIS.encode(input);
        if is_error {
            Err(Error::ShiftJisEncodeFailure)
        } else {
            Ok(cow.to_vec())
        }
    }

    /// Encode string to bytes of iso-2022-jp encoding.
    pub fn encode_iso_2022_jp(input: &str) -> Result<Vec<u8>> {
        let (cow, _, is_error) = ISO_2022_JP.encode(input);
        if is_error {
            Err(Error::Iso2022JpEncodeFailure)
        } else {
            Ok(cow.to_vec())
        }
    }

    /// Encode string to bytes of euc-jp encoding.
    pub fn encode_euc_jp(input: &str) -> Result<Vec<u8>> {
        let (cow, _, is_error) = EUC_JP.encode(input);
        if is_error {
            Err(Error::EucJpEncodeFailure)
        } else {
            Ok(cow.to_vec())
        }
    }

    /// Encode string to bytes of utf8 encoding.
    pub fn encode_utf8(input: &str) -> Result<Vec<u8>> {
        Ok(input.as_bytes().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHIFT_JIS_BYTES: [u8; 6] = [130, 179, 130, 173, 130, 231];
    const ISO_2022_JP_BYTES: [u8; 12] = [27, 36, 66, 36, 53, 36, 47, 36, 105, 27, 40, 66];
    const EUC_JP_BYTES: [u8; 6] = [164, 181, 164, 175, 164, 233];
    const UTF8_BYTES: [u8; 9] = [227, 129, 149, 227, 129, 143, 227, 130, 137];

    #[test]
    fn test_encode_ascii() -> Result<()> {
        let input = "sakura";
        let bytes = Encoder::encode_ascii(input).unwrap();

        assert_eq!(bytes, b"sakura");

        Ok(())
    }

    #[test]
    fn test_encode_ascii_failed_pass_not_represented_ascii() -> Result<()> {
        let input = "さくら";
        let result = Encoder::encode_ascii(input);

        assert!(result.is_err());
        matches!(result.unwrap_err(), Error::AsciiEncodeFailure { .. });

        Ok(())
    }

    #[test]
    fn test_encode_sjis() -> Result<()> {
        let input = "さくら";
        let bytes = Encoder::encode_sjis(input).unwrap();

        assert_eq!(bytes, SHIFT_JIS_BYTES);

        Ok(())
    }

    #[test]
    fn test_encode_sjis_failed_pass_not_represented_sjis() -> Result<()> {
        let input = "🐇";
        let result = Encoder::encode_sjis(input);

        assert!(result.is_err());
        matches!(result.unwrap_err(), Error::ShiftJisEncodeFailure);

        Ok(())
    }

    #[test]
    fn test_encode_iso_2022_jp() -> Result<()> {
        let input = "さくら";
        let bytes = Encoder::encode_iso_2022_jp(input).unwrap();

        assert_eq!(bytes, ISO_2022_JP_BYTES);

        Ok(())
    }

    #[test]
    fn test_encode_iso_2022_jp_failed_pass_not_represented_iso2022jp() -> Result<()> {
        let input = "🐇";
        let result = Encoder::encode_iso_2022_jp(input);

        assert!(result.is_err());
        matches!(result.unwrap_err(), Error::Iso2022JpEncodeFailure);

        Ok(())
    }

    #[test]
    fn test_encode_euc_jp() -> Result<()> {
        let input = "さくら";
        let bytes = Encoder::encode_euc_jp(input).unwrap();

        assert_eq!(bytes, EUC_JP_BYTES);

        Ok(())
    }

    #[test]
    fn test_encode_euc_jp_failed_pass_not_represented_eucjp() -> Result<()> {
        let input = "🐇";
        let result = Encoder::encode_euc_jp(input);

        assert!(result.is_err());
        matches!(result.unwrap_err(), Error::EucJpEncodeFailure);

        Ok(())
    }

    #[test]
    fn test_encode_utf8() -> Result<()> {
        let input = "さくら";
        let bytes = Encoder::encode_utf8(input).unwrap();

        assert_eq!(bytes, UTF8_BYTES);

        Ok(())
    }
}
