// use std::error::Error;
// use std::fs::File;
use std::fs;
use std::fs::DirEntry;
use std::io;
// use std::io::prelude::*;
use std::ffi::OsString;
use std::fmt;
use std::io::Error as IoError;
use std::path::{Path, PathBuf};

// TODO: use SYMBOL and size...
const SEPARATOR: &str = "---------------------------------";

pub mod args;

#[derive(Debug)]
pub enum Error {
    ArgsError(args::Error),
    IoError(IoError),
}

impl From<args::Error> for Error {
    fn from(e: args::Error) -> Self {
        Error::ArgsError(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::IoError(e)
    }
}

pub struct ComparisonReport {
    absent_in_ref: Vec<PathBuf>,
    absent_in_cmp: Vec<PathBuf>,
}

//TODO: Show dirs differently ('/' at the end of path, ...)
impl fmt::Display for ComparisonReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}\n", SEPARATOR);
        writeln!(f, "Files only in A")?;
        write!(f, "{}\n", SEPARATOR);
        for p in self.absent_in_cmp.iter() {
            writeln!(f, ">> {}", p.to_str().unwrap())?;
        }

        write!(f, "\n{}\n", SEPARATOR);
        writeln!(f, "Files only in B")?;
        write!(f, "{}\n", SEPARATOR);

        for p in self.absent_in_ref.iter() {
            writeln!(f, "<< {}", p.to_str().unwrap())?;
        }

        Ok(())
    }
}

impl ComparisonReport {
    pub fn append(&mut self, mut other: Self) {
        self.absent_in_ref.append(&mut other.absent_in_ref);
        self.absent_in_cmp.append(&mut other.absent_in_cmp);
    }
}

#[derive(Debug)]
enum SameEntries {
    SameDirs,
    SameFiles,
    SameSymlinks,
    Different,
}
impl SameEntries {
    pub fn is_different(&self) -> bool {
        if let SameEntries::Different = self {
            true
        } else {
            false
        }
    }
    pub fn is_same(&self) -> bool {
        !self.is_different()
    }
}

fn are_dir_entries_same(a: &DirEntry, b: &DirEntry) -> Result<SameEntries, io::Error> {
    let a_type = a.file_type()?;
    if a.file_name() == b.file_name() && a_type == b.file_type()? {
        if a_type.is_dir() {
            Ok(SameEntries::SameDirs)
        } else if a_type.is_file() {
            Ok(SameEntries::SameFiles)
        } else {
            Ok(SameEntries::SameSymlinks)
        }
    } else {
        Ok(SameEntries::Different)
    }
}

fn get_absent_entries(
    ref_content: &[DirEntry],
    cmp_content: &[DirEntry],
) -> Result<Vec<PathBuf>, IoError> {
    let mut res = Vec::new();

    for ref_e in ref_content.iter() {
        // TODO: Add to a list of errors ?
        let mut found = false;
        for cmp_e in cmp_content.iter() {
            if are_dir_entries_same(ref_e, cmp_e)?.is_same() {
                found = true;
                break;
            }
        }

        if !found {
            res.push(ref_e.path());
        }
    }

    Ok(res)
}

// TODO: Resolve or separate symlinks ?
fn get_present_entries(
    a_content: &[DirEntry],
    b_content: &[DirEntry],
) -> Result<Vec<(SameEntries, OsString)>, IoError> {
    let mut res = Vec::new();

    for a_e in a_content.iter() {
        // TODO: Add to a list of errors ?
        for b_e in b_content.iter() {
            let are_eq = are_dir_entries_same(a_e, b_e)?;
            if are_eq.is_same() {
                res.push((are_eq, a_e.file_name()));
                break;
            }
        }
    }

    Ok(res)
}

pub fn compare_folders<P, Q>(ref_folder: P, cmp_folder: Q) -> Result<ComparisonReport, IoError>
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
        absent_in_cmp: get_absent_entries(&ref_content, &cmp_content)?,
        absent_in_ref: get_absent_entries(&cmp_content, &ref_content)?,
    };

    let present_entries = get_present_entries(&ref_content, &cmp_content)?;

    // TODO: Parallel ?
    for (file_type, file_name) in present_entries {
        let mut ref_path = ref_folder.as_ref().to_path_buf();
        let mut cmp_path = cmp_folder.as_ref().to_path_buf();

        ref_path.push(&file_name);
        cmp_path.push(&file_name);

        if let SameEntries::SameDirs = file_type {
            comparison.append(compare_folders(ref_path, cmp_path)?);
        } else {
            //TODO: compare_files(ref_path, cmp_path)?
        };
    }

    Ok(comparison)
}

pub fn run(args: args::Args) -> Result<(), IoError> {
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
