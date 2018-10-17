// use std::error::Error;
// use std::fs::File;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

// TODO: use SYMBOL and size...
const SEPARATOR: &str = "---------------------------------";
const BUF_SIZE: usize = 4096;

pub mod args;
pub mod comparison_report;
mod entries_cmp;
// TODO: better way to do that : ?
type ComparisonReport = comparison_report::ComparisonReport;

#[derive(Debug)]
pub enum Error {
    ArgsError(args::Error),
    IoError(io::Error),
}

impl From<args::Error> for Error {
    fn from(e: args::Error) -> Self {
        Error::ArgsError(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

// TODO: trust that args are files?
pub fn are_files_equal<P, Q>(ref_file: P, cmp_file: Q) -> Result<bool, io::Error>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut ref_file = fs::File::open(ref_file)?;
    let mut cmp_file = fs::File::open(cmp_file)?;

    let mut ref_buf = [0; BUF_SIZE];
    let mut cmp_buf = [0; BUF_SIZE];

    // TODO: Wouldn't a hash be faster?
    Ok(loop {
        let ref_read = ref_file.read(&mut ref_buf)?;
        let cmp_read = cmp_file.read(&mut cmp_buf)?;

        if ref_read == 0 || cmp_read == 0 {
            break true;
        // TODO: is it for read not to read the maximum size possible?
        // (when there is still data to be read on the next call)
        } else if ref_read != cmp_read || ref_buf[..ref_read] != cmp_buf[..cmp_read] {
            break false;
        }
    })
}

// TODO: trust that args are folders?
pub fn compare_folders<P, Q>(ref_folder: P, cmp_folder: Q) -> Result<ComparisonReport, io::Error>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let ref_content_iter = fs::read_dir(&ref_folder)?;
    let cmp_content_iter = fs::read_dir(&cmp_folder)?;
    let mut ref_content = Vec::new();
    let mut cmp_content = Vec::new();

    for e in ref_content_iter {
        ref_content.push(e?);
    }
    for e in cmp_content_iter {
        cmp_content.push(e?);
    }

    let mut comparison = ComparisonReport {
        absent_in_cmp: entries_cmp::get_absent_entries(&ref_content, &cmp_content)?,
        absent_in_ref: entries_cmp::get_absent_entries(&cmp_content, &ref_content)?,
        different_files: Vec::new(),
    };

    let present_entries = entries_cmp::get_present_entries(&ref_content, &cmp_content)?;

    // TODO: Parallel ?
    for (file_type, file_name) in present_entries {
        let mut ref_path = ref_folder.as_ref().to_path_buf();
        let mut cmp_path = cmp_folder.as_ref().to_path_buf();

        ref_path.push(&file_name);
        cmp_path.push(&file_name);

        if let entries_cmp::SameEntries::SameDirs = file_type {
            comparison.append(compare_folders(ref_path, cmp_path)?);
        } else {
            if !are_files_equal(&ref_path, cmp_path)? {
                comparison.different_files.push(ref_path);
            }
        };
    }

    Ok(comparison)
}

pub fn run(args: args::Args) -> Result<(), io::Error> {
    // Open file
    print!("{}\n", SEPARATOR);
    println!("Folder A : '{}'", args.ref_folder);
    println!("Folder B : '{}'", args.cmp_folder);

    let ref_folder = Path::new(args.ref_folder.as_str());
    let cmp_folder = Path::new(args.cmp_folder.as_str());

    let res = compare_folders(ref_folder, cmp_folder)?;

    println!("{}", res);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
