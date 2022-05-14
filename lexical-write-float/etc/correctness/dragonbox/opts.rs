// Shared command-line options.

use clap::Parser;

#[derive(Parser)]
pub struct Opts {
    #[clap(short, long, default_value = "10000000")]
    pub iterations: usize,
}
