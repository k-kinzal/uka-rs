pub use bytes_macro;

/// read! is a macro that reads bytes from a cursor.
///
/// # Example
///
/// ```rust
/// # use read_macro::read;
/// # use std::io::Cursor;
/// #
/// let mut cursor = Cursor::new(b"hello");
/// assert_eq!(read!(cursor, 5).as_ref().unwrap(), b"hello");
/// ```
#[macro_export]
macro_rules! read {
    ($cursor:expr, $len:expr) => {{
        use std::io::Read;
        let mut buffer = [0; $len];
        $cursor.read_exact(&mut buffer[..]).map(|_| buffer)
    }};
}

/// read_expect! is a macro that reads bytes from a cursor.
/// If the read bytes are not the expected bytes, it returns an error.
///
/// # Example
///
/// ```rust
/// # use read_macro::read_expect;
/// # use std::io::Cursor;
/// #
/// let mut cursor = Cursor::new(b"hello");
/// assert_eq!(read_expect!(cursor, b"hello").as_ref().unwrap(), b"hello");
/// assert!(read_expect!(cursor, b"world").is_err());
/// ```
#[macro_export]
macro_rules! read_expect {
    ( $cursor:expr, $bytes:expr ) => {{
        $crate::read!($cursor, $bytes.len()).and_then(|buffer| {
            if &buffer == $bytes {
                Ok(buffer)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "it is not the expected bytes: expect {:?}, actual {:?}",
                        $bytes, buffer
                    ),
                ))
            }
        })
    }};
}

/// read_repeat! is a macro that reads bytes from a cursor until the bytes are not the expected bytes.
///
/// # Example
///
/// ```rust
/// # use read_macro::{read_expect, read_repeat};
/// # use std::io::Cursor;
/// #
/// let mut cursor = Cursor::new(b"aaaaa:bbbbb");
/// assert_eq!(
///     read_repeat!(cursor, b"a").as_ref().unwrap(),
///     b"aaaaa"
/// );
/// assert_eq!(
///     read_expect!(cursor, b":").as_ref().unwrap(),
///     b":"
/// );
/// assert_eq!(
///     read_repeat!(cursor, b"b").as_ref().unwrap(),
///     b"bbbbb"
/// );
/// ```
#[macro_export]
macro_rules! read_repeat {
    ( $cursor:expr, $bytes:expr ) => {{
        use std::io::Seek;

        let mut buffer = Vec::new();
        loop {
            match $crate::read!($cursor, $bytes.len()) {
                Ok(buf) if &buf == $bytes => buffer.append(&mut buf.to_vec()),
                Ok(_) => {
                    if let Err(e) = $cursor.seek(std::io::SeekFrom::Current(-($bytes.len() as i64)))
                    {
                        break Err(e);
                    } else {
                        break Ok(buffer);
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break Ok(buffer),
                Err(e) => break Err(e),
            }
        }
    }};
}

/// read_until! is a macro that reads bytes from a cursor until the bytes are the expected bytes.
///
/// # Example
///
/// ```rust
/// # use read_macro::read_until;
/// # use std::io::Cursor;
/// #
/// let mut cursor = Cursor::new(b"hello:world\r\n");
/// assert_eq!(
///     read_until!(cursor, b":").as_ref().unwrap(),
///     b"hello"
/// );
/// assert_eq!(
///     read_until!(cursor, b"\r\n").as_ref().unwrap(),
///     b"world"
/// );
#[macro_export]
macro_rules! read_until {
    ( $cursor:expr, $bytes:expr ) => {{
        use std::io::Seek;

        let mut buffer = Vec::new();
        loop {
            match $crate::read!($cursor, 1) {
                Ok(buf) => {
                    buffer.extend(&buf);
                    if buffer.ends_with($bytes) {
                        buffer.truncate(buffer.len() - $bytes.len());
                        break Ok(buffer);
                    }
                }
                Err(e) => break Err(e),
            }
        }
    }};
}

/// read_match! is a macro that reads bytes from a cursor and matches the bytes.
///
/// # Example
///
/// ```rust
/// # use read_macro::read_match;
/// # use std::io::Cursor;
/// #
/// let mut cursor = Cursor::new(b"hello");
/// assert_eq!(
///     read_match!(cursor, {
///         b"hello" => "hello",
///         b"world" => "world",
///     }).unwrap(),
///     "hello"
/// );
///
/// let mut cursor = Cursor::new(b"1010");
/// assert_eq!(
///     read_match!(cursor, {
///         b"1000" => 1000,
///         b"1010" => 1010,
///     }, 3).unwrap(),
///     1010
/// );
/// ```
#[macro_export]
macro_rules! read_match {
    ( $cursor:ident, { $( $bytes:literal => $expr:expr ),*, } ) => {
        {
            $crate::read!($cursor, 1).and_then(|first| match &first {
                $(
                    $crate::bytes_macro::bytes_slice!($bytes, 0, 1) => {
                        if $crate::bytes_macro::bytes_length!($bytes) == 1 {
                            Ok($expr)
                        } else {
                            $crate::read_expect!($cursor, $crate::bytes_macro::bytes_slice!($bytes, 1)).map(|_| $expr)
                        }
                    },
                )*
                _ => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("it is not the expected first byte: {:?}", first))),
            })
        }
    };
    ( $cursor:ident, { $( $bytes:literal => $expr:expr ),*, }, $len:literal ) => {
        {
            $crate::read!($cursor, $len).and_then(|first| match &first {
                $(
                    $crate::bytes_macro::bytes_slice!($bytes, 0, $len) => {
                        if $crate::bytes_macro::bytes_length!($bytes) == $len {
                            Ok($expr)
                        } else {
                            $crate::read_expect!($cursor, $crate::bytes_macro::bytes_slice!($bytes, $len)).map(|_| $expr)
                        }
                    },
                )*
                _ => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("it is not the expected first byte: {:?}", first))),
            })
        }
    };
}

/// lookahead! is a macro that reads bytes from a cursor and returns the bytes.
/// The cursor position is not changed.
///
/// # Example
///
/// ```rust
/// # use read_macro::lookahead;
/// # use std::io::Cursor;
/// #
/// let mut cursor = Cursor::new(b"hello");
/// assert_eq!(
///     lookahead!(cursor, 2).unwrap(),
///     [104, 101]
/// );
/// assert_eq!(
///     cursor.position(),
///     0
/// );
#[macro_export]
macro_rules! lookahead {
    ( $cursor:expr, $len:expr ) => {{
        use std::io::Seek;

        $crate::read!($cursor, $len).and_then(|buffer| {
            $cursor
                .seek(std::io::SeekFrom::Current(-($len as i64)))
                .map(|_| buffer)
        })
    }};
}
