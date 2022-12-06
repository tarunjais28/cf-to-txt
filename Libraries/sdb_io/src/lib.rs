//! `sdb_io` provides a collection of convenience wrappers around common I/O operations
//! used in the SuperDB project.

use std::fs::OpenOptions;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error};

/// Opens the file at the `path` you specify, in read mode.
///
/// Errors returned are from the standard library's [`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)
pub fn open_file_read(
    path: &str
) -> Result<File, Error> {
    return OpenOptions::new()
        .read(true)
        .open(path);
}

/// Returns a buffered writer to the `path` you specify.
///
/// Errors returned are from the standard library's [`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)
pub fn new_buf_rdr(
    path: &str
) -> Result<BufReader<File>, Error> {
    let file = open_file_read(path)?;
    return Ok(BufReader::new(file));
}

/// Returns a buffered writer to the `path` you specify.
///
/// You can also pass a `buf_size`, if you you'd like. This sets a buffer for the file writer.
/// If you choose to not pass a size, a suitable size will be picked by default.
///
/// Errors returned are from the standard library's [`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html)
pub fn buf_file_wrtr(
    path: &str,
    buf_size: Option<usize>
) -> Result<BufWriter<File>, Error> {
    let mut ownd_path = path.to_string();

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(
            ownd_path.clone()
        )?;

    if buf_size.is_some() {
        return Ok(
            BufWriter::with_capacity(
                buf_size.unwrap(),
                file
            )
        );
    } else {
        return Ok(
            BufWriter::new(
                file
            )
        );
    }
}