use std::borrow::Cow;
use strong_xml::{XmlRead, XmlWrite};


#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "GPIF")]
pub struct GPIF<'a> {
    #[xml(flatten_text = "GPVersion")]
    pub gpversion: Cow<'a, str>,
    #[xml(child = "Score")]
    pub score: Score<'a>
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
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
    pub notices: Vec<Cow<'a, str>>
}
