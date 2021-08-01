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
//          if (std::distance(string, reinterpret_cast<const char*>(end)) != strlen(string)) {
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
//      Run `fsharpi` to enter the interpreter.
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
//      Save to `main.m` and run `gcc -o main -lobjc -lgnustep-base main.m -fconstant-string-class=NSConstantString`.
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

#![cfg(feature = "parse")]
#![cfg(feature = "format")]

use crate::error::{Error, ErrorCode};
use crate::format_builder::NumberFormatBuilder;
use crate::format_flags as flags;
use crate::lib::num;

use static_assertions::const_assert;

/// Add multiple flags to SyntaxFormat.
macro_rules! from_flag {
    ($format:ident, $flag:ident) => {{
        $format & flags::$flag != 0
    }};
}

/// Wrapper for the 128-bit packed struct.
pub struct NumberFormat<const FORMAT: u128>;

#[rustfmt::skip]
impl<const FORMAT: u128> NumberFormat<FORMAT> {
    /// Create new instance (for methods and validation).
    pub const fn new() -> Self {
        Self {}
    }

    // VALIDATION

    /// Determine if the number format is valid.
    pub const fn is_valid(&self) -> bool {
        // Note: enum equality is not a const fn, so use integer equality.
        self.error().code as i32 == ErrorCode::Success as i32
    }

    /// Get error from invalid number format.
    pub const fn error(&self) -> Error {
        if !flags::is_valid_radix(self.mantissa_radix()) {
            Error::new(ErrorCode::InvalidMantissaRadix)
        } else if !flags::is_valid_radix(self.exponent_base()) {
            Error::new(ErrorCode::InvalidExponentBase)
        } else if !flags::is_valid_radix(self.exponent_radix()) {
            Error::new(ErrorCode::InvalidExponentRadix)
        } else if !flags::is_valid_digit_separator(FORMAT) {
            Error::new(ErrorCode::InvalidDigitSeparator)
        } else if !flags::is_valid_decimal_point(FORMAT) {
            Error::new(ErrorCode::InvalidDecimalPoint)
        } else if !flags::is_valid_exponent(FORMAT) {
            Error::new(ErrorCode::InvalidExponentSymbol)
        } else if !flags::is_valid_base_prefix(FORMAT) {
            Error::new(ErrorCode::InvalidBasePrefix)
        } else if !flags::is_valid_base_suffix(FORMAT) {
            Error::new(ErrorCode::InvalidBaseSuffix)
        } else if !flags::is_valid_punctuation(FORMAT) {
            Error::new(ErrorCode::InvalidPunctuation)
        } else if self.exponent_flags() == flags::NO_EXPONENT_NOTATION {
            Error::new(ErrorCode::InvalidExponentFlags)
        } else if self.no_positive_mantissa_sign() && self.required_mantissa_sign() {
            Error::new(ErrorCode::InvalidMantissaSign)
        } else if self.no_positive_exponent_sign() && self.required_exponent_sign() {
            Error::new(ErrorCode::InvalidExponentSign)
        } else if self.no_special() && self.case_sensitive_special() {
            Error::new(ErrorCode::InvalidSpecial)
        } else if self.no_special() && self.special_digit_separator() {
            Error::new(ErrorCode::InvalidSpecial)
        } else if self.integer_digit_separator_flags() == flags::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR {
            Error::new(ErrorCode::InvalidConsecutiveIntegerDigitSeparator)
        } else if self.fraction_digit_separator_flags() == flags::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR {
            Error::new(ErrorCode::InvalidConsecutiveFractionDigitSeparator)
        } else if self.exponent_digit_separator_flags() == flags::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR {
            Error::new(ErrorCode::InvalidConsecutiveExponentDigitSeparator)
        } else {
            Error::new(ErrorCode::Success)
        }
    }

    // NON-DIGIT SEPARATOR FLAGS & MASKS

    /// If digits are required before the decimal point.
    pub const REQUIRED_INTEGER_DIGITS: bool = from_flag!(FORMAT, REQUIRED_INTEGER_DIGITS);

    /// Get if digits are required before the decimal point.
    #[inline(always)]
    pub const fn required_integer_digits(&self) -> bool {
        Self::REQUIRED_INTEGER_DIGITS
    }

    /// If digits are required after the decimal point.
    pub const REQUIRED_FRACTION_DIGITS: bool = from_flag!(FORMAT, REQUIRED_FRACTION_DIGITS);

    /// Get if digits are required after the decimal point.
    #[inline(always)]
    pub const fn required_fraction_digits(&self) -> bool {
        Self::REQUIRED_FRACTION_DIGITS
    }

    /// If digits are required after the exponent character.
    pub const REQUIRED_EXPONENT_DIGITS: bool = from_flag!(FORMAT, REQUIRED_EXPONENT_DIGITS);

    /// Get if digits are required after the exponent character.
    #[inline(always)]
    pub const fn required_exponent_digits(&self) -> bool {
        Self::REQUIRED_EXPONENT_DIGITS
    }

    /// If significant digits are required.
    pub const REQUIRED_MANTISSA_DIGITS: bool = from_flag!(FORMAT, REQUIRED_MANTISSA_DIGITS);

    /// Get if significant digits are required.
    #[inline(always)]
    pub const fn required_mantissa_digits(&self) -> bool {
        Self::REQUIRED_MANTISSA_DIGITS
    }

    /// If at least 1 digit in the number is required.
    pub const REQUIRED_DIGITS: bool = from_flag!(FORMAT, REQUIRED_DIGITS);

    /// Get if at least 1 digit in the number is required.
    #[inline(always)]
    pub const fn required_digits(&self) -> bool {
        Self::REQUIRED_DIGITS
    }

    /// If a positive sign before the mantissa is not allowed.
    pub const NO_POSITIVE_MANTISSA_SIGN: bool = from_flag!(FORMAT, NO_POSITIVE_MANTISSA_SIGN);

    /// Get if a positive sign before the mantissa is not allowed.
    #[inline(always)]
    pub const fn no_positive_mantissa_sign(&self) -> bool {
        Self::NO_POSITIVE_MANTISSA_SIGN
    }

    /// If a sign symbol before the mantissa is required.
    pub const REQUIRED_MANTISSA_SIGN: bool = from_flag!(FORMAT, REQUIRED_MANTISSA_SIGN);

    /// Get if a sign symbol before the mantissa is required.
    #[inline(always)]
    pub const fn required_mantissa_sign(&self) -> bool {
        Self::REQUIRED_MANTISSA_SIGN
    }

    /// If exponent notation is not allowed.
    pub const NO_EXPONENT_NOTATION: bool = from_flag!(FORMAT, NO_EXPONENT_NOTATION);

    /// Get if exponent notation is not allowed.
    #[inline(always)]
    pub const fn no_exponent_notation(&self) -> bool {
        Self::NO_EXPONENT_NOTATION
    }

    /// If a positive sign before the exponent is not allowed.
    pub const NO_POSITIVE_EXPONENT_SIGN: bool = from_flag!(FORMAT, NO_POSITIVE_EXPONENT_SIGN);

    /// Get if a positive sign before the exponent is not allowed.
    #[inline(always)]
    pub const fn no_positive_exponent_sign(&self) -> bool {
        Self::NO_POSITIVE_EXPONENT_SIGN
    }

    /// If a sign symbol before the exponent is required.
    pub const REQUIRED_EXPONENT_SIGN: bool = from_flag!(FORMAT, REQUIRED_EXPONENT_SIGN);

    /// Get if a sign symbol before the exponent is required.
    #[inline(always)]
    pub const fn required_exponent_sign(&self) -> bool {
        Self::REQUIRED_EXPONENT_SIGN
    }

    /// If an exponent without fraction is not allowed.
    pub const NO_EXPONENT_WITHOUT_FRACTION: bool = from_flag!(FORMAT, NO_EXPONENT_WITHOUT_FRACTION);

    /// Get if an exponent without fraction is not allowed.
    #[inline(always)]
    pub const fn no_exponent_without_fraction(&self) -> bool {
        Self::NO_EXPONENT_WITHOUT_FRACTION
    }

    /// If special (non-finite) values are not allowed.
    pub const NO_SPECIAL: bool = from_flag!(FORMAT, NO_SPECIAL);

    /// Get if special (non-finite) values are not allowed.
    #[inline(always)]
    pub const fn no_special(&self) -> bool {
        Self::NO_SPECIAL
    }

    /// If special (non-finite) values are case-sensitive.
    pub const CASE_SENSITIVE_SPECIAL: bool = from_flag!(FORMAT, CASE_SENSITIVE_SPECIAL);

    /// Get if special (non-finite) values are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_special(&self) -> bool {
        Self::CASE_SENSITIVE_SPECIAL
    }

    /// If leading zeros before an integer are not allowed.
    pub const NO_INTEGER_LEADING_ZEROS: bool = from_flag!(FORMAT, NO_INTEGER_LEADING_ZEROS);

    /// Get if leading zeros before an integer are not allowed.
    #[inline(always)]
    pub const fn no_integer_leading_zeros(&self) -> bool {
        Self::NO_INTEGER_LEADING_ZEROS
    }

    /// If leading zeros before a float are not allowed.
    pub const NO_FLOAT_LEADING_ZEROS: bool = from_flag!(FORMAT, NO_FLOAT_LEADING_ZEROS);

    /// Get if leading zeros before a float are not allowed.
    #[inline(always)]
    pub const fn no_float_leading_zeros(&self) -> bool {
        Self::NO_FLOAT_LEADING_ZEROS
    }

    /// If exponent notation is required.
    pub const REQUIRED_EXPONENT_NOTATION: bool = from_flag!(FORMAT, REQUIRED_EXPONENT_NOTATION);

    /// Get if exponent notation is required.
    #[inline(always)]
    pub const fn required_exponent_notation(&self) -> bool {
        Self::REQUIRED_EXPONENT_NOTATION
    }

    /// If exponent characters are case-sensitive.
    pub const CASE_SENSITIVE_EXPONENT: bool = from_flag!(FORMAT, CASE_SENSITIVE_EXPONENT);

    /// Get if exponent characters are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_exponent(&self) -> bool {
        Self::CASE_SENSITIVE_EXPONENT
    }

    /// If base prefixes are case-sensitive.
    pub const CASE_SENSITIVE_BASE_PREFIX: bool = from_flag!(FORMAT, CASE_SENSITIVE_BASE_PREFIX);

    /// Get if base prefixes are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_base_prefix(&self) -> bool {
        Self::CASE_SENSITIVE_BASE_PREFIX
    }

    /// If base suffixes are case-sensitive.
    pub const CASE_SENSITIVE_BASE_SUFFIX: bool = from_flag!(FORMAT, CASE_SENSITIVE_BASE_SUFFIX);

    /// Get if base suffixes are case-sensitive.
    #[inline(always)]
    pub const fn case_sensitive_base_suffix(&self) -> bool {
        Self::CASE_SENSITIVE_BASE_SUFFIX
    }

    // DIGIT SEPARATOR FLAGS & MASKS

    // If digit separators are allowed between integer digits.
    pub const INTEGER_INTERNAL_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, INTEGER_INTERNAL_DIGIT_SEPARATOR);

    /// Get if digit separators are allowed between integer digits.
    #[inline(always)]
    pub const fn integer_internal_digit_separator(&self) -> bool {
        Self::INTEGER_INTERNAL_DIGIT_SEPARATOR
    }

    /// If digit separators are allowed between fraction digits.
    pub const FRACTION_INTERNAL_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, FRACTION_INTERNAL_DIGIT_SEPARATOR);

    /// Get if digit separators are allowed between fraction digits.
    #[inline(always)]
    pub const fn fraction_internal_digit_separator(&self) -> bool {
        Self::FRACTION_INTERNAL_DIGIT_SEPARATOR
    }

    /// If digit separators are allowed between exponent digits.
    pub const EXPONENT_INTERNAL_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, EXPONENT_INTERNAL_DIGIT_SEPARATOR);

    /// Get if digit separators are allowed between exponent digits.
    #[inline(always)]
    pub const fn exponent_internal_digit_separator(&self) -> bool {
        Self::EXPONENT_INTERNAL_DIGIT_SEPARATOR
    }

    /// If digit separators are allowed between digits.
    pub const INTERNAL_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, INTERNAL_DIGIT_SEPARATOR);

    /// Get if digit separators are allowed between digits.
    #[inline(always)]
    pub const fn internal_digit_separator(&self) -> bool {
        Self::INTERNAL_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed before any integer digits.
    pub const INTEGER_LEADING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, INTEGER_LEADING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed before any integer digits.
    #[inline(always)]
    pub const fn integer_leading_digit_separator(&self) -> bool {
        Self::INTEGER_LEADING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed before any integer digits.
    pub const FRACTION_LEADING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, FRACTION_LEADING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed before any fraction digits.
    #[inline(always)]
    pub const fn fraction_leading_digit_separator(&self) -> bool {
        Self::FRACTION_LEADING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed before any exponent digits.
    pub const EXPONENT_LEADING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, EXPONENT_LEADING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed before any exponent digits.
    #[inline(always)]
    pub const fn exponent_leading_digit_separator(&self) -> bool {
        Self::EXPONENT_LEADING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed before any digits.
    pub const LEADING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, LEADING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed before any digits.
    #[inline(always)]
    pub const fn leading_digit_separator(&self) -> bool {
        Self::LEADING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed after any integer digits.
    pub const INTEGER_TRAILING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, INTEGER_TRAILING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed after any integer digits.
    #[inline(always)]
    pub const fn integer_trailing_digit_separator(&self) -> bool {
        Self::INTEGER_TRAILING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed after any fraction digits.
    pub const FRACTION_TRAILING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, FRACTION_TRAILING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed after any fraction digits.
    #[inline(always)]
    pub const fn fraction_trailing_digit_separator(&self) -> bool {
        Self::FRACTION_TRAILING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed after any exponent digits.
    pub const EXPONENT_TRAILING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, EXPONENT_TRAILING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed after any exponent digits.
    #[inline(always)]
    pub const fn exponent_trailing_digit_separator(&self) -> bool {
        Self::EXPONENT_TRAILING_DIGIT_SEPARATOR
    }

    /// If a digit separator is allowed after any digits.
    pub const TRAILING_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, TRAILING_DIGIT_SEPARATOR);

    /// Get if a digit separator is allowed after any digits.
    #[inline(always)]
    pub const fn trailing_digit_separator(&self) -> bool {
        Self::TRAILING_DIGIT_SEPARATOR
    }

    /// If multiple consecutive integer digit separators are allowed.
    pub const INTEGER_CONSECUTIVE_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, INTEGER_CONSECUTIVE_DIGIT_SEPARATOR);

    /// Get if multiple consecutive integer digit separators are allowed.
    #[inline(always)]
    pub const fn integer_consecutive_digit_separator(&self) -> bool {
        Self::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR
    }

    /// If multiple consecutive fraction digit separators are allowed.
    pub const FRACTION_CONSECUTIVE_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, FRACTION_CONSECUTIVE_DIGIT_SEPARATOR);

    /// Get if multiple consecutive fraction digit separators are allowed.
    #[inline(always)]
    pub const fn fraction_consecutive_digit_separator(&self) -> bool {
        Self::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR
    }

    /// If multiple consecutive exponent digit separators are allowed.
    pub const EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR);

    /// Get if multiple consecutive exponent digit separators are allowed.
    #[inline(always)]
    pub const fn exponent_consecutive_digit_separator(&self) -> bool {
        Self::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR
    }

    /// If multiple consecutive digit separators are allowed.
    pub const CONSECUTIVE_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, CONSECUTIVE_DIGIT_SEPARATOR);

    /// Get if multiple consecutive digit separators are allowed.
    #[inline(always)]
    pub const fn consecutive_digit_separator(&self) -> bool {
        Self::CONSECUTIVE_DIGIT_SEPARATOR
    }

    /// If any digit separators are allowed in special (non-finite) values.
    pub const SPECIAL_DIGIT_SEPARATOR: bool = from_flag!(FORMAT, SPECIAL_DIGIT_SEPARATOR);

    /// Get if any digit separators are allowed in special (non-finite) values.
    #[inline(always)]
    pub const fn special_digit_separator(&self) -> bool {
        Self::SPECIAL_DIGIT_SEPARATOR
    }

    // CHARACTERS

    /// The digit separator character in the packed struct.
    pub const DIGIT_SEPARATOR: u8 = flags::digit_separator(FORMAT);

    /// Get the digit separator character.
    ///
    /// If the digit separator is 0, digit separators are not allowed.
    #[inline(always)]
    pub const fn digit_separator(&self) -> u8 {
        Self::DIGIT_SEPARATOR
    }

    /// The decimal point character in the packed struct.
    pub const DECIMAL_POINT: u8 = flags::decimal_point(FORMAT);

    /// Get the decimal point character.
    #[inline(always)]
    pub const fn decimal_point(&self) -> u8 {
        Self::DECIMAL_POINT
    }

    /// The exponent character in the packed struct.
    pub const EXPONENT: u8 = flags::exponent(FORMAT);

    /// Get the exponent character.
    #[inline(always)]
    pub const fn exponent(&self) -> u8 {
        Self::EXPONENT
    }

    /// The base prefix character in the packed struct.
    pub const BASE_PREFIX: u8 = flags::base_prefix(FORMAT);

    /// Get the character for the base prefix.
    ///
    /// If the base prefix is 0, base prefixes are not allowed.
    /// The number will have then have the format `0$base_prefix...`.
    /// For example, a hex base prefix would be `0x`. Base prefixes are
    /// always optional.
    #[inline(always)]
    pub const fn base_prefix(&self) -> u8 {
        Self::BASE_PREFIX
    }

    /// The base suffix character in the packed struct.
    pub const BASE_SUFFIX: u8 = flags::base_suffix(FORMAT);

    /// Character for the base suffix.
    ///
    /// If not provided, base suffixes are not allowed.
    /// The number will have then have the format `...$base_suffix`.
    /// For example, a hex base prefix would be `0x`. Base prefixes are
    /// always optional.
    #[inline(always)]
    pub const fn base_suffix(&self) -> u8 {
        Self::BASE_SUFFIX
    }

    // RADIX

    /// The radix for the significant digits in the packed struct.
    pub const MANTISSA_RADIX: u32 = flags::mantissa_radix(FORMAT);

    /// Get the radix for the mantissa digits.
    #[inline(always)]
    pub const fn mantissa_radix(&self) -> u32 {
        Self::MANTISSA_RADIX
    }

    /// The radix for the significant digits in the packed struct.
    /// Alias for `MANTISSA_RADIX`.
    pub const RADIX: u32 = Self::MANTISSA_RADIX;

    /// Get the radix for the significant digits.
    #[inline(always)]
    pub const fn radix(&self) -> u32 {
        Self::RADIX
    }

    /// The base for the exponent.
    pub const EXPONENT_BASE: u32 = flags::exponent_base(FORMAT);

    /// Get the base for the exponent.
    ///
    /// IE, a base of 2 means we have `mantissa * 2^exponent`.
    /// If not provided, it defaults to `radix`.
    #[inline(always)]
    pub const fn exponent_base(&self) -> u32 {
        Self::EXPONENT_BASE
    }

    /// The radix for the exponent digits.
    pub const EXPONENT_RADIX: u32 = flags::exponent_radix(FORMAT);

    /// Get the radix for the exponent digits.
    ///
    /// If not provided, defaults to `radix`.
    #[inline(always)]
    pub const fn exponent_radix(&self) -> u32 {
        Self::EXPONENT_RADIX
    }

    // FLAGS

    /// Get the flags from the number format.
    #[inline(always)]
    pub const fn flags(&self) -> u128 {
        FORMAT & flags::FLAG_MASK
    }

    /// Get the interface flags from the number format.
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
}

// PRE-DEFINED CONSTANTS// PRE-DEFINED
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
/// Number format for a Rust literal floating-point number.
#[rustfmt::skip]
pub const RUST_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .required_digits(true)
    .no_positive_mantissa_sign(true)
    .no_special(true)
    .internal_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .build();

const_assert!(NumberFormat::<{ RUST_LITERAL }> {}.is_valid());

// RUST STRING [0134567MN]
/// Number format to parse a Rust float from string.
#[rustfmt::skip]
pub const RUST_STRING: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ RUST_STRING }> {}.is_valid());

/// Number format for a Python literal floating-point number.
pub const PYTHON_LITERAL: u128 = PYTHON3_LITERAL;

/// Number format to parse a Python float from string.
pub const PYTHON_STRING: u128 = PYTHON3_STRING;

/// Number format for a Python3 literal floating-point number.
pub const PYTHON3_LITERAL: u128 = PYTHON36_LITERAL;

// PYTHON3 STRING [0134567MN]
/// Number format to parse a Python3 float from string.
#[rustfmt::skip]
pub const PYTHON3_STRING: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ PYTHON3_STRING }> {}.is_valid());

// PYTHON3.6+ LITERAL [013456N-_]
/// Number format for a Python3.6 or higher literal floating-point number.
#[rustfmt::skip]
pub const PYTHON36_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .no_special(true)
    .no_integer_leading_zeros(true)
    .build();

const_assert!(NumberFormat::<{ PYTHON36_LITERAL }> {}.is_valid());

// PYTHON3.5- LITERAL [013456N]
/// Number format for a Python3.5 or lower literal floating-point number.
#[rustfmt::skip]
pub const PYTHON35_LITERAL: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .no_integer_leading_zeros(true)
    .build();

const_assert!(NumberFormat::<{ PYTHON35_LITERAL }> {}.is_valid());

// PYTHON2 LITERAL [013456MN]
/// Number format for a Python2 literal floating-point number.
#[rustfmt::skip]
pub const PYTHON2_LITERAL: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build();

const_assert!(NumberFormat::<{ PYTHON2_LITERAL }> {}.is_valid());

// PYTHON2 STRING [0134567MN]
/// Number format to parse a Python2 float from string.
#[rustfmt::skip]
pub const PYTHON2_STRING: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ PYTHON2_STRING }> {}.is_valid());

/// Number format for a C++ literal floating-point number.
pub const CXX_LITERAL: u128 = CXX20_LITERAL;

/// Number format to parse a C++ float from string.
pub const CXX_STRING: u128 = CXX20_STRING;

// C++20 LITERAL [013456789ABMN-']
/// Number format for a C++20 literal floating-point number.
#[rustfmt::skip]
pub const CXX20_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'\''))
    .case_sensitive_special(true)
    .internal_digit_separator(true)
    .build();

const_assert!(NumberFormat::<{ CXX20_LITERAL }> {}.is_valid());

// C++20 STRING [0134567MN]
/// Number format for a C++20 string floating-point number.
#[rustfmt::skip]
pub const CXX20_STRING: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ CXX20_STRING }> {}.is_valid());

// C++17 LITERAL [013456789ABMN-']
/// Number format for a C++17 literal floating-point number.
#[rustfmt::skip]
pub const CXX17_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'\''))
    .case_sensitive_special(true)
    .internal_digit_separator(true)
    .build();

const_assert!(NumberFormat::<{ CXX17_LITERAL }> {}.is_valid());

// C++17 STRING [0134567MN]
/// Number format for a C++17 string floating-point number.
#[rustfmt::skip]
pub const CXX17_STRING: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ CXX17_STRING }> {}.is_valid());

// C++14 LITERAL [013456789ABMN-']
/// Number format for a C++14 literal floating-point number.
#[rustfmt::skip]
pub const CXX14_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'\''))
    .case_sensitive_special(true)
    .internal_digit_separator(true)
    .build();

const_assert!(NumberFormat::<{ CXX14_LITERAL }> {}.is_valid());

// C++14 STRING [0134567MN]
/// Number format for a C++14 string floating-point number.
#[rustfmt::skip]
pub const CXX14_STRING: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ CXX14_STRING }> {}.is_valid());

// C++11 LITERAL [01345678MN]
/// Number format for a C++11 literal floating-point number.
#[rustfmt::skip]
pub const CXX11_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build();

const_assert!(NumberFormat::<{ CXX11_LITERAL }> {}.is_valid());

// C++11 STRING [0134567MN]
/// Number format for a C++11 string floating-point number.
#[rustfmt::skip]
pub const CXX11_STRING: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ CXX11_STRING }> {}.is_valid());

// C++03 LITERAL [01345678MN]
/// Number format for a C++03 literal floating-point number.
#[rustfmt::skip]
pub const CXX03_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build();

const_assert!(NumberFormat::<{ CXX03_LITERAL }> {}.is_valid());

// C++03 STRING [0134567MN]
/// Number format for a C++03 string floating-point number.
#[rustfmt::skip]
pub const CXX03_STRING: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ CXX03_STRING }> {}.is_valid());

// C++98 LITERAL [01345678MN]
/// Number format for a C++98 literal floating-point number.
#[rustfmt::skip]
pub const CXX98_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build();

const_assert!(NumberFormat::<{ CXX98_LITERAL }> {}.is_valid());

// C++98 STRING [0134567MN]
/// Number format for a C++98 string floating-point number.
#[rustfmt::skip]
pub const CXX98_STRING: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ CXX98_STRING }> {}.is_valid());

/// Number format for a C literal floating-point number.
pub const C_LITERAL: u128 = C18_LITERAL;

/// Number format to parse a C float from string.
pub const C_STRING: u128 = C18_STRING;

// C18 LITERAL [01345678MN]
/// Number format for a C++98 literal floating-point number.
#[rustfmt::skip]
pub const C18_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build();

const_assert!(NumberFormat::<{ C18_LITERAL }> {}.is_valid());

// C18 STRING [0134567MN]
/// Number format for a C++98 string floating-point number.
#[rustfmt::skip]
pub const C18_STRING: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ C18_STRING }> {}.is_valid());

// C11 LITERAL [01345678MN]
/// Number format for a C++98 literal floating-point number.
#[rustfmt::skip]
pub const C11_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build();

const_assert!(NumberFormat::<{ C11_LITERAL }> {}.is_valid());

// C11 STRING [0134567MN]
/// Number format for a C++98 string floating-point number.
#[rustfmt::skip]
pub const C11_STRING: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ C11_STRING }> {}.is_valid());

// C99 LITERAL [01345678MN]
/// Number format for a C++98 literal floating-point number.
#[rustfmt::skip]
pub const C99_LITERAL: u128 = NumberFormatBuilder::new()
    .case_sensitive_special(true)
    .build();

const_assert!(NumberFormat::<{ C99_LITERAL }> {}.is_valid());

// C99 STRING [0134567MN]
/// Number format for a C++98 string floating-point number.
#[rustfmt::skip]
pub const C99_STRING: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ C99_STRING }> {}.is_valid());

// C90 LITERAL [013456MN]
/// Number format for a C++98 literal floating-point number.
#[rustfmt::skip]
pub const C90_LITERAL: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build();

const_assert!(NumberFormat::<{ C90_LITERAL }> {}.is_valid());

// C90 STRING [0134567MN]
/// Number format for a C++98 string floating-point number.
#[rustfmt::skip]
pub const C90_STRING: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ C90_STRING }> {}.is_valid());

// C89 LITERAL [013456MN]
/// Number format for a C++98 literal floating-point number.
#[rustfmt::skip]
pub const C89_LITERAL: u128 = NumberFormatBuilder::new()
    .no_special(true)
    .build();

const_assert!(NumberFormat::<{ C89_LITERAL }> {}.is_valid());

// C89 STRING [0134567MN]
/// Number format for a C++98 string floating-point number.
#[rustfmt::skip]
pub const C89_STRING: u128 = NumberFormatBuilder::new().build();
const_assert!(NumberFormat::<{ C89_STRING }> {}.is_valid());

// RUBY LITERAL [345689AMN-_]
/// Number format for a Ruby literal floating-point number.
#[rustfmt::skip]
pub const RUBY_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .required_digits(true)
    .no_special(true)
    .no_integer_leading_zeros(true)
    .no_float_leading_zeros(true)
    .internal_digit_separator(true)
    .build();

const_assert!(NumberFormat::<{ RUBY_LITERAL }> {}.is_valid());

// RUBY OCTAL LITERAL [345689AN-_]
/// Number format for a Ruby literal floating-point number.
#[rustfmt::skip]
#[cfg(feature = "power-of-two")]
pub const RUBY_OCTAL_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .mantissa_radix(8)
    .required_digits(true)
    .no_special(true)
    .internal_digit_separator(true)
    .build();

#[cfg(feature = "power-of-two")]
const_assert!(NumberFormat::<{ RUBY_OCTAL_LITERAL }> {}.is_valid());

// RUBY STRING [01234569ABMN-_]
// Note: Amazingly, Ruby 1.8+ do not allow parsing special values.
/// Number format to parse a Ruby float from string.
#[rustfmt::skip]
pub const RUBY_STRING: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .no_special(true)
    .internal_digit_separator(true)
    .build();

const_assert!(NumberFormat::<{ RUBY_STRING }> {}.is_valid());

// SWIFT LITERAL [34569ABFGHIJKMN-_]
/// Number format for a Swift literal floating-point number.
#[rustfmt::skip]
pub const SWIFT_LITERAL: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'_'))
    .required_digits(true)
    .no_special(true)
    .internal_digit_separator(true)
    .trailing_digit_separator(true)
    .consecutive_digit_separator(true)
    .build();

const_assert!(NumberFormat::<{ SWIFT_LITERAL }> {}.is_valid());

// SWIFT STRING [134567MN]
/// Number format to parse a Swift float from string.
#[rustfmt::skip]
pub const SWIFT_STRING: u128 = NumberFormatBuilder::new()
    .required_fraction_digits(true)
    .build();

const_assert!(NumberFormat::<{ SWIFT_STRING }> {}.is_valid());

// GO LITERAL [13456MN]
/// Number format for a Golang literal floating-point number.
#[rustfmt::skip]
pub const GO_LITERAL: u128 = NumberFormatBuilder::new()
    .required_fraction_digits(true)
    .no_special(true)
    .build();

const_assert!(NumberFormat::<{ GO_LITERAL }> {}.is_valid());

// GO STRING [134567MN]
/// Number format to parse a Golang float from string.
#[rustfmt::skip]
pub const GO_STRING: u128 = NumberFormatBuilder::new()
    .required_fraction_digits(true)
    .build();

const_assert!(NumberFormat::<{ GO_STRING }> {}.is_valid());

// TODO(ahuszagh) Add a few constants.

//// PRE-DEFINED
////
//// Sample Format Shorthand:
//// ------------------------
////
//// The format shorthand lists the test cases, and if applicable,
//// the digit separator character. For example, the shorthand
//// `[134-_]` specifies it passes tests 1, 3, and 4, and uses
//// `'_'` as a digit-separator character. Meanwhile, `[0]` means it
//// passes test 0, and has no digit separator.
//
//// HASKELL LITERAL [456MN]
///// Number format for a Haskell literal floating-point number.
//const HASKELL_LITERAL = (
//    Self::REQUIRED_DIGITS.bits
//    | Self::NO_POSITIVE_MANTISSA_SIGN.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// HASKELL STRING [45678MN]
///// Number format to parse a Haskell float from string.
//const HASKELL_STRING = (
//    Self::REQUIRED_DIGITS.bits
//    | Self::NO_POSITIVE_MANTISSA_SIGN.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// JAVASCRIPT LITERAL [01345678M]
///// Number format for a Javascript literal floating-point number.
//const JAVASCRIPT_LITERAL = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//    | Self::NO_FLOAT_LEADING_ZEROS.bits
//);
//
//// JAVASCRIPT STRING [012345678MN]
///// Number format to parse a Javascript float from string.
//const JAVASCRIPT_STRING = (
//    Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// PERL LITERAL [0134569ABDEFGHIJKMN-_]
///// Number format for a Perl literal floating-point number.
//const PERL_LITERAL = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
//    | Self::EXPONENT_LEADING_DIGIT_SEPARATOR.bits
//    | Self::TRAILING_DIGIT_SEPARATOR.bits
//    | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
//);
//
//// PERL STRING [01234567MN]
///// Number format to parse a Perl float from string.
//const PERL_STRING = Self::PERMISSIVE.bits;
//
//// PHP LITERAL [01345678MN]
///// Number format for a PHP literal floating-point number.
//const PHP_LITERAL = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// PHP STRING [0123456MN]
///// Number format to parse a PHP float from string.
//const PHP_STRING = (
//    Self::NO_SPECIAL.bits
//);
//
//// JAVA LITERAL [0134569ABIJKMN-_]
///// Number format for a Java literal floating-point number.
//const JAVA_LITERAL = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
//);
//
//// JAVA STRING [01345678MN]
///// Number format to parse a Java float from string.
//const JAVA_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// R LITERAL [01345678MN]
///// Number format for a R literal floating-point number.
//const R_LITERAL = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// R STRING [01234567MN]
///// Number format to parse a R float from string.
//const R_STRING = Self::PERMISSIVE.bits;
//
//// KOTLIN LITERAL [0134569ABIJKN-_]
///// Number format for a Kotlin literal floating-point number.
//const KOTLIN_LITERAL = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//    | Self::NO_INTEGER_LEADING_ZEROS.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
//);
//
//// KOTLIN STRING [0134568MN]
///// Number format to parse a Kotlin float from string.
//const KOTLIN_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// JULIA LITERAL [01345689AMN-_]
///// Number format for a Julia literal floating-point number.
//const JULIA_LITERAL = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//    | Self::INTEGER_INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::FRACTION_INTERNAL_DIGIT_SEPARATOR.bits
//);
//
//// JULIA STRING [01345678MN]
///// Number format to parse a Julia float from string.
//const JULIA_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//);
//
///// Number format for a C# literal floating-point number.
//const CSHARP_LITERAL = Self::CSHARP7_LITERAL.bits;
//
///// Number format to parse a C# float from string.
//const CSHARP_STRING = Self::CSHARP7_STRING.bits;
//
//// CSHARP7 LITERAL [034569ABIJKMN-_]
///// Number format for a C#7 literal floating-point number.
//const CSHARP7_LITERAL = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_FRACTION_DIGITS.bits
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
//);
//
//// CSHARP7 STRING [0134568MN]
///// Number format to parse a C#7 float from string.
//const CSHARP7_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// CSHARP6 LITERAL [03456MN]
///// Number format for a C#6 literal floating-point number.
//const CSHARP6_LITERAL = (
//    Self::REQUIRED_FRACTION_DIGITS.bits
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// CSHARP6 STRING [0134568MN]
///// Number format to parse a C#6 float from string.
//const CSHARP6_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// CSHARP5 LITERAL [03456MN]
///// Number format for a C#5 literal floating-point number.
//const CSHARP5_LITERAL = (
//    Self::REQUIRED_FRACTION_DIGITS.bits
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// CSHARP5 STRING [0134568MN]
///// Number format to parse a C#5 float from string.
//const CSHARP5_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// CSHARP4 LITERAL [03456MN]
///// Number format for a C#4 literal floating-point number.
//const CSHARP4_LITERAL = (
//    Self::REQUIRED_FRACTION_DIGITS.bits
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// CSHARP4 STRING [0134568MN]
///// Number format to parse a C#4 float from string.
//const CSHARP4_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// CSHARP3 LITERAL [03456MN]
///// Number format for a C#3 literal floating-point number.
//const CSHARP3_LITERAL = (
//    Self::REQUIRED_FRACTION_DIGITS.bits
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// CSHARP3 STRING [0134568MN]
///// Number format to parse a C#3 float from string.
//const CSHARP3_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// CSHARP2 LITERAL [03456MN]
///// Number format for a C#2 literal floating-point number.
//const CSHARP2_LITERAL = (
//    Self::REQUIRED_FRACTION_DIGITS.bits
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// CSHARP2 STRING [0134568MN]
///// Number format to parse a C#2 float from string.
//const CSHARP2_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// CSHARP1 LITERAL [03456MN]
///// Number format for a C#1 literal floating-point number.
//const CSHARP1_LITERAL = (
//    Self::REQUIRED_FRACTION_DIGITS.bits
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// CSHARP1 STRING [0134568MN]
///// Number format to parse a C#1 float from string.
//const CSHARP1_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// KAWA LITERAL [013456MN]
///// Number format for a Kawa literal floating-point number.
//const KAWA_LITERAL = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// KAWA STRING [013456MN]
///// Number format to parse a Kawa float from string.
//const KAWA_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// GAMBITC LITERAL [013456MN]
///// Number format for a Gambit-C literal floating-point number.
//const GAMBITC_LITERAL = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// GAMBITC STRING [013456MN]
///// Number format to parse a Gambit-C float from string.
//const GAMBITC_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// GUILE LITERAL [013456MN]
///// Number format for a Guile literal floating-point number.
//const GUILE_LITERAL = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// GUILE STRING [013456MN]
///// Number format to parse a Guile float from string.
//const GUILE_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// CLOJURE LITERAL [13456MN]
///// Number format for a Clojure literal floating-point number.
//const CLOJURE_LITERAL = (
//    Self::REQUIRED_INTEGER_DIGITS.bits
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// CLOJURE STRING [01345678MN]
///// Number format to parse a Clojure float from string.
//const CLOJURE_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// ERLANG LITERAL [34578MN]
///// Number format for an Erlang literal floating-point number.
//const ERLANG_LITERAL = (
//    Self::REQUIRED_DIGITS.bits
//    | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// ERLANG STRING [345MN]
///// Number format to parse an Erlang float from string.
//const ERLANG_STRING = (
//    Self::REQUIRED_DIGITS.bits
//    | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// ELM LITERAL [456]
///// Number format for an Elm literal floating-point number.
//const ELM_LITERAL = (
//    Self::REQUIRED_DIGITS.bits
//    | Self::NO_POSITIVE_MANTISSA_SIGN.bits
//    | Self::NO_INTEGER_LEADING_ZEROS.bits
//    | Self::NO_FLOAT_LEADING_ZEROS.bits
//);
//
//// ELM STRING [01345678MN]
///// Number format to parse an Elm float from string.
//// Note: There is no valid representation of NaN, just Infinity.
//const ELM_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// SCALA LITERAL [3456]
///// Number format for a Scala literal floating-point number.
//const SCALA_LITERAL = (
//    Self::REQUIRED_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//    | Self::NO_INTEGER_LEADING_ZEROS.bits
//    | Self::NO_FLOAT_LEADING_ZEROS.bits
//);
//
//// SCALA STRING [01345678MN]
///// Number format to parse a Scala float from string.
//const SCALA_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// ELIXIR LITERAL [3459ABMN-_]
///// Number format for an Elixir literal floating-point number.
//const ELIXIR_LITERAL = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_DIGITS.bits
//    | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
//    | Self::NO_SPECIAL.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//);
//
//// ELIXIR STRING [345MN]
///// Number format to parse an Elixir float from string.
//const ELIXIR_STRING = (
//    Self::REQUIRED_DIGITS.bits
//    | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// FORTRAN LITERAL [013456MN]
///// Number format for a FORTRAN literal floating-point number.
//const FORTRAN_LITERAL = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// FORTRAN STRING [0134567MN]
///// Number format to parse a FORTRAN float from string.
//const FORTRAN_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//);
//
//// D LITERAL [0134569ABFGHIJKN-_]
///// Number format for a D literal floating-point number.
//const D_LITERAL = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//    | Self::NO_INTEGER_LEADING_ZEROS.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::TRAILING_DIGIT_SEPARATOR.bits
//    | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
//);
//
//// D STRING [01345679AFGMN-_]
///// Number format to parse a D float from string.
//const D_STRING = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::INTEGER_INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::FRACTION_INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::INTEGER_TRAILING_DIGIT_SEPARATOR.bits
//    | Self::FRACTION_TRAILING_DIGIT_SEPARATOR.bits
//);
//
//// COFFEESCRIPT LITERAL [01345678]
///// Number format for a Coffeescript literal floating-point number.
//const COFFEESCRIPT_LITERAL = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//    | Self::NO_INTEGER_LEADING_ZEROS.bits
//    | Self::NO_FLOAT_LEADING_ZEROS.bits
//);
//
//// COFFEESCRIPT STRING [012345678MN]
///// Number format to parse a Coffeescript float from string.
//const COFFEESCRIPT_STRING = (
//    Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// COBOL LITERAL [0345MN]
///// Number format for a Cobol literal floating-point number.
//const COBOL_LITERAL = (
//    Self::REQUIRED_FRACTION_DIGITS.bits
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// COBOL STRING [012356MN]
///// Number format to parse a Cobol float from string.
//const COBOL_STRING = (
//    Self::REQUIRED_EXPONENT_SIGN.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// FSHARP LITERAL [13456789ABIJKMN-_]
///// Number format for a F# literal floating-point number.
//const FSHARP_LITERAL = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_INTEGER_DIGITS.bits
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
//);
//
//// FSHARP STRING [013456789ABCDEFGHIJKLMN-_]
///// Number format to parse a F# float from string.
//const FSHARP_STRING = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//    | Self::LEADING_DIGIT_SEPARATOR.bits
//    | Self::TRAILING_DIGIT_SEPARATOR.bits
//    | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
//    | Self::SPECIAL_DIGIT_SEPARATOR.bits
//);
//
//// VB LITERAL [03456MN]
///// Number format for a Visual Basic literal floating-point number.
//const VB_LITERAL = (
//    Self::REQUIRED_FRACTION_DIGITS.bits
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// VB STRING [01345678MN]
///// Number format to parse a Visual Basic float from string.
//// Note: To my knowledge, Visual Basic cannot parse infinity.
//const VB_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// OCAML LITERAL [1456789ABDFGHIJKMN-_]
///// Number format for an OCaml literal floating-point number.
//const OCAML_LITERAL = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_INTEGER_DIGITS.bits
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_POSITIVE_MANTISSA_SIGN.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
//    | Self::TRAILING_DIGIT_SEPARATOR.bits
//    | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
//);
//
//// OCAML STRING [01345679ABCDEFGHIJKLMN-_]
///// Number format to parse an OCaml float from string.
//const OCAML_STRING = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::LEADING_DIGIT_SEPARATOR.bits
//    | Self::TRAILING_DIGIT_SEPARATOR.bits
//    | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
//    | Self::SPECIAL_DIGIT_SEPARATOR.bits
//);
//
//// OBJECTIVEC LITERAL [013456MN]
///// Number format for an Objective-C literal floating-point number.
//const OBJECTIVEC_LITERAL = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// OBJECTIVEC STRING [013456MN]
///// Number format to parse an Objective-C float from string.
//const OBJECTIVEC_STRING = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// REASONML LITERAL [13456789ABDFGHIJKMN-_]
///// Number format for a ReasonML literal floating-point number.
//const REASONML_LITERAL = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_INTEGER_DIGITS.bits
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
//    | Self::TRAILING_DIGIT_SEPARATOR.bits
//    | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
//);
//
//// REASONML STRING [01345679ABCDEFGHIJKLMN-_]
///// Number format to parse a ReasonML float from string.
//const REASONML_STRING = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::LEADING_DIGIT_SEPARATOR.bits
//    | Self::TRAILING_DIGIT_SEPARATOR.bits
//    | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
//    | Self::SPECIAL_DIGIT_SEPARATOR.bits
//);
//
//// OCTAVE LITERAL [013456789ABDFGHIJKMN-_]
///// Number format for an Octave literal floating-point number.
//// Note: Octave accepts both NaN and nan, Inf and inf.
//const OCTAVE_LITERAL = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
//    | Self::TRAILING_DIGIT_SEPARATOR.bits
//    | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
//);
//
//// OCTAVE STRING [01345679ABCDEFGHIJKMN-,]
///// Number format to parse an Octave float from string.
//const OCTAVE_STRING = (
//    flags::digit_separator_to_flags(b',')
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::LEADING_DIGIT_SEPARATOR.bits
//    | Self::TRAILING_DIGIT_SEPARATOR.bits
//    | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
//);
//
//// MATLAB LITERAL [013456789ABDFGHIJKMN-_]
///// Number format for an Matlab literal floating-point number.
//// Note: Matlab accepts both NaN and nan, Inf and inf.
//const MATLAB_LITERAL = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
//    | Self::TRAILING_DIGIT_SEPARATOR.bits
//    | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
//);
//
//// MATLAB STRING [01345679ABCDEFGHIJKMN-,]
///// Number format to parse an Matlab float from string.
//const MATLAB_STRING = (
//    flags::digit_separator_to_flags(b',')
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::LEADING_DIGIT_SEPARATOR.bits
//    | Self::TRAILING_DIGIT_SEPARATOR.bits
//    | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
//);
//
//// ZIG LITERAL [1456MN]
///// Number format for a Zig literal floating-point number.
//const ZIG_LITERAL = (
//    Self::REQUIRED_INTEGER_DIGITS.bits
//    | Self::NO_POSITIVE_MANTISSA_SIGN.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// ZIG STRING [01234567MN]
///// Number format to parse a Zig float from string.
//const ZIG_STRING = Self::PERMISSIVE.bits;
//
//// SAGE LITERAL [012345678MN]
///// Number format for a Sage literal floating-point number.
//// Note: Both Infinity and infinity are accepted.
//const SAGE_LITERAL = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// SAGE STRING [01345679ABMN-_]
///// Number format to parse a Sage float from string.
//const SAGE_STRING = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//);

// JSON [456]
/// Number format for a JSON literal floating-point number.
#[rustfmt::skip]
pub const JSON: u128 = NumberFormatBuilder::new()
    .digit_separator(num::NonZeroU8::new(b'\''))
    .required_digits(true)
    .no_positive_mantissa_sign(true)
    .no_special(true)
    .no_integer_leading_zeros(true)
    .no_float_leading_zeros(true)
    .build();

const_assert!(NumberFormat::<{ JSON }> {}.is_valid());

//// TOML [34569AB]
///// Number format for a TOML literal floating-point number.
//const TOML = (
//    Self::REQUIRED_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//    | Self::INTERNAL_DIGIT_SEPARATOR.bits
//    | Self::NO_INTEGER_LEADING_ZEROS.bits
//    | Self::NO_FLOAT_LEADING_ZEROS.bits
//);
//
//// YAML (defined in-terms of JSON schema).
///// Number format for a YAML literal floating-point number.
//const YAML = Self::JSON.bits;
//
//// XML [01234578MN]
///// Number format for a XML literal floating-point number.
//const XML = (
//    Self::CASE_SENSITIVE_SPECIAL.bits
//);
//
//// SQLITE [013456MN]
///// Number format for a SQLite literal floating-point number.
//const SQLITE = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// POSTGRESQL [013456MN]
///// Number format for a PostgreSQL literal floating-point number.
//const POSTGRESQL = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// MYSQL [013456MN]
///// Number format for a MySQL literal floating-point number.
//const MYSQL = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::NO_SPECIAL.bits
//);
//
//// MONGODB [01345678M]
///// Number format for a MongoDB literal floating-point number.
//const MONGODB = (
//    Self::REQUIRED_EXPONENT_DIGITS.bits
//    | Self::CASE_SENSITIVE_SPECIAL.bits
//    | Self::NO_FLOAT_LEADING_ZEROS.bits
//);
//
//// HIDDEN DEFAULTS AND INTERFACES
//
///// Number format when no flags are set.
//#[doc(hidden)]
//const PERMISSIVE = 0;
//
///// Permissive interface float format flags.
//#[doc(hidden)]
//const PERMISSIVE_INTERFACE = Self::PERMISSIVE.bits & Self::INTERFACE_FLAG_MASK.bits;
//
///// Standard float format.
//#[doc(hidden)]
//const STANDARD = Self::RUST_STRING.bits;
//
///// Standard interface float format flags.
//#[doc(hidden)]
//const STANDARD_INTERFACE = Self::STANDARD.bits & Self::INTERFACE_FLAG_MASK.bits;
//
///// Number format when all digit separator flags are set.
//#[doc(hidden)]
//const IGNORE = (
//    flags::digit_separator_to_flags(b'_')
//    | Self::DIGIT_SEPARATOR_FLAG_MASK.bits
//);
//
///// Ignore interface float format flags.
//#[doc(hidden)]
//const IGNORE_INTERFACE = Self::IGNORE.bits & Self::INTERFACE_FLAG_MASK.bits;
