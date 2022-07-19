use std::fs;
use std::process::exit;
use crate::converter::Converter;

mod config;
mod converter;
mod util;

fn main() {
    let config = config::load_config();
    println!("Config list:");
    println!("cf={:?}", config.color_format);
    println!("out_c={}", config.out_c);
    println!("out_bin={}", config.out_bin);
    println!("path={}", config.path);
    println!("cf_palette_bgr_en={}", config.cf_palette_bgr_en);
    println!("===========================================================");

    let mut converter = Converter::new(&config, true);
    match fs::read_dir(&config.path) {
        Ok(dir) => {
            for entry in dir {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if !path.is_file() {
                            continue;
                        }
                        if let Some(ext) = path.extension() {
                            if ext == "c" || ext == "bin" || ext == "h" {
                                continue;
                            }
                        }
                        if let Some(filename) = path.file_name() {
                            if filename == ".gitignore" {
                                continue;
                            }
                        }

                        println!("{:?}", path);
                        println!("converting...");
                        converter.convert(&path);
                        println!("saving...");
                    }
                    Err(e) => {
                        println!("Error when read directory entry: {}", e);
                        exit(1);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error when read directory '{}': {}", &config.path, e);
            exit(1);
        }
    }
}
