extern crate foldiff;

use foldiff::args::Args;
use std::env;

// const COMPARE_CONTENT: bool = true;

fn main() -> Result<(), foldiff::Error> {
    let args = env::args();
    let args = Args::new(args)?;

    foldiff::run(&args)?;

    Ok(())
}
