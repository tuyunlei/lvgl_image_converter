use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use crate::util::load_file;

#[derive(Debug)]
pub struct ColorFormatError(String);

impl Display for ColorFormatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown color format: {}", self.0)
    }
}

impl<'a> Error for ColorFormatError {}

#[derive(Debug)]
pub enum ColorFormat {
    TrueColor332 = 0,
    TrueColor565 = 1,
    TrueColor565Swap = 2,
    TrueColor888 = 3,
    Alpha1Bit = 4,
    Alpha2Bit = 5,
    Alpha4Bit = 6,
    Alpha8Bit = 7,
    Indexed1Bit = 8,
    Indexed2Bit = 9,
    Indexed4Bit = 10,
    Indexed8Bit = 11,
    Raw = 12,
    RawAlpha = 13,
    // RawChroma = 12,
    TrueColor = 100,
    TrueColorAlpha = 101,
    TrueColorChroma = 102,
}

impl ColorFormat {
    pub fn from_string(string: &str) -> Result<Self, ColorFormatError> {
        match string {
            "CF_TRUE_COLOR_332" => Ok(ColorFormat::TrueColor332),
            "CF_TRUE_COLOR_565" => Ok(ColorFormat::TrueColor565),
            "CF_TRUE_COLOR_565_SWAP" => Ok(ColorFormat::TrueColor565Swap),
            "CF_TRUE_COLOR_888" => Ok(ColorFormat::TrueColor888),
            "CF_ALPHA_1_BIT" => Ok(ColorFormat::Alpha1Bit),
            "CF_ALPHA_2_BIT" => Ok(ColorFormat::Alpha2Bit),
            "CF_ALPHA_4_BIT" => Ok(ColorFormat::Alpha4Bit),
            "CF_ALPHA_8_BIT" => Ok(ColorFormat::Alpha8Bit),
            "CF_INDEXED_1_BIT" => Ok(ColorFormat::Indexed1Bit),
            "CF_INDEXED_2_BIT" => Ok(ColorFormat::Indexed2Bit),
            "CF_INDEXED_4_BIT" => Ok(ColorFormat::Indexed4Bit),
            "CF_INDEXED_8_BIT" => Ok(ColorFormat::Indexed8Bit),
            "CF_RAW" => Ok(ColorFormat::Raw),
            "CF_RAW_ALPHA" => Ok(ColorFormat::RawAlpha),
            // "CF_RAW_CHROMA" => Ok(ColorFormat::RawChroma),
            "CF_RAW_CHROMA" => Ok(ColorFormat::Raw),
            "CF_TRUE_COLOR" => Ok(ColorFormat::TrueColor),
            "CF_TRUE_COLOR_ALPHA" => Ok(ColorFormat::TrueColorAlpha),
            "CF_TRUE_COLOR_CHROMA" => Ok(ColorFormat::TrueColorChroma),
            _ => Err(ColorFormatError(format!("Unknown color format: {}", string)))
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub color_format: ColorFormat,
    pub out_c: bool,
    pub out_bin: bool,
    pub path: String,
    pub cf_palette_bgr_en: bool,
}

pub fn load_config() -> Config {
    let file = load_file("config");
    let lines = BufReader::new(file).lines();

    let mut config = Config {
        color_format: ColorFormat::Alpha2Bit,
        out_c: false,
        out_bin: false,
        path: "images".to_string(),
        cf_palette_bgr_en: false,
    };

    for (i, line) in lines.into_iter().enumerate() {
        match line {
            Ok(line) => {
                let split: Vec<_> = line.split('=').collect();
                if split.len() != 2 {
                    println!("Error config line {}: {}", i, line);
                    exit(1);
                }
                let key = split[0].trim();
                let value = split[1].trim();
                match key {
                    "color_format" => {
                        let result = ColorFormat::from_string(value);
                        match result {
                            Ok(color_format) => config.color_format = color_format,
                            Err(e) => {
                                println!("Error when read color format: {}", e);
                                exit(1);
                            }
                        }
                    }
                    "out_c" => {
                        config.out_c = match value {
                            "yes" => true,
                            "no" => false,
                            _ => {
                                println!("Error out_c value: {}", value);
                                exit(1);
                            }
                        }
                    }
                    "out_bin" => {
                        config.out_bin = match value {
                            "yes" => true,
                            "no" => false,
                            _ => {
                                println!("Error out_bin value: {}", value);
                                exit(1);
                            }
                        }
                    }
                    "cf_palette_bgr_en" => {
                        config.cf_palette_bgr_en = match value {
                            "yes" => true,
                            "no" => false,
                            _ => {
                                println!("Error cf_palette_bgr_en value: {}", value);
                                exit(1);
                            }
                        }
                    }
                    "path" => config.path = value.to_string(),
                    _ => {}
                }
            }
            Err(e) => {
                println!("Error when read lines: {}", e.to_string());
                exit(1);
            }
        }
    }

    config
}