use std::borrow::Cow;
use strong_xml::{XmlError, XmlRead, XmlReader, XmlResult};

use super::{Properties, PropertyParser, parser_ext::ReaderExt};

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Notes")]
pub struct Notes<'a> {
    #[xml(child = "Note")]
    pub notes: Vec<Note<'a>>,
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Note")]
pub struct Note<'a> {
    #[xml(flatten_text = "InstrumentArticulation")]
    instrument_articulation: Cow<'a, str>,
    #[xml(child = "Properties")]
    properties: NoteProperties
}

impl<'a> Note<'a> {
    pub fn properties(&self) -> &Vec<NoteProperty> {
        &self.properties.properties.properties
    }
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Properties")]
pub struct NoteProperties {
     #[xml(child = "Property")]
    properties: Properties<NoteProperty>,
}

#[derive(PartialEq, Debug)]
// #[xml(tag = "Property")]
pub enum NoteProperty {
    ConcertPitch(Pitch),
    Fret(u8),
    Harmonic(bool),
    HarmonicFret(f32),
    HarmonicType(String),
    HopoOrigin(bool),
    HopoDestination(bool),
    Tapped(bool),
    LeftHandTapped(bool),
    Midi(u8),
    Slide(u8),
    Muted(bool),
    PalmMuted(bool),
    String(u8),
    TransposedPitch(Pitch)
}

fn enable<'a, 'r>(reader: &'r mut ReaderExt<'r, 'a>) -> XmlResult<bool> {
    match reader.empty_tag()?.as_ref() {
        "Enable" => Ok(true),
        "Disable" => Ok(false),
        t => Err(XmlError::TagMismatch {
            expected: "Enable/Disable".into(),
            found: t.into() })
    }
}

fn simple_number_tag<'a, 'r, T>(tag: &str, reader: &'r mut ReaderExt<'r, 'a>) -> XmlResult<T>
where T:  std::str::FromStr,
<T as std::str::FromStr>::Err: std::error::Error + Send + Sync + 'static
{
    let val = reader.simple_tag_text(tag)?;
    val.parse::<T>().map_err(|e| XmlError::FromStr(Box::new(e)))
}



impl<'a> PropertyParser<'a> for NoteProperty {
    fn parse_property(typ: &str, reader: &mut XmlReader<'a>) -> XmlResult<Self> {
        let mut reader = ReaderExt::new(reader);
        match typ {
            "Harmonic" =>
                Ok(NoteProperty::Harmonic(enable(&mut reader)?)) ,
            "LeftHandTapped" =>
                Ok(NoteProperty::LeftHandTapped(enable(&mut reader)?)),
            "Tapped" =>
                Ok(NoteProperty::Tapped(enable(&mut reader)?)),
            "HopoOrigin" =>
                Ok(NoteProperty::HopoOrigin(enable(&mut reader)?)),
            "HopoDestination" =>
                Ok(NoteProperty::HopoDestination(enable(&mut reader)?)),
            "HopoOrigin" =>
                Ok(NoteProperty::HopoOrigin(enable(&mut reader)?)),
            "Muted" =>
                Ok(NoteProperty::Muted(enable(&mut reader)?)),
            "PalmMuted" =>
                Ok(NoteProperty::PalmMuted(enable(&mut reader)?)),
            "Fret" =>
                Ok(NoteProperty::Fret(simple_number_tag::<u8>("Fret", &mut reader)?)),
            "HarmonicFret" =>
                Ok(NoteProperty::HarmonicFret(simple_number_tag::<f32>("HFret", &mut reader)?)),
            "Midi" =>
                Ok(NoteProperty::Midi(simple_number_tag::<u8>("Number", &mut reader)?)),
            "Slide" =>
                Ok(NoteProperty::Slide(simple_number_tag::<u8>("Flags", &mut reader)?)),
            "String" =>
                Ok(NoteProperty::String(simple_number_tag::<u8>("String", &mut reader)?)),
            "HarmonicType" =>
                Ok(NoteProperty::HarmonicType(reader.simple_tag_text("HType")?.into())),
            "ConcertPitch" => {
                reader.skip_to_open()?;
                let pitch = Pitch::from_reader(reader.as_mut())?;
                Ok(NoteProperty::ConcertPitch(pitch))
            },
            "TransposedPitch" => {
                reader.skip_to_open()?;
                let pitch = Pitch::from_reader(reader.as_mut())?;
                Ok(NoteProperty::TransposedPitch(pitch))
            },
            _ => Err(XmlError::UnrecognizedSymbol { symbol: typ.into() })
        }
    }
}

#[derive(XmlRead, PartialEq, Debug)]
#[xml(tag = "Pitch")]
pub struct Pitch {
    #[xml(flatten_text = "Step")]
    step: String,
    #[xml(flatten_text = "Accidental")]
    accidental: String,
    #[xml(flatten_text = "Octave")]
    octave: u8
}

// #[derive(XmlRead, PartialEq, Debug)]
// #[xml(tag = "Pitch")]
// pub struct Pitch<'a> {
//     #[xml(flatten_text = "Step")]
//     step: Cow<'a, str>,
//     #[xml(flatten_text = "Accidental")]
//     accidental: Cow<'a, str>,
//     #[xml(flatten_text = "Octave")]
//     octave: u8
// }





