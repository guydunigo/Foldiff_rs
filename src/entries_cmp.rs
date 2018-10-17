use std::ffi::OsString;
use std::fs::DirEntry;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum SameEntries {
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

pub fn are_dir_entries_same(a: &DirEntry, b: &DirEntry) -> Result<SameEntries, io::Error> {
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

pub fn get_absent_entries(
    ref_content: &[DirEntry],
    cmp_content: &[DirEntry],
) -> Result<Vec<PathBuf>, io::Error> {
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
pub fn get_present_entries(
    a_content: &[DirEntry],
    b_content: &[DirEntry],
) -> Result<Vec<(SameEntries, OsString)>, io::Error> {
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
