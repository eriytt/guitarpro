use std::borrow::Cow;
use strong_xml::XmlRead;

use super::{Score, MasterTrack, MasterBars, Bars, Voices, Beats, Notes, Rhythms};

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "GPIF")]
pub struct GPIF<'a> {
    #[xml(flatten_text = "GPVersion")]
    pub gpversion: Cow<'a, str>,
    #[xml(child = "Score")]
    pub score: Score<'a>,

    #[xml(child = "MasterTrack")]
    pub master_track: MasterTrack<'a>,

    #[xml(child = "MasterBars")]
    pub master_bars: MasterBars<'a>,
    #[xml(child = "Bars")]
    pub bars: Bars<'a>,
    #[xml(child = "Voices")]
    pub voices: Voices,
    #[xml(child = "Beats")]
    pub beats: Beats,
    #[xml(child = "Notes")]
    pub notes: Notes<'a>,
    #[xml(child = "Rhythms")]
    pub rhythms: Rhythms<'a>,
}
