#![cfg(all(feature = "format", feature = "parse"))]

use lexical_util::skip::{self, SkipIter};

fn skip_iter_eq<const FORMAT: u128>(input: &[u8], output: &[u8]) {
    // next is done in terms of peek, so we're safe here.
    assert!(input.skip_iter::<10, FORMAT>().eq(output.iter()));
}

#[test]
fn test_skip_iter_i() {
    // Test iterators that skip single, internal-only digit separators.
    skip_iter_eq::<{ skip::I }>(b"123.45", b"123.45");
    skip_iter_eq::<{ skip::I }>(b"1e45", b"1e45");
    skip_iter_eq::<{ skip::I }>(b"1e", b"1e");
    skip_iter_eq::<{ skip::I }>(b"1", b"1");
    skip_iter_eq::<{ skip::I }>(b"_45", b"_45");
    skip_iter_eq::<{ skip::I }>(b"__45", b"__45");
    skip_iter_eq::<{ skip::I }>(b"_.45", b"_.45");
    skip_iter_eq::<{ skip::I }>(b"__.45", b"__.45");
    skip_iter_eq::<{ skip::I }>(b"4_5", b"45");
    skip_iter_eq::<{ skip::I }>(b"4__5", b"4_5");
    skip_iter_eq::<{ skip::I }>(b"4_", b"4_");
    skip_iter_eq::<{ skip::I }>(b"4__", b"4__");
    skip_iter_eq::<{ skip::I }>(b"4_.", b"4_.");
    skip_iter_eq::<{ skip::I }>(b"4__.", b"4__.");
    skip_iter_eq::<{ skip::I }>(b"_45_5", b"_455");
    skip_iter_eq::<{ skip::I }>(b"__45__5", b"__45_5");
    skip_iter_eq::<{ skip::I }>(b"_.45_5", b"_.455");
    skip_iter_eq::<{ skip::I }>(b"__.45__5", b"__.45_5");
    skip_iter_eq::<{ skip::I }>(b"4_5_", b"45_");
    skip_iter_eq::<{ skip::I }>(b"4__5__", b"4_5__");
    skip_iter_eq::<{ skip::I }>(b"4_5_.5", b"45_.5");
    skip_iter_eq::<{ skip::I }>(b"4__5__.5", b"4_5__.5");
    skip_iter_eq::<{ skip::I }>(b"_45_", b"_45_");
    skip_iter_eq::<{ skip::I }>(b"__45__", b"__45__");
    skip_iter_eq::<{ skip::I }>(b"_45_.56", b"_45_.56");
    skip_iter_eq::<{ skip::I }>(b"__45__.56", b"__45__.56");
    skip_iter_eq::<{ skip::I }>(b"_4_5_", b"_45_");
    skip_iter_eq::<{ skip::I }>(b"__4__5__", b"__4_5__");
    skip_iter_eq::<{ skip::I }>(b"_4_5_.56", b"_45_.56");
    skip_iter_eq::<{ skip::I }>(b"__4__5__.56", b"__4_5__.56");
}

#[test]
fn test_skip_iter_l() {
    // Test iterators that skip single, leading-only digit separators.
    skip_iter_eq::<{ skip::L }>(b"123.45", b"123.45");
    skip_iter_eq::<{ skip::L }>(b"1e45", b"1e45");
    skip_iter_eq::<{ skip::L }>(b"1e", b"1e");
    skip_iter_eq::<{ skip::L }>(b"1", b"1");
    skip_iter_eq::<{ skip::L }>(b"_45", b"45");
    skip_iter_eq::<{ skip::L }>(b"__45", b"_45");
    skip_iter_eq::<{ skip::L }>(b"_.45", b".45");
    skip_iter_eq::<{ skip::L }>(b"__.45", b"_.45");
    skip_iter_eq::<{ skip::L }>(b"4_5", b"4_5");
    skip_iter_eq::<{ skip::L }>(b"4__5", b"4__5");
    skip_iter_eq::<{ skip::L }>(b"4_", b"4_");
    skip_iter_eq::<{ skip::L }>(b"4__", b"4__");
    skip_iter_eq::<{ skip::L }>(b"4_.", b"4_.");
    skip_iter_eq::<{ skip::L }>(b"4__.", b"4__.");
    skip_iter_eq::<{ skip::L }>(b"_45_5", b"45_5");
    skip_iter_eq::<{ skip::L }>(b"__45__5", b"_45__5");
    skip_iter_eq::<{ skip::L }>(b"_.45_5", b".45_5");
    skip_iter_eq::<{ skip::L }>(b"__.45__5", b"_.45__5");
    skip_iter_eq::<{ skip::L }>(b"4_5_", b"4_5_");
    skip_iter_eq::<{ skip::L }>(b"4__5__", b"4__5__");
    skip_iter_eq::<{ skip::L }>(b"4_5_.5", b"4_5_.5");
    skip_iter_eq::<{ skip::L }>(b"4__5__.5", b"4__5__.5");
    skip_iter_eq::<{ skip::L }>(b"_45_", b"45_");
    skip_iter_eq::<{ skip::L }>(b"__45__", b"_45__");
    skip_iter_eq::<{ skip::L }>(b"_45_.56", b"45_.56");
    skip_iter_eq::<{ skip::L }>(b"__45__.56", b"_45__.56");
    skip_iter_eq::<{ skip::L }>(b"_4_5_", b"4_5_");
    skip_iter_eq::<{ skip::L }>(b"__4__5__", b"_4__5__");
    skip_iter_eq::<{ skip::L }>(b"_4_5_.56", b"4_5_.56");
    skip_iter_eq::<{ skip::L }>(b"__4__5__.56", b"_4__5__.56");
}

#[test]
fn test_skip_iter_t() {
    // Test iterators that skip single, trailing-only digit separators.
    skip_iter_eq::<{ skip::T }>(b"123.45", b"123.45");
    skip_iter_eq::<{ skip::T }>(b"1e45", b"1e45");
    skip_iter_eq::<{ skip::T }>(b"1e", b"1e");
    skip_iter_eq::<{ skip::T }>(b"1", b"1");
    skip_iter_eq::<{ skip::T }>(b"_45", b"_45");
    skip_iter_eq::<{ skip::T }>(b"__45", b"__45");
    skip_iter_eq::<{ skip::T }>(b"_.45", b".45");
    skip_iter_eq::<{ skip::T }>(b"__.45", b"_.45");
    skip_iter_eq::<{ skip::T }>(b"4_5", b"4_5");
    skip_iter_eq::<{ skip::T }>(b"4__5", b"4__5");
    skip_iter_eq::<{ skip::T }>(b"4_", b"4");
    skip_iter_eq::<{ skip::T }>(b"4__", b"4_");
    skip_iter_eq::<{ skip::T }>(b"4_.", b"4.");
    skip_iter_eq::<{ skip::T }>(b"4__.", b"4_.");
    skip_iter_eq::<{ skip::T }>(b"_45_5", b"_45_5");
    skip_iter_eq::<{ skip::T }>(b"__45__5", b"__45__5");
    skip_iter_eq::<{ skip::T }>(b"_.45_5", b".45_5");
    skip_iter_eq::<{ skip::T }>(b"__.45__5", b"_.45__5");
    skip_iter_eq::<{ skip::T }>(b"4_5_", b"4_5");
    skip_iter_eq::<{ skip::T }>(b"4__5__", b"4__5_");
    skip_iter_eq::<{ skip::T }>(b"4_5_.5", b"4_5.5");
    skip_iter_eq::<{ skip::T }>(b"4__5__.5", b"4__5_.5");
    skip_iter_eq::<{ skip::T }>(b"_45_", b"_45");
    skip_iter_eq::<{ skip::T }>(b"__45__", b"__45_");
    skip_iter_eq::<{ skip::T }>(b"_45_.56", b"_45.56");
    skip_iter_eq::<{ skip::T }>(b"__45__.56", b"__45_.56");
    skip_iter_eq::<{ skip::T }>(b"_4_5_", b"_4_5");
    skip_iter_eq::<{ skip::T }>(b"__4__5__", b"__4__5_");
    skip_iter_eq::<{ skip::T }>(b"_4_5_.56", b"_4_5.56");
    skip_iter_eq::<{ skip::T }>(b"__4__5__.56", b"__4__5_.56");
}

#[test]
fn test_skip_iter_il() {
    // Test iterators that skip single, internal or leading-only digit separators.
    skip_iter_eq::<{ skip::IL }>(b"123.45", b"123.45");
    skip_iter_eq::<{ skip::IL }>(b"1e45", b"1e45");
    skip_iter_eq::<{ skip::IL }>(b"1e", b"1e");
    skip_iter_eq::<{ skip::IL }>(b"1", b"1");
    skip_iter_eq::<{ skip::IL }>(b"_45", b"45");
    skip_iter_eq::<{ skip::IL }>(b"__45", b"_45");
    skip_iter_eq::<{ skip::IL }>(b"_.45", b".45");
    skip_iter_eq::<{ skip::IL }>(b"__.45", b"_.45");
    skip_iter_eq::<{ skip::IL }>(b"4_5", b"45");
    skip_iter_eq::<{ skip::IL }>(b"4__5", b"4_5");
    skip_iter_eq::<{ skip::IL }>(b"4_", b"4_");
    skip_iter_eq::<{ skip::IL }>(b"4__", b"4__");
    skip_iter_eq::<{ skip::IL }>(b"4_.", b"4_.");
    skip_iter_eq::<{ skip::IL }>(b"4__.", b"4__.");
    skip_iter_eq::<{ skip::IL }>(b"_45_5", b"455");
    skip_iter_eq::<{ skip::IL }>(b"__45__5", b"_45_5");
    skip_iter_eq::<{ skip::IL }>(b"_.45_5", b".455");
    skip_iter_eq::<{ skip::IL }>(b"__.45__5", b"_.45_5");
    skip_iter_eq::<{ skip::IL }>(b"4_5_", b"45_");
    skip_iter_eq::<{ skip::IL }>(b"4__5__", b"4_5__");
    skip_iter_eq::<{ skip::IL }>(b"4_5_.5", b"45_.5");
    skip_iter_eq::<{ skip::IL }>(b"4__5__.5", b"4_5__.5");
    skip_iter_eq::<{ skip::IL }>(b"_45_", b"45_");
    skip_iter_eq::<{ skip::IL }>(b"__45__", b"_45__");
    skip_iter_eq::<{ skip::IL }>(b"_45_.56", b"45_.56");
    skip_iter_eq::<{ skip::IL }>(b"__45__.56", b"_45__.56");
    skip_iter_eq::<{ skip::IL }>(b"_4_5_", b"45_");
    skip_iter_eq::<{ skip::IL }>(b"__4__5__", b"_4_5__");
    skip_iter_eq::<{ skip::IL }>(b"_4_5_.56", b"45_.56");
    skip_iter_eq::<{ skip::IL }>(b"__4__5__.56", b"_4_5__.56");
}

#[test]
fn test_skip_iter_it() {
    // Test iterators that skip single, internal or trailing-only digit separators.
    skip_iter_eq::<{ skip::IT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ skip::IT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ skip::IT }>(b"1e", b"1e");
    skip_iter_eq::<{ skip::IT }>(b"1", b"1");
    skip_iter_eq::<{ skip::IT }>(b"_45", b"_45");
    skip_iter_eq::<{ skip::IT }>(b"__45", b"__45");
    skip_iter_eq::<{ skip::IT }>(b"_.45", b".45");
    skip_iter_eq::<{ skip::IT }>(b"__.45", b"_.45");
    skip_iter_eq::<{ skip::IT }>(b"4_5", b"45");
    skip_iter_eq::<{ skip::IT }>(b"4__5", b"4_5");
    skip_iter_eq::<{ skip::IT }>(b"4_", b"4");
    skip_iter_eq::<{ skip::IT }>(b"4__", b"4_");
    skip_iter_eq::<{ skip::IT }>(b"4_.", b"4.");
    skip_iter_eq::<{ skip::IT }>(b"4__.", b"4_.");
    skip_iter_eq::<{ skip::IT }>(b"_45_5", b"_455");
    skip_iter_eq::<{ skip::IT }>(b"__45__5", b"__45_5");
    skip_iter_eq::<{ skip::IT }>(b"_.45_5", b".455");
    skip_iter_eq::<{ skip::IT }>(b"__.45__5", b"_.45_5");
    skip_iter_eq::<{ skip::IT }>(b"4_5_", b"45");
    skip_iter_eq::<{ skip::IT }>(b"4__5__", b"4_5_");
    skip_iter_eq::<{ skip::IT }>(b"4_5_.5", b"45.5");
    skip_iter_eq::<{ skip::IT }>(b"4__5__.5", b"4_5_.5");
    skip_iter_eq::<{ skip::IT }>(b"_45_", b"_45");
    skip_iter_eq::<{ skip::IT }>(b"__45__", b"__45_");
    skip_iter_eq::<{ skip::IT }>(b"_45_.56", b"_45.56");
    skip_iter_eq::<{ skip::IT }>(b"__45__.56", b"__45_.56");
    skip_iter_eq::<{ skip::IT }>(b"_4_5_", b"_45");
    skip_iter_eq::<{ skip::IT }>(b"__4__5__", b"__4_5_");
    skip_iter_eq::<{ skip::IT }>(b"_4_5_.56", b"_45.56");
    skip_iter_eq::<{ skip::IT }>(b"__4__5__.56", b"__4_5_.56");
}

#[test]
fn test_skip_iter_lt() {
    // Test iterators that skip single, leading or trailing-only digit separators.
    skip_iter_eq::<{ skip::LT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ skip::LT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ skip::LT }>(b"1e", b"1e");
    skip_iter_eq::<{ skip::LT }>(b"1", b"1");
    skip_iter_eq::<{ skip::LT }>(b"_45", b"45");
    skip_iter_eq::<{ skip::LT }>(b"__45", b"_45");
    skip_iter_eq::<{ skip::LT }>(b"_.45", b".45");
    skip_iter_eq::<{ skip::LT }>(b"__.45", b"_.45");
    skip_iter_eq::<{ skip::LT }>(b"4_5", b"4_5");
    skip_iter_eq::<{ skip::LT }>(b"4__5", b"4__5");
    skip_iter_eq::<{ skip::LT }>(b"4_", b"4");
    skip_iter_eq::<{ skip::LT }>(b"4__", b"4_");
    skip_iter_eq::<{ skip::LT }>(b"4_.", b"4.");
    skip_iter_eq::<{ skip::LT }>(b"4__.", b"4_.");
    skip_iter_eq::<{ skip::LT }>(b"_45_5", b"45_5");
    skip_iter_eq::<{ skip::LT }>(b"__45__5", b"_45__5");
    skip_iter_eq::<{ skip::LT }>(b"_.45_5", b".45_5");
    skip_iter_eq::<{ skip::LT }>(b"__.45__5", b"_.45__5");
    skip_iter_eq::<{ skip::LT }>(b"4_5_", b"4_5");
    skip_iter_eq::<{ skip::LT }>(b"4__5__", b"4__5_");
    skip_iter_eq::<{ skip::LT }>(b"4_5_.5", b"4_5.5");
    skip_iter_eq::<{ skip::LT }>(b"4__5__.5", b"4__5_.5");
    skip_iter_eq::<{ skip::LT }>(b"_45_", b"45");
    skip_iter_eq::<{ skip::LT }>(b"__45__", b"_45_");
    skip_iter_eq::<{ skip::LT }>(b"_45_.56", b"45.56");
    skip_iter_eq::<{ skip::LT }>(b"__45__.56", b"_45_.56");
    skip_iter_eq::<{ skip::LT }>(b"_4_5_", b"4_5");
    skip_iter_eq::<{ skip::LT }>(b"__4__5__", b"_4__5_");
    skip_iter_eq::<{ skip::LT }>(b"_4_5_.56", b"4_5.56");
    skip_iter_eq::<{ skip::LT }>(b"__4__5__.56", b"_4__5_.56");
}

#[test]
fn test_skip_iter_ilt() {
    // Test iterators that skip single digit separators.
    skip_iter_eq::<{ skip::ILT }>(b"123.45", b"123.45");
    skip_iter_eq::<{ skip::ILT }>(b"1e45", b"1e45");
    skip_iter_eq::<{ skip::ILT }>(b"1e", b"1e");
    skip_iter_eq::<{ skip::ILT }>(b"1", b"1");
    skip_iter_eq::<{ skip::ILT }>(b"_45", b"45");
    skip_iter_eq::<{ skip::ILT }>(b"__45", b"_45");
    skip_iter_eq::<{ skip::ILT }>(b"_.45", b".45");
    skip_iter_eq::<{ skip::ILT }>(b"__.45", b"_.45");
    skip_iter_eq::<{ skip::ILT }>(b"4_5", b"45");
    skip_iter_eq::<{ skip::ILT }>(b"4__5", b"4_5");
    skip_iter_eq::<{ skip::ILT }>(b"4_", b"4");
    skip_iter_eq::<{ skip::ILT }>(b"4__", b"4_");
    skip_iter_eq::<{ skip::ILT }>(b"4_.", b"4.");
    skip_iter_eq::<{ skip::ILT }>(b"4__.", b"4_.");
    skip_iter_eq::<{ skip::ILT }>(b"_45_5", b"455");
    skip_iter_eq::<{ skip::ILT }>(b"__45__5", b"_45_5");
    skip_iter_eq::<{ skip::ILT }>(b"_.45_5", b".455");
    skip_iter_eq::<{ skip::ILT }>(b"__.45__5", b"_.45_5");
    skip_iter_eq::<{ skip::ILT }>(b"4_5_", b"45");
    skip_iter_eq::<{ skip::ILT }>(b"4__5__", b"4_5_");
    skip_iter_eq::<{ skip::ILT }>(b"4_5_.5", b"45.5");
    skip_iter_eq::<{ skip::ILT }>(b"4__5__.5", b"4_5_.5");
    skip_iter_eq::<{ skip::ILT }>(b"_45_", b"45");
    skip_iter_eq::<{ skip::ILT }>(b"__45__", b"_45_");
    skip_iter_eq::<{ skip::ILT }>(b"_45_.56", b"45.56");
    skip_iter_eq::<{ skip::ILT }>(b"__45__.56", b"_45_.56");
    skip_iter_eq::<{ skip::ILT }>(b"_4_5_", b"45");
    skip_iter_eq::<{ skip::ILT }>(b"__4__5__", b"_4_5_");
    skip_iter_eq::<{ skip::ILT }>(b"_4_5_.56", b"45.56");
    skip_iter_eq::<{ skip::ILT }>(b"__4__5__.56", b"_4_5_.56");
}

#[test]
fn test_skip_iter_ic() {
    // Test iterators that skip multiple, internal digit separators.
    skip_iter_eq::<{ skip::IC }>(b"123.45", b"123.45");
    skip_iter_eq::<{ skip::IC }>(b"1e45", b"1e45");
    skip_iter_eq::<{ skip::IC }>(b"1e", b"1e");
    skip_iter_eq::<{ skip::IC }>(b"1", b"1");
    skip_iter_eq::<{ skip::IC }>(b"_45", b"_45");
    skip_iter_eq::<{ skip::IC }>(b"__45", b"__45");
    skip_iter_eq::<{ skip::IC }>(b"_.45", b"_.45");
    skip_iter_eq::<{ skip::IC }>(b"__.45", b"__.45");
    skip_iter_eq::<{ skip::IC }>(b"4_5", b"45");
    skip_iter_eq::<{ skip::IC }>(b"4__5", b"45");
    skip_iter_eq::<{ skip::IC }>(b"4_", b"4_");
    skip_iter_eq::<{ skip::IC }>(b"4__", b"4__");
    skip_iter_eq::<{ skip::IC }>(b"4_.", b"4_.");
    skip_iter_eq::<{ skip::IC }>(b"4__.", b"4__.");
    skip_iter_eq::<{ skip::IC }>(b"_45_5", b"_455");
    skip_iter_eq::<{ skip::IC }>(b"__45__5", b"__455");
    skip_iter_eq::<{ skip::IC }>(b"_.45_5", b"_.455");
    skip_iter_eq::<{ skip::IC }>(b"__.45__5", b"__.455");
    skip_iter_eq::<{ skip::IC }>(b"4_5_", b"45_");
    skip_iter_eq::<{ skip::IC }>(b"4__5__", b"45__");
    skip_iter_eq::<{ skip::IC }>(b"4_5_.5", b"45_.5");
    skip_iter_eq::<{ skip::IC }>(b"4__5__.5", b"45__.5");
    skip_iter_eq::<{ skip::IC }>(b"_45_", b"_45_");
    skip_iter_eq::<{ skip::IC }>(b"__45__", b"__45__");
    skip_iter_eq::<{ skip::IC }>(b"_45_.56", b"_45_.56");
    skip_iter_eq::<{ skip::IC }>(b"__45__.56", b"__45__.56");
    skip_iter_eq::<{ skip::IC }>(b"_4_5_", b"_45_");
    skip_iter_eq::<{ skip::IC }>(b"__4__5__", b"__45__");
    skip_iter_eq::<{ skip::IC }>(b"_4_5_.56", b"_45_.56");
    skip_iter_eq::<{ skip::IC }>(b"__4__5__.56", b"__45__.56");
}

#[test]
fn test_skip_iter_lc() {
    // Test iterators that skip multiple, leading digit separators.
    skip_iter_eq::<{ skip::LC }>(b"123.45", b"123.45");
    skip_iter_eq::<{ skip::LC }>(b"1e45", b"1e45");
    skip_iter_eq::<{ skip::LC }>(b"1e", b"1e");
    skip_iter_eq::<{ skip::LC }>(b"1", b"1");
    skip_iter_eq::<{ skip::LC }>(b"_45", b"45");
    skip_iter_eq::<{ skip::LC }>(b"__45", b"45");
    skip_iter_eq::<{ skip::LC }>(b"_.45", b".45");
    skip_iter_eq::<{ skip::LC }>(b"__.45", b".45");
    skip_iter_eq::<{ skip::LC }>(b"4_5", b"4_5");
    skip_iter_eq::<{ skip::LC }>(b"4__5", b"4__5");
    skip_iter_eq::<{ skip::LC }>(b"4_", b"4_");
    skip_iter_eq::<{ skip::LC }>(b"4__", b"4__");
    skip_iter_eq::<{ skip::LC }>(b"4_.", b"4_.");
    skip_iter_eq::<{ skip::LC }>(b"4__.", b"4__.");
    skip_iter_eq::<{ skip::LC }>(b"_45_5", b"45_5");
    skip_iter_eq::<{ skip::LC }>(b"__45__5", b"45__5");
    skip_iter_eq::<{ skip::LC }>(b"_.45_5", b".45_5");
    skip_iter_eq::<{ skip::LC }>(b"__.45__5", b".45__5");
    skip_iter_eq::<{ skip::LC }>(b"4_5_", b"4_5_");
    skip_iter_eq::<{ skip::LC }>(b"4__5__", b"4__5__");
    skip_iter_eq::<{ skip::LC }>(b"4_5_.5", b"4_5_.5");
    skip_iter_eq::<{ skip::LC }>(b"4__5__.5", b"4__5__.5");
    skip_iter_eq::<{ skip::LC }>(b"_45_", b"45_");
    skip_iter_eq::<{ skip::LC }>(b"__45__", b"45__");
    skip_iter_eq::<{ skip::LC }>(b"_45_.56", b"45_.56");
    skip_iter_eq::<{ skip::LC }>(b"__45__.56", b"45__.56");
    skip_iter_eq::<{ skip::LC }>(b"_4_5_", b"4_5_");
    skip_iter_eq::<{ skip::LC }>(b"__4__5__", b"4__5__");
    skip_iter_eq::<{ skip::LC }>(b"_4_5_.56", b"4_5_.56");
    skip_iter_eq::<{ skip::LC }>(b"__4__5__.56", b"4__5__.56");
}

#[test]
fn test_skip_iter_tc() {
    // Test iterators that skip multiple, trailing digit separators.
    skip_iter_eq::<{ skip::TC }>(b"123.45", b"123.45");
    skip_iter_eq::<{ skip::TC }>(b"1e45", b"1e45");
    skip_iter_eq::<{ skip::TC }>(b"1e", b"1e");
    skip_iter_eq::<{ skip::TC }>(b"1", b"1");
    skip_iter_eq::<{ skip::TC }>(b"_45", b"_45");
    skip_iter_eq::<{ skip::TC }>(b"__45", b"__45");
    skip_iter_eq::<{ skip::TC }>(b"_.45", b".45");
    skip_iter_eq::<{ skip::TC }>(b"__.45", b".45");
    skip_iter_eq::<{ skip::TC }>(b"4_5", b"4_5");
    skip_iter_eq::<{ skip::TC }>(b"4__5", b"4__5");
    skip_iter_eq::<{ skip::TC }>(b"4_", b"4");
    skip_iter_eq::<{ skip::TC }>(b"4__", b"4");
    skip_iter_eq::<{ skip::TC }>(b"4_.", b"4.");
    skip_iter_eq::<{ skip::TC }>(b"4__.", b"4.");
    skip_iter_eq::<{ skip::TC }>(b"_45_5", b"_45_5");
    skip_iter_eq::<{ skip::TC }>(b"__45__5", b"__45__5");
    skip_iter_eq::<{ skip::TC }>(b"_.45_5", b".45_5");
    skip_iter_eq::<{ skip::TC }>(b"__.45__5", b".45__5");
    skip_iter_eq::<{ skip::TC }>(b"4_5_", b"4_5");
    skip_iter_eq::<{ skip::TC }>(b"4__5__", b"4__5");
    skip_iter_eq::<{ skip::TC }>(b"4_5_.5", b"4_5.5");
    skip_iter_eq::<{ skip::TC }>(b"4__5__.5", b"4__5.5");
    skip_iter_eq::<{ skip::TC }>(b"_45_", b"_45");
    skip_iter_eq::<{ skip::TC }>(b"__45__", b"__45");
    skip_iter_eq::<{ skip::TC }>(b"_45_.56", b"_45.56");
    skip_iter_eq::<{ skip::TC }>(b"__45__.56", b"__45.56");
    skip_iter_eq::<{ skip::TC }>(b"_4_5_", b"_4_5");
    skip_iter_eq::<{ skip::TC }>(b"__4__5__", b"__4__5");
    skip_iter_eq::<{ skip::TC }>(b"_4_5_.56", b"_4_5.56");
    skip_iter_eq::<{ skip::TC }>(b"__4__5__.56", b"__4__5.56");
}

#[test]
fn test_skip_iter_ilc() {
    // Test iterators that skip multiple, internal or leading digit separators.
    skip_iter_eq::<{ skip::ILC }>(b"123.45", b"123.45");
    skip_iter_eq::<{ skip::ILC }>(b"1e45", b"1e45");
    skip_iter_eq::<{ skip::ILC }>(b"1e", b"1e");
    skip_iter_eq::<{ skip::ILC }>(b"1", b"1");
    skip_iter_eq::<{ skip::ILC }>(b"_45", b"45");
    skip_iter_eq::<{ skip::ILC }>(b"__45", b"45");
    skip_iter_eq::<{ skip::ILC }>(b"_.45", b".45");
    skip_iter_eq::<{ skip::ILC }>(b"__.45", b".45");
    skip_iter_eq::<{ skip::ILC }>(b"4_5", b"45");
    skip_iter_eq::<{ skip::ILC }>(b"4__5", b"45");
    skip_iter_eq::<{ skip::ILC }>(b"4_", b"4_");
    skip_iter_eq::<{ skip::ILC }>(b"4__", b"4__");
    skip_iter_eq::<{ skip::ILC }>(b"4_.", b"4_.");
    skip_iter_eq::<{ skip::ILC }>(b"4__.", b"4__.");
    skip_iter_eq::<{ skip::ILC }>(b"_45_5", b"455");
    skip_iter_eq::<{ skip::ILC }>(b"__45__5", b"455");
    skip_iter_eq::<{ skip::ILC }>(b"_.45_5", b".455");
    skip_iter_eq::<{ skip::ILC }>(b"__.45__5", b".455");
    skip_iter_eq::<{ skip::ILC }>(b"4_5_", b"45_");
    skip_iter_eq::<{ skip::ILC }>(b"4__5__", b"45__");
    skip_iter_eq::<{ skip::ILC }>(b"4_5_.5", b"45_.5");
    skip_iter_eq::<{ skip::ILC }>(b"4__5__.5", b"45__.5");
    skip_iter_eq::<{ skip::ILC }>(b"_45_", b"45_");
    skip_iter_eq::<{ skip::ILC }>(b"__45__", b"45__");
    skip_iter_eq::<{ skip::ILC }>(b"_45_.56", b"45_.56");
    skip_iter_eq::<{ skip::ILC }>(b"__45__.56", b"45__.56");
    skip_iter_eq::<{ skip::ILC }>(b"_4_5_", b"45_");
    skip_iter_eq::<{ skip::ILC }>(b"__4__5__", b"45__");
    skip_iter_eq::<{ skip::ILC }>(b"_4_5_.56", b"45_.56");
    skip_iter_eq::<{ skip::ILC }>(b"__4__5__.56", b"45__.56");
}

#[test]
fn test_skip_iter_itc() {
    // Test iterators that skip multiple, internal or trailing digit separators.
    skip_iter_eq::<{ skip::ITC }>(b"123.45", b"123.45");
    skip_iter_eq::<{ skip::ITC }>(b"1e45", b"1e45");
    skip_iter_eq::<{ skip::ITC }>(b"1e", b"1e");
    skip_iter_eq::<{ skip::ITC }>(b"1", b"1");
    skip_iter_eq::<{ skip::ITC }>(b"_45", b"_45");
    skip_iter_eq::<{ skip::ITC }>(b"__45", b"__45");
    skip_iter_eq::<{ skip::ITC }>(b"_.45", b".45");
    skip_iter_eq::<{ skip::ITC }>(b"__.45", b".45");
    skip_iter_eq::<{ skip::ITC }>(b"4_5", b"45");
    skip_iter_eq::<{ skip::ITC }>(b"4__5", b"45");
    skip_iter_eq::<{ skip::ITC }>(b"4_", b"4");
    skip_iter_eq::<{ skip::ITC }>(b"4__", b"4");
    skip_iter_eq::<{ skip::ITC }>(b"4_.", b"4.");
    skip_iter_eq::<{ skip::ITC }>(b"4__.", b"4.");
    skip_iter_eq::<{ skip::ITC }>(b"_45_5", b"_455");
    skip_iter_eq::<{ skip::ITC }>(b"__45__5", b"__455");
    skip_iter_eq::<{ skip::ITC }>(b"_.45_5", b".455");
    skip_iter_eq::<{ skip::ITC }>(b"__.45__5", b".455");
    skip_iter_eq::<{ skip::ITC }>(b"4_5_", b"45");
    skip_iter_eq::<{ skip::ITC }>(b"4__5__", b"45");
    skip_iter_eq::<{ skip::ITC }>(b"4_5_.5", b"45.5");
    skip_iter_eq::<{ skip::ITC }>(b"4__5__.5", b"45.5");
    skip_iter_eq::<{ skip::ITC }>(b"_45_", b"_45");
    skip_iter_eq::<{ skip::ITC }>(b"__45__", b"__45");
    skip_iter_eq::<{ skip::ITC }>(b"_45_.56", b"_45.56");
    skip_iter_eq::<{ skip::ITC }>(b"__45__.56", b"__45.56");
    skip_iter_eq::<{ skip::ITC }>(b"_4_5_", b"_45");
    skip_iter_eq::<{ skip::ITC }>(b"__4__5__", b"__45");
    skip_iter_eq::<{ skip::ITC }>(b"_4_5_.56", b"_45.56");
    skip_iter_eq::<{ skip::ITC }>(b"__4__5__.56", b"__45.56");
}

#[test]
fn test_skip_iter_ltc() {
    // Test iterators that skip multiple, leading or trailing digit separators.
    skip_iter_eq::<{ skip::LTC }>(b"123.45", b"123.45");
    skip_iter_eq::<{ skip::LTC }>(b"1e45", b"1e45");
    skip_iter_eq::<{ skip::LTC }>(b"1e", b"1e");
    skip_iter_eq::<{ skip::LTC }>(b"1", b"1");
    skip_iter_eq::<{ skip::LTC }>(b"_45", b"45");
    skip_iter_eq::<{ skip::LTC }>(b"__45", b"45");
    skip_iter_eq::<{ skip::LTC }>(b"_.45", b".45");
    skip_iter_eq::<{ skip::LTC }>(b"__.45", b".45");
    skip_iter_eq::<{ skip::LTC }>(b"4_5", b"4_5");
    skip_iter_eq::<{ skip::LTC }>(b"4__5", b"4__5");
    skip_iter_eq::<{ skip::LTC }>(b"4_", b"4");
    skip_iter_eq::<{ skip::LTC }>(b"4__", b"4");
    skip_iter_eq::<{ skip::LTC }>(b"4_.", b"4.");
    skip_iter_eq::<{ skip::LTC }>(b"4__.", b"4.");
    skip_iter_eq::<{ skip::LTC }>(b"_45_5", b"45_5");
    skip_iter_eq::<{ skip::LTC }>(b"__45__5", b"45__5");
    skip_iter_eq::<{ skip::LTC }>(b"_.45_5", b".45_5");
    skip_iter_eq::<{ skip::LTC }>(b"__.45__5", b".45__5");
    skip_iter_eq::<{ skip::LTC }>(b"4_5_", b"4_5");
    skip_iter_eq::<{ skip::LTC }>(b"4__5__", b"4__5");
    skip_iter_eq::<{ skip::LTC }>(b"4_5_.5", b"4_5.5");
    skip_iter_eq::<{ skip::LTC }>(b"4__5__.5", b"4__5.5");
    skip_iter_eq::<{ skip::LTC }>(b"_45_", b"45");
    skip_iter_eq::<{ skip::LTC }>(b"__45__", b"45");
    skip_iter_eq::<{ skip::LTC }>(b"_45_.56", b"45.56");
    skip_iter_eq::<{ skip::LTC }>(b"__45__.56", b"45.56");
    skip_iter_eq::<{ skip::LTC }>(b"_4_5_", b"4_5");
    skip_iter_eq::<{ skip::LTC }>(b"__4__5__", b"4__5");
    skip_iter_eq::<{ skip::LTC }>(b"_4_5_.56", b"4_5.56");
    skip_iter_eq::<{ skip::LTC }>(b"__4__5__.56", b"4__5.56");
}

#[test]
fn test_skip_iter_iltc() {
    // Test iterators that skip multiple digit separators.
    skip_iter_eq::<{ skip::ILTC }>(b"123.45", b"123.45");
    skip_iter_eq::<{ skip::ILTC }>(b"1e45", b"1e45");
    skip_iter_eq::<{ skip::ILTC }>(b"1e", b"1e");
    skip_iter_eq::<{ skip::ILTC }>(b"1", b"1");
    skip_iter_eq::<{ skip::ILTC }>(b"_45", b"45");
    skip_iter_eq::<{ skip::ILTC }>(b"__45", b"45");
    skip_iter_eq::<{ skip::ILTC }>(b"_.45", b".45");
    skip_iter_eq::<{ skip::ILTC }>(b"__.45", b".45");
    skip_iter_eq::<{ skip::ILTC }>(b"4_5", b"45");
    skip_iter_eq::<{ skip::ILTC }>(b"4__5", b"45");
    skip_iter_eq::<{ skip::ILTC }>(b"4_", b"4");
    skip_iter_eq::<{ skip::ILTC }>(b"4__", b"4");
    skip_iter_eq::<{ skip::ILTC }>(b"4_.", b"4.");
    skip_iter_eq::<{ skip::ILTC }>(b"4__.", b"4.");
    skip_iter_eq::<{ skip::ILTC }>(b"_45_5", b"455");
    skip_iter_eq::<{ skip::ILTC }>(b"__45__5", b"455");
    skip_iter_eq::<{ skip::ILTC }>(b"_.45_5", b".455");
    skip_iter_eq::<{ skip::ILTC }>(b"__.45__5", b".455");
    skip_iter_eq::<{ skip::ILTC }>(b"4_5_", b"45");
    skip_iter_eq::<{ skip::ILTC }>(b"4__5__", b"45");
    skip_iter_eq::<{ skip::ILTC }>(b"4_5_.5", b"45.5");
    skip_iter_eq::<{ skip::ILTC }>(b"4__5__.5", b"45.5");
    skip_iter_eq::<{ skip::ILTC }>(b"_45_", b"45");
    skip_iter_eq::<{ skip::ILTC }>(b"__45__", b"45");
    skip_iter_eq::<{ skip::ILTC }>(b"_45_.56", b"45.56");
    skip_iter_eq::<{ skip::ILTC }>(b"__45__.56", b"45.56");
    skip_iter_eq::<{ skip::ILTC }>(b"_4_5_", b"45");
    skip_iter_eq::<{ skip::ILTC }>(b"__4__5__", b"45");
    skip_iter_eq::<{ skip::ILTC }>(b"_4_5_.56", b"45.56");
    skip_iter_eq::<{ skip::ILTC }>(b"__4__5__.56", b"45.56");
}
