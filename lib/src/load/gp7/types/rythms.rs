use std::borrow::Cow;
use strong_xml::XmlRead;

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Rhythms")]
pub struct Rhythms<'a> {
    #[xml(child = "Rhythm")]
    pub rythms: Vec<Rhythm<'a>>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Rhythm")]
pub struct Rhythm<'a>{
    #[xml(flatten_text = "NoteValue")]
    pub note_value: Cow<'a, str>,
    #[xml(child = "PrimaryTuplet")]
    pub primary_tuplet: Option<PrimaryTuplet>,
    #[xml(child = "AugmentationDot")]
    pub augmentation_dot: Option<AugmentationDot>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "PrimaryTuplet")]
pub struct PrimaryTuplet{
    #[xml(attr = "num")]
    pub num: u8,
    #[xml(attr = "den")]
    pub den: u8,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "AugmentationDot")]
pub struct AugmentationDot{
    #[xml(attr = "count")]
    pub count: u8,
}
