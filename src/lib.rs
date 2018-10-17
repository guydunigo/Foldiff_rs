// use std::error::Error;
// use std::fs::File;
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::io;
use std::io::prelude::*;
use std::io::Error as IoError;
use std::path::{Path, PathBuf};
use std::ffi::OsString;

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

enum SameEntries {
    SameDirs,
    SameFiles,
    SameSymlinks,
    Different
}
impl SameEntries {
    pub fn is_different(&self) -> bool {
        if let SameEntries::Different = self {
            true
        } else {
            false
        }
    }
}

fn are_dir_entries_equal(a: &DirEntry, b: &DirEntry) -> Result<SameEntries, io::Error> {
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
        for cmp_e in cmp_content.iter() {
            if are_dir_entries_equal(ref_e, cmp_e)?.is_different() {
                res.push(ref_e.path());
                break;
            }
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
            let are_eq = are_dir_entries_equal(a_e, b_e)?;
            if !are_eq.is_different() {
                    res.push((are_eq, a_e.file_name()));
                    break;
            }
        }
    }

    Ok(res)
}

pub fn compare_folders<P, Q>(ref_folder: P, cmp_folder: Q) -> Result<(), IoError>
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

    let absents_cmp = get_absent_entries(&ref_content, &cmp_content)?;
    let absents_ref = get_absent_entries(&cmp_content, &ref_content)?;
    let present_entries = get_present_entries(&ref_content, &cmp_content)?;

    // TODO: Parallel ?
    for (file_type, file_name) in present_entries {
        let mut ref_path = ref_folder.as_ref().to_path_buf();
        let mut cmp_path = cmp_folder.as_ref().to_path_buf();

        ref_path.push(&file_name);
        cmp_path.push(&file_name);
        // TODO: recursivity
        let comparison = if let SameEntries::SameDirs = file_type {
            compare_folders(ref_path, cmp_path)?
        } else {
            // compare_files(ref_path, cmp_path)? 
        };
    }

    Ok(())
}

pub fn run(args: args::Args) -> Result<(), IoError> {
    // Open file
    println!("Reference folder : '{}'", args.ref_folder);
    println!("Folder to compare : '{}'", args.cmp_folder);

    let ref_folder = Path::new(args.ref_folder.as_str());
    let cmp_folder = Path::new(args.cmp_folder.as_str());

    let res = compare_folders(ref_folder, cmp_folder)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
