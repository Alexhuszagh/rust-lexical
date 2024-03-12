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

// The following constants have the following signifiers:
//  ${X}_LITERAL - Applies to all literal values for that language.
//  ${X}_STRING - Applies to all string values for that language.
//  ${X} - Applies to all values for that language.
//  ${X}_(NAN|INF|INFINITY) - Applies to only a single special value.
//  IF it's not defined, all values are the default.

macro_rules! literal {
    ($name:ident, $value:ident) => {
        pub const $name: Option<&[u8]> = $value;
    };
    ($name:ident, $value:literal) => {
        pub const $name: Option<&[u8]> = Some($value);
    };
}

literal!(RUST_LITERAL, None);
// RUST_STRING
literal!(PYTHON_LITERAL, None);
// PYTHON_STRING
literal!(CXX_LITERAL_NAN, b"NAN");
literal!(CXX_LITERAL_INF, b"INFINITY");
literal!(CXX_LITERAL_INFINITY, b"INFINITY");
// CXX_STRING
literal!(C_LITERAL_NAN, b"NAN");
literal!(C_LITERAL_INF, b"INFINITY");
literal!(C_LITERAL_INFINITY, b"INFINITY");
// RUBY_LITERAL
literal!(RUBY_LITERAL_NAN, b"NaN");
literal!(RUBY_LITERAL_INF, b"Infinity");
literal!(RUBY_STRING_NONE, None);
// C_STRING
literal!(SWIFT_LITERAL, None);
// SWIFT_STRING
literal!(GO_LITERAL, None);
// GO_STRING
literal!(HASKELL_LITERAL, None);
literal!(HASKELL_STRING_INF, b"Infinity");
literal!(HASKELL_STRING_INFINITY, b"Infinity");
literal!(JAVASCRIPT_INF, b"Infinity");
literal!(JAVASCRIPT_INFINITY, b"Infinity");
literal!(PERL_LITERAL, None);
// PERL_STRING
literal!(PHP_LITERAL_NAN, b"NAN");
literal!(PHP_LITERAL_INF, b"INF");
literal!(PHP_LITERAL_INFINITY, b"INF");
// PHP_STRING
literal!(JAVA_LITERAL, None);
literal!(JAVA_STRING_INF, b"Infinity");
literal!(JAVA_STRING_INFINITY, b"Infinity");
literal!(R_LITERAL_INF, b"Inf");
literal!(R_LITERAL_INFINITY, b"Inf");
// R_STRING
literal!(KOTLIN_LITERAL, None);
literal!(KOTLIN_STRING_INF, b"Infinity");
literal!(KOTLIN_STRING_INFINITY, b"Infinity");
literal!(JULIA_LITERAL_INF, b"Inf");
literal!(JULIA_LITERAL_INFINITY, b"Inf");
// JULIA_STRING
literal!(CSHARP_LITERAL, None);
literal!(CSHARP_STRING_INF, b"Infinity");
literal!(CSHARP_STRING_INFINITY, b"Infinity");
literal!(KAWA, None);
literal!(GAMBITC, None);
literal!(GUILE, None);
literal!(CLOJURE_LITERAL, None);
literal!(CLOJURE_STRING_INF, b"Infinity");
literal!(CLOJURE_STRING_INFINITY, b"Infinity");
literal!(ERLANG_LITERAL_NAN, b"nan");
literal!(ERLANG_STRING, None);
literal!(ELM_LITERAL, None);
literal!(ELM_STRING_NAN, None);
literal!(ELM_STRING_INF, b"Infinity");
literal!(ELM_STRING_INFINITY, b"Infinity");
literal!(SCALA_LITERAL, None);
literal!(SCALA_STRING_INF, b"Infinity");
literal!(SCALA_STRING_INFINITY, b"Infinity");
literal!(ELIXIR, None);
literal!(FORTRAN_LITERAL, None);
// FORTRAN_STRING
literal!(D_LITERAL, None);
// D_STRING
literal!(COFFEESCRIPT_INF, b"Infinity");
literal!(COFFEESCRIPT_INFINITY, b"Infinity");
literal!(COBOL, None);
literal!(FSHARP_LITERAL_NAN, b"nan");
literal!(FSHARP_LITERAL_INF, b"infinity");
literal!(FSHARP_LITERAL_INFINITY, b"infinity");
// FSHARP_STRING
literal!(VB_LITERAL, None);
literal!(VB_STRING_INF, None);
literal!(VB_STRING_INFINITY, None);
literal!(OCAML_LITERAL_NAN, b"nan");
literal!(OCAML_LITERAL_INF, b"infinity");
literal!(OCAML_LITERAL_INFINITY, b"infinity");
// OCAML_STRING
literal!(OBJECTIVEC, None);
literal!(REASONML_LITERAL_NAN, b"nan");
literal!(REASONML_LITERAL_INF, b"infinity");
literal!(REASONML_LITERAL_INFINITY, b"infinity");
// REASONML_STRING
literal!(MATLAB_LITERAL_INF, b"inf");
literal!(MATLAB_LITERAL_INFINITY, b"Inf");
// MATLAB_STRING
literal!(ZIG_LITERAL, None);
// ZIG_STRING
literal!(SAGE_LITERAL_INF, b"infinity");
literal!(SAGE_LITERAL_INFINITY, b"Infinity");
// SAGE_STRING
literal!(JSON, None);
literal!(TOML, None);
literal!(YAML, None);
literal!(XML_INF, None);
literal!(XML_INFINITY, None);
literal!(SQLITE, None);
literal!(POSTGRESQL, None);
literal!(MYSQL, None);
literal!(MONGODB_INF, b"Infinity");
literal!(MONGODB_INFINITY, b"Infinity");
