#![forbid(unsafe_code)]

use std::io::{stdin, stdout};

use log::*;
use structopt::StructOpt;

use ripgzip::decompress;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opts {
    /// Decompress data
    #[structopt(short = "d", long = "decompress")]
    decompress: bool,
    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
}

fn main() {
    let opts = Opts::from_args();

    stderrlog::new()
        .verbosity(1 + opts.verbose)
        .timestamp(stderrlog::Timestamp::Off)
        .init()
        .expect("failed to initialize logging");

    if opts.decompress {
        if let Err(err) = decompress(stdin().lock(), stdout().lock()) {
            error!("{:#}", err);
            std::process::exit(1);
        }
    }
}
