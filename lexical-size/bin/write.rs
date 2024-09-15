#[allow(unused_macros)]
macro_rules! integer_module {
    ($t:ty) => {
        use std::io::BufRead;
        #[cfg(not(feature = "lexical"))]
        use std::io::Write;

        #[cfg(feature = "lexical")]
        use lexical_write_integer::ToLexical;

        pub fn main() {
            let value: $t = unsafe {
                core::ptr::read_unaligned::<$t>(
                    std::io::stdin()
                        .lock()
                        .lines()
                        .next()
                        .unwrap()
                        .unwrap()
                        .trim()
                        .as_bytes()
                        .as_ptr() as *const _,
                )
            };

            #[cfg(feature = "lexical")]
            {
                let mut buffer: [u8; 128] = [0u8; 128];
                println!("{}", value.to_lexical(&mut buffer).len());
            }

            #[cfg(not(feature = "lexical"))]
            {
                let mut buffer = Vec::with_capacity(128);
                buffer.write_fmt(format_args!("{}", value)).unwrap();
                println!("{}", buffer.len());
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! float_module {
    ($t:ty) => {
        use std::io::BufRead;
        #[cfg(not(feature = "lexical"))]
        use std::io::Write;

        #[cfg(feature = "lexical")]
        use lexical_write_float::ToLexical;

        pub fn main() {
            let value: $t = unsafe {
                core::ptr::read_unaligned::<$t>(
                    std::io::stdin()
                        .lock()
                        .lines()
                        .next()
                        .unwrap()
                        .unwrap()
                        .trim()
                        .as_bytes()
                        .as_ptr() as *const _,
                )
            };

            #[cfg(feature = "lexical")]
            {
                let mut buffer = [0u8; 128];
                println!("{}", value.to_lexical(&mut buffer).len());
            }

            #[cfg(not(feature = "lexical"))]
            {
                let mut buffer = Vec::with_capacity(128);
                buffer.write_fmt(format_args!("{}", value)).unwrap();
                println!("{}", buffer.len());
            }
        }
    };
}
