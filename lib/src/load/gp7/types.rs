use std::{borrow::Cow, num::ParseIntError};
use strong_xml::{XmlRead, XmlReader, XmlResult, XmlError};


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
    pub master_bar: MasterBars<'a>,
    #[xml(child = "Bars")]
    pub bars: Bars<'a>,
    #[xml(child = "Voices")]
    pub voices: Voices,
    #[xml(child = "Beats")]
    pub beats: Beats,
    #[xml(child = "Rhythms")]
    pub rhythms: Rhythms<'a>,
}

// TODO: CDATA on relevant fields
#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Score")]
pub struct Score<'a> {
    #[xml(flatten_text = "Title")]
    pub title: Cow<'a, str>,
    #[xml(flatten_text = "SubTitle")]
    pub subtitle: Cow<'a, str>,
    #[xml(flatten_text = "Artist")]
    pub artist: Cow<'a, str>,
    #[xml(flatten_text = "Album")]
    pub album: Cow<'a, str>,
    #[xml(flatten_text = "Copyright")]
    pub copyright: Cow<'a, str>,
    #[xml(flatten_text = "Tabber")]
    pub tabber: Cow<'a, str>,
    #[xml(flatten_text = "Music")]
    pub music: Cow<'a, str>,
    #[xml(flatten_text = "Words")]
    pub words: Cow<'a, str>,
    #[xml(flatten_text = "Instructions")]
    pub instructions: Cow<'a, str>,
    #[xml(flatten_text = "Notices")]
    pub notices: Vec<Cow<'a, str>>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "MasterTrack")]
pub struct MasterTrack<'a> {
    #[xml(flatten_text = "Tracks")]
    pub tracks: Cow<'a, str>,
    // #[xml(flatten_text = "Bars")]
    // pub bars: Vec<u8>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "MasterBars")]
pub struct MasterBars<'a> {
    #[xml(child = "MasterBar")]
    pub master_bars: Vec<MasterBar<'a>>,
}

#[derive(PartialEq, Debug)]
pub struct IdVec<T> {
    pub vec: Vec<T>
}

impl<T> std::str::FromStr for IdVec<T>
where T: std::str::FromStr<Err = ParseIntError>
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err>  {
        let r: Result<Vec<_>, _> = s.split(" ").into_iter()
            .map(|id| id.parse())
            .collect();

        Ok(Self {
            vec: r?
        })
    }
}


impl<'a, T> XmlRead<'a> for IdVec<T>
where T: std::str::FromStr<Err = ParseIntError>
{
    fn from_reader(reader: &mut XmlReader<'a>) -> XmlResult<Self> {
        if let Some(tag) = reader.find_element_start(None)? {
            reader.next();
            if let Some(attr) = reader.find_attribute()? {
                return Err(XmlError::UnexpectedToken { token: attr.0.to_owned() })
            }
            reader.next();
            let text = reader.read_text(tag)?;
            let v = <IdVec<T> as std::str::FromStr>::from_str(text.as_ref())
                .map_err(|e| XmlError::FromStr(Box::new(e)))?;

            reader.next();
            return Ok(v);
        }
        Err(XmlError::UnexpectedEof)
    }
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "MasterBar")]
pub struct MasterBar<'a> {
    #[xml(flatten_text = "Time")]
    pub time: Cow<'a, str>,
    #[xml(child = "Bars")]
    pub bars: IdVec<u16>,
}


#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Bars")]
pub struct Bars<'a> {
    #[xml(child = "Bar")]
    pub master_bars: Vec<Bar<'a>>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Bar")]
pub struct Bar<'a> {
    #[xml(flatten_text = "Clef")]
    pub clef: Cow<'a, str>,
    #[xml(flatten_text = "Voices")]
    pub voices: IdVec<i16>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Voices")]
pub struct Voices {
    #[xml(child = "Voice")]
    pub voices: Vec<Voice>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Voice")]
pub struct Voice {
    #[xml(flatten_text = "Beats")]
    pub beats: IdVec<u16>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Beats")]
pub struct Beats {
    #[xml(child = "Beat")]
    pub beats: Vec<Beat>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Beat")]
pub struct Beat {
    #[xml(child = "Rhythm")]
    pub rhythm_ref: RhythmRef,
    #[xml(flatten_text = "Notes")]
    pub notes: Option<IdVec<u16>>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Rhythm")]
pub struct RhythmRef {
    #[xml(attr = "ref")]
    pub rhythm: u32,
}


#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Rhythms")]
pub struct Rhythms<'a> {
    #[xml(child = "Rhythm")]
    pub beats: Vec<Rhythm<'a>>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Rhythm")]
pub struct Rhythm<'a>{
    #[xml(flatten_text = "NoteValue")]
    pub note_value: Cow<'a, str>,
}
