//! Shared traits for the options API.

#[cfg(feature = "write")]
use crate::constants::FormattedSize;

// TRAITS
// ------

/// Shared trait for all writer options.
#[cfg(feature = "write")]
pub trait WriteOptions: Default {
    /// Determine if the options are valid.
    fn is_valid(&self) -> bool;

    /// Get an upper bound on the buffer size.
    fn buffer_size<T: FormattedSize, const FORMAT: u128>(&self) -> usize;
}

/// Shared trait for all parser options.
#[cfg(feature = "parse")]
pub trait ParseOptions: Default {
    /// Determine if the options are valid.
    fn is_valid(&self) -> bool;
}

// PRE-DEFINED CONSTANTS
// ---------------------

// TODO(ahuszagh)
//  Scala? Elixir? Fortran? D? Coffeescript?
//  Cobol? F3? VB? OCAML? Objective-C? ReasonML? Octave? Matlab?
//  Zig? Sage? JSON? TOML? YAML? SQLITE? POSTGRES? XML? MONGODB?

macro_rules! literal {
    ($name:ident, $value:ident) => {
        pub const $name: Option<&[u8]> = $value;
    };
    ($name:ident, $value:literal) => {
        pub const $name: Option<&[u8]> = Some($value);
    };
}

literal!(HASKELL_STRING_INF, b"Infinity");
literal!(HASKELL_STRING_INFINITY, b"Infinity");
literal!(JAVASCRIPT_INF, b"Infinity");
literal!(JAVASCRIPT_INFINITY, b"Infinity");
literal!(PHP_LITERAL_NAN, b"NAN");
literal!(PHP_LITERAL_INF, b"INF");
literal!(PHP_LITERAL_INFINITY, b"INF");
literal!(JAVA_STRING_INF, b"Infinity");
literal!(JAVA_STRING_INFINITY, b"Infinity");
literal!(R_LITERAL_INF, b"Inf");
literal!(R_LITERAL_INFINITY, b"Inf");
literal!(JULIA_LITERAL_INF, b"Inf");
literal!(JULIA_LITERAL_INFINITY, b"Inf");
literal!(CSHARP_STRING_INF, b"Infinity");
literal!(CSHARP_STRING_INFINITY, b"Infinity");
literal!(CLOJURE_STRING_INF, b"Infinity");
literal!(CLOJURE_STRING_INFINITY, b"Infinity");
literal!(ERLANG_LITERAL_NAN, b"nan");
literal!(ELM_LITERAL_NAN, None);
literal!(ELM_LITERAL_INF, b"Infinity");
literal!(ELM_LITERAL_INFINITY, b"Infinity");
