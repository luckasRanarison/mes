use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    UnsupportedFileFormat,
    UnsupportedVersion,
    UnexpectedEndOfInput { expected: String, length: usize },
    UnsupportedMapper(u8),
}

impl Error {
    pub fn eof(expected: &str, length: usize) -> Self {
        Self::UnexpectedEndOfInput {
            expected: expected.into(),
            length,
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::UnsupportedFileFormat => write!(f, "The loaded file is not an iNES file"),
            Error::UnsupportedVersion => write!(f, "iNES 2.0 is not supported"),
            Error::UnexpectedEndOfInput { expected, length } => {
                write!(
                    f,
                    "Unexpected end of input, expected {expected} (length: {length})",
                )
            }
            Error::UnsupportedMapper(id) => write!(f, "Unsupported mapper {id}"),
        }
    }
}

impl core::error::Error for Error {}
