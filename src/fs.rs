use std::io::{Error, ErrorKind, Result};
use std::path::Path;

/// A trait to allow replacing the file system lookup mechanisms.
///
/// As it stands, this is imperfect: it’s still using the types and some operations from std::path,
/// which constrain it to the target platform’s norms. This could be ameliorated by the use of
/// associated types for Path and PathBuf, and putting all remaining methods on this trait
/// (is_absolute, parent, join, &c.); but that would infect too many other APIs to be desirable, so
/// we live with it as it is—which is also acceptable, because the motivating example use case is
/// mostly using this as an optimisation over the real platform underneath.
pub trait Fs: std::fmt::Debug {
    fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn read(&self, path: &Path) -> Result<Vec<u8>> {
        std::fs::read(path)
    }
}

/// Use [`std::fs`] to read any files from disk.
///
/// This is the default file system implementation.
#[derive(Debug)]
pub struct StdFs;

impl Fs for StdFs {}

/// A file system implementation that acts like it’s completely empty.
///
/// This may be useful for security as it denies all access to the file system (so `@import` is
/// prevented from leaking anything); you’ll need to use [`from_string`][crate::from_string] for
/// this to make any sense.
#[derive(Debug)]
pub struct NullFs;

impl Fs for NullFs {
    fn is_file(&self, _path: &Path) -> bool {
        false
    }

    fn is_dir(&self, _path: &Path) -> bool {
        false
    }

    fn read(&self, _path: &Path) -> Result<Vec<u8>> {
        Err(Error::new(
            ErrorKind::NotFound,
            "NullFs, there is no file system",
        ))
    }
}
