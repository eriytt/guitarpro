use std::{io::{Read, Seek}, borrow::Cow};
use thiserror::Error;
use strong_xml::{XmlRead, XmlReader};
use zip;


use super::Song;

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

    log::debug!("Number of tracks: {}", gpif.master_bars.master_bars[0].time);
    log::debug!("Number of tracks: {:?}", gpif.master_bars.master_bars[0].bars);
    //log::debug!("Rythms: {:?}", gpif.rhythms);
    //log::debug!("Notes: {:?}", gpif.notes);

    let song = Song {
        artist: gpif.score.artist.to_string(),
        name: gpif.score.title.to_string(),
        album: gpif.score.album.to_string(),
        author: gpif.score.music.to_string(),
        words: gpif.score.words.to_string(),
        copyright: gpif.score.copyright.to_string(),
        transcriber: gpif.score.tabber.to_string(),
        instructions: gpif.score.instructions.to_string(),
        notice: gpif.score.notices.iter().map(Cow::to_string).collect(),
        tracks: convert::gpif_to_tracks(&gpif),
        ..Default::default()
    };
    Ok(song)
}
