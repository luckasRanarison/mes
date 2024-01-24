use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("The loaded file is not an iNES file")]
    UnsupportedFileFormat,
    #[error("NES 2.0 is not supported")]
    UnsupportedVersion,
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Unsupported mapper")]
    UnsupportedMapper,
}
