use std::path::Path;
use thiserror::Error;

use crate::gp::Song;

mod gp7;

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("Could not detect version: {0}")]
    VersionDetection(String),
    #[error("Unsupported version {0}")]
    Unsupported(String),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("GP7 load failed: {0}")]
    GP7LoadError(#[from] gp7::LoadError)
}

pub fn load(file: &Path) -> Result<Song, LoadError>{
    let ext = file.extension()
        .and_then(std::ffi::OsStr::to_str)
        .ok_or(LoadError::VersionDetection("Cannot get input file extension".into()))?;

    let f = std::fs::File::open(file)?;
    let reader = std::io::BufReader::new(f);

    match ext.to_uppercase().as_str() {
        "GP3" => Err(LoadError::Unsupported("GP3".into())),
        "GP4" => Err(LoadError::Unsupported("GP4".into())),
        "GP5" => Err(LoadError::Unsupported("GP5".into())),
        "GPX" => Err(LoadError::Unsupported("GPX".into())),
        "GP" => gp7::load(reader).map_err(|e| LoadError::GP7LoadError(e)),
        s => Err(LoadError::VersionDetection(format!("Unknown version {}", s))),
    }
}
