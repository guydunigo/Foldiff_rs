use std::env;

pub struct Args {
    /// Folder A
    pub ref_folder: String,
    /// Folder B
    pub cmp_folder: String,
    /// If `true`, the content of the files is also compared.
    pub compare_content: bool,
}

#[derive(Debug)]
pub enum Error {
    NoGivenRefFolder,
    NoGivenCompareFolder,
}

impl Args {
    pub fn new(mut args: env::Args) -> Result<Self, Error> {
        args.next();

        // TODO: I don't like that...
        let mut compare_content = false;
        let ref_folder = loop {
            let first = args.next().ok_or(Error::NoGivenRefFolder)?;
            if first == "-c" {
                compare_content = true;
            } else {
                break first;
            }
        };

        let cmp_folder = match args.next() {
            Some(arg) => arg,
            None => return Err(Error::NoGivenCompareFolder),
        };

        Ok(Args {
            ref_folder,
            cmp_folder,
            compare_content,
        })
    }
}
