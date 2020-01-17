//! Float format enumerations and bit masks.

use super::config;

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
//      (read-string "3.0")
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

// HELPERS

// Determine if character is valid ASCII.
#[inline]
fn is_ascii(ch: u8) -> bool {
    ch.is_ascii()
}

/// Determine if the digit separator is valid.
#[inline]
#[cfg(not(feature = "radix"))]
fn is_valid_separator(ch: u8) -> bool {
    match ch {
        b'0' ..= b'9'       => false,
        b'+' | b'.' | b'-'  => false,
        _                   => (
            is_ascii(ch)
            && ch != config::get_exponent_default_char()
        )
    }
}

/// Determine if the digit separator is valid.
#[inline]
#[cfg(feature = "radix")]
fn is_valid_separator(ch: u8) -> bool {
    match ch {
        b'A' ..= b'Z'       => false,
        b'a' ..= b'z'       => false,
        b'0' ..= b'9'       => false,
        b'+' | b'.' | b'-'  => false,
        _                   => (
            is_ascii(ch)
            && ch != config::get_exponent_default_char()
            && ch != config::get_exponent_backup_char()
        )
    }
}

/// Serialize digit separator to flags.
#[inline]
const fn digit_separator_to_flags(ch: u8) -> u32 {
    (ch as u32) << 24
}

/// Serialize digit separator to flags.
#[inline]
const fn digit_separator_from_flags(ch: u32) -> u8 {
    (ch >> 24) as u8
}

// BITFLAGS

bitflags! {
    /// Bitflags for the float format.
    ///
    /// This is used to derive the high-level bitflags.The default
    /// representation has no digit separators, no required integer or
    /// fraction digits, required exponent digits, and no digit separators.
    ///
    /// Bit Flags Layout
    /// ----------------
    ///
    /// The bitflags has the lower bits designated for flags that modify
    /// the parsing behavior of lexical, and the upper 8 bits set for the
    /// digit separator, allowing any valid ASCII character as a
    /// separator.
    ///
    /// ```text
    ///  0   1   2   3   4   5   6   7   8   9   0   1   2   3   4   5
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |I/R|F/R|E/R|+/M|R/M|e/e|+/E|R/E|e/F|I/I|F/I|E/I|I/L|F/L|E/L|I/T|
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    ///  0   1   2   3   4   5   6   7   8   9   0   1   2   3   4   5
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    /// |F/T|E/T|I/C|F/C|E/C|           |        Digit Separator        |
    /// +---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
    ///
    /// Where:
    ///     I/R = Required integer digits.
    ///     F/R = Required fraction digits.
    ///     E/R = Required exponent digits.
    ///     +/M = No mantissa positive sign.
    ///     R/M = No mantissa positive sign.
    ///     e/e = No exponent notation.
    ///     +/E = No exponent positive sign.
    ///     R/E = No exponent positive sign.
    ///     e/F = No exponent without fraction.
    ///     I/I = Integer internal digit separator.
    ///     F/I = Fraction internal digit separator.
    ///     E/I = Exponent internal digit separator.
    ///     I/L = Integer leading digit separator.
    ///     F/L = Fraction leading digit separator.
    ///     E/L = Exponent leading digit separator.
    ///     I/T = Integer trailing digit separator.
    ///     F/T = Fraction trailing digit separator.
    ///     E/T = Exponent trailing digit separator.
    ///     I/C = Integer consecutive digit separator.
    ///     F/C = Fraction consecutive digit separator.
    ///     E/C = Exponent consecutive digit separator.
    /// ```
    ///
    /// Note:
    /// -----
    ///
    /// In order to limit the format specification and avoid parsing
    /// non-float data, all float formats require some significant digits.
    ///  Examples of always invalid floats include:
    /// - ``
    /// - `.`
    /// - `e`
    /// - `e7`
    ///
    /// Test Cases:
    /// -----------
    ///
    /// The following test-cases are used to define whether a literal or
    /// a string float is valid in a given language, and these tests are
    /// used to denote features in pre-defined formats.
    ///
    /// ```text
    /// 0: '.3'         // Non-required integer.
    /// 1: '3.'         // Non-required fraction.
    /// 2: '3e'         // Non-required exponent.
    /// 3. '+3.0'       // Mantissa positive sign.
    /// 4: '3.0e7'      // Exponent notation.
    /// 5: '3.0e+7'     // Exponent positive sign.
    /// 6. '3e7'        // Exponent notation without fraction.
    /// 7: '3_4.01'     // Integer internal digit separator.
    /// 8: '3.0_1'      // Fraction internal digit separator.
    /// 9: '3.0e7_1'    // Exponent internal digit separator.
    /// A: '_3.01'      // Integer leading digit separator.
    /// B: '3._01'      // Fraction leading digit separator.
    /// C: '3.0e_71'    // Exponent leading digit separator.
    /// D: '3_.01'      // Integer trailing digit separator.
    /// E: '3.01_'      // Fraction trailing digit separator.
    /// F: '3.0e71_'    // Exponent trailing digit separator.
    /// G: '3__4.01'    // Integer consecutive digit separator.
    /// H: '3.0__1'     // Fraction consecutive digit separator.
    /// I: '3.0e7__1'   // Exponent consecutive digit separator.
    /// ```
    ///
    /// Currently Supported Programming and Data Languages:
    /// ---------------------------------------------------
    ///
    /// 1. Rust
    /// 2. Python
    /// 3. C++ (98, 03, 11, 14, 17)
    /// 4. C (89, 90, 99, 11, 18)
    /// 5. Ruby
    /// 6. Swift
    /// 7. Go
    /// 8. Haskell
    /// 9. Javascript
    /// 10. Perl
    /// 11. PHP
    /// 12. Java
    /// 13. R
    /// 14. Kotlin
    /// 15. Julia
    /// 16. C# (ISO-1, ISO-2, 3, 4, 5, 6, 7)
    /// 17. Kawa
    /// 18. Gambit-C
    /// 19. Guile
    /// 20. Clojure
    /// 21. Erlang
    /// 22. Elm
    /// 23. Scala
    /// 24. Elixir
    /// 25. FORTRAN
    /// 26. D
    /// 27. Coffeescript
    /// 28. Cobol
    /// 29. F#
    /// 30. Visual Basic
    /// 31. OCaml
    /// 32. Objective-C
    /// 33. ReasonML
    /// 34. Octave
    /// 35. Matlab
    /// 36. Zig
    /// 37. SageMath
    /// 38. JSON
    /// 39. TOML
    /// 40. XML
    /// 41. SQLite
    /// 42. PostgreSQL
    /// 43. MySQL
    /// 44. MongoDB
    #[derive(Default)]
    pub struct FloatFormat: u32 {
        // MASKS & FLAGS

        // Mask to extract the flag bits.
        #[doc(hidden)]
        const FLAG_MASK                             = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::REQUIRED_MANTISSA_SIGN.bits
            | Self::NO_EXPONENT_NOTATION.bits
            | Self::NO_POSITIVE_EXPONENT_SIGN.bits
            | Self::REQUIRED_EXPONENT_SIGN.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        /// Mask to extract if any digit separator flags are set.
        #[doc(hidden)]
        const DIGIT_SEPARATOR_FLAG_MASK             = (
            Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        /// Mask to extract if any exponent flags are set.
        #[doc(hidden)]
        const EXPONENT_FLAG_MASK                    = (
            Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_POSITIVE_EXPONENT_SIGN.bits
            | Self::REQUIRED_EXPONENT_SIGN.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::EXPONENT_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_LEADING_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_TRAILING_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        /// Digits are required before the decimal point.
        #[doc(hidden)]
        const REQUIRED_INTEGER_DIGITS               = 0b000000000000000000001;

        /// Digits are required after the decimal point.
        #[doc(hidden)]
        const REQUIRED_FRACTION_DIGITS              = 0b000000000000000000010;

        /// Digits are required after the exponent character.
        #[doc(hidden)]
        const REQUIRED_EXPONENT_DIGITS              = 0b000000000000000000100;

        /// Digits are required before or after the control characters.
        #[doc(hidden)]
        const REQUIRED_DIGITS                       = (
            Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        /// Positive sign before the mantissa is not allowed.
        #[doc(hidden)]
        const NO_POSITIVE_MANTISSA_SIGN             = 0b000000000000000001000;

        /// Positive sign before the mantissa is required.
        #[doc(hidden)]
        const REQUIRED_MANTISSA_SIGN                = 0b000000000000000010000;

        /// Exponent notation is not allowed.
        #[doc(hidden)]
        const NO_EXPONENT_NOTATION                  = 0b000000000000000100000;

        /// Positive sign before the exponent is not allowed.
        #[doc(hidden)]
        const NO_POSITIVE_EXPONENT_SIGN             = 0b000000000000001000000;

        /// Positive sign before the exponent is required.
        #[doc(hidden)]
        const REQUIRED_EXPONENT_SIGN                = 0b000000000000010000000;

        /// Exponent without fraction is not allowed.
        #[doc(hidden)]
        const NO_EXPONENT_WITHOUT_FRACTION          = 0b000000000000100000000;

        /// Digit separators are allowed between integer digits.
        #[doc(hidden)]
        const INTEGER_INTERNAL_DIGIT_SEPARATOR      = 0b000000000001000000000;

        /// Digit separators are allowed between fraction digits.
        #[doc(hidden)]
        const FRACTION_INTERNAL_DIGIT_SEPARATOR     = 0b000000000010000000000;

        /// Digit separators are allowed between exponent digits.
        #[doc(hidden)]
        const EXPONENT_INTERNAL_DIGIT_SEPARATOR     = 0b000000000100000000000;

        /// Digit separators are allowed between digits.
        #[doc(hidden)]
        const INTERNAL_DIGIT_SEPARATOR              = (
            Self::INTEGER_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_INTERNAL_DIGIT_SEPARATOR.bits
        );

        /// A digit separator is allowed before any integer digits.
        #[doc(hidden)]
        const INTEGER_LEADING_DIGIT_SEPARATOR       = 0b000000001000000000000;

        /// A digit separator is allowed before any fraction digits.
        #[doc(hidden)]
        const FRACTION_LEADING_DIGIT_SEPARATOR      = 0b000000010000000000000;

        /// A digit separator is allowed before any exponent digits.
        #[doc(hidden)]
        const EXPONENT_LEADING_DIGIT_SEPARATOR      = 0b000000100000000000000;

        /// A digit separator is allowed before any digits.
        #[doc(hidden)]
        const LEADING_DIGIT_SEPARATOR               = (
            Self::INTEGER_LEADING_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_LEADING_DIGIT_SEPARATOR.bits
        );

        /// A digit separator is allowed after any integer digits.
        #[doc(hidden)]
        const INTEGER_TRAILING_DIGIT_SEPARATOR      = 0b000001000000000000000;

        /// A digit separator is allowed after any fraction digits.
        #[doc(hidden)]
        const FRACTION_TRAILING_DIGIT_SEPARATOR     = 0b000010000000000000000;

        /// A digit separator is allowed after any exponent digits.
        #[doc(hidden)]
        const EXPONENT_TRAILING_DIGIT_SEPARATOR     = 0b000100000000000000000;

        /// A digit separator is allowed after any digits.
        #[doc(hidden)]
        const TRAILING_DIGIT_SEPARATOR              = (
            Self::INTEGER_TRAILING_DIGIT_SEPARATOR.bits
            | Self::FRACTION_TRAILING_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_TRAILING_DIGIT_SEPARATOR.bits
        );

        /// Multiple consecutive integer digit separators are allowed.
        #[doc(hidden)]
        const INTEGER_CONSECUTIVE_DIGIT_SEPARATOR   = 0b001000000000000000000;

        /// Multiple consecutive fraction digit separators are allowed.
        #[doc(hidden)]
        const FRACTION_CONSECUTIVE_DIGIT_SEPARATOR  = 0b010000000000000000000;

        /// Multiple consecutive exponent digit separators are allowed.
        #[doc(hidden)]
        const EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR  = 0b100000000000000000000;

        /// Multiple consecutive digit separators are allowed.
        #[doc(hidden)]
        const CONSECUTIVE_DIGIT_SEPARATOR           = (
            Self::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR.bits
            | Self::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // PRE-DEFINED
        //
        // Sample Format Shorthand:
        // ------------------------
        //
        // The format shorthand lists the test cases, and if applicable,
        // the digit separator character. For example, the shorthand
        // `[134-_]` specifies it passes tests 1, 3, and 4, and uses
        // `'_'` as a digit-separator character. Meanwhile, `[0]` means it
        // passes test 0, and has no digit separator.

        // RUST LITERAL [456789DEFGHI-_]
        /// Float format for a Rust literal floating-point number.
        const RUST_LITERAL = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // RUST STRING [013456]
        /// Float format to parse a Rust float from string.
        const RUST_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // PYTHON LITERAL [013456]
        /// Float format for a Python literal floating-point number.
        const PYTHON_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // PYTHON STRING [013456]
        /// Float format to parse a Python float from string.
        const PYTHON_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C++17 LITERAL [013456789-']
        /// Float format for a C++17 literal floating-point number.
        const CXX17_LITERAL = (
            digit_separator_to_flags(b'\'')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // C++17 STRING [013456]
        const CXX17_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C++14 LITERAL [013456789-']
        /// Float format for a C++14 literal floating-point number.
        const CXX14_LITERAL = (
            digit_separator_to_flags(b'\'')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // C++14 STRING [013456]
        /// Float format to parse a C++14 float from string.
        const CXX14_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C++11 LITERAL [013456]
        /// Float format for a C++11 literal floating-point number.
        const CXX11_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C++11 STRING [013456]
        /// Float format to parse a C++11 float from string.
        const CXX11_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C++03 LITERAL [013456]
        /// Float format for a C++03 literal floating-point number.
        const CXX03_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C++03 STRING [013456]
        /// Float format to parse a C++03 float from string.
        const CXX03_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C++98 LITERAL [013456]
        /// Float format for a C++98 literal floating-point number.
        const CXX98_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C++98 STRING [013456]
        /// Float format to parse a C++98 float from string.
        const CXX98_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C18 LITERAL [013456]
        /// Float format for a C18 literal floating-point number.
        const C18_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C18 STRING [013456]
        /// Float format to parse a C18 float from string.
        const C18_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C11 LITERAL [013456]
        /// Float format for a C11 literal floating-point number.
        const C11_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C11 STRING [013456]
        /// Float format to parse a C11 float from string.
        const C11_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C99 LITERAL [013456]
        /// Float format for a C99 literal floating-point number.
        const C99_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C99 STRING [013456]
        /// Float format to parse a C99 float from string.
        const C99_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C90 LITERAL [013456]
        /// Float format for a C90 literal floating-point number.
        const C90_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C90 STRING [013456]
        /// Float format to parse a C90 float from string.
        const C90_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C89 LITERAL [013456]
        /// Float format for a C89 literal floating-point number.
        const C89_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // C89 STRING [013456]
        /// Float format to parse a C89 float from string.
        const C89_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // RUBY LITERAL [3456789-_]
        /// Float format for a Ruby literal floating-point number.
        const RUBY_LITERAL = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // RUBY STRING [0123456789-_]
        /// Float format to parse a Ruby float from string.
        const RUBY_STRING = (
            digit_separator_to_flags(b'_')
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // SWIFT LITERAL [3456789DEFGHI-_]
        /// Float format for a Swift literal floating-point number.
        const SWIFT_LITERAL = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // SWIFT STRING [13456]
        /// Float format to parse a Swift float from string.
        const SWIFT_STRING = Self::REQUIRED_FRACTION_DIGITS.bits;

        // GO LITERAL [013456]
        /// Float format for a Golang literal floating-point number.
        const GO_LITERAL = Self::REQUIRED_FRACTION_DIGITS.bits;

        // GO STRING [013456]
        /// Float format to parse a Golang float from string.
        const GO_STRING = Self::REQUIRED_FRACTION_DIGITS.bits;

        // HASKELL LITERAL [456]
        /// Float format for a Haskell literal floating-point number.
        const HASKELL_LITERAL = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
        );

        // HASKELL STRING [456]
        /// Float format to parse a Haskell float from string.
        const HASKELL_STRING = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
        );

        // JAVASCRIPT LITERAL [013456]
        /// Float format for a Javascript literal floating-point number.
        const JAVASCRIPT_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // JAVASCRIPT STRING [0123456]
        /// Float format to parse a Javascript float from string.
        const JAVASCRIPT_STRING = 0;

        // PERL LITERAL [013456789BCDEFGHI-_]
        /// Float format for a Perl literal floating-point number.
        const PERL_LITERAL = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::EXPONENT_LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // PERL STRING [0123456]
        /// Float format to parse a Perl float from string.
        const PERL_STRING = 0;

        // PHP LITERAL [013456]
        /// Float format for a PHP literal floating-point number.
        const PHP_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // PHP STRING [0123456]
        /// Float format to parse a PHP float from string.
        const PHP_STRING = 0;

        // JAVA LITERAL [013456789GHI-_]
        /// Float format for a Java literal floating-point number.
        const JAVA_LITERAL = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // JAVA STRING [013456]
        /// Float format to parse a Java float from string.
        const JAVA_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // R LITERAL [013456]
        /// Float format for a R literal floating-point number.
        const R_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // R STRING [0123456]
        /// Float format to parse a R float from string.
        const R_STRING = 0;

        // KOTLIN LITERAL [013456789GHI-_]
        /// Float format for a Kotlin literal floating-point number.
        const KOTLIN_LITERAL = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // KOTLIN STRING [013456]
        /// Float format to parse a Kotlin float from string.
        const KOTLIN_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // JULIA LITERAL [01345678-_]
        /// Float format for a Julia literal floating-point number.
        const JULIA_LITERAL = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTEGER_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_INTERNAL_DIGIT_SEPARATOR.bits
        );

        // JULIA STRING [013456]
        /// Float format to parse a Julia float from string.
        const JULIA_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // CSHARP7 LITERAL [03456789GHI-_]
        /// Float format for a C#7 literal floating-point number.
        const CSHARP7_LITERAL = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // CSHARP7 STRING [013456]
        /// Float format to parse a C#7 float from string.
        const CSHARP7_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // CSHARP6 LITERAL [03456]
        /// Float format for a C#6 literal floating-point number.
        const CSHARP6_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // CSHARP6 STRING [013456]
        /// Float format to parse a C#6 float from string.
        const CSHARP6_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // CSHARP5 LITERAL [03456]
        /// Float format for a C#5 literal floating-point number.
        const CSHARP5_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // CSHARP5 STRING [013456]
        /// Float format to parse a C#5 float from string.
        const CSHARP5_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // CSHARP4 LITERAL [03456]
        /// Float format for a C#4 literal floating-point number.
        const CSHARP4_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // CSHARP4 STRING [013456]
        /// Float format to parse a C#4 float from string.
        const CSHARP4_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // CSHARP3 LITERAL [03456]
        /// Float format for a C#3 literal floating-point number.
        const CSHARP3_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // CSHARP3 STRING [013456]
        /// Float format to parse a C#3 float from string.
        const CSHARP3_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // CSHARP2 LITERAL [03456]
        /// Float format for a C#2 literal floating-point number.
        const CSHARP2_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // CSHARP2 STRING [013456]
        /// Float format to parse a C#2 float from string.
        const CSHARP2_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // CSHARP1 LITERAL [03456]
        /// Float format for a C#1 literal floating-point number.
        const CSHARP1_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // CSHARP1 STRING [013456]
        /// Float format to parse a C#1 float from string.
        const CSHARP1_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // KAWA LITERAL [013456]
        /// Float format for a Kawa literal floating-point number.
        const KAWA_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // KAWA STRING [013456]
        /// Float format to parse a Kawa float from string.
        const KAWA_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // GAMBITC LITERAL [013456]
        /// Float format for a Gambit-C literal floating-point number.
        const GAMBITC_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // GAMBITC STRING [013456]
        /// Float format to parse a Gambit-C float from string.
        const GAMBITC_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // GUILE LITERAL [013456]
        /// Float format for a Guile literal floating-point number.
        const GUILE_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // GUILE STRING [013456]
        /// Float format to parse a Guile float from string.
        const GUILE_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // CLOJURE LITERAL [13456]
        /// Float format for a Clojure literal floating-point number.
        const CLOJURE_LITERAL = (
            Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // CLOJURE STRING [013456]
        /// Float format to parse a Clojure float from string.
        const CLOJURE_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // ERLANG LITERAL [345]
        /// Float format for an Erlang literal floating-point number.
        const ERLANG_LITERAL = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
        );

        // ERLANG STRING [345]
        /// Float format to parse an Erlang float from string.
        const ERLANG_STRING = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
        );

        // ELM LITERAL [456]
        /// Float format for an Elm literal floating-point number.
        const ELM_LITERAL = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
        );

        // ELM STRING [013456]
        /// Float format to parse an Elm float from string.
        const ELM_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // SCALA LITERAL [3456]
        /// Float format for a Scala literal floating-point number.
        const SCALA_LITERAL = Self::REQUIRED_DIGITS.bits;

        // SCALA STRING [013456]
        /// Float format to parse a Scala float from string.
        const SCALA_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // ELIXIR LITERAL [345789-_]
        /// Float format for an Elixir literal floating-point number.
        const ELIXIR_LITERAL = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // ELIXIR STRING [345]
        /// Float format to parse an Elixir float from string.
        const ELIXIR_STRING = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
        );

        // FORTRAN LITERAL [013456]
        /// Float format for a FORTRAN literal floating-point number.
        const FORTRAN_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // FORTRAN STRING [013456]
        /// Float format to parse a FORTRAN float from string.
        const FORTRAN_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // D LITERAL [013456789DEFGHI-_]
        /// Float format for a D literal floating-point number.
        const D_LITERAL = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // D STRING [01345678DE-_]
        /// Float format to parse a D float from string.
        const D_STRING = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTEGER_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_INTERNAL_DIGIT_SEPARATOR.bits
            | Self::INTEGER_TRAILING_DIGIT_SEPARATOR.bits
            | Self::FRACTION_TRAILING_DIGIT_SEPARATOR.bits
        );

        // COFFEESCRIPT LITERAL [013456]
        /// Float format for a Coffeescript literal floating-point number.
        const COFFEESCRIPT_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // COFFEESCRIPT STRING [0123456]
        /// Float format to parse a Coffeescript float from string.
        const COFFEESCRIPT_STRING = 0;

        // COBOL LITERAL [0345]
        /// Float format for a Cobol literal floating-point number.
        const COBOL_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_EXPONENT_WITHOUT_FRACTION.bits
        );

        // COBOL STRING [012356]
        /// Float format to parse a Cobol float from string.
        const COBOL_STRING = Self::REQUIRED_EXPONENT_SIGN.bits;

        // FSHARP LITERAL [13456789GHI-_]
        /// Float format for a F# literal floating-point number.
        const FSHARP_LITERAL = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // FSHARP STRING [013456789:BCDEFGHI-_]
        /// Float format to parse a F# float from string.
        const FSHARP_STRING = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // VB LITERAL [03456]
        /// Float format for a Visual Basic literal floating-point number.
        const VB_LITERAL = (
            Self::REQUIRED_FRACTION_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
        );

        // VB STRING [013456]
        /// Float format to parse a Visual Basic float from string.
        const VB_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // OCAML LITERAL [1456789BDEFGHI-_]
        /// Float format for an OCaml literal floating-point number.
        const OCAML_LITERAL = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // OCAML STRING [013456789:BCDEFGHI-_]
        /// Float format to parse an OCaml float from string.
        const OCAML_STRING = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // OBJECTIVEC LITERAL [013456]
        /// Float format for an Objective-C literal floating-point number.
        const OBJECTIVEC_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // OBJECTIVEC STRING [013456]
        /// Float format to parse an Objective-C float from string.
        const OBJECTIVEC_STRING = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // REASONML LITERAL [13456789BDEFGHI-_]
        /// Float format for a ReasonML literal floating-point number.
        const REASONML_LITERAL = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // REASONML STRING [013456789:BCDEFGHI-_]
        /// Float format to parse a ReasonML float from string.
        const REASONML_STRING = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // OCTAVE LITERAL [013456789BDEFGHI-_]
        /// Float format for an Octave literal floating-point number.
        const OCTAVE_LITERAL = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // OCTAVE STRING [013456789:BCDEFGHI-,]
        /// Float format to parse an Octave float from string.
        const OCTAVE_STRING = (
            digit_separator_to_flags(b',')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // MATLAB LITERAL [013456789BDEFGHI-_]
        /// Float format for an Matlab literal floating-point number.
        const MATLAB_LITERAL = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::FRACTION_LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // MATLAB STRING [013456789:BCDEFGHI-,]
        /// Float format to parse an Matlab float from string.
        const MATLAB_STRING = (
            digit_separator_to_flags(b',')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
            | Self::LEADING_DIGIT_SEPARATOR.bits
            | Self::TRAILING_DIGIT_SEPARATOR.bits
            | Self::CONSECUTIVE_DIGIT_SEPARATOR.bits
        );

        // ZIG LITERAL [1456]
        /// Float format for a Zig literal floating-point number.
        const ZIG_LITERAL = (
            Self::REQUIRED_INTEGER_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
        );

        // ZIG STRING [0123456]
        /// Float format to parse a Zig float from string.
        const ZIG_STRING = 0;

        // SAGE LITERAL [0123456]
        /// Float format for a Sage literal floating-point number.
        const SAGE_LITERAL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // SAGE STRING [013456789-_]
        /// Float format to parse a Sage float from string.
        const SAGE_STRING = (
            digit_separator_to_flags(b'_')
            | Self::REQUIRED_EXPONENT_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // JSON [456]
        /// Float format for a JSON literal floating-point number.
        const JSON = (
            Self::REQUIRED_DIGITS.bits
            | Self::NO_POSITIVE_MANTISSA_SIGN.bits
        );

        // TOML [3456789]
        /// Float format for a TOML literal floating-point number.
        const TOML = (
            Self::REQUIRED_DIGITS.bits
            | Self::INTERNAL_DIGIT_SEPARATOR.bits
        );

        // YAML (defined in-terms of JSON schema).
        /// Float format for a YAML literal floating-point number.
        const YAML = Self::JSON.bits;

        // XML [012345]
        /// Float format for a XML literal floating-point number.
        const XML = 0;

        // SQLITE [013456]
        /// Float format for a SQLite literal floating-point number.
        const SQLITE = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // POSTGRESQL [013456]
        /// Float format for a PostgreSQL literal floating-point number.
        const POSTGRESQL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // MYSQL [013456]
        /// Float format for a MySQL literal floating-point number.
        const MYSQL = Self::REQUIRED_EXPONENT_DIGITS.bits;

        // MONGODB [013456]
        /// Float format for a MongoDB literal floating-point number.
        const MONGODB = Self::REQUIRED_EXPONENT_DIGITS.bits;
    }
}

// Ensure all our bit flags are valid.
macro_rules! check_subsequent_flags {
    ($x:ident, $y:ident) => (
        const_assert!(FloatFormat::$x.bits << 1 == FloatFormat::$y.bits);
    );
}
check_subsequent_flags!(REQUIRED_INTEGER_DIGITS, REQUIRED_FRACTION_DIGITS);
check_subsequent_flags!(REQUIRED_FRACTION_DIGITS, REQUIRED_EXPONENT_DIGITS);
check_subsequent_flags!(REQUIRED_EXPONENT_DIGITS, NO_POSITIVE_MANTISSA_SIGN);
check_subsequent_flags!(NO_POSITIVE_MANTISSA_SIGN, REQUIRED_MANTISSA_SIGN);
check_subsequent_flags!(REQUIRED_MANTISSA_SIGN, NO_EXPONENT_NOTATION);
check_subsequent_flags!(NO_EXPONENT_NOTATION, NO_POSITIVE_EXPONENT_SIGN);
check_subsequent_flags!(NO_POSITIVE_EXPONENT_SIGN, REQUIRED_EXPONENT_SIGN);
check_subsequent_flags!(REQUIRED_EXPONENT_SIGN, NO_EXPONENT_WITHOUT_FRACTION);
check_subsequent_flags!(NO_EXPONENT_WITHOUT_FRACTION, INTEGER_INTERNAL_DIGIT_SEPARATOR);
check_subsequent_flags!(INTEGER_INTERNAL_DIGIT_SEPARATOR, FRACTION_INTERNAL_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_INTERNAL_DIGIT_SEPARATOR, EXPONENT_INTERNAL_DIGIT_SEPARATOR);
check_subsequent_flags!(EXPONENT_INTERNAL_DIGIT_SEPARATOR, INTEGER_LEADING_DIGIT_SEPARATOR);
check_subsequent_flags!(INTEGER_LEADING_DIGIT_SEPARATOR, FRACTION_LEADING_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_LEADING_DIGIT_SEPARATOR, EXPONENT_LEADING_DIGIT_SEPARATOR);
check_subsequent_flags!(EXPONENT_LEADING_DIGIT_SEPARATOR, INTEGER_TRAILING_DIGIT_SEPARATOR);
check_subsequent_flags!(INTEGER_TRAILING_DIGIT_SEPARATOR, FRACTION_TRAILING_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_TRAILING_DIGIT_SEPARATOR, EXPONENT_TRAILING_DIGIT_SEPARATOR);
check_subsequent_flags!(EXPONENT_TRAILING_DIGIT_SEPARATOR, INTEGER_CONSECUTIVE_DIGIT_SEPARATOR);
check_subsequent_flags!(INTEGER_CONSECUTIVE_DIGIT_SEPARATOR, FRACTION_CONSECUTIVE_DIGIT_SEPARATOR);
check_subsequent_flags!(FRACTION_CONSECUTIVE_DIGIT_SEPARATOR, EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR);

/// Add flag to flags
macro_rules! add_flag {
    ($flags:ident, $bool:ident, $flag:ident) => {
        if $bool {
            $flags |= FloatFormat::$flag;
        }
    };
}

impl FloatFormat {
    /// Compile float format value from specifications.
    ///
    /// * `digit_separator`                         - Character to separate digits.
    /// * `required_integer_digits`                 - If digits are required before the decimal point.
    /// * `required_fraction_digits`                - If digits are required after the decimal point.
    /// * `required_exponent_digits`                - If digits are required after the exponent character.
    /// * `no_positive_mantissa_sign`               - If positive sign before the mantissa is not allowed.
    /// * `required_mantissa_sign`                  - If positive sign before the mantissa is required.
    /// * `no_exponent_notation`                    - If exponent notation is not allowed.
    /// * `no_positive_exponent_sign`               - If positive sign before the exponent is not allowed.
    /// * `required_exponent_sign`                  - If positive sign before the exponent is required.
    /// * `no_exponent_without_fraction`            - If exponent without fraction is not allowed.
    /// * `integer_internal_digit_separator`        - If digit separators are allowed between integer digits.
    /// * `fraction_internal_digit_separator`       - If digit separators are allowed between fraction digits.
    /// * `exponent_internal_digit_separator`       - If digit separators are allowed between exponent digits.
    /// * `integer_leading_digit_separator`         - If a digit separator is allowed before any integer digits.
    /// * `fraction_leading_digit_separator`        - If a digit separator is allowed before any fraction digits.
    /// * `exponent_leading_digit_separator`        - If a digit separator is allowed before any exponent digits.
    /// * `integer_trailing_digit_separator`        - If a digit separator is allowed after any integer digits.
    /// * `fraction_trailing_digit_separator`       - If a digit separator is allowed after any fraction digits.
    /// * `exponent_trailing_digit_separator`       - If a digit separator is allowed after any exponent digits.
    /// * `integer_consecutive_digit_separator`     - If multiple consecutive integer digit separators are allowed.
    /// * `fraction_consecutive_digit_separator`    - If multiple consecutive fraction digit separators are allowed.
    /// * `exponent_consecutive_digit_separator`    - If multiple consecutive exponent digit separators are allowed.
    ///
    /// Returns the value if it was able to compile the format,
    /// otherwise, returns None.
    #[cfg_attr(feature = "radix", doc = " Digit separators must not be in the character group `[A-Za-z0-9+.-]`, nor be equal to")]
    #[cfg_attr(feature = "radix", doc = " [`get_exponent_default_char`](fn.get_exponent_default_char.html) or")]
    #[cfg_attr(feature = "radix", doc = " [`get_exponent_backup_char`](fn.get_exponent_backup_char.html).")]
    #[cfg_attr(not(feature = "radix"), doc = " Digit separators must not be in the character group `[0-9+.-]`, nor be equal to")]
    #[cfg_attr(not(feature = "radix"), doc = " [get_exponent_default_char](fn.get_exponent_default_char.html).")]
    #[inline]
    pub fn compile(
        digit_separator: u8,
        required_integer_digits: bool,
        required_fraction_digits: bool,
        required_exponent_digits: bool,
        no_positive_mantissa_sign: bool,
        required_mantissa_sign: bool,
        no_exponent_notation: bool,
        no_positive_exponent_sign: bool,
        required_exponent_sign: bool,
        no_exponent_without_fraction: bool,
        integer_internal_digit_separator: bool,
        fraction_internal_digit_separator: bool,
        exponent_internal_digit_separator: bool,
        integer_leading_digit_separator: bool,
        fraction_leading_digit_separator: bool,
        exponent_leading_digit_separator: bool,
        integer_trailing_digit_separator: bool,
        fraction_trailing_digit_separator: bool,
        exponent_trailing_digit_separator: bool,
        integer_consecutive_digit_separator: bool,
        fraction_consecutive_digit_separator: bool,
        exponent_consecutive_digit_separator: bool
    ) -> Option<FloatFormat> {
        let mut format = FloatFormat::default();
        // Generic flags.
        add_flag!(format, required_integer_digits, REQUIRED_INTEGER_DIGITS);
        add_flag!(format, required_fraction_digits, REQUIRED_FRACTION_DIGITS);
        add_flag!(format, required_exponent_digits, REQUIRED_EXPONENT_DIGITS);
        add_flag!(format, no_positive_mantissa_sign, NO_POSITIVE_MANTISSA_SIGN);
        add_flag!(format, required_mantissa_sign, REQUIRED_MANTISSA_SIGN);
        add_flag!(format, no_exponent_notation, NO_EXPONENT_NOTATION);
        add_flag!(format, no_positive_exponent_sign, NO_POSITIVE_EXPONENT_SIGN);
        add_flag!(format, required_exponent_sign, REQUIRED_EXPONENT_SIGN);
        add_flag!(format, no_exponent_without_fraction, NO_EXPONENT_WITHOUT_FRACTION);

        // Digit separator flags.
        add_flag!(format, integer_internal_digit_separator, INTEGER_INTERNAL_DIGIT_SEPARATOR);
        add_flag!(format, fraction_internal_digit_separator, FRACTION_INTERNAL_DIGIT_SEPARATOR);
        add_flag!(format, exponent_internal_digit_separator, EXPONENT_INTERNAL_DIGIT_SEPARATOR);
        add_flag!(format, integer_leading_digit_separator, INTEGER_LEADING_DIGIT_SEPARATOR);
        add_flag!(format, fraction_leading_digit_separator, FRACTION_LEADING_DIGIT_SEPARATOR);
        add_flag!(format, exponent_leading_digit_separator, EXPONENT_LEADING_DIGIT_SEPARATOR);
        add_flag!(format, integer_trailing_digit_separator, INTEGER_TRAILING_DIGIT_SEPARATOR);
        add_flag!(format, fraction_trailing_digit_separator, FRACTION_TRAILING_DIGIT_SEPARATOR);
        add_flag!(format, exponent_trailing_digit_separator, EXPONENT_TRAILING_DIGIT_SEPARATOR);
        add_flag!(format, integer_consecutive_digit_separator, INTEGER_CONSECUTIVE_DIGIT_SEPARATOR);
        add_flag!(format, fraction_consecutive_digit_separator, FRACTION_CONSECUTIVE_DIGIT_SEPARATOR);
        add_flag!(format, exponent_consecutive_digit_separator, EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR);

        // Digit separator.
        if format.intersects(FloatFormat::DIGIT_SEPARATOR_FLAG_MASK) {
            if !is_valid_separator(digit_separator) {
                return None;
            }
            format.bits |= digit_separator_to_flags(digit_separator);
        }

        // Validation.
        if format.intersects(FloatFormat::NO_EXPONENT_NOTATION) && format.intersects(FloatFormat::EXPONENT_FLAG_MASK) {
            return None
        }
        if no_positive_mantissa_sign && required_mantissa_sign {
            return None
        }
        if no_positive_exponent_sign && required_exponent_sign {
            return None
        }

        Some(format)
    }

    /// Compile float format value from specifications.
    ///
    /// * `digit_separator`                         - Character to separate digits.
    ///
    /// Returns the value if it was able to compile the format,
    /// otherwise, returns None.
    pub fn ignore(digit_separator: u8) -> Option<FloatFormat> {
        // Provide the options by-name, because by-value will be unmaintainable.
        // Set all the default flags to false.
        let required_integer_digits = false;
        let required_fraction_digits = false;
        let required_exponent_digits = false;
        let no_positive_mantissa_sign = false;
        let required_mantissa_sign = false;
        let no_exponent_notation = false;
        let no_positive_exponent_sign = false;
        let required_exponent_sign = false;
        let no_exponent_without_fraction = false;
        // Set all the digit separator flags to true.
        let integer_internal_digit_separator = true;
        let fraction_internal_digit_separator = true;
        let exponent_internal_digit_separator = true;
        let integer_leading_digit_separator = true;
        let fraction_leading_digit_separator = true;
        let exponent_leading_digit_separator = true;
        let integer_trailing_digit_separator = true;
        let fraction_trailing_digit_separator = true;
        let exponent_trailing_digit_separator = true;
        let integer_consecutive_digit_separator = true;
        let fraction_consecutive_digit_separator = true;
        let exponent_consecutive_digit_separator = true;

        FloatFormat::compile(
            digit_separator,
            required_integer_digits,
            required_fraction_digits,
            required_exponent_digits,
            no_positive_mantissa_sign,
            required_mantissa_sign,
            no_exponent_notation,
            no_positive_exponent_sign,
            required_exponent_sign,
            no_exponent_without_fraction,
            integer_internal_digit_separator,
            fraction_internal_digit_separator,
            exponent_internal_digit_separator,
            integer_leading_digit_separator,
            fraction_leading_digit_separator,
            exponent_leading_digit_separator,
            integer_trailing_digit_separator,
            fraction_trailing_digit_separator,
            exponent_trailing_digit_separator,
            integer_consecutive_digit_separator,
            fraction_consecutive_digit_separator,
            exponent_consecutive_digit_separator
        )
    }

    /// Get the flag bits from the compiled float format.
    #[inline]
    pub fn flags(self) -> FloatFormat {
        return self & FloatFormat::FLAG_MASK
    }

    /// Get the digit separator from the compiled float format.
    #[inline]
    pub fn digit_separator(self) -> u8 {
        digit_separator_from_flags(self.bits)
    }

    /// Get if digits are required before the decimal point.
    #[inline]
    pub fn required_integer_digits(self) -> bool {
        self.intersects(FloatFormat::REQUIRED_INTEGER_DIGITS)
    }

    /// Get if digits are required after the decimal point.
    #[inline]
    pub fn required_fraction_digits(self) -> bool {
        self.intersects(FloatFormat::REQUIRED_FRACTION_DIGITS)
    }

    /// Get if digits are required after the exponent character.
    #[inline]
    pub fn required_exponent_digits(self) -> bool {
        self.intersects(FloatFormat::REQUIRED_EXPONENT_DIGITS)
    }

    /// Get if digits are required before or after the decimal point.
    #[inline]
    pub fn required_digits(self) -> bool {
        self.intersects(FloatFormat::REQUIRED_DIGITS)
    }

    /// Get if positive sign before the mantissa is not allowed.
    #[inline]
    pub fn no_positive_mantissa_sign(self) -> bool {
        self.intersects(FloatFormat::NO_POSITIVE_MANTISSA_SIGN)
    }

    /// Get if positive sign before the mantissa is required.
    #[inline]
    pub fn required_mantissa_sign(self) -> bool {
        self.intersects(FloatFormat::REQUIRED_MANTISSA_SIGN)
    }

    /// Get if exponent notation is not allowed.
    #[inline]
    pub fn no_exponent_notation(self) -> bool {
        self.intersects(FloatFormat::NO_EXPONENT_NOTATION)
    }

    /// Get if positive sign before the exponent is not allowed.
    #[inline]
    pub fn no_positive_exponent_sign(self) -> bool {
        self.intersects(FloatFormat::NO_POSITIVE_EXPONENT_SIGN)
    }

    /// Get if positive sign before the exponent is required.
    #[inline]
    pub fn required_exponent_sign(self) -> bool {
        self.intersects(FloatFormat::REQUIRED_EXPONENT_SIGN)
    }

    /// Get if exponent without fraction is not allowed.
    #[inline]
    pub fn no_exponent_without_fraction(self) -> bool {
        self.intersects(FloatFormat::NO_EXPONENT_WITHOUT_FRACTION)
    }

    /// Get if digit separators are allowed between integer digits.
    #[inline]
    pub fn integer_internal_digit_separator(self) -> bool {
        self.intersects(FloatFormat::INTEGER_INTERNAL_DIGIT_SEPARATOR)
    }

    /// Get if digit separators are allowed between fraction digits.
    #[inline]
    pub fn fraction_internal_digit_separator(self) -> bool {
        self.intersects(FloatFormat::FRACTION_INTERNAL_DIGIT_SEPARATOR)
    }

    /// Get if digit separators are allowed between exponent digits.
    #[inline]
    pub fn exponent_internal_digit_separator(self) -> bool {
        self.intersects(FloatFormat::EXPONENT_INTERNAL_DIGIT_SEPARATOR)
    }

    /// Get if digit separators are allowed between digits.
    #[inline]
    pub fn internal_digit_separator(self) -> bool {
        self.intersects(FloatFormat::INTERNAL_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed before any integer digits.
    #[inline]
    pub fn integer_leading_digit_separator(self) -> bool {
        self.intersects(FloatFormat::INTEGER_LEADING_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed before any fraction digits.
    #[inline]
    pub fn fraction_leading_digit_separator(self) -> bool {
        self.intersects(FloatFormat::FRACTION_LEADING_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed before any exponent digits.
    #[inline]
    pub fn exponent_leading_digit_separator(self) -> bool {
        self.intersects(FloatFormat::EXPONENT_LEADING_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed before any digits.
    #[inline]
    pub fn leading_digit_separator(self) -> bool {
        self.intersects(FloatFormat::LEADING_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed after any integer digits.
    #[inline]
    pub fn integer_trailing_digit_separator(self) -> bool {
        self.intersects(FloatFormat::INTEGER_TRAILING_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed after any fraction digits.
    #[inline]
    pub fn fraction_trailing_digit_separator(self) -> bool {
        self.intersects(FloatFormat::FRACTION_TRAILING_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed after any exponent digits.
    #[inline]
    pub fn exponent_trailing_digit_separator(self) -> bool {
        self.intersects(FloatFormat::EXPONENT_TRAILING_DIGIT_SEPARATOR)
    }

    /// Get if a digit separator is allowed after any digits.
    #[inline]
    pub fn trailing_digit_separator(self) -> bool {
        self.intersects(FloatFormat::TRAILING_DIGIT_SEPARATOR)
    }

    /// Get if multiple consecutive integer digit separators are allowed.
    #[inline]
    pub fn integer_consecutive_digit_separator(self) -> bool {
        self.intersects(FloatFormat::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR)
    }

    /// Get if multiple consecutive fraction digit separators are allowed.
    #[inline]
    pub fn fraction_consecutive_digit_separator(self) -> bool {
        self.intersects(FloatFormat::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR)
    }

    /// Get if multiple consecutive exponent digit separators are allowed.
    #[inline]
    pub fn exponent_consecutive_digit_separator(self) -> bool {
        self.intersects(FloatFormat::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR)
    }

    /// Get if multiple consecutive digit separators are allowed.
    #[inline]
    pub fn consecutive_digit_separator(self) -> bool {
        self.intersects(FloatFormat::CONSECUTIVE_DIGIT_SEPARATOR)
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_separator() {
        assert_eq!(is_valid_separator(b'_'), true);
        assert_eq!(is_valid_separator(b'\''), true);
        assert_eq!(is_valid_separator(b'0'), false);
        assert_eq!(is_valid_separator(128), false);
    }

    #[test]
    fn test_compile() {
        // Test all false
        let flags = FloatFormat::compile(b'_', false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false).unwrap();
        assert_eq!(flags.flags(), FloatFormat::default());
        assert_eq!(flags.digit_separator(), 0);
    }

    #[test]
    fn test_ignore() {
        let flags = FloatFormat::ignore(b'_').unwrap();
        assert_eq!(flags.flags(), FloatFormat::DIGIT_SEPARATOR_FLAG_MASK);
        assert_eq!(flags.digit_separator(), b'_');
        assert_eq!(flags.required_integer_digits(), false);
        assert_eq!(flags.required_fraction_digits(), false);
        assert_eq!(flags.required_exponent_digits(), false);
        assert_eq!(flags.required_digits(), false);
        assert_eq!(flags.no_positive_mantissa_sign(), false);
        assert_eq!(flags.required_mantissa_sign(), false);
        assert_eq!(flags.no_exponent_notation(), false);
        assert_eq!(flags.no_positive_exponent_sign(), false);
        assert_eq!(flags.required_exponent_sign(), false);
        assert_eq!(flags.no_exponent_without_fraction(), false);
        assert_eq!(flags.integer_internal_digit_separator(), true);
        assert_eq!(flags.fraction_internal_digit_separator(), true);
        assert_eq!(flags.exponent_internal_digit_separator(), true);
        assert_eq!(flags.internal_digit_separator(), true);
        assert_eq!(flags.integer_leading_digit_separator(), true);
        assert_eq!(flags.fraction_leading_digit_separator(), true);
        assert_eq!(flags.exponent_leading_digit_separator(), true);
        assert_eq!(flags.leading_digit_separator(), true);
        assert_eq!(flags.integer_trailing_digit_separator(), true);
        assert_eq!(flags.fraction_trailing_digit_separator(), true);
        assert_eq!(flags.exponent_trailing_digit_separator(), true);
        assert_eq!(flags.trailing_digit_separator(), true);
        assert_eq!(flags.integer_consecutive_digit_separator(), true);
        assert_eq!(flags.fraction_consecutive_digit_separator(), true);
        assert_eq!(flags.exponent_consecutive_digit_separator(), true);
        assert_eq!(flags.consecutive_digit_separator(), true);
    }

    #[test]
    fn test_flags() {
        let flags = [
            FloatFormat::REQUIRED_INTEGER_DIGITS,
            FloatFormat::REQUIRED_FRACTION_DIGITS,
            FloatFormat::REQUIRED_EXPONENT_DIGITS,
            FloatFormat::NO_POSITIVE_MANTISSA_SIGN,
            FloatFormat::REQUIRED_MANTISSA_SIGN,
            FloatFormat::NO_EXPONENT_NOTATION,
            FloatFormat::NO_POSITIVE_EXPONENT_SIGN,
            FloatFormat::REQUIRED_EXPONENT_SIGN,
            FloatFormat::NO_EXPONENT_WITHOUT_FRACTION,
            FloatFormat::INTEGER_INTERNAL_DIGIT_SEPARATOR,
            FloatFormat::FRACTION_INTERNAL_DIGIT_SEPARATOR,
            FloatFormat::EXPONENT_INTERNAL_DIGIT_SEPARATOR,
            FloatFormat::INTEGER_LEADING_DIGIT_SEPARATOR,
            FloatFormat::FRACTION_LEADING_DIGIT_SEPARATOR,
            FloatFormat::EXPONENT_LEADING_DIGIT_SEPARATOR,
            FloatFormat::INTEGER_TRAILING_DIGIT_SEPARATOR,
            FloatFormat::FRACTION_TRAILING_DIGIT_SEPARATOR,
            FloatFormat::EXPONENT_TRAILING_DIGIT_SEPARATOR,
            FloatFormat::INTEGER_CONSECUTIVE_DIGIT_SEPARATOR,
            FloatFormat::FRACTION_CONSECUTIVE_DIGIT_SEPARATOR,
            FloatFormat::EXPONENT_CONSECUTIVE_DIGIT_SEPARATOR
        ];
        for &flag in flags.iter() {
            assert_eq!(flag.flags(), flag);
            assert_eq!(flag.digit_separator(), 0);
        }
    }

    #[test]
    fn test_constants() {
        let flags = [
            FloatFormat::RUST_LITERAL,
            FloatFormat::RUST_STRING,
            FloatFormat::PYTHON_LITERAL,
            FloatFormat::PYTHON_STRING,
            FloatFormat::CXX17_LITERAL,
            FloatFormat::CXX17_STRING,
            FloatFormat::CXX14_LITERAL,
            FloatFormat::CXX14_STRING,
            FloatFormat::CXX11_LITERAL,
            FloatFormat::CXX11_STRING,
            FloatFormat::CXX03_LITERAL,
            FloatFormat::CXX03_STRING,
            FloatFormat::CXX98_LITERAL,
            FloatFormat::CXX98_STRING,
            FloatFormat::C18_LITERAL,
            FloatFormat::C18_STRING,
            FloatFormat::C11_LITERAL,
            FloatFormat::C11_STRING,
            FloatFormat::C99_LITERAL,
            FloatFormat::C99_STRING,
            FloatFormat::C90_LITERAL,
            FloatFormat::C90_STRING,
            FloatFormat::C89_LITERAL,
            FloatFormat::C89_STRING,
            FloatFormat::RUBY_LITERAL,
            FloatFormat::RUBY_STRING,
            FloatFormat::SWIFT_LITERAL,
            FloatFormat::SWIFT_STRING,
            FloatFormat::GO_LITERAL,
            FloatFormat::GO_STRING,
            FloatFormat::HASKELL_LITERAL,
            FloatFormat::HASKELL_STRING,
            FloatFormat::JAVASCRIPT_LITERAL,
            FloatFormat::JAVASCRIPT_STRING,
            FloatFormat::PERL_LITERAL,
            FloatFormat::PERL_STRING,
            FloatFormat::PHP_LITERAL,
            FloatFormat::PHP_STRING,
            FloatFormat::JAVA_LITERAL,
            FloatFormat::JAVA_STRING,
            FloatFormat::R_LITERAL,
            FloatFormat::R_STRING,
            FloatFormat::KOTLIN_LITERAL,
            FloatFormat::KOTLIN_STRING,
            FloatFormat::JULIA_LITERAL,
            FloatFormat::JULIA_STRING,
            FloatFormat::CSHARP7_LITERAL,
            FloatFormat::CSHARP7_STRING,
            FloatFormat::CSHARP6_LITERAL,
            FloatFormat::CSHARP6_STRING,
            FloatFormat::CSHARP5_LITERAL,
            FloatFormat::CSHARP5_STRING,
            FloatFormat::CSHARP4_LITERAL,
            FloatFormat::CSHARP4_STRING,
            FloatFormat::CSHARP3_LITERAL,
            FloatFormat::CSHARP3_STRING,
            FloatFormat::CSHARP2_LITERAL,
            FloatFormat::CSHARP2_STRING,
            FloatFormat::CSHARP1_LITERAL,
            FloatFormat::CSHARP1_STRING,
            FloatFormat::KAWA_LITERAL,
            FloatFormat::KAWA_STRING,
            FloatFormat::GAMBITC_LITERAL,
            FloatFormat::GAMBITC_STRING,
            FloatFormat::GUILE_LITERAL,
            FloatFormat::GUILE_STRING,
            FloatFormat::CLOJURE_LITERAL,
            FloatFormat::CLOJURE_STRING,
            FloatFormat::ERLANG_LITERAL,
            FloatFormat::ERLANG_STRING,
            FloatFormat::ELM_LITERAL,
            FloatFormat::ELM_STRING,
            FloatFormat::SCALA_LITERAL,
            FloatFormat::SCALA_STRING,
            FloatFormat::ELIXIR_LITERAL,
            FloatFormat::ELIXIR_STRING,
            FloatFormat::FORTRAN_LITERAL,
            FloatFormat::FORTRAN_STRING,
            FloatFormat::D_LITERAL,
            FloatFormat::D_STRING,
            FloatFormat::COFFEESCRIPT_LITERAL,
            FloatFormat::COFFEESCRIPT_STRING,
            FloatFormat::COBOL_LITERAL,
            FloatFormat::COBOL_STRING,
            FloatFormat::FSHARP_LITERAL,
            FloatFormat::FSHARP_STRING,
            FloatFormat::VB_LITERAL,
            FloatFormat::VB_STRING,
            FloatFormat::OCAML_LITERAL,
            FloatFormat::OCAML_STRING,
            FloatFormat::OBJECTIVEC_LITERAL,
            FloatFormat::OBJECTIVEC_STRING,
            FloatFormat::REASONML_LITERAL,
            FloatFormat::REASONML_STRING,
            FloatFormat::OCTAVE_LITERAL,
            FloatFormat::OCTAVE_STRING,
            FloatFormat::MATLAB_LITERAL,
            FloatFormat::MATLAB_STRING,
            FloatFormat::ZIG_LITERAL,
            FloatFormat::ZIG_STRING,
            FloatFormat::SAGE_LITERAL,
            FloatFormat::SAGE_STRING,
            FloatFormat::JSON,
            FloatFormat::TOML,
            FloatFormat::YAML,
            FloatFormat::XML,
            FloatFormat::SQLITE,
            FloatFormat::POSTGRESQL,
            FloatFormat::MYSQL,
            FloatFormat::MONGODB
        ];
        for &flag in flags.iter() {
            // Just wanna check the flags are defined.
            assert!((flag.bits == 0) | true);
            assert!((flag.digit_separator() == 0) | true);
        }
    }
}
