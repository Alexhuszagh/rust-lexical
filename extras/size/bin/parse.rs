#[allow(unused_macros)]
macro_rules! integer_module {
    ($t:ty) => {
        #[cfg(not(feature = "lexical"))]
        mod core_parse;

        use std::io::BufRead;

        #[cfg(not(feature = "lexical"))]
        use core_parse::parse_int;
        #[cfg(feature = "lexical")]
        use lexical_parse_integer::FromLexical;

        pub fn main() {
            #[cfg(feature = "lexical")]
            println!(
                "{}",
                <$t>::from_lexical(
                    std::io::stdin().lock().lines().next().unwrap().unwrap().trim().as_bytes()
                )
                .unwrap() as usize
            );

            #[cfg(not(feature = "lexical"))]
            println!(
                "{}",
                parse_int::<$t>(std::io::stdin().lock().lines().next().unwrap().unwrap().trim(), 10)
                    .unwrap() as usize
            );
        }
    };
}

#[allow(unused_macros)]
macro_rules! float_module {
    ($t:ty) => {
        use std::io::BufRead;

        #[cfg(feature = "lexical")]
        use lexical_parse_float::FromLexical;

        pub fn main() {
            #[cfg(feature = "lexical")]
            println!(
                "{}",
                <$t>::from_lexical(
                    std::io::stdin().lock().lines().next().unwrap().unwrap().trim().as_bytes()
                )
                .unwrap() as usize
            );

            #[cfg(not(feature = "lexical"))]
            println!(
                "{}",
                std::io::stdin()
                    .lock()
                    .lines()
                    .next()
                    .unwrap()
                    .unwrap()
                    .trim()
                    .parse::<$t>()
                    .unwrap() as usize
            );
        }
    };
}
