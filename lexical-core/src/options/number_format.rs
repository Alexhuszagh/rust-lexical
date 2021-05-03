//! Constants for NumberFormat when `feature = "format"` is enabled.

#![cfg(feature = "format")]

use super::lexer::*;
use super::number::*;
use super::syntax::*;

/// Define a new constant using a standard lexer format.
macro_rules! standard {
    ($Self:ident, $name:ident) => (pub const $name: $Self = $Self {
        syntax: SyntaxFormat::$name,
        lexer: LexerFormat::STANDARD,
    };);
}

// TODO(ahuszagh) Rename
impl NumberFormatV2 {
    // PRE-DEFINED

    /// Number format for a Rust literal floating-point number.
    standard!(Self, RUST_LITERAL);

    /// Number format to parse a Rust float from string.
    standard!(Self, RUST_STRING);

    /// `RUST_STRING`, but enforces strict equality for special values.
    standard!(Self, RUST_STRING_STRICT);

    /// Number format for a Python literal floating-point number.
    standard!(Self, PYTHON_LITERAL);

    /// Number format to parse a Python float from string.
    standard!(Self, PYTHON_STRING);

    /// Number format for a Python3 literal floating-point number.
    standard!(Self, PYTHON3_LITERAL);

    /// Number format to parse a Python3 float from string.
    standard!(Self, PYTHON3_STRING);

    /// Number format for a Python2 literal floating-point number.
    standard!(Self, PYTHON2_LITERAL);

    /// Number format to parse a Python2 float from string.
    standard!(Self, PYTHON2_STRING);

    /// Number format for a C++ literal floating-point number.
    standard!(Self, CXX_LITERAL);

    /// Number format to parse a C++ float from string.
    standard!(Self, CXX_STRING);

    /// Number format for a C++17 literal floating-point number.
    standard!(Self, CXX17_LITERAL);

    /// Number format for a C++17 string floating-point number.
    standard!(Self, CXX17_STRING);

    /// Number format for a C++14 literal floating-point number.
    standard!(Self, CXX14_LITERAL);

    /// Number format to parse a C++14 float from string.
    standard!(Self, CXX14_STRING);

    /// Number format for a C++11 literal floating-point number.
    standard!(Self, CXX11_LITERAL);

    /// Number format to parse a C++11 float from string.
    standard!(Self, CXX11_STRING);

    /// Number format for a C++03 literal floating-point number.
    standard!(Self, CXX03_LITERAL);

    /// Number format to parse a C++03 float from string.
    standard!(Self, CXX03_STRING);

    /// Number format for a C++98 literal floating-point number.
    standard!(Self, CXX98_LITERAL);

    /// Number format to parse a C++98 float from string.
    standard!(Self, CXX98_STRING);

    /// Number format for a C literal floating-point number.
    standard!(Self, C_LITERAL);

    /// Number format to parse a C float from string.
    standard!(Self, C_STRING);

    /// Number format for a C18 literal floating-point number.
    standard!(Self, C18_LITERAL);

    /// Number format to parse a C18 float from string.
    standard!(Self, C18_STRING);

    /// Number format for a C11 literal floating-point number.
    standard!(Self, C11_LITERAL);

    /// Number format to parse a C11 float from string.
    standard!(Self, C11_STRING);

    /// Number format for a C99 literal floating-point number.
    standard!(Self, C99_LITERAL);

    /// Number format to parse a C99 float from string.
    standard!(Self, C99_STRING);

    /// Number format for a C90 literal floating-point number.
    standard!(Self, C90_LITERAL);

    /// Number format to parse a C90 float from string.
    standard!(Self, C90_STRING);

    /// Number format for a C89 literal floating-point number.
    standard!(Self, C89_LITERAL);

    /// Number format to parse a C89 float from string.
    standard!(Self, C89_STRING);

    /// Number format for a Ruby literal floating-point number.
    standard!(Self, RUBY_LITERAL);

    /// Number format to parse a Ruby float from string.
    standard!(Self, RUBY_STRING);

    /// Number format for a Swift literal floating-point number.
    standard!(Self, SWIFT_LITERAL);

    /// Number format to parse a Swift float from string.
    standard!(Self, SWIFT_STRING);

    /// Number format for a Golang literal floating-point number.
    standard!(Self, GO_LITERAL);

    /// Number format to parse a Golang float from string.
    standard!(Self, GO_STRING);

    /// Number format for a Haskell literal floating-point number.
    standard!(Self, HASKELL_LITERAL);

    /// Number format to parse a Haskell float from string.
    standard!(Self, HASKELL_STRING);

    /// Number format for a Javascript literal floating-point number.
    standard!(Self, JAVASCRIPT_LITERAL);

    /// Number format to parse a Javascript float from string.
    standard!(Self, JAVASCRIPT_STRING);

    /// Number format for a Perl literal floating-point number.
    standard!(Self, PERL_LITERAL);

    /// Number format to parse a Perl float from string.
    standard!(Self, PERL_STRING);

    /// Number format for a PHP literal floating-point number.
    standard!(Self, PHP_LITERAL);

    /// Number format to parse a PHP float from string.
    standard!(Self, PHP_STRING);

    /// Number format for a Java literal floating-point number.
    standard!(Self, JAVA_LITERAL);

    /// Number format to parse a Java float from string.
    standard!(Self, JAVA_STRING);

    /// Number format for a R literal floating-point number.
    standard!(Self, R_LITERAL);

    /// Number format to parse a R float from string.
    standard!(Self, R_STRING);

    /// Number format for a Kotlin literal floating-point number.
    standard!(Self, KOTLIN_LITERAL);

    /// Number format to parse a Kotlin float from string.
    standard!(Self, KOTLIN_STRING);

    /// Number format for a Julia literal floating-point number.
    standard!(Self, JULIA_LITERAL);

    /// Number format to parse a Julia float from string.
    standard!(Self, JULIA_STRING);

    /// Number format for a C# literal floating-point number.
    standard!(Self, CSHARP_LITERAL);

    /// Number format to parse a C# float from string.
    standard!(Self, CSHARP_STRING);

    /// Number format for a C#7 literal floating-point number.
    standard!(Self, CSHARP7_LITERAL);

    /// Number format to parse a C#7 float from string.
    standard!(Self, CSHARP7_STRING);

    /// Number format for a C#6 literal floating-point number.
    standard!(Self, CSHARP6_LITERAL);

    /// Number format to parse a C#6 float from string.
    standard!(Self, CSHARP6_STRING);

    /// Number format for a C#5 literal floating-point number.
    standard!(Self, CSHARP5_LITERAL);

    /// Number format to parse a C#5 float from string.
    standard!(Self, CSHARP5_STRING);

    /// Number format for a C#4 literal floating-point number.
    standard!(Self, CSHARP4_LITERAL);

    /// Number format to parse a C#4 float from string.
    standard!(Self, CSHARP4_STRING);

    /// Number format for a C#3 literal floating-point number.
    standard!(Self, CSHARP3_LITERAL);

    /// Number format to parse a C#3 float from string.
    standard!(Self, CSHARP3_STRING);

    /// Number format for a C#2 literal floating-point number.
    standard!(Self, CSHARP2_LITERAL);

    /// Number format to parse a C#2 float from string.
    standard!(Self, CSHARP2_STRING);

    /// Number format for a C#1 literal floating-point number.
    standard!(Self, CSHARP1_LITERAL);

    /// Number format to parse a C#1 float from string.
    standard!(Self, CSHARP1_STRING);

    /// Number format for a Kawa literal floating-point number.
    standard!(Self, KAWA_LITERAL);

    /// Number format to parse a Kawa float from string.
    standard!(Self, KAWA_STRING);

    /// Number format for a Gambit-C literal floating-point number.
    standard!(Self, GAMBITC_LITERAL);

    /// Number format to parse a Gambit-C float from string.
    standard!(Self, GAMBITC_STRING);

    /// Number format for a Guile literal floating-point number.
    standard!(Self, GUILE_LITERAL);

    /// Number format to parse a Guile float from string.
    standard!(Self, GUILE_STRING);

    /// Number format for a Clojure literal floating-point number.
    standard!(Self, CLOJURE_LITERAL);

    /// Number format to parse a Clojure float from string.
    standard!(Self, CLOJURE_STRING);

    /// Number format for an Erlang literal floating-point number.
    standard!(Self, ERLANG_LITERAL);

    /// Number format to parse an Erlang float from string.
    standard!(Self, ERLANG_STRING);

    /// Number format for an Elm literal floating-point number.
    standard!(Self, ELM_LITERAL);

    /// Number format to parse an Elm float from string.
    standard!(Self, ELM_STRING);

    /// Number format for a Scala literal floating-point number.
    standard!(Self, SCALA_LITERAL);

    /// Number format to parse a Scala float from string.
    standard!(Self, SCALA_STRING);

    /// Number format for an Elixir literal floating-point number.
    standard!(Self, ELIXIR_LITERAL);

    /// Number format to parse an Elixir float from string.
    standard!(Self, ELIXIR_STRING);

    /// Number format for a FORTRAN literal floating-point number.
    standard!(Self, FORTRAN_LITERAL);

    /// Number format to parse a FORTRAN float from string.
    standard!(Self, FORTRAN_STRING);

    /// Number format for a D literal floating-point number.
    standard!(Self, D_LITERAL);

    /// Number format to parse a D float from string.
    standard!(Self, D_STRING);

    /// Number format for a Coffeescript literal floating-point number.
    standard!(Self, COFFEESCRIPT_LITERAL);

    /// Number format to parse a Coffeescript float from string.
    standard!(Self, COFFEESCRIPT_STRING);

    /// Number format for a Cobol literal floating-point number.
    standard!(Self, COBOL_LITERAL);

    /// Number format to parse a Cobol float from string.
    standard!(Self, COBOL_STRING);

    /// Number format for a F# literal floating-point number.
    standard!(Self, FSHARP_LITERAL);

    /// Number format to parse a F# float from string.
    standard!(Self, FSHARP_STRING);

    /// Number format for a Visual Basic literal floating-point number.
    standard!(Self, VB_LITERAL);

    /// Number format to parse a Visual Basic float from string.
    standard!(Self, VB_STRING);

    /// Number format for an OCaml literal floating-point number.
    standard!(Self, OCAML_LITERAL);

    /// Number format to parse an OCaml float from string.
    standard!(Self, OCAML_STRING);

    /// Number format for an Objective-C literal floating-point number.
    standard!(Self, OBJECTIVEC_LITERAL);

    /// Number format to parse an Objective-C float from string.
    standard!(Self, OBJECTIVEC_STRING);

    /// Number format for a ReasonML literal floating-point number.
    standard!(Self, REASONML_LITERAL);

    /// Number format to parse a ReasonML float from string.
    standard!(Self, REASONML_STRING);

    /// Number format for an Octave literal floating-point number.
    standard!(Self, OCTAVE_LITERAL);

    /// Number format to parse an Octave float from string.
    standard!(Self, OCTAVE_STRING);

    /// Number format for an Matlab literal floating-point number.
    standard!(Self, MATLAB_LITERAL);

    /// Number format to parse an Matlab float from string.
    standard!(Self, MATLAB_STRING);

    /// Number format for a Zig literal floating-point number.
    standard!(Self, ZIG_LITERAL);

    /// Number format to parse a Zig float from string.
    standard!(Self, ZIG_STRING);

    /// Number format for a Sage literal floating-point number.
    standard!(Self, SAGE_LITERAL);

    /// Number format to parse a Sage float from string.
    standard!(Self, SAGE_STRING);

    /// Number format for a JSON literal floating-point number.
    standard!(Self, JSON);

    /// Number format for a TOML literal floating-point number.
    standard!(Self, TOML);

    /// Number format for a YAML literal floating-point number.
    standard!(Self, YAML);

    /// Number format for a XML literal floating-point number.
    standard!(Self, XML);

    /// Number format for a SQLite literal floating-point number.
    standard!(Self, SQLITE);

    /// Number format for a PostgreSQL literal floating-point number.
    standard!(Self, POSTGRESQL);

    /// Number format for a MySQL literal floating-point number.
    standard!(Self, MYSQL);

    /// Number format for a MongoDB literal floating-point number.
    standard!(Self, MONGODB);

    // HIDDEN DEFAULTS

    /// Number format when no flags are set.
    #[doc(hidden)]
    standard!(Self, PERMISSIVE);

    /// Number format when all digit separator flags are set.
    #[doc(hidden)]
    standard!(Self, IGNORE);
}
