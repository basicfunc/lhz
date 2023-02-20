use std::env;
use std::fs::{metadata, read_dir, File};
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};

const CHUNK_SIZE: usize = 64 * 1024; // Hence, 64KB per chunk.

fn open_file(file_name: PathBuf) -> Result<Vec<[u8; CHUNK_SIZE]>, String> {
    let mut chunks: Vec<[u8; CHUNK_SIZE]> = vec![];
    let mut file;

    match File::open(file_name) {
        Ok(f) => file = f,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Err(format!(
                    "Unable to open {:?}: not found.",
                    file_name.as_os_str()
                ));
            } else if e.kind() == ErrorKind::PermissionDenied {
                return Err(format!(
                    "Unable to open {:?}: permission denied.",
                    file_name.as_os_str()
                ));
            } else {
                return Err(format!(
                    "Unable to open {:?}: unknown error occured.",
                    file_name.as_os_str()
                ));
            }
        }
    }

    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let bytes_read;

        match file.read(&mut buffer) {
            Ok(bytes) => bytes_read = bytes,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    return Err(format!("error specified file does not exist."));
                } else if e.kind() == ErrorKind::PermissionDenied {
                    return Err(format!(
                        "The current user does not have permission to access the specified file."
                    ));
                } else if e.kind() == ErrorKind::Interrupted {
                    return Err(format!(
                        "The read operation was interrupted by another signal."
                    ));
                } else if e.kind() == ErrorKind::UnexpectedEof {
                    return Err(format!(
                        "An unexpected end of file was encountered during the read operation."
                    ));
                } else {
                    return Err(format!("Unepected error occured while reading from specified file, there are many reasons of this error such as invalid data in the file, disk errors, or insufficient memory."));
                }
            }
        }

        if bytes_read == 0 {
            break;
        }

        chunks.push(buffer);
    }

    Ok(chunks)
}

fn open_folder(dir_name: &String) -> Result<String, std::io::Error> {
    let dir = Path::new(dir_name);

    for entry in read_dir(dir)? {
        let entry = entry?;
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let f = metadata(&args[1]).unwrap();

        if f.is_file() {
            println!("{:?}", open_file(PathBuf::from(&args[1])))
        } else if f.is_dir() {
            _ = open_folder(&args[1])
        } else {
            panic!("Unknown error occured!")
        }
    }
}
