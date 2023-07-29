use std::io::{Read, Seek};
use thiserror::Error;
use strong_xml::{XmlRead, XmlReader};
use zip;

use crate::gp::Song;

mod types;
mod convert;


#[derive(Error, Debug)]
pub enum LoadError {
    #[error("error loading zip file")]
    Zip(#[from] zip::result::ZipError),
    #[error("error parsing score")]
    Parse(#[from] strong_xml::XmlError),
    #[error("error reading zip contents")]
    Read(#[from] std::io::Error),
    #[error("error reading zip contents")]
    Convert(#[from] std::string::FromUtf8Error),

}


type Result<T> = std::result::Result<T, LoadError>;

pub fn load(reader: impl Read + Seek) -> Result<Song> {
    let mut zip = zip::ZipArchive::new(reader)?;

    let mut score = zip.by_name("Content/score.gpif")?;

    let mut buf = Vec::with_capacity(score.size() as usize);
    score.read_to_end(&mut buf)?;

    let strbuf = String::from_utf8(buf)?;
    let mut reader = XmlReader::new(&strbuf);
    let gpif = types::GPIF::from_reader(&mut reader)?;

    log::debug!("MasterTrack: {:?}", gpif.master_track);
    log::debug!("Number of MasterBars: {}", gpif.master_bars.master_bars.len());

    log::debug!("Initial time signature: {}", gpif.master_bars.master_bars[0].time);
    log::debug!("Number of tracks: {:?}", gpif.master_bars.master_bars[0].bars.len());
    log::debug!("Rythms: {:?}", gpif.rhythms.rythms.len());
    log::debug!("Notes: {:?}", gpif.notes.notes.len());

    Ok(convert::gpif_to_song(&gpif))
}
