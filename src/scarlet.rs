//! Scarlet: only what `std` offers, so no symlink-specific updates.
use crate::FileTime;
use std::fs;
use std::io;
use std::path::Path;

pub fn set_symlink_file_times(_p: &Path, _atime: FileTime, _mtime: FileTime) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "symlink times are not supported on Scarlet",
    ))
}

fn convert(time: io::Result<std::time::SystemTime>) -> Option<FileTime> {
    time.ok().map(FileTime::from_system_time)
}

pub fn from_last_modification_time(meta: &fs::Metadata) -> FileTime {
    convert(meta.modified()).unwrap_or_else(FileTime::zero)
}

pub fn from_last_access_time(meta: &fs::Metadata) -> FileTime {
    convert(meta.accessed()).unwrap_or_else(FileTime::zero)
}

pub fn from_creation_time(meta: &fs::Metadata) -> Option<FileTime> {
    convert(meta.created())
}

pub fn open(path: &Path) -> io::Result<fs::File> {
    fs::OpenOptions::new().write(true).open(path)
}
