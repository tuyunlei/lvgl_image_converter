use std::path::Path;

use crate::config::{ColorFormat, Config};
use crate::util::read_file;

pub struct Converter<'a> {
    config: &'a Config,
    dith: bool,
    d_out: Vec<u8>,
}

impl<'a> Converter<'a> {
    pub fn new(config: &'a Config, dith: bool) -> Self {
        Self {
            config,
            dith,
            d_out: Vec::new(),
        }
    }

    pub fn convert(&mut self, path: &Path) {
        if let ColorFormat::Raw | ColorFormat::RawAlpha = self.config.color_format {
            self.d_out = read_file(path);
            return;
        }

        let palette_size = match &self.config.color_format {
            ColorFormat::Indexed1Bit => 2,
            ColorFormat::Indexed2Bit => 4,
            ColorFormat::Indexed4Bit => 16,
            ColorFormat::Indexed8Bit => 256,
            _ => 0
        };

        if palette_size != 0 {}
    }
}
