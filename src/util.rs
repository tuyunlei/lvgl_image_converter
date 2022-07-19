use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

pub fn load_file<P: AsRef<Path>>(path: P) -> File {
    match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error when open file: {}", e);
            exit(1)
        }
    }
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let path = path.as_ref();
    return match fs::metadata(path) {
        Ok(metadata) => {
            let mut file = load_file(path);
            let mut buffer = vec![0; metadata.len() as usize];
            file.read(&mut buffer).expect("buffer overflow");
            buffer
        }
        Err(e) => {
            println!("Error when read metadata from file {:?}: {}", path, e);
            exit(1);
        }
    }
}