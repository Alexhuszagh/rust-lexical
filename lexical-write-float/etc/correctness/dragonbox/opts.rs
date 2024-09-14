// Shared command-line options.

use clap::Parser;

const fn iterations() -> &'static str {
    if cfg!(miri) {
        "50"
    } else {
        "10000000"
    }
}

#[derive(Parser)]
pub struct Opts {
    #[clap(short, long, default_value = iterations())]
    pub iterations: usize,
}
