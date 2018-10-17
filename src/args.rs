use std::env;

pub struct Args {
    pub ref_folder: String,
    pub cmp_folder: String,
}

#[derive(Debug)]
pub enum Error {
    NoGivenRefFolder,
    NoGivenCompareFolder,
}

impl Args {
    pub fn new(mut args: env::Args) -> Result<Self, Error> {
        args.next();

        let ref_folder = match args.next() {
            Some(arg) => arg,
            None => return Err(Error::NoGivenRefFolder),
        };

        let cmp_folder = match args.next() {
            Some(arg) => arg,
            None => return Err(Error::NoGivenCompareFolder),
        };

        Ok(Args {
            ref_folder,
            cmp_folder,
        })
    }
}
