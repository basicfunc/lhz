use std::fs::{metadata, read_dir, File};
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};

use lhz::{CHUNK, CHUNKS, CHUNK_SIZE};

pub fn open_file(file_name: PathBuf) -> Result<CHUNKS, &'static str> {
    let mut chunks: CHUNKS = vec![];
    let mut file;

    match File::open(&file_name) {
        Ok(f) => file = f,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Err("Error: Unable to open {file_name:?}: not found.");
            } else if e.kind() == ErrorKind::PermissionDenied {
                return Err("Error: Unable to open {file_name:?}: permission denied.");
            } else {
                return Err("Error: Unable to open {file_name:?}: unknown error occured.");
            }
        }
    }

    loop {
        let mut buffer: CHUNK = [0; CHUNK_SIZE];
        let bytes_read;

        match file.read(&mut buffer) {
            Ok(bytes) => bytes_read = bytes,
            Err(e) => return match e.kind() {
                ErrorKind::NotFound =>
                    Err("Error: specified file does not exist."),
                ErrorKind::PermissionDenied =>
                    Err(
                        "Error: The current user does not have permission to access the specified file."
                    ),
                ErrorKind::Interrupted =>
                    Err(
                        "Error: The read operation was interrupted by another signal."
                    ),
                ErrorKind::UnexpectedEof =>
                    Err(
                        "Error: An unexpected end of file was encountered during the read operation."
                    ),
                _ =>
                    Err("Error: Unepected error occured while reading from specified file, there are many reasons of this error such as invalid data in the file, disk errors, or insufficient memory.")

            }
        }

        if bytes_read == 0 {
            break;
        }

        chunks.push(buffer);
    }

    Ok(chunks)
}

fn open_folder(dir_name: &String) -> Result<CHUNKS, &'static str> {
    let dir = Path::new(dir_name);

    for entry in read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            println!("Processing file: {:?}", path);
            _ = open_file(path)
        } else if path.is_dir() {
            _ = open_folder(&path.into_os_string().into_string().unwrap())
        } else {
            panic!("Unknown error occured!")
        }
    }

    unimplemented!("Operation for directories aren't implemented yet!!")
}

pub fn open(src: &String) -> Result<CHUNKS, &'static str> {
    let f = metadata(&src).unwrap();

    if f.is_file() {
        open_file(PathBuf::from(&src))
    } else if f.is_dir() {
        open_folder(src)
    } else {
        panic!("Unknown error occured!")
    }
}
