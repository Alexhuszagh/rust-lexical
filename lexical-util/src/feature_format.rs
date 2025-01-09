//! Configuration options for parsing and formatting numbers.
//!
//! This comprises 2 parts: a low-level API for generating packed structs
//! containing enumerating for number formats (both syntax and lexer).
//!
//! # Syntax Format
//!
//! The syntax format defines **which** numeric string are valid.
//! For example, if exponent notation is required or not
//! allowed.
//!
//! # Control Format
//!
//! The control format defines what characters are valid, that is, which
//! characters should be consider valid to continue tokenization.

#![cfg(feature = "format")]

// Sample test code for each language used:
//
//  Rust
//  ----
//
//  Setup:
//      Save to `main.rs` and run `rustc main.rs -o main`.
//
//  Code:
//      ```text
//      pub fn main() {
//          println!("{:?}", 3_.0f32);
//          println!("{:?}", "3_.0".parse::<f32>());
//      }
//      ```
//
// Python
// ------
//
//  Setup:
//      Run `python` to enter the interpreter.
//
//  Code:
//      ```text
//      print(3_.0)
//      print(float("3_.0"))
//      ```
//
//  C++
//  ---
//
//  Setup:
//      Save to `main.cc` and run `g++ main.cc -o main -std=c++XX`,
//      where XX is one of the following values:
//          - 98
//          - 03
//          - 11
//          - 14
//          - 17
//
//  Code:
//      ```text
//      #include <cstdlib>
//      #include <cstring>
//      #include <iostream>
//      #include <iterator>
//      #include <stdexcept>
//
//      double parse(const char* string) {
//          char* end;
//          double result = strtod(string, &end);
//          auto endp = reinterpret_cast<const char*>(end);
//          if (std::distance(string, endp) != strlen(string)) {
//              throw std::invalid_argument("did not consume entire string.");
//          }
//          return result;
//      }
//
//      int main() {
//          std::cout << 3'.0 << std::endl;
//          std::cout << parse("3'.0") << std::endl;
//      }
//      ```
//
//  C
//  -
//
//  Setup:
//      Save to `main.c` and run `gcc main.c -o main -std=cXX`,
//      where XX is one of the following values:
//          - 89
//          - 90
//          - 99
//          - 11
//          - 18
//
//  Code:
//      ```text
//      #include <stdint.h>
//      #include <stdlib.h>
//      #include <string.h>
//      #include <stdio.h>
//
//      size_t distance(const char* first, const char* last) {
//          uintptr_t x = (uintptr_t) first;
//          uintptr_t y = (uintptr_t) last;
//          return (size_t) (y - x);
//      }
//
//      double parse(const char* string) {
//          char* end;
//          double result = strtod(string, &end);
//          if (distance(string, (const char*) end) != strlen(string)) {
//              abort();
//          }
//          return result;
//      }
//
//      int main() {
//          printf("%f\n", 3'.);
//          printf("%f\n", parse("3'."));
//      }
//      ```
//
// Ruby
// ----
//
//  Setup:
//      Run `irb` to enter the interpreter.
//
//  Code:
//      ```text
//      puts 3.0_1;
//      puts "3.0_1".to_f;
//      ```
// Swift
// -----
//
//  Setup:
//      Run `swift` to enter the interpreter.
//
//  Code:
//      ```text
//      print(3.0);
//      print(Float("3.0"));
//      ```
// Golang
// ------
//
// Setup:
//      Save to `main.go` and run `go run main.go`
//
// Code:
//      ```text
//      package main
//
//      import (
//          "fmt"
//          "strconv"
//      )
//
//      func main() {
//          fmt.Println(3.0)
//          fmt.Println(strconv.ParseFloat("3.0", 64))
//      }
//      ```
//
// Haskell
// -------
//
// Setup:
//      Run `ghci` to enter the interpreter.
//
// Code:
//      ```text
//      :m Numeric
//      showFloat 3.0 ""
//      let x = "3.0"
//      read x :: Float
//      ```
//
// Javascript
// ----------
//
// Setup:
//      Run `nodejs` (or `node`) to enter the interpreter.
//
// Code:
//      ```text
//          console.log(3.0)
//          console.log(parseFloat("3.0"))
//      ```
//
// Perl
// ----
//
// Setup:
//      Run `perl -de1` to enter the interpret.
//
// Code:
//      ```text
//      print 3.01;
//      print '3.01' * 1;
//      ```
//
// PHP
// ---
//
// Setup:
//      Run `php -a` to enter the interpret.
//
// Code:
//      ```text
//      printf("%f\n", 3.0);
//      printf("%f\n", floatval("3.0"));
//      ```
//
// Java
// ----
//
// Setup:
//      Save to `main.java` and run `javac main.java`, then run `java Main`.
//
// Code:
//      ```text
//      class Main {
//          public static void main(String args[]) {
//              System.out.println(3.0);
//              System.out.println(Float.parseFloat("3.0"));
//          }
//      }
//      ```
//
// R
// -
//
// Setup:
//      Run `R` to enter the interpret.
//
// Code:
//      ```text
//      print(3.0);
//      print(as.numeric("3.0"));
//      ```
//
// Kotlin
// ------
//
// Setup:
//      Save file to `main.kt` and run `kotlinc main.kt -d main.jar`,
//      then run `java -jar main.jar`.
//
// Code:
//      ```text
//      fun main() {
//          println(3.0)
//          println("3.0".toDouble())
//      }
//      ```
//
// Julia
// -----
//
// Setup:
//      Run `julia` to enter the interpret.
//
// Code:
//      ```text
//      print(3.0);
//      print(parse(Float64, "3.0"));
//      ```
//
// C#
// --
//
// Note:
//      Mono accepts both integer and fraction decimal separators, Mono is
//      just buggy, see https://github.com/dotnet/csharplang/issues/55#issuecomment-574902516.
//
// Setup:
//      Run `csharp -langversion:X` to enter the interpret,
//      where XX is one of the following values:
//          - ISO-1
//          - ISO-2
//          - 3
//          - 4
//          - 5
//          - 6
//          - 7
//
// Code:
//      ```text
//      Console.WriteLine("{0}", 3.0);
//      Console.WriteLine("{0}", float.Parse("3.0"));
//      ```
//
// Kawa
// ----
//
// Setup:
//      Run `kawa` to enter the interpreter.
//
// Code:
//      ```text
//      3.0
//      (string->number "3.0")
//      ```
//
// Gambit-C
// --------
//
// Setup:
//      Run `gsc` to enter the interpreter.
//
// Code:
//      ```text
//      3.0
//      (string->number "3.0")
//      ```
//
// Guile
// -----
//
// Setup:
//      Run `guile` to enter the interpreter.
//
// Code:
//      ```text
//      3.0
//      (string->number "3.0")
//      ```
//
// Clojure
// -------
//
// Setup:
//      Run `clojure` to enter the interpreter.
//
// Code:
//      ```text
//      3.0
//      (Float/parseFloat "3.0")
//      ```
//
// Erlang
// ------
//
// Setup:
//      Run `erl` to enter the interpreter.
//
// Code:
//      ```text
//      io:format("~p~n", [3.0]).
//      string:to_float("3.0").
//      ```
//
// Elm
// ---
//
// Setup:
//      Run `elm repl` to enter the interpreter.
//
// Code:
//      ```text
//      3.0
//      String.toFloat "3.0"
//      ```
//
// Scala
// -----
//
// Setup:
//      Run `scala` to enter the interpreter.
//
// Code:
//      ```text
//      3.0
//      "3.0".toFloat
//      ```
//
// Elixir
// ------
//
// Setup:
//      Run `iex` to enter the interpreter.
//
// Code:
//      ```text
//      3.0;
//      String.to_float("3.0");
//      ```
//
// FORTRAN
// -------
//
// Setup:
//      Save to `main.f90` and run `gfortran -o main main.f90`
//
// Code:
//      ```text
//      program main
//        real :: x
//        character (len=30) :: word
//        word = "3."
//        read(word, *) x
//        print *, 3.
//        print *, x
//      end program main
//      ```
//
// D
// -
//
// Setup:
//      Save to `main.d` and run `dmd -run main.d`
//
// Code:
//      ```text
//      import std.conv;
//      import std.stdio;
//
//      void main()
//      {
//          writeln(3.0);
//          writeln(to!double("3.0"));
//      }
//      ```
//
// Coffeescript
// ------------
//
// Setup:
//      Run `coffee` to enter the interpreter.
//
// Code:
//      ```text
//      3.0;
//      parseFloat("3.0");
//      ```
//
// Cobol
// -----
//
// Setup:
//      Save to `main.cbl` and run `cobc main.cbl` then `cobcrun main`.
//
// Code:
//      ```text
//                IDENTIFICATION DIVISION.
//                PROGRAM-ID. main.
//
//                DATA DIVISION.
//                   WORKING-STORAGE SECTION.
//                   01 R PIC X(20)   VALUE "3.0".
//                   01 TOTAL        USAGE IS COMP-2.
//
//                PROCEDURE DIVISION.
//                   COMPUTE TOTAL = FUNCTION NUMVAL(R).
//                   Display 3.0.
//                   Display TOTAL.
//                   STOP RUN.
//      ```
//
// F#
// --
//
// Setup:
//      Run `dotnet fsi` to enter the interpreter.
//
// Code:
//      ```text
//      printfn "%f" 3.0;;
//      let f = float "3.0";;
//      printfn "%f" f;;
//      ```
//
// Visual Basic
// ------------
//
// Setup:
//      Save to `main.vb` and run `vbnc main.vb`.
//
// Code:
//      ```text
//      Imports System
//
//      Module Module1
//          Sub Main()
//              Console.WriteLine(Format$(3.0, "0.0000000000000"))
//              Console.WriteLine(Format$(CDbl("3.0"), "0.0000000000000"))
//          End Sub
//      End Module
//      ```
//
// OCaml
// -----
//
// Setup:
//      Save to `main.ml` and run `ocamlc -o main main.ml`.
//
// Code:
//      ```text
//      Printf.printf "%f\n" 3.0
//      let () =
//          let f = float_of_string "3.0" in
//          Printf.printf "%f\n" f
//      ```
//
// Objective-C
// -----------
//
// Setup:
//      Save to `main.m` and run `gcc -o main -lobjc -lgnustep-base main.m
// -fconstant-string-class=NSConstantString`.
//
// Code:
//      ```text
//      #import <Foundation/Foundation.h>
//      #import <stdio.h>
//
//      int main(int argv, char* argc[])
//      {
//          printf("%f\n", 3.0);
//          NSString *s = @"3.0";
//          double f = [s doubleValue];
//          printf("%f\n", f);
//      }
//      ```
//
// ReasonML
// --------
//
// Setup:
//      Run `rtop` to enter the interpreter.
//
// Code:
//      ```text
//      Printf.printf("%f\n", 3.0);
//      Printf.printf("%f\n", float_of_string("3.0"));
//      ```
//
// Zig
// ---
//
// Setup:
//      Save to `main.zig` and run `zig build-exe main.zig`
//
// Code:
//      ```text
//      const std = @import("std");
//
//      pub fn main() void {
//          const f: f64 = 3.0;
//          std.debug.warn("{}\n", f);
//          const x: f64 = std.fmt.parseFloat(f64, "3.0") catch unreachable;
//          std.debug.warn("{}\n", x);
//      }
//      ```
//
//
// Octave (and Matlab)
// -------------------
//
// Setup:
//      Run `octave` to enter the interpreter, or
//      run `octave --traditional` to enter the Matlab interpret.
//
// Code:
//      ```text
//      3.0
//      str2double("3.0")
//      ```
//
// Sage
// ----
//
// Setup:
//      Run `sage` to enter the interpreter.
//
// Code:
//      ```text
//      3.0
//      float("3.0")
//      ```
//
// JSON
// ----
//
// Setup:
//      Run `node` (or `nodejs`) to enter the JS interpreter.
//
// Code:
//      ```text
//      JSON.parse("3.0")
//      ```
//
// TOML
// ----
//
// Setup:
//      Run `python` to enter the Python interpreter.
//
// Code:
//      ```text
//      import tomlkit
//      tomlkit.parse("a = 3.0")
//      ```
//
// XML
// ---
//
// Setup:
//      Run `python` to enter the Python interpreter.
//
// Code:
//      ```text
//      from lxml import etree
//
//      def validate_xml(xsd, xml):
//          '''Validate XML file against schema'''
//
//          schema = etree.fromstring(xsd)
//          doc = etree.fromstring(xml)
//          xmlschema = etree.XMLSchema(schema)
//
//          return xmlschema.validate(doc)
//
//
//      xsd = b'''<?xml version="1.0" encoding="UTF-8"?>
//      <xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema">
//          <xs:element name="prize" type="xs:float"/>
//      </xs:schema>'''
//
//      xml = b'''<?xml version="1.0" encoding="UTF-8"?>
//      <prize>3.0</prize>
//      '''
//
//      validate_xml(xsd, xml)
//      ```
//
// SQLite
// ------
//
// Setup:
//      Run `sqlite3 :memory:` to enter the sqlite3 interpreter
//      with an in-memory database.
//
// Code:
//      ```text
//      CREATE TABLE stocks (price real);
//      INSERT INTO stocks VALUES (3.0);
//      SELECT * FROM stocks;
//      ```
//
// PostgreSQL
// ----------
//
// Setup:
//      Run `initdb -D db` to create a database data direction,
//      then run `pg_ctl -D db start` to start the server, then run
//      `createdb` to create a user database and `psql` to start the
//      interpreter.
//
// Code:
//      ```text
//      CREATE TABLE stocks (price real);
//      INSERT INTO stocks VALUES (3.0);
//      SELECT * FROM stocks;
//      ```
//
// MySQL
// -----
//
// Setup:
//      Run `mysqld` to start the server, then run `mysql` to start the
//      interpreter.
//
// Code:
//      ```text
//      USE mysql;
//      CREATE TABLE stocks (price real);
//      INSERT INTO stocks VALUES (3.0);
//      SELECT * FROM stocks;
//      ```
//
// MongoDB
// -------
//
// Setup:
//      Run `mongod --dbpath data/db` to start the server, then run
//      `mongo` to start the interpreter.
//
// Code:
//      ```text
//      use mydb
//      db.movie.insert({"name": 3.0})
//      db.movie.find()
//      ```

use core::num;

use crate::error::Error;
use crate::format_builder::NumberFormatBuilder;
use crate::format_flags as flags;

/// Add multiple flags to `SyntaxFormat`.
macro_rules! from_flag {
    ($format:ident, $flag:ident) => {{
        $format & flags::$flag != 0
    }};
}

/// Helper to access features from the packed format struct.
///
/// This contains accessory methods to read the formatting settings
/// without using bitmasks directly on the underlying packed struct.
///
/// Some of the core functionality includes support for:
/// - Digit separators: ignored characters used to make numbers more readable,
///   such as `100,000`.
/// - Non-decimal radixes: writing or parsing numbers written in binary,
///   hexadecimal, or other bases.
/// - Special numbers: disabling support for special floating-point, such as
///   [`NaN`][f64::NAN].
/// - Number components: require signs, significant digits, and more.
///
/// This should always be constructed via [`NumberFormatBuilder`].
/// See [`NumberFormatBuilder`] for the fields for the packed struct.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "format")] {
/// use lexical_util::format::{RUST_LITERAL, NumberFormat};
///
/// let format = NumberFormat::<{ RUST_LITERAL }> {};
/// assert!(format.no_positive_mantissa_sign());
/// assert!(format.no_special());
/// assert!(format.internal_digit_separator());
/// assert!(format.trailing_digit_separator());
/// assert!(format.consecutive_digit_separator());
/// assert!(!format.no_exponent_notation());
/// # }
/// ```
pub struct NumberFormat<const FORMAT: u128>;

#[rustfmt::skip]
impl<const FORMAT: u128> NumberFormat<FORMAT> {
    // CONSTRUCTORS

    /// Create new instance (for methods and validation).
    ///
    /// This uses the same settings as in the `FORMAT` packed struct.
    pub const fn new() -> Self {
        Self {}
    }

    // VALIDATION

    /// Determine if the number format is valid.
    pub const fn is_valid(&self) -> bool {
        self.error().is_success()
    }

    /// Get the error type from the format.
    ///
    /// If [`Error::Success`] is returned, then no error occurred.
    pub const fn error(&self) -> Error {
        format_error_impl(FORMAT)
    }

    // NON-DIGIT SEPARATOR FLAGS & MASKS

    /// If digits are required before the decimal point.
    ///
    /// See [`required_integer_digits`][Self::required_integer_digits].
    pub const REQUIRED_INTEGER_DIGITS: bool = from_flag!(FORMAT, REQUIRED_INTEGER_DIGITS);

    /// Get if digits are required before the decimal point.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to [`false`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `0.1` | ✔️ |
    /// | `1` | ✔️ |
    /// | `.1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn required_integer_digits(&self) -> bool {
        Self::REQUIRED_INTEGER_DIGITS
    }

    /// If digits are required after the decimal point.
    ///
    /// See [`required_fraction_digits`][Self::required_fraction_digits].
    pub const REQUIRED_FRACTION_DIGITS: bool = from_flag!(FORMAT, REQUIRED_FRACTION_DIGITS);

    /// Get if digits are required after the decimal point.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to [`false`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `1` | ✔️ |
    /// | `1.` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn required_fraction_digits(&self) -> bool {
        Self::REQUIRED_FRACTION_DIGITS
    }

    /// If digits are required after the exponent character.
    ///
    /// See [`required_exponent_digits`][Self::required_exponent_digits].
    pub const REQUIRED_EXPONENT_DIGITS: bool = from_flag!(FORMAT, REQUIRED_EXPONENT_DIGITS);

    /// Get if digits are required after the exponent character.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to [`true`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e+3` | ✔️ |
    /// | `1.1e3` | ✔️ |
    /// | `1.1e+` | ❌ |
    /// | `1.1e` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn required_exponent_digits(&self) -> bool {
        Self::REQUIRED_EXPONENT_DIGITS
    }

    /// If significant digits are required.
    ///
    /// See [`required_mantissa_digits`][Self::required_mantissa_digits].
    pub const REQUIRED_MANTISSA_DIGITS: bool = from_flag!(FORMAT, REQUIRED_MANTISSA_DIGITS);

    /// Get if at least 1 significant digit is required.
    ///
    /// If not required, then values like `.` (`0`) are valid, but empty strings
    /// are still invalid. Can only be modified with [`feature`][crate#features]
    /// `format`. Defaults to [`true`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `.` | ✔️ |
    /// | `e10` | ✔️ |
    /// | `.e10` | ✔️ |
    /// | `` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn required_mantissa_digits(&self) -> bool {
        Self::REQUIRED_MANTISSA_DIGITS
    }

    /// If at least 1 digit in the number is required.
    ///
    /// See [`required_digits`][Self::required_digits].
    pub const REQUIRED_DIGITS: bool = from_flag!(FORMAT, REQUIRED_DIGITS);

    /// Get if at least 1 digit in the number is required.
    ///
    /// This requires either [`mantissa`] or [`exponent`] digits.
    ///
    /// [`mantissa`]: Self::required_mantissa_digits
    /// [`exponent`]: Self::required_exponent_digits
    #[inline(always)]
    pub const fn required_digits(&self) -> bool {
        Self::REQUIRED_DIGITS
    }

    /// If a positive sign before the mantissa is not allowed.
    ///
    /// See [`no_positive_mantissa_sign`][Self::no_positive_mantissa_sign].
    pub const NO_POSITIVE_MANTISSA_SIGN: bool = from_flag!(FORMAT, NO_POSITIVE_MANTISSA_SIGN);

    /// Get if a positive sign before the mantissa is not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `-1.1` | ✔️ |
    /// | `+1.1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    #[inline(always)]
    pub const fn no_positive_mantissa_sign(&self) -> bool {
        Self::NO_POSITIVE_MANTISSA_SIGN
    }

    /// If a sign symbol before the mantissa is required.
    ///
    /// See [`required_mantissa_sign`][Self::required_mantissa_sign].
    pub const REQUIRED_MANTISSA_SIGN: bool = from_flag!(FORMAT, REQUIRED_MANTISSA_SIGN);

    /// Get if a sign symbol before the mantissa is required.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ❌ |
    /// | `-1.1` | ✔️ |
    /// | `+1.1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    #[inline(always)]
    pub const fn required_mantissa_sign(&self) -> bool {
        Self::REQUIRED_MANTISSA_SIGN
    }

    /// If exponent notation is not allowed.
    ///
    /// See [`no_exponent_notation`][Self::no_exponent_notation].
    pub const NO_EXPONENT_NOTATION: bool = from_flag!(FORMAT, NO_EXPONENT_NOTATION);

    /// Get if exponent notation is not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `1.1` | ✔️ |
    /// | `1.1e` | ❌ |
    /// | `1.1e5` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Write Float
    #[inline(always)]
    pub const fn no_exponent_notation(&self) -> bool {
        Self::NO_EXPONENT_NOTATION
    }

    /// If a positive sign before the exponent is not allowed.
    ///
    /// See [`no_positive_exponent_sign`][Self::no_positive_exponent_sign].
    pub const NO_POSITIVE_EXPONENT_SIGN: bool = from_flag!(FORMAT, NO_POSITIVE_EXPONENT_SIGN);

    /// Get if a positive sign before the exponent is not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e3` | ✔️ |
    /// | `1.1e-3` | ✔️ |
    /// | `1.1e+3` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Write Float
    #[inline(always)]
    pub const fn no_positive_exponent_sign(&self) -> bool {
        Self::NO_POSITIVE_EXPONENT_SIGN
    }

    /// If a sign symbol before the exponent is required.
    ///
    /// See [`required_exponent_sign`][Self::required_exponent_sign].
    pub const REQUIRED_EXPONENT_SIGN: bool = from_flag!(FORMAT, REQUIRED_EXPONENT_SIGN);

    /// Get if a sign symbol before the exponent is required.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e3` | ❌ |
    /// | `1.1e-3` | ✔️ |
    /// | `1.1e+3` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Write Float
    #[inline(always)]
    pub const fn required_exponent_sign(&self) -> bool {
        Self::REQUIRED_EXPONENT_SIGN
    }

    /// If an exponent without fraction is not allowed.
    ///
    /// See [`no_exponent_without_fraction`][Self::no_exponent_without_fraction].
    pub const NO_EXPONENT_WITHOUT_FRACTION: bool = from_flag!(FORMAT, NO_EXPONENT_WITHOUT_FRACTION);

    /// Get if an exponent without fraction is not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1e3` | ❌ |
    /// | `1.e3` | ❌ |
    /// | `1.1e` | ✔️ |
    /// | `.1e3` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn no_exponent_without_fraction(&self) -> bool {
        Self::NO_EXPONENT_WITHOUT_FRACTION
    }

    /// If special (non-finite) values are not allowed.
    ///
    /// See [`no_special`][Self::no_special].
    pub const NO_SPECIAL: bool = from_flag!(FORMAT, NO_SPECIAL);

    /// Get if special (non-finite) values are not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to `false`.
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `NaN` | ❌ |
    /// | `inf` | ❌ |
    /// | `-Infinity` | ❌ |
    /// | `1.1e` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn no_special(&self) -> bool {
        Self::NO_SPECIAL
    }

    /// If special (non-finite) values are case-sensitive.
    ///
    /// See [`case_sensitive_special`][Self::case_sensitive_special].
    pub const CASE_SENSITIVE_SPECIAL: bool = from_flag!(FORMAT, CASE_SENSITIVE_SPECIAL);

    /// Get if special (non-finite) values are case-sensitive.
    ///
    /// If set to [`true`], then `NaN` and `nan` are treated as the same value
    /// ([Not a Number][f64::NAN]). Can only be modified with
    /// [`feature`][crate#features] `format`. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn case_sensitive_special(&self) -> bool {
        Self::CASE_SENSITIVE_SPECIAL
    }

    /// If leading zeros before an integer are not allowed.
    ///
    /// See [`no_integer_leading_zeros`][Self::no_integer_leading_zeros].
    pub const NO_INTEGER_LEADING_ZEROS: bool = from_flag!(FORMAT, NO_INTEGER_LEADING_ZEROS);

    /// Get if leading zeros before an integer are not allowed.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to [`false`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `01` | ❌ |
    /// | `0` | ✔️ |
    /// | `10` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Integer
    #[inline(always)]
    pub const fn no_integer_leading_zeros(&self) -> bool {
        Self::NO_INTEGER_LEADING_ZEROS
    }

    /// If leading zeros before a float are not allowed.
    ///
    /// See [`no_float_leading_zeros`][Self::no_float_leading_zeros].
    pub const NO_FLOAT_LEADING_ZEROS: bool = from_flag!(FORMAT, NO_FLOAT_LEADING_ZEROS);

    /// Get if leading zeros before a float are not allowed.
    ///
    /// This is before the significant digits of the float, that is, if there is
    /// 1 or more digits in the integral component and the leading digit is 0,
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to [`false`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `01` | ❌ |
    /// | `01.0` | ❌ |
    /// | `0` | ✔️ |
    /// | `10` | ✔️ |
    /// | `0.1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn no_float_leading_zeros(&self) -> bool {
        Self::NO_FLOAT_LEADING_ZEROS
    }

    /// If exponent notation is required.
    ///
    /// See [`required_exponent_notation`][Self::required_exponent_notation].
    pub const REQUIRED_EXPONENT_NOTATION: bool = from_flag!(FORMAT, REQUIRED_EXPONENT_NOTATION);

    /// Get if exponent notation is required.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to [`false`].
    ///
    /// # Examples
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ❌ |
    /// | `1.0` | ❌ |
    /// | `1e3` | ✔️ |
    /// | `1.1e3` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Write Float
    #[inline(always)]
    pub const fn required_exponent_notation(&self) -> bool {
        Self::REQUIRED_EXPONENT_NOTATION
    }

    /// If exponent characters are case-sensitive.
    ///
    /// See [`case_sensitive_exponent`][Self::case_sensitive_exponent].
    pub const CASE_SENSITIVE_EXPONENT: bool = from_flag!(FORMAT, CASE_SENSITIVE_EXPONENT);

    /// Get if exponent characters are case-sensitive.
    ///
    /// If set to [`true`], then the exponent character `e` would be considered
    /// the different from `E`. Can only be modified with
    /// [`feature`][crate#features] `format`. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn case_sensitive_exponent(&self) -> bool {
        Self::CASE_SENSITIVE_EXPONENT
    }

    /// If base prefixes are case-sensitive.
    ///
    /// See [`case_sensitive_base_prefix`][Self::case_sensitive_base_prefix].
    pub const CASE_SENSITIVE_BASE_PREFIX: bool = from_flag!(FORMAT, CASE_SENSITIVE_BASE_PREFIX);

    /// Get if base prefixes are case-sensitive.
    ///
    /// If set to [`true`], then the base prefix `x` would be considered the
    /// different from `X`. Can only be modified with
    /// [`feature`][crate#features] `power-of-two` or `radix` along with
    /// `format`. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn case_sensitive_base_prefix(&self) -> bool {
        Self::CASE_SENSITIVE_BASE_PREFIX
    }

    /// If base suffixes are case-sensitive.
    ///
    /// See [`case_sensitive_base_suffix`][Self::case_sensitive_base_suffix].
    pub const CASE_SENSITIVE_BASE_SUFFIX: bool = from_flag!(FORMAT, CASE_SENSITIVE_BASE_SUFFIX);

    /// Get if base suffixes are case-sensitive.
    ///
    /// If set to [`true`], then the base suffix `x` would be considered the
    /// different from `X`. Can only be modified with
    /// [`feature`][crate#features] `power-of-two` or `radix` along with
    /// `format`. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn case_sensitive_base_suffix(&self) -> bool {
        Self::CASE_SENSITIVE_BASE_SUFFIX
    }

    // DIGIT SEPARATOR FLAGS & MASKS

    /// If digit separators are allowed between integer digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    ///
    /// See [`integer_internal_digit_separator`][Self::integer_internal_digit_separator].
    pub const INTEGER_INTERNAL_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, INTEGER_INTERNAL_DIGIT_SEPARATOR);

    /// Get if digit separators are allowed between integer digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits. Can only be modified with [`feature`][crate#features] `format`.
    /// Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `_` | ❌ |
    /// | `1_1` | ✔️ |
    /// | `1_` | ❌ |
    /// | `_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn integer_internal_digit_separator(&self) -> bool {
        Self::INTEGER_INTERNAL_DIGIT_SEPARATOR
    }

    /// If digit separators are allowed between fraction digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    ///
    /// See [`fraction_internal_digit_separator`][Self::fraction_internal_digit_separator].
    pub const FRACTION_INTERNAL_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, FRACTION_INTERNAL_DIGIT_SEPARATOR);

    /// Get if digit separators are allowed between fraction digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits. Can only be modified with [`feature`][crate#features] `format`.
    /// Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `1._` | ❌ |
    /// | `1.1_1` | ✔️ |
    /// | `1.1_` | ❌ |
    /// | `1._1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn fraction_internal_digit_separator(&self) -> bool {
        Self::FRACTION_INTERNAL_DIGIT_SEPARATOR
    }

    /// If digit separators are allowed between exponent digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    ///
    /// See [`exponent_internal_digit_separator`][Self::exponent_internal_digit_separator].
    pub const EXPONENT_INTERNAL_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, EXPONENT_INTERNAL_DIGIT_SEPARATOR);

    /// Get if digit separators are allowed between exponent digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits. Can only be modified with [`feature`][crate#features] `format`.
    /// Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e1` | ✔️ |
    /// | `1.1e_` | ❌ |
    /// | `1.1e1_1` | ✔️ |
    /// | `1.1e1_` | ❌ |
    /// | `1.1e_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn exponent_internal_digit_separator(&self) -> bool {
        Self::EXPONENT_INTERNAL_DIGIT_SEPARATOR
    }

    /// If digit separators are allowed between digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits.
    ///
    /// See [`internal_digit_separator`][Self::internal_digit_separator].
    pub const INTERNAL_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, INTERNAL_DIGIT_SEPARATOR);

    /// Get if digit separators are allowed between digits.
    ///
    /// This will not consider an input of only the digit separator
    /// to be a valid separator: the digit separator must be surrounded by
    /// digits. This is equivalent to any of [`integer_internal_digit_separator`],
    /// [`fraction_internal_digit_separator`], or
    /// [`exponent_internal_digit_separator`] being set.
    ///
    /// [`integer_internal_digit_separator`]: Self::integer_internal_digit_separator
    /// [`fraction_internal_digit_separator`]: Self::fraction_internal_digit_separator
    /// [`exponent_internal_digit_separator`]: Self::exponent_internal_digit_separator
    #[inline(always)]
    pub const fn internal_digit_separator(&self) -> bool {
        Self::INTERNAL_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed before any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    ///
    /// See [`integer_leading_digit_separator`][Self::integer_leading_digit_separator].
    pub const INTEGER_LEADING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, INTEGER_LEADING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed before any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Can only be modified with
    /// [`feature`][crate#features] `format`. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `_` | ❌ |
    /// | `1_1` | ❌ |
    /// | `1_` | ❌ |
    /// | `_1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn integer_leading_digit_separator(&self) -> bool {
        Self::INTEGER_LEADING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed before any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    ///
    /// See [`fraction_leading_digit_separator`][Self::fraction_leading_digit_separator].
    pub const FRACTION_LEADING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, FRACTION_LEADING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed before any fraction digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Can only be modified with
    /// [`feature`][crate#features] `format`. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `1._` | ❌ |
    /// | `1.1_1` | ❌ |
    /// | `1.1_` | ❌ |
    /// | `1._1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn fraction_leading_digit_separator(&self) -> bool {
        Self::FRACTION_LEADING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed before any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    ///
    /// See [`exponent_leading_digit_separator`][Self::exponent_leading_digit_separator].
    pub const EXPONENT_LEADING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, EXPONENT_LEADING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed before any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Can only be modified with
    /// [`feature`][crate#features] `format`. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e1` | ✔️ |
    /// | `1.1e_` | ❌ |
    /// | `1.1e1_1` | ❌ |
    /// | `1.1e1_` | ❌ |
    /// | `1.1e_1` | ✔️ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn exponent_leading_digit_separator(&self) -> bool {
        Self::EXPONENT_LEADING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed before any digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    ///
    /// See [`leading_digit_separator`][Self::leading_digit_separator].
    pub const LEADING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, LEADING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed before any digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. This is equivalent to
    /// any of [`integer_leading_digit_separator`],
    /// [`fraction_leading_digit_separator`], or
    /// [`exponent_leading_digit_separator`] being set.
    ///
    /// [`integer_leading_digit_separator`]: Self::integer_leading_digit_separator
    /// [`fraction_leading_digit_separator`]: Self::fraction_leading_digit_separator
    /// [`exponent_leading_digit_separator`]: Self::exponent_leading_digit_separator
    #[inline(always)]
    pub const fn leading_digit_separator(&self) -> bool {
        Self::LEADING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed after any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    ///
    /// See [`integer_trailing_digit_separator`][Self::integer_trailing_digit_separator].
    pub const INTEGER_TRAILING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, INTEGER_TRAILING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed after any integer digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Can only be modified with
    /// [`feature`][crate#features] `format`. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `_` | ❌ |
    /// | `1_1` | ❌ |
    /// | `1_` | ✔️ |
    /// | `_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn integer_trailing_digit_separator(&self) -> bool {
        Self::INTEGER_TRAILING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed after any fraction digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    ///
    /// See [`fraction_trailing_digit_separator`][Self::fraction_trailing_digit_separator].
    pub const FRACTION_TRAILING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, FRACTION_TRAILING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed after any fraction digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Can only be modified with
    /// [`feature`][crate#features] `format`. Defaults to [`false`]. # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1` | ✔️ |
    /// | `1._` | ❌ |
    /// | `1.1_1` | ❌ |
    /// | `1.1_` | ✔️ |
    /// | `1._1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn fraction_trailing_digit_separator(&self) -> bool {
        Self::FRACTION_TRAILING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed after any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    ///
    /// See [`exponent_trailing_digit_separator`][Self::exponent_trailing_digit_separator].
    pub const EXPONENT_TRAILING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, EXPONENT_TRAILING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed after any exponent digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. Can only be modified with
    /// [`feature`][crate#features] `format`. Defaults to [`false`].
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1.1e1` | ✔️ |
    /// | `1.1e_` | ❌ |
    /// | `1.1e1_1` | ❌ |
    /// | `1.1e1_` | ✔️ |
    /// | `1.1e_1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn exponent_trailing_digit_separator(&self) -> bool {
        Self::EXPONENT_TRAILING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed after any digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input.
    ///
    /// See [`trailing_digit_separator`][Self::trailing_digit_separator].
    pub const TRAILING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, TRAILING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed after any digits.
    ///
    /// This will consider an input of only the digit separator
    /// to be a identical to empty input. This is equivalent to
    /// any of [`integer_trailing_digit_separator`],
    /// [`fraction_trailing_digit_separator`], or
    /// [`exponent_trailing_digit_separator`] being set.
    ///
    /// [`integer_trailing_digit_separator`]: Self::integer_trailing_digit_separator
    /// [`fraction_trailing_digit_separator`]: Self::fraction_trailing_digit_separator
    /// [`exponent_trailing_digit_separator`]: Self::exponent_trailing_digit_separator
    #[inline(always)]
    pub const fn trailing_digit_separator(&self) -> bool {
        Self::TRAILING_DIGIT_SEPARATOR
    }

    /// If multiple consecutive integer digit separators are allowed.
    ///
    /// See [`integer_consecutive_digit_separator`][Self::integer_consecutive_digit_separator].
    pub const INTEGER_CONSECUTIVE_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, INTEGER_CONSECUTIVE_DIGIT_SEPARATOR);

    /// Get if multiple consecutive integer digit separators are allowed.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed where any
    /// digit separators (leading, trailing, internal) are allowed in the
    /// integer. Can only be modified with [`feature`][crate#features] `format`.
    /// Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn integer_consecutive_digit_separator(&self) -> bool {
        Self::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR
    }

    /// If multiple consecutive fraction digit separators are allowed.
    ///
    /// See [`fraction_consecutive_digit_separator`][Self::fraction_consecutive_digit_separator].
    pub const FRACTION_CONSECUTIVE_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, FRACTION_CONSECUTIVE_DIGIT_SEPARATOR);

    /// Get if multiple consecutive fraction digit separators are allowed.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed where any
    /// digit separators (leading, trailing, internal) are allowed in the
    /// fraction. Can only be modified with [`feature`][crate#features]
    /// `format`. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn fraction_consecutive_digit_separator(&self) -> bool {
        Self::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR
    }

    /// If multiple consecutive exponent digit separators are allowed.
    ///
    /// See [`exponent_consecutive_digit_separator`][Self::exponent_consecutive_digit_separator].
    pub const EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR);

    /// Get if multiple consecutive exponent digit separators are allowed.
    ///
    /// That is, using `_` as a digit separator `__` would be allowed where any
    /// digit separators (leading, trailing, internal) are allowed in the
    /// exponent. Can only be modified with [`feature`][crate#features]
    /// `format`. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn exponent_consecutive_digit_separator(&self) -> bool {
        Self::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR
    }

    /// If multiple consecutive digit separators are allowed.
    ///
    /// See [`consecutive_digit_separator`][Self::consecutive_digit_separator].
    pub const CONSECUTIVE_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, CONSECUTIVE_DIGIT_SEPARATOR);

    /// Get if multiple consecutive digit separators are allowed.
    ///
    /// This is equivalent to any of [`integer_consecutive_digit_separator`],
    /// [`fraction_consecutive_digit_separator`], or
    /// [`exponent_consecutive_digit_separator`] being set.
    ///
    /// [`integer_consecutive_digit_separator`]: Self::integer_consecutive_digit_separator
    /// [`fraction_consecutive_digit_separator`]: Self::fraction_consecutive_digit_separator
    /// [`exponent_consecutive_digit_separator`]: Self::exponent_consecutive_digit_separator
    #[inline(always)]
    pub const fn consecutive_digit_separator(&self) -> bool {
        Self::CONSECUTIVE_DIGIT_SEPARATOR
    }

    /// If any digit separators are allowed in special (non-finite) values.
    ///
    /// See [`special_digit_separator`][Self::special_digit_separator].
    pub const SPECIAL_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, SPECIAL_DIGIT_SEPARATOR);

    /// Get if any digit separators are allowed in special (non-finite) values.
    ///
    /// This enables leading, trailing, internal, and consecutive digit
    /// separators for any special floats: for example, `N__a_N_` is considered
    /// the same as `NaN`. Can only be modified with [`feature`][crate#features]
    /// `format`. Defaults to [`false`].
    ///
    /// # Used For
    ///
    /// - Parse Float
    #[inline(always)]
    pub const fn special_digit_separator(&self) -> bool {
        Self::SPECIAL_DIGIT_SEPARATOR
    }

    // CHARACTERS

    /// The digit separator character in the packed struct.
    ///
    /// See [`digit_separator`][Self::digit_separator].
    pub const DIGIT_SEPARATOR: u8 = flags::digit_separator(FORMAT);

    /// Get the digit separator for the number format.
    ///
    /// Digit separators are frequently used in number literals to group
    /// digits: `1,000,000` is a lot more readable than `1000000`, but
    /// the `,` characters should be ignored in the parsing of the number.
    ///
    /// Can only be modified with [`feature`][crate#features] `format`. Defaults
    /// to `0`, or no digit separators allowed.
    ///
    /// # Examples
    ///
    /// Using a digit separator of `_` (note that the validity
    /// oh where a digit separator can appear depends on the other digit
    /// separator flags).
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `1_4` | ✔️ |
    /// | `+_14` | ✔️ |
    /// | `+14e3_5` | ✔️ |
    /// | `1_d` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn digit_separator(&self) -> u8 {
        Self::DIGIT_SEPARATOR
    }

    /// Get if the format has a digit separator.
    #[inline(always)]
    pub const fn has_digit_separator(&self) -> bool {
        self.digit_separator() != 0
    }

    /// The base prefix character in the packed struct.
    ///
    /// See [`base_prefix`][Self::base_prefix].
    pub const BASE_PREFIX: u8 = flags::base_prefix(FORMAT);

    /// Get the optional character for the base prefix.
    ///
    /// This character will come after a leading zero, so for example
    /// setting the base prefix to `x` means that a leading `0x` will
    /// be ignore, if present. Can only be modified with
    /// [`feature`][crate#features] `power-of-two` or `radix` along with
    /// `format`. Defaults to `0`, or no base prefix allowed.
    ///
    /// # Examples
    ///
    /// Using a base prefix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `0x1` | ✔️ |
    /// | `x1` | ❌ |
    /// | `1` | ✔️ |
    /// | `1x` | ❌ |
    /// | `1x1` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn base_prefix(&self) -> u8 {
        Self::BASE_PREFIX
    }

    /// Get if the format has a base suffix.
    #[inline(always)]
    pub const fn has_base_prefix(&self) -> bool {
        self.base_prefix() != 0
    }

    /// The base suffix character in the packed struct.
    ///
    /// See [`base_suffix`][Self::base_suffix].
    pub const BASE_SUFFIX: u8 = flags::base_suffix(FORMAT);

    /// Get the optional character for the base suffix.
    ///
    /// This character will at the end of the buffer, so for example
    /// setting the base prefix to `x` means that a trailing `x` will
    /// be ignored, if present.  Can only be modified with
    /// [`feature`][crate#features] `power-of-two` or `radix` along with
    /// `format`. Defaults to `0`, or no base suffix allowed.
    ///
    /// # Examples
    ///
    /// Using a base suffix of `x`.
    ///
    /// | Input | Valid? |
    /// |:-:|:-:|
    /// | `1` | ✔️ |
    /// | `1x` | ✔️ |
    /// | `1d` | ❌ |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn base_suffix(&self) -> u8 {
        Self::BASE_SUFFIX
    }

    /// Get if the format has a base suffix.
    #[inline(always)]
    pub const fn has_base_suffix(&self) -> bool {
        self.base_suffix() != 0
    }

    // RADIX

    /// The radix for the significant digits in the packed struct.
    ///
    /// See [`mantissa_radix`][Self::mantissa_radix].
    pub const MANTISSA_RADIX: u32 = flags::mantissa_radix(FORMAT);

    /// Get the radix for mantissa digits.
    ///
    /// This is only used for the significant digits, that is, the integral and
    /// fractional components. Can only be modified with
    /// [`feature`][crate#features] `power-of-two` or `radix`. Defaults
    /// to `10`.
    ///
    /// | Radix | String | Number |
    /// |:-:|:-:|:-:|
    /// | 2 | "10011010010" | 1234 |
    /// | 3 | "1200201" | 1234 |
    /// | 8 | "2322" | 1234 |
    /// | 10 | "1234" | 1234 |
    /// | 16 | "4d2" | 1234 |
    /// | 31 | "18p" | 1234 |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    /// - Write Float
    /// - Write Integer
    #[inline(always)]
    pub const fn mantissa_radix(&self) -> u32 {
        Self::MANTISSA_RADIX
    }

    /// The radix for the significant digits in the packed struct.
    ///
    /// Alias for [`MANTISSA_RADIX`][Self::MANTISSA_RADIX].
    pub const RADIX: u32 = Self::MANTISSA_RADIX;

    /// Get the radix for the significant digits.
    ///
    /// This is an alias for [`mantissa_radix`][Self::mantissa_radix].
    #[inline(always)]
    pub const fn radix(&self) -> u32 {
        Self::RADIX
    }

    /// Get the `radix^2` for the significant digits.
    #[inline(always)]
    pub const fn radix2(&self) -> u32 {
        self.radix().wrapping_mul(self.radix())
    }

    /// Get the `radix^4` for the significant digits.
    #[inline(always)]
    pub const fn radix4(&self) -> u32 {
        self.radix2().wrapping_mul(self.radix2())
    }

    /// Get the `radix^8` for the significant digits.
    #[inline(always)]
    pub const fn radix8(&self) -> u32 {
        // NOTE: radix >= 16 will overflow here but this has no security concerns
        self.radix4().wrapping_mul(self.radix4())
    }

    /// The base for the exponent.
    ///
    /// See [`exponent_base`][Self::exponent_base].
    pub const EXPONENT_BASE: u32 = flags::exponent_base(FORMAT);

    /// Get the radix for the exponent.
    ///
    /// For example, in `1.234e3`, it means `1.234 * 10^3`, and the exponent
    /// base here is 10. Some programming languages, like C, support hex floats
    /// with an exponent base of 2, for example `0x1.8p3`, or `1.5 * 2^3`.
    /// Defaults to `10`. Can only be modified with [`feature`][crate#features]
    /// `power-of-two` or `radix`. Defaults to `10`.
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn exponent_base(&self) -> u32 {
        Self::EXPONENT_BASE
    }

    /// The radix for the exponent digits.
    ///
    /// See [`exponent_radix`][Self::exponent_radix].
    pub const EXPONENT_RADIX: u32 = flags::exponent_radix(FORMAT);

    /// Get the radix for exponent digits.
    ///
    /// This is only used for the exponent digits. We assume the radix for the
    /// significant digits ([`mantissa_radix`][Self::mantissa_radix]) is
    /// 10 as is the exponent base. Defaults to `10`. Can only be modified with
    /// [`feature`][crate#features] `power-of-two` or `radix`. Defaults to `10`.
    ///
    /// | Radix | String | Number |
    /// |:-:|:-:|:-:|
    /// | 2 | "1.234^1100" | 1.234e9 |
    /// | 3 | "1.234^110" | 1.234e9 |
    /// | 8 | "1.234^14" | 1.234e9 |
    /// | 10 | "1.234^12" | 1.234e9 |
    /// | 16 | "1.234^c" | 1.234e9 |
    /// | 31 | "1.234^c" | 1.234e9 |
    ///
    /// # Used For
    ///
    /// - Parse Float
    /// - Parse Integer
    #[inline(always)]
    pub const fn exponent_radix(&self) -> u32 {
        Self::EXPONENT_RADIX
    }

    // FLAGS

    /// Get the flags from the number format.
    ///
    /// This contains all the non-character and non-radix values
    /// in the packed struct.
    #[inline(always)]
    pub const fn flags(&self) -> u128 {
        FORMAT & flags::FLAG_MASK
    }

    /// Get the interface flags from the number format.
    ///
    /// This contains all the flags that dictate code flows, and
    /// therefore excludes logic like case-sensitive characters.
    #[inline(always)]
    pub const fn interface_flags(&self) -> u128 {
        FORMAT & flags::INTERFACE_FLAG_MASK
    }

    /// Get the digit separator flags from the number format.
    #[inline(always)]
    pub const fn digit_separator_flags(&self) -> u128 {
        FORMAT & flags::DIGIT_SEPARATOR_FLAG_MASK
    }

    /// Get the exponent flags from the number format.
    ///
    /// This contains all the flags pertaining to exponent
    /// formats, including digit separators.
    #[inline(always)]
    pub const fn exponent_flags(&self) -> u128 {
        FORMAT & flags::EXPONENT_FLAG_MASK
    }

    /// Get the integer digit separator flags from the number format.
    #[inline(always)]
    pub const fn integer_digit_separator_flags(&self) -> u128 {
        FORMAT & flags::INTEGER_DIGIT_SEPARATOR_FLAG_MASK
    }

    /// Get the fraction digit separator flags from the number format.
    #[inline(always)]
    pub const fn fraction_digit_separator_flags(&self) -> u128 {
        FORMAT & flags::FRACTION_DIGIT_SEPARATOR_FLAG_MASK
    }

    /// Get the exponent digit separator flags from the number format.
    #[inline(always)]
    pub const fn exponent_digit_separator_flags(&self) -> u128 {
        FORMAT & flags::EXPONENT_DIGIT_SEPARATOR_FLAG_MASK
    }

    // BUILDER

    /// Get [`NumberFormatBuilder`] as a static function.
    #[inline(always)]
    pub const fn builder() -> NumberFormatBuilder {
        NumberFormatBuilder::new()
    }

    /// Create [`NumberFormatBuilder`] using existing values.
    #[inline(always)]
    pub const fn rebuild() -> NumberFormatBuilder {
        NumberFormatBuilder::rebuild(FORMAT)
    }
}

impl<const FORMAT: u128> Default for NumberFormat<FORMAT> {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the error type from the format.
#[inline(always)]
#[allow(clippy::if_same_then_else)] // reason="all are different logic conditions"
pub(crate) const fn format_error_impl(format: u128) -> Error {
    if !flags::is_valid_radix(flags::mantissa_radix(format)) {
        Error::InvalidMantissaRadix
    } else if !flags::is_valid_radix(flags::exponent_base(format)) {
        Error::InvalidExponentBase
    } else if !flags::is_valid_radix(flags::exponent_radix(format)) {
        Error::InvalidExponentRadix
    } else if !flags::is_valid_digit_separator(format) {
        Error::InvalidDigitSeparator
    } else if !flags::is_valid_base_prefix(format) {
        Error::InvalidBasePrefix
    } else if !flags::is_valid_base_suffix(format) {
        Error::InvalidBaseSuffix
    } else if !flags::is_valid_punctuation(format) {
        Error::InvalidPunctuation
    } else if !flags::is_valid_exponent_flags(format) {
        Error::InvalidExponentFlags
    } else if from_flag!(format, NO_POSITIVE_MANTISSA_SIGN)
        && from_flag!(format, REQUIRED_MANTISSA_SIGN)
    {
        Error::InvalidMantissaSign
    } else if from_flag!(format, NO_POSITIVE_EXPONENT_SIGN)
        && from_flag!(format, REQUIRED_EXPONENT_SIGN)
    {
        Error::InvalidExponentSign
    } else if from_flag!(format, NO_SPECIAL) && from_flag!(format, CASE_SENSITIVE_SPECIAL) {
        Error::InvalidSpecial
    } else if from_flag!(format, NO_SPECIAL) && from_flag!(format, SPECIAL_DIGIT_SEPARATOR) {
        Error::InvalidSpecial
    } else if (format & flags::INTEGER_DIGIT_SEPARATOR_FLAG_MASK)
        == flags::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR
    {
        Error::InvalidConsecutiveIntegerDigitSeparator
    } else if (format & flags::FRACTION_DIGIT_SEPARATOR_FLAG_MASK)
        == flags::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR
    {
        Error::InvalidConsecutiveFractionDigitSeparator
    } else if (format & flags::EXPONENT_DIGIT_SEPARATOR_FLAG_MASK)
        == flags::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR
    {
        Error::InvalidConsecutiveExponentDigitSeparator
    } else {
        Error::Success
    }
}

// PRE-DEFINED CONSTANTS
// ---------------------
//
// Sample Format Shorthand:
// ------------------------
//
// The format shorthand lists the test cases, and if applicable,
// the digit separator character. For example, the shorthand
// `[134-_]` specifies it passes tests 1, 3, and 4, and uses
// `'_'` as a digit-separator character. Meanwhile, `[0]` means it
// passes test 0, and has no digit separator.

// RUST LITERAL [4569ABFGHIJKMN-_]
/// Number format for a [`Rust`] literal floating-point number.
///
/// [`Rust`]: https://www.rust-lang.org/
#[rustfmt::skip]
pub const RUST_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .required_digits(true)
    .no_positive_mantissa_sign(true)
    .no_special(true)
    .internal_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

// RUST STRING [0134567MN]
/// Number format to parse a [`Rust`] float from string.
///
/// [`Rust`]: https://www.rust-lang.org/
#[rustfmt::skip]
pub const RUST_STRING: u128 = NumberFormatBuilder::new().build_strict();

/// Number format for a [`Python`] literal floating-point number.
///
/// [`Python`]: https://www.python.org/
pub const PYTHON_LITERAL: u128 = PYTHON3_LITERAL;

/// Number format to parse a [`Python`] float from string.
///
/// [`Python`]: https://www.python.org/
pub const PYTHON_STRING: u128 = PYTHON3_STRING;

/// Number format for a [`Python3`] literal floating-point number.
///
/// [`Python3`]: https://www.python.org/
pub const PYTHON3_LITERAL: u128 = PYTHON36_LITERAL;

// PYTHON3 STRING [0134567MN]
/// Number format to parse a [`Python3`] float from string.
///
/// [`Python3`]: https://www.python.org/
#[rustfmt::skip]
pub const PYTHON3_STRING: u128 = NumberFormatBuilder::new().build_strict();

// PYTHON3.6+ LITERAL [013456N-_]
/// Number format for a [`Python3.6`] or higher literal floating-point number.
///
/// [`Python3.6`]: https://www.python.org/downloads/release/python-360/
#[rustfmt::skip]
pub const PYTHON36_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .no_special(true)
    .no_integer_leading_zeros(true)
    .internal_digit_separator(true)
    .build_strict();

// PYTHON3.5- LITERAL [013456N]
/// Number format for a [`Python3.5`] or lower literal floating-point number.
///
/// [`Python3.5`]: https://www.python.org/downloads/release/python-350/
#[rustfmt::skip]
pub const PYTHON35_LITERAL: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .no_integer_leading_zeros(true)
    .build_strict();

// PYTHON2 LITERAL [013456MN]
/// Number format for a [`Python2`] literal floating-point number.
///
/// [`Python2`]: https://www.python.org/downloads/release/python-270/
#[rustfmt::skip]
pub const PYTHON2_LITERAL: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build_strict();

// PYTHON2 STRING [0134567MN]
/// Number format to parse a [`Python2`] float from string.
///
/// [`Python2`]: https://www.python.org/downloads/release/python-270/
#[rustfmt::skip]
pub const PYTHON2_STRING: u128 = NumberFormatBuilder::new().build_strict();

/// Number format for a [`C++`] literal floating-point number.
///
/// [`C++`]: https://en.cppreference.com/w/
pub const CXX_LITERAL: u128 = CXX20_LITERAL;

/// Number format to parse a [`C++`] float from string.
///
/// [`C++`]: https://en.cppreference.com/w/
pub const CXX_STRING: u128 = CXX20_STRING;

/// Number format for a [`C++`] literal hexadecimal floating-point number.
///
/// [`C++`]: https://en.cppreference.com/w/
#[cfg(feature = "power-of-two")]
pub const CXX_HEX_LITERAL: u128 = CXX20_HEX_LITERAL;

/// Number format to parse a [`C++`] hexadecimal float from string.
///
/// [`C++`]: https://en.cppreference.com/w/
#[cfg(feature = "power-of-two")]
pub const CXX_HEX_STRING: u128 = CXX20_HEX_STRING;

// C++20 LITERAL [013456789ABMN-']
/// Number format for a [`C++20`] literal floating-point number.
///
/// [`C++20`]: https://en.cppreference.com/w/cpp/20
#[rustfmt::skip]
pub const CXX20_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'\''))
    .case_sensitive_special(true)
    .internal_digit_separator(true)
    .build_strict();

// C++20 STRING [0134567MN]
/// Number format for a [`C++20`] string floating-point number.
///
/// [`C++20`]: https://en.cppreference.com/w/cpp/20
#[rustfmt::skip]
pub const CXX20_STRING: u128 = NumberFormatBuilder::new().build_strict();

// C++20 HEX LITERAL [013456789ABMN-']
/// Number format for a [`C++20`] literal hexadecimal floating-point number.
///
/// [`C++20`]: https://en.cppreference.com/w/cpp/20
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const CXX20_HEX_LITERAL: u128 = NumberFormatBuilder::new()
    .required_exponent_notation(true)
    .digit_separator(num::NonZeroU8::new(b'\''))
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .case_sensitive_special(true)
    .internal_digit_separator(true)
    .build_strict();

// C++20 HEX STRING [0134567MN]
/// Number format for a [`C++20`] string hexadecimal floating-point number.
///
/// [`C++20`]: https://en.cppreference.com/w/cpp/20
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const CXX20_HEX_STRING: u128 = NumberFormatBuilder::new()
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build_strict();

// C++17 LITERAL [013456789ABMN-']
/// Number format for a [`C++17`] literal floating-point number.
///
/// [`C++17`]: https://en.cppreference.com/w/cpp/17
#[rustfmt::skip]
pub const CXX17_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'\''))
    .case_sensitive_special(true)
    .internal_digit_separator(true)
    .build_strict();

// C++17 STRING [0134567MN]
/// Number format for a [`C++17`] string floating-point number.
///
/// [`C++17`]: https://en.cppreference.com/w/cpp/17
#[rustfmt::skip]
pub const CXX17_STRING: u128 = NumberFormatBuilder::new().build_strict();

// C++17 HEX LITERAL [013456789ABMN-']
/// Number format for a [`C++17`] literal hexadecimal floating-point number.
///
/// [`C++17`]: https://en.cppreference.com/w/cpp/17
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const CXX17_HEX_LITERAL: u128 = NumberFormatBuilder::new()
    .required_exponent_notation(true)
    .digit_separator(num::NonZeroU8::new(b'\''))
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .case_sensitive_special(true)
    .internal_digit_separator(true)
    .build_strict();

// C++17 HEX STRING [0134567MN]
/// Number format for a [`C++17`] string hexadecimal floating-point number.
///
/// [`C++17`]: https://en.cppreference.com/w/cpp/17
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const CXX17_HEX_STRING: u128 = NumberFormatBuilder::new()
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build_strict();

// C++14 LITERAL [013456789ABMN-']
/// Number format for a [`C++14`] literal floating-point number.
///
/// [`C++14`]: https://en.cppreference.com/w/cpp/14
#[rustfmt::skip]
pub const CXX14_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'\''))
    .case_sensitive_special(true)
    .internal_digit_separator(true)
    .build_strict();

// C++14 STRING [0134567MN]
/// Number format for a [`C++14`] string floating-point number.
///
/// [`C++14`]: https://en.cppreference.com/w/cpp/14
#[rustfmt::skip]
pub const CXX14_STRING: u128 = NumberFormatBuilder::new().build_strict();

// C++14 HEX STRING [0134567MN]
/// Number format for a [`C++14`] string hexadecimal floating-point number.
///
/// [`C++14`]: https://en.cppreference.com/w/cpp/14
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const CXX14_HEX_STRING: u128 = NumberFormatBuilder::new()
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build_strict();

// C++11 LITERAL [01345678MN]
/// Number format for a [`C++11`] literal floating-point number.
///
/// [`C++11`]: https://en.cppreference.com/w/cpp/11
#[rustfmt::skip]
pub const CXX11_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// C++11 STRING [0134567MN]
/// Number format for a [`C++11`] string floating-point number.
///
/// [`C++11`]: https://en.cppreference.com/w/cpp/11
#[rustfmt::skip]
pub const CXX11_STRING: u128 = NumberFormatBuilder::new().build_strict();

// C++11 HEX STRING [0134567MN]
/// Number format for a [`C++11`] string hexadecimal floating-point number.
///
/// [`C++11`]: https://en.cppreference.com/w/cpp/11
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const CXX11_HEX_STRING: u128 = NumberFormatBuilder::new()
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build_strict();

// C++03 LITERAL [01345678MN]
/// Number format for a [`C++03`] literal floating-point number.
///
/// [`C++03`]: https://en.wikipedia.org/wiki/C%2B%2B03
#[rustfmt::skip]
pub const CXX03_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// C++03 STRING [0134567MN]
/// Number format for a [`C++03`] string floating-point number.
///
/// [`C++03`]: https://en.wikipedia.org/wiki/C%2B%2B03
#[rustfmt::skip]
pub const CXX03_STRING: u128 = NumberFormatBuilder::new().build_strict();

// C++98 LITERAL [01345678MN]
/// Number format for a [`C++98`] literal floating-point number.
///
/// [`C++98`]: https://en.cppreference.com/w/
#[rustfmt::skip]
pub const CXX98_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// C++98 STRING [0134567MN]
/// Number format for a [`C++98`] string floating-point number.
///
/// [`C++98`]: https://en.cppreference.com/w/
#[rustfmt::skip]
pub const CXX98_STRING: u128 = NumberFormatBuilder::new().build_strict();

/// Number format for a [`C`] literal floating-point number.
///
/// [`C`]: https://en.cppreference.com/w/c
pub const C_LITERAL: u128 = C18_LITERAL;

/// Number format to parse a [`C`] float from string.
///
/// [`C`]: https://en.cppreference.com/w/c
pub const C_STRING: u128 = C18_STRING;

/// Number format for a [`C`] literal hexadecimal floating-point number.
///
/// [`C`]: https://en.cppreference.com/w/c
#[cfg(feature = "power-of-two")]
pub const C_HEX_LITERAL: u128 = C18_HEX_LITERAL;

/// Number format to parse a [`C`] hexadecimal float from string.
///
/// [`C`]: https://en.cppreference.com/w/c
#[cfg(feature = "power-of-two")]
pub const C_HEX_STRING: u128 = C18_HEX_STRING;

// C18 LITERAL [01345678MN]
/// Number format for a [`C18`] literal floating-point number.
///
/// [`C18`]: https://en.cppreference.com/w/c/17
#[rustfmt::skip]
pub const C18_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// C18 STRING [0134567MN]
/// Number format for a [`C18`] string floating-point number.
///
/// [`C18`]: https://en.cppreference.com/w/c/17
#[rustfmt::skip]
pub const C18_STRING: u128 = NumberFormatBuilder::new().build_strict();

// C18 HEX LITERAL [01345678MN]
/// Number format for a [`C18`] literal hexadecimal floating-point number.
///
/// [`C18`]: https://en.cppreference.com/w/c/17
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const C18_HEX_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .required_exponent_notation(true)
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build_strict();

// C18 HEX STRING [0134567MN]
/// Number format for a [`C18`] string hexadecimal floating-point number.
///
/// [`C18`]: https://en.cppreference.com/w/c/17
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const C18_HEX_STRING: u128 = NumberFormatBuilder::new()
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build_strict();

// C11 LITERAL [01345678MN]
/// Number format for a [`C11`] literal floating-point number.
///
/// [`C11`]: https://en.cppreference.com/w/c/11
#[rustfmt::skip]
pub const C11_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// C11 STRING [0134567MN]
/// Number format for a [`C11`] string floating-point number.
///
/// [`C11`]: https://en.cppreference.com/w/c/11
#[rustfmt::skip]
pub const C11_STRING: u128 = NumberFormatBuilder::new().build_strict();

// C11 HEX LITERAL [01345678MN]
/// Number format for a [`C11`] literal hexadecimal floating-point number.
///
/// [`C11`]: https://en.cppreference.com/w/c/11
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const C11_HEX_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .required_exponent_notation(true)
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build_strict();

// C11 HEX STRING [0134567MN]
/// Number format for a [`C11`] string hexadecimal floating-point number.
///
/// [`C11`]: https://en.cppreference.com/w/c/11
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const C11_HEX_STRING: u128 = NumberFormatBuilder::new()
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build_strict();

// C99 LITERAL [01345678MN]
/// Number format for a [`C99`] literal floating-point number.
///
/// [`C99`]: https://en.cppreference.com/w/c/99
#[rustfmt::skip]
pub const C99_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// C99 STRING [0134567MN]
/// Number format for a [`C99`] string floating-point number.
///
/// [`C99`]: https://en.cppreference.com/w/c/99
#[rustfmt::skip]
pub const C99_STRING: u128 = NumberFormatBuilder::new().build_strict();

// C99 HEX LITERAL [01345678MN]
/// Number format for a [`C99`] literal hexadecimal floating-point number.
///
/// [`C99`]: https://en.cppreference.com/w/c/99
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const C99_HEX_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .required_exponent_notation(true)
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build_strict();

// C99 HEX STRING [0134567MN]
/// Number format for a [`C99`] string hexadecimal floating-point number.
///
/// [`C99`]: https://en.cppreference.com/w/c/99
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const C99_HEX_STRING: u128 = NumberFormatBuilder::new()
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build_strict();

// C90 LITERAL [013456MN]
/// Number format for a [`C90`] literal floating-point number.
///
/// [`C90`]: https://en.cppreference.com/w/c
#[rustfmt::skip]
pub const C90_LITERAL: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build_strict();

// C90 STRING [0134567MN]
/// Number format for a [`C90`] string floating-point number.
///
/// [`C90`]: https://en.cppreference.com/w/c
#[rustfmt::skip]
pub const C90_STRING: u128 = NumberFormatBuilder::new().build_strict();

// C90 HEX STRING [0134567MN]
/// Number format for a [`C90`] string hexadecimal floating-point number.
///
/// [`C90`]: https://en.cppreference.com/w/c
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const C90_HEX_STRING: u128 = NumberFormatBuilder::new()
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build_strict();

// C89 LITERAL [013456MN]
/// Number format for a [`C89`] literal floating-point number.
///
/// [`C89`]: https://en.cppreference.com/w/c
#[rustfmt::skip]
pub const C89_LITERAL: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build_strict();

// C89 STRING [0134567MN]
/// Number format for a [`C89`] string floating-point number.
///
/// [`C89`]: https://en.cppreference.com/w/c
#[rustfmt::skip]
pub const C89_STRING: u128 = NumberFormatBuilder::new().build_strict();

// C89 HEX STRING [0134567MN]
/// Number format for a [`C89`] string hexadecimal floating-point number.
///
/// [`C89`]: https://en.cppreference.com/w/c
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const C89_HEX_STRING: u128 = NumberFormatBuilder::new()
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build_strict();

// RUBY LITERAL [345689AMN-_]
/// Number format for a [`Ruby`] literal floating-point number.
///
/// [`Ruby`]: https://www.ruby-lang.org/en/
#[rustfmt::skip]
pub const RUBY_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .required_exponent_sign(true)
    .required_digits(true)
    .no_special(true)
    .no_integer_leading_zeros(true)
    .no_float_leading_zeros(true)
    .internal_digit_separator(true)
    .build_strict();

// RUBY OCTAL LITERAL [345689AN-_]
/// Number format for an octal [`Ruby`] literal floating-point number.
///
/// [`Ruby`]: https://www.ruby-lang.org/en/
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const RUBY_OCTAL_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .mantissa_radix(8)
    .required_digits(true)
    .no_special(true)
    .internal_digit_separator(true)
    .build_strict();

// RUBY STRING [01234569ABMN-_]
// Note: Amazingly, Ruby 1.8+ do not allow parsing special values.
/// Number format to parse a [`Ruby`] float from string.
///
/// [`Ruby`]: https://www.ruby-lang.org/en/
#[rustfmt::skip]
pub const RUBY_STRING: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .no_special(true)
    .internal_digit_separator(true)
    .build_strict();

// SWIFT LITERAL [34569ABFGHIJKMN-_]
/// Number format for a [`Swift`] literal floating-point number.
///
/// [`Swift`]: https://developer.apple.com/swift/
#[rustfmt::skip]
pub const SWIFT_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .required_digits(true)
    .no_special(true)
    .internal_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

// SWIFT STRING [134567MN]
/// Number format to parse a [`Swift`] float from string.
///
/// [`Swift`]: https://developer.apple.com/swift/
#[rustfmt::skip]
pub const SWIFT_STRING: u128 = NumberFormatBuilder::new()
    .required_fraction_digits(true)
    .build_strict();

// GO LITERAL [13456MN]
/// Number format for a [`Golang`] literal floating-point number.
///
/// [`Golang`]: https://go.dev/
#[rustfmt::skip]
pub const GO_LITERAL: u128 = NumberFormatBuilder::new()
    .required_fraction_digits(true)
    .no_special(true)
    .build_strict();

// GO STRING [134567MN]
/// Number format to parse a [`Golang`] float from string.
///
/// [`Golang`]: https://go.dev/
#[rustfmt::skip]
pub const GO_STRING: u128 = NumberFormatBuilder::new()
    .required_fraction_digits(true)
    .build_strict();

// HASKELL LITERAL [456MN]
/// Number format for a [`Haskell`] literal floating-point number.
///
/// [`Haskell`]: https://www.haskell.org/
#[rustfmt::skip]
pub const HASKELL_LITERAL: u128 = NumberFormatBuilder::new()
    .required_digits(true)
    .no_positive_mantissa_sign(true)
    .no_special(true)
    .build_strict();

// HASKELL STRING [45678MN]
/// Number format to parse a [`Haskell`] float from string.
///
/// [`Haskell`]: https://www.haskell.org/
#[rustfmt::skip]
pub const HASKELL_STRING: u128 = NumberFormatBuilder::new()
    .required_digits(true)
    .no_positive_mantissa_sign(true)
    .case_sensitive_special(true)
    .build_strict();

// JAVASCRIPT LITERAL [01345678M]
/// Number format for a [`Javascript`] literal floating-point number.
///
/// [`Javascript`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript
#[rustfmt::skip]
pub const JAVASCRIPT_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .no_float_leading_zeros(true)
    .build_strict();

// JAVASCRIPT STRING [012345678MN]
/// Number format to parse a [`Javascript`] float from string.
///
/// [`Javascript`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript
#[rustfmt::skip]
pub const JAVASCRIPT_STRING: u128 = NumberFormatBuilder::new()
    .required_exponent_digits(false)
    .case_sensitive_special(true)
    .build_strict();

// PERL LITERAL [0134569ABDEFGHIJKMN-_]
/// Number format for a [`Perl`] literal floating-point number.
///
/// [`Perl`]: https://www.perl.org/
#[rustfmt::skip]
pub const PERL_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .no_special(true)
    .internal_digit_separator(true)
    .fraction_leading_digit_separator(true)
    .exponent_leading_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

// PERL STRING [01234567MN]
/// Number format to parse a [`Perl`] float from string.
///
/// [`Perl`]: https://www.perl.org/
pub const PERL_STRING: u128 = PERMISSIVE;

// PHP LITERAL [01345678MN]
/// Number format for a [`PHP`] literal floating-point number.
///
/// [`PHP`]: https://www.php.net/
#[rustfmt::skip]
pub const PHP_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// PHP STRING [0123456MN]
/// Number format to parse a [`PHP`] float from string.
///
/// [`PHP`]: https://www.php.net/
#[rustfmt::skip]
pub const PHP_STRING: u128 = NumberFormatBuilder::new()
    .required_exponent_digits(false)
    .no_special(true)
    .build_strict();

// JAVA LITERAL [0134569ABIJKMN-_]
/// Number format for a [`Java`] literal floating-point number.
///
/// [`Java`]: https://www.java.com/en/
#[rustfmt::skip]
pub const JAVA_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .no_special(true)
    .internal_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

// JAVA STRING [01345678MN]
/// Number format to parse a [`Java`] float from string.
///
/// [`Java`]: https://www.java.com/en/
#[rustfmt::skip]
pub const JAVA_STRING: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// R LITERAL [01345678MN]
/// Number format for an [`R`] literal floating-point number.
///
/// [`R`]: https://www.r-project.org/
#[rustfmt::skip]
pub const R_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// R STRING [01234567MN]
/// Number format to parse an [`R`] float from string.
///
/// [`R`]: https://www.r-project.org/
pub const R_STRING: u128 = PERMISSIVE;

// KOTLIN LITERAL [0134569ABIJKN-_]
/// Number format for a [`Kotlin`] literal floating-point number.
///
/// [`Kotlin`]: https://kotlinlang.org/
#[rustfmt::skip]
pub const KOTLIN_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .no_special(true)
    .no_integer_leading_zeros(true)
    .internal_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

// KOTLIN STRING [0134568MN]
/// Number format to parse a [`Kotlin`] float from string.
///
/// [`Kotlin`]: https://kotlinlang.org/
#[rustfmt::skip]
pub const KOTLIN_STRING: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// JULIA LITERAL [01345689AMN-_]
/// Number format for a [`Julia`] literal floating-point number.
///
/// [`Julia`]: https://julialang.org/
#[rustfmt::skip]
pub const JULIA_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .case_sensitive_special(true)
    .integer_internal_digit_separator(true)
    .fraction_internal_digit_separator(true)
    .build_strict();

// JULIA STRING [01345678MN]
/// Number format to parse a [`Julia`] float from string.
///
/// [`Julia`]: https://julialang.org/
#[rustfmt::skip]
pub const JULIA_STRING: u128 = NumberFormatBuilder::new().build_strict();

// JULIA HEX LITERAL [01345689AMN-_]
/// Number format for a [`Julia`] literal floating-point number.
///
/// [`Julia`]: https://julialang.org/
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const JULIA_HEX_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .case_sensitive_special(true)
    .integer_internal_digit_separator(true)
    .fraction_internal_digit_separator(true)
    .build_strict();

// JULIA HEX STRING [01345678MN]
/// Number format to parse a [`Julia`] float from string.
///
/// [`Julia`]: https://julialang.org/
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const JULIA_HEX_STRING: u128 = NumberFormatBuilder::new()
    .mantissa_radix(16)
    .exponent_base(num::NonZeroU8::new(2))
    .exponent_radix(num::NonZeroU8::new(10))
    .build_strict();

/// Number format for a [`C#`] literal floating-point number.
///
/// [`C#`]: https://learn.microsoft.com/en-us/dotnet/csharp/
pub const CSHARP_LITERAL: u128 = CSHARP7_LITERAL;

/// Number format to parse a [`C#`] float from string.
///
/// [`C#`]: https://learn.microsoft.com/en-us/dotnet/csharp/
pub const CSHARP_STRING: u128 = CSHARP7_STRING;

// CSHARP7 LITERAL [034569ABIJKMN-_]
/// Number format for a [`C#7`] literal floating-point number.
///
/// [`C#7`]: https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history#c-version-73
#[rustfmt::skip]
pub const CSHARP7_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .required_fraction_digits(true)
    .no_special(true)
    .internal_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

// CSHARP7 STRING [0134568MN]
/// Number format to parse a [`C#7`] float from string.
///
/// [`C#7`]: https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history#c-version-73
#[rustfmt::skip]
pub const CSHARP7_STRING: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// CSHARP6 LITERAL [03456MN]
/// Number format for a [`C#6`] literal floating-point number.
///
/// [`C#6`]: https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history#c-version-60
#[rustfmt::skip]
pub const CSHARP6_LITERAL: u128 = NumberFormatBuilder::new()
    .required_fraction_digits(true)
    .no_special(true)
    .build_strict();

// CSHARP6 STRING [0134568MN]
/// Number format to parse a [`C#6`] float from string.
///
/// [`C#6`]: https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history#c-version-60
#[rustfmt::skip]
pub const CSHARP6_STRING: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// CSHARP5 LITERAL [03456MN]
/// Number format for a [`C#5`] literal floating-point number.
///
/// [`C#5`]: https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history#c-version-50
#[rustfmt::skip]
pub const CSHARP5_LITERAL: u128 = NumberFormatBuilder::new()
    .required_fraction_digits(true)
    .no_special(true)
    .build_strict();

// CSHARP5 STRING [0134568MN]
/// Number format to parse a [`C#5`] float from string.
///
/// [`C#5`]: https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history#c-version-50
#[rustfmt::skip]
pub const CSHARP5_STRING: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// CSHARP4 LITERAL [03456MN]
/// Number format for a [`C#4`] literal floating-point number.
///
/// [`C#4`]: https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history#c-version-40
#[rustfmt::skip]
pub const CSHARP4_LITERAL: u128 = NumberFormatBuilder::new()
    .required_fraction_digits(true)
    .no_special(true)
    .build_strict();

// CSHARP4 STRING [0134568MN]
/// Number format to parse a [`C#4`] float from string.
///
/// [`C#4`]: https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history#c-version-40
#[rustfmt::skip]
pub const CSHARP4_STRING: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// CSHARP3 LITERAL [03456MN]
/// Number format for a [`C#3`] literal floating-point number.
///
/// [`C#3`]: https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history#c-version-30
#[rustfmt::skip]
pub const CSHARP3_LITERAL: u128 = NumberFormatBuilder::new()
    .required_fraction_digits(true)
    .no_special(true)
    .build_strict();

// CSHARP3 STRING [0134568MN]
/// Number format to parse a [`C#3`] float from string.
///
/// [`C#3`]: https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history#c-version-30
#[rustfmt::skip]
pub const CSHARP3_STRING: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// CSHARP2 LITERAL [03456MN]
/// Number format for a [`C#2`] literal floating-point number.
///
/// [`C#2`]: https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history#c-version-20
#[rustfmt::skip]
pub const CSHARP2_LITERAL: u128 = NumberFormatBuilder::new()
    .required_fraction_digits(true)
    .no_special(true)
    .build_strict();

// CSHARP2 STRING [0134568MN]
/// Number format to parse a [`C#2`] float from string.
///
/// [`C#2`]: https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history#c-version-20
#[rustfmt::skip]
pub const CSHARP2_STRING: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// CSHARP1 LITERAL [03456MN]
/// Number format for a [`C#1`] literal floating-point number.
///
/// [`C#1`]: https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history#c-version-12-1
#[rustfmt::skip]
pub const CSHARP1_LITERAL: u128 = NumberFormatBuilder::new()
    .required_fraction_digits(true)
    .no_special(true)
    .build_strict();

// CSHARP1 STRING [0134568MN]
/// Number format to parse a [`C#1`] float from string.
///
/// [`C#1`]: https://learn.microsoft.com/en-us/dotnet/csharp/whats-new/csharp-version-history#c-version-12-1
#[rustfmt::skip]
pub const CSHARP1_STRING: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// KAWA LITERAL [013456MN]
/// Number format for a [`Kawa`] literal floating-point number.
///
/// [`Kawa`]: https://www.gnu.org/software/kawa/
#[rustfmt::skip]
pub const KAWA_LITERAL: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build_strict();

// KAWA STRING [013456MN]
/// Number format to parse a [`Kawa`] float from string.
///
/// [`Kawa`]: https://www.gnu.org/software/kawa/
#[rustfmt::skip]
pub const KAWA_STRING: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build_strict();

// GAMBITC LITERAL [013456MN]
/// Number format for a [`Gambit-C`] literal floating-point number.
///
/// [`Gambit-C`]: https://gambitscheme.org/
#[rustfmt::skip]
pub const GAMBITC_LITERAL: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build_strict();

// GAMBITC STRING [013456MN]
/// Number format to parse a [`Gambit-C`] float from string.
///
/// [`Gambit-C`]: https://gambitscheme.org/
#[rustfmt::skip]
pub const GAMBITC_STRING: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build_strict();

// GUILE LITERAL [013456MN]
/// Number format for a [`Guile`] literal floating-point number.
///
/// [`Guile`]: https://www.gnu.org/software/guile/
#[rustfmt::skip]
pub const GUILE_LITERAL: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build_strict();

// GUILE STRING [013456MN]
/// Number format to parse a [`Guile`] float from string.
///
/// [`Guile`]: https://www.gnu.org/software/guile/
#[rustfmt::skip]
pub const GUILE_STRING: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build_strict();

// CLOJURE LITERAL [13456MN]
/// Number format for a [`Clojure`] literal floating-point number.
///
/// [`Clojure`]: https://clojure.org/
#[rustfmt::skip]
pub const CLOJURE_LITERAL: u128 = NumberFormatBuilder::new()
    .required_integer_digits(true)
    .no_special(true)
    .build_strict();

// CLOJURE STRING [01345678MN]
/// Number format to parse a [`Clojure`] float from string.
///
/// [`Clojure`]: https://clojure.org/
#[rustfmt::skip]
pub const CLOJURE_STRING: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// ERLANG LITERAL [34578MN]
/// Number format for an [`Erlang`] literal floating-point number.
///
/// [`Erlang`]: https://www.erlang.org/
#[rustfmt::skip]
pub const ERLANG_LITERAL: u128 = NumberFormatBuilder::new()
    .required_digits(true)
    .no_exponent_without_fraction(true)
    .case_sensitive_special(true)
    .build_strict();

// ERLANG STRING [345MN]
/// Number format to parse an [`Erlang`] float from string.
///
/// [`Erlang`]: https://www.erlang.org/
#[rustfmt::skip]
pub const ERLANG_STRING: u128 = NumberFormatBuilder::new()
    .required_digits(true)
    .no_exponent_without_fraction(true)
    .no_special(true)
    .build_strict();

// ELM LITERAL [456]
/// Number format for an [`Elm`] literal floating-point number.
///
/// [`Elm`]: https://elm-lang.org/
#[rustfmt::skip]
pub const ELM_LITERAL: u128 = NumberFormatBuilder::new()
    .required_digits(true)
    .no_positive_mantissa_sign(true)
    .no_integer_leading_zeros(true)
    .no_float_leading_zeros(true)
    .no_special(true)
    .build_strict();

// ELM STRING [01345678MN]
// Note: There is no valid representation of NaN, just Infinity.
/// Number format to parse an [`Elm`] float from string.
///
/// [`Elm`]: https://elm-lang.org/
#[rustfmt::skip]
pub const ELM_STRING: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// SCALA LITERAL [3456]
/// Number format for a [`Scala`] literal floating-point number.
///
/// [`Scala`]: https://www.scala-lang.org/
#[rustfmt::skip]
pub const SCALA_LITERAL: u128 = NumberFormatBuilder::new()
    .required_digits(true)
    .no_special(true)
    .no_integer_leading_zeros(true)
    .no_float_leading_zeros(true)
    .build_strict();

// SCALA STRING [01345678MN]
/// Number format to parse a [`Scala`] float from string.
///
/// [`Scala`]: https://www.scala-lang.org/
#[rustfmt::skip]
pub const SCALA_STRING: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// ELIXIR LITERAL [3459ABMN-_]
/// Number format for an [`Elixir`] literal floating-point number.
///
/// [`Elixir`]: https://elixir-lang.org/
#[rustfmt::skip]
pub const ELIXIR_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .required_digits(true)
    .no_exponent_without_fraction(true)
    .no_special(true)
    .internal_digit_separator(true)
    .build_strict();

// ELIXIR STRING [345MN]
/// Number format to parse an [`Elixir`] float from string.
///
/// [`Elixir`]: https://elixir-lang.org/
#[rustfmt::skip]
pub const ELIXIR_STRING: u128 = NumberFormatBuilder::new()
    .required_digits(true)
    .no_exponent_without_fraction(true)
    .no_special(true)
    .build_strict();

// FORTRAN LITERAL [013456MN]
/// Number format for a [`FORTRAN`] literal floating-point number.
///
/// [`FORTRAN`]: https://fortran-lang.org/
#[rustfmt::skip]
pub const FORTRAN_LITERAL: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build_strict();

// FORTRAN STRING [0134567MN]
/// Number format to parse a [`FORTRAN`] float from string.
///
/// [`FORTRAN`]: https://fortran-lang.org/
#[rustfmt::skip]
pub const FORTRAN_STRING: u128 = NumberFormatBuilder::new().build_strict();

// D LITERAL [0134569ABFGHIJKN-_]
/// Number format for a [`D`] literal floating-point number.
///
/// [`D`]: https://dlang.org/
#[rustfmt::skip]
pub const D_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .no_special(true)
    .no_integer_leading_zeros(true)
    .internal_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

// D STRING [01345679AFGMN-_]
/// Number format to parse a [`D`] float from string.
///
/// [`D`]: https://dlang.org/
#[rustfmt::skip]
pub const D_STRING: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .integer_internal_digit_separator(true)
    .fraction_internal_digit_separator(true)
    .integer_trailing_digit_separator(true)
    .fraction_trailing_digit_separator(true)
    .build_strict();

// COFFEESCRIPT LITERAL [01345678]
/// Number format for a [`Coffeescript`] literal floating-point number.
///
/// [`Coffeescript`]: https://coffeescript.org/
#[rustfmt::skip]
pub const COFFEESCRIPT_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .no_integer_leading_zeros(true)
    .no_float_leading_zeros(true)
    .build_strict();

// COFFEESCRIPT STRING [012345678MN]
/// Number format to parse a [`Coffeescript`] float from string.
///
/// [`Coffeescript`]: https://coffeescript.org/
#[rustfmt::skip]
pub const COFFEESCRIPT_STRING: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// COBOL LITERAL [0345MN]
/// Number format for a [`Cobol`] literal floating-point number.
///
/// [`Cobol`]: https://www.ibm.com/think/topics/cobol
#[rustfmt::skip]
pub const COBOL_LITERAL: u128 = NumberFormatBuilder::new()
    .required_fraction_digits(true)
    .no_exponent_without_fraction(true)
    .no_special(true)
    .build_strict();

// COBOL STRING [012356MN]
/// Number format to parse a [`Cobol`] float from string.
///
/// [`Cobol`]: https://www.ibm.com/think/topics/cobol
#[rustfmt::skip]
pub const COBOL_STRING: u128 = NumberFormatBuilder::new()
    .required_exponent_sign(true)
    .no_special(true)
    .build_strict();

// FSHARP LITERAL [13456789ABIJKMN-_]
/// Number format for a [`F#`] literal floating-point number.
///
/// [`F#`]: https://fsharp.org/
#[rustfmt::skip]
pub const FSHARP_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .required_integer_digits(true)
    .required_exponent_digits(true)
    .case_sensitive_special(true)
    .internal_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

// FSHARP STRING [013456789ABCDEFGHIJKLMN-_]
/// Number format to parse a [`F#`] float from string.
///
/// [`F#`]: https://fsharp.org/
#[rustfmt::skip]
pub const FSHARP_STRING: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .internal_digit_separator(true)
    .leading_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .special_digit_separator(true)
    .build_strict();

// VB LITERAL [03456MN]
/// Number format for a [`Visual Basic`] literal floating-point number.
///
/// [`Visual Basic`]: https://learn.microsoft.com/en-us/dotnet/visual-basic/
#[rustfmt::skip]
pub const VB_LITERAL: u128 = NumberFormatBuilder::new()
    .required_fraction_digits(true)
    .no_special(true)
    .build_strict();

// VB STRING [01345678MN]
/// Number format to parse a [`Visual Basic`] float from string.
///
/// [`Visual Basic`]: https://learn.microsoft.com/en-us/dotnet/visual-basic/
// Note: To my knowledge, Visual Basic cannot parse infinity.
#[rustfmt::skip]
pub const VB_STRING: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// OCAML LITERAL [1456789ABDFGHIJKMN-_]
/// Number format for an [`OCaml`] literal floating-point number.
///
/// [`OCaml`]: https://ocaml.org/
#[rustfmt::skip]
pub const OCAML_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .required_integer_digits(true)
    .required_exponent_digits(true)
    .no_positive_mantissa_sign(true)
    .case_sensitive_special(true)
    .internal_digit_separator(true)
    .fraction_leading_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

// OCAML STRING [01345679ABCDEFGHIJKLMN-_]
/// Number format to parse an [`OCaml`] float from string.
///
/// [`OCaml`]: https://ocaml.org/
#[rustfmt::skip]
pub const OCAML_STRING: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .internal_digit_separator(true)
    .leading_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .special_digit_separator(true)
    .build_strict();

// OBJECTIVEC LITERAL [013456MN]
/// Number format for an [`Objective-C`] literal floating-point number.
///
/// [`Objective-C`]: https://en.wikipedia.org/wiki/Objective-C
#[rustfmt::skip]
pub const OBJECTIVEC_LITERAL: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build_strict();

// OBJECTIVEC STRING [013456MN]
/// Number format to parse an [`Objective-C`] float from string.
///
/// [`Objective-C`]: https://en.wikipedia.org/wiki/Objective-C
#[rustfmt::skip]
pub const OBJECTIVEC_STRING: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build_strict();

// REASONML LITERAL [13456789ABDFGHIJKMN-_]
/// Number format for a [`ReasonML`] literal floating-point number.
///
/// [`ReasonML`]: https://reasonml.github.io/
#[rustfmt::skip]
pub const REASONML_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .required_integer_digits(true)
    .required_exponent_digits(true)
    .case_sensitive_special(true)
    .internal_digit_separator(true)
    .fraction_leading_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

// REASONML STRING [01345679ABCDEFGHIJKLMN-_]
/// Number format to parse a [`ReasonML`] float from string.
///
/// [`ReasonML`]: https://reasonml.github.io/
#[rustfmt::skip]
pub const REASONML_STRING: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .internal_digit_separator(true)
    .leading_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .special_digit_separator(true)
    .build_strict();

// OCTAVE LITERAL [013456789ABDFGHIJKMN-_]
// Note: Octave accepts both NaN and nan, Inf and inf.
/// Number format for an [`Octave`] literal floating-point number.
///
/// [`Octave`]: https://octave.org/
#[rustfmt::skip]
pub const OCTAVE_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .case_sensitive_special(true)
    .internal_digit_separator(true)
    .fraction_leading_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

// OCTAVE STRING [01345679ABCDEFGHIJKMN-,]
/// Number format to parse an [`Octave`] float from string.
///
/// [`Octave`]: https://octave.org/
#[rustfmt::skip]
pub const OCTAVE_STRING: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b','))
    .internal_digit_separator(true)
    .leading_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

// MATLAB LITERAL [013456789ABDFGHIJKMN-_]
// Note: Matlab accepts both NaN and nan, Inf and inf.
/// Number format for an [`Matlab`] literal floating-point number.
///
/// [`Matlab`]: https://www.mathworks.com/products/matlab.html
#[rustfmt::skip]
pub const MATLAB_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .case_sensitive_special(true)
    .internal_digit_separator(true)
    .fraction_leading_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

// MATLAB STRING [01345679ABCDEFGHIJKMN-,]
/// Number format to parse an [`Matlab`] float from string.
///
/// [`Matlab`]: https://www.mathworks.com/products/matlab.html
#[rustfmt::skip]
pub const MATLAB_STRING: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b','))
    .internal_digit_separator(true)
    .leading_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .build_strict();

// ZIG LITERAL [1456MN]
/// Number format for a [`Zig`] literal floating-point number.
///
/// [`Zig`]: https://ziglang.org/
#[rustfmt::skip]
pub const ZIG_LITERAL: u128 = NumberFormatBuilder::new()
    .required_integer_digits(true)
    .no_positive_mantissa_sign(true)
    .no_special(true)
    .build_strict();

// ZIG STRING [01234567MN]
/// Number format to parse a [`Zig`] float from string.
///
/// [`Zig`]: https://ziglang.org/
pub const ZIG_STRING: u128 = PERMISSIVE;

// SAGE LITERAL [012345678MN]
// Note: Both Infinity and infinity are accepted.
/// Number format for a [`Sage`] literal floating-point number.
///
/// [`Sage`]: https://www.sagemath.org/
#[rustfmt::skip]
pub const SAGE_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build_strict();

// SAGE STRING [01345679ABMN-_]
/// Number format to parse a [`Sage`] float from string.
///
/// [`Sage`]: https://www.sagemath.org/
#[rustfmt::skip]
pub const SAGE_STRING: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .internal_digit_separator(true)
    .build_strict();

// JSON [456]
/// Number format for a [`JSON`][`JSON-REF`] literal floating-point number.
///
/// [`JSON-REF`]: https://www.json.org/json-en.html
#[rustfmt::skip]
pub const JSON: u128 = NumberFormatBuilder::new()
    .required_digits(true)
    .no_positive_mantissa_sign(true)
    .no_special(true)
    .no_integer_leading_zeros(true)
    .no_float_leading_zeros(true)
    .build_strict();

// TOML [34569AB]
/// Number format for a [`TOML`][`TOML-REF`] literal floating-point number.
///
/// [`TOML-REF`]: https://toml.io/en/
#[rustfmt::skip]
pub const TOML: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .required_digits(false)
    .no_special(true)
    .no_integer_leading_zeros(true)
    .no_float_leading_zeros(true)
    .internal_digit_separator(true)
    .build_strict();

// YAML (defined in-terms of JSON schema).
/// Number format for a [`YAML`][`YAML-REF`] literal floating-point number.
///
/// [`YAML-REF`]: https://yaml.org/
pub const YAML: u128 = JSON;

// XML [01234578MN]
/// Number format for an [`XML`][`XML-REF`] literal floating-point number.
///
/// [`XML-REF`]: https://en.wikipedia.org/wiki/XML
#[rustfmt::skip]
pub const XML: u128 = NumberFormatBuilder::new()
    .required_exponent_digits(false)
    .case_sensitive_special(true)
    .build_strict();

// SQLITE [013456MN]
/// Number format for a [`SQLite`] literal floating-point number.
///
/// [`SQLite`]: https://www.sqlite.org/
#[rustfmt::skip]
pub const SQLITE: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build_strict();

// POSTGRESQL [013456MN]
/// Number format for a [`PostgreSQL`] literal floating-point number.
///
/// [`PostgreSQL`]: https://www.postgresql.org/
#[rustfmt::skip]
pub const POSTGRESQL: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build_strict();

// MYSQL [013456MN]
/// Number format for a [`MySQL`] literal floating-point number.
///
/// [`MySQL`]: https://www.mysql.com/
#[rustfmt::skip]
pub const MYSQL: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build_strict();

// MONGODB [01345678M]
/// Number format for a [`MongoDB`] literal floating-point number.
///
/// [`MongoDB`]: https://www.mongodb.com/
#[rustfmt::skip]
pub const MONGODB: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .no_float_leading_zeros(true)
    .build_strict();

// HIDDEN DEFAULTS AND INTERFACES

/// Number format when no flags are set.
#[doc(hidden)]
#[rustfmt::skip]
pub const PERMISSIVE: u128 = NumberFormatBuilder::new()
    .required_exponent_digits(false)
    .required_mantissa_digits(false)
    .build_strict();

/// Number format when all digit separator flags are set.
#[doc(hidden)]
#[rustfmt::skip]
pub const IGNORE: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .digit_separator_flags(true)
    .required_exponent_digits(false)
    .required_mantissa_digits(false)
    .build_strict();
