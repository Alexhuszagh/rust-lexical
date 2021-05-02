//! Parser result type.

cfg_if! {
if #[cfg(any(feature = "atof", feature = "atoi"))] {
    use crate::lib::result::Result as StdResult;
    use super::error::{Error, ErrorCode};

    /// A specialized Result type for lexical operations.
    pub type Result<T> = StdResult<T, Error>;

    /// Specialized error type for format parsers.
    pub(crate) type ParseError = (ErrorCode, *const u8);

    /// Specialized result type for format parsers.
    pub(crate) type ParseResult<T> = StdResult<T, ParseError>;

    /// Type definition for result when testing parsing.
    #[cfg(all(test, feature = "atof"))]
    pub(crate) type ParseTestResult<T> = StdResult<T, ErrorCode>;
}}  // cfg_if
