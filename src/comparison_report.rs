use super::SEPARATOR;
use std::fmt;
use std::path::PathBuf;

pub struct ComparisonReport {
    pub absent_in_ref: Vec<PathBuf>,
    pub absent_in_cmp: Vec<PathBuf>,
    pub different_files: Vec<PathBuf>,
}

//TODO: Show dirs differently ('/' at the end of path, ...)
impl fmt::Display for ComparisonReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Subfunction
        writeln!(f, "\n{}", SEPARATOR);
        writeln!(f, "Elements only in A")?;
        writeln!(f, "{}", SEPARATOR);
        for p in self.absent_in_cmp.iter() {
            writeln!(f, ">> {}", p.to_str().unwrap())?;
        }

        writeln!(f, "\n{}", SEPARATOR);
        writeln!(f, "Elements only in B")?;
        writeln!(f, "{}", SEPARATOR);

        for p in self.absent_in_ref.iter() {
            writeln!(f, "<< {}", p.to_str().unwrap())?;
        }

        writeln!(f, "\n{}", SEPARATOR);
        writeln!(f, "Files that are different")?;
        writeln!(f, "{}", SEPARATOR);

        // TODO: truncate the part from reference ?
        for p in self.different_files.iter() {
            writeln!(f, "-- {}", p.to_str().unwrap())?;
        }

        Ok(())
    }
}

impl ComparisonReport {
    pub fn append(&mut self, mut other: Self) {
        self.absent_in_ref.append(&mut other.absent_in_ref);
        self.absent_in_cmp.append(&mut other.absent_in_cmp);
        self.different_files.append(&mut other.different_files);
    }
}
