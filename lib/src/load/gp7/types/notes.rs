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

impl<'a> PropertyParser<'a> for NoteProperty {
    fn parse_property(typ: &str, reader: &mut XmlReader<'a>) -> XmlResult<Self> {
        let mut reader = ReaderExt::new(reader);
        match typ {
            "ConcertPitch" => {
                reader.skip_to_open()?;
                let pitch = Pitch::from_reader(reader.as_mut())?;
                log::debug!("After pitch read: {:?}", reader.peek());
                //reader.skip_over_close()?;
                log::debug!("Done reading concert pitch: {:?}", reader.peek());
                Ok(NoteProperty::ConcertPitch(pitch))
            },
            "Fret" => {
                reader.skip_to_open()?;
                reader.open_tag_named("Fret")?;
                reader.skip_to_text()?;
                let val = reader.text()?;
                reader.close_tag_named("Fret")?;
                Ok(NoteProperty::Fret(val.parse::<u8>().map_err(|e| XmlError::FromStr(Box::new(e)))?))
            },
            "Harmonic" => {
                reader.skip_to_open()?;
                reader.open_tag_named("Enable")?;
                reader.skip_over_close_empty()?;
                Ok(NoteProperty::Harmonic(true))
            },
            "HarmonicFret" => {
                reader.skip_to_open()?;
                reader.open_tag_named("HFret")?;
                reader.skip_to_text()?;
                let val = reader.text()?;
                reader.close_tag_named("HFret")?;
                Ok(NoteProperty::HarmonicFret(val.parse::<f32>().map_err(|e| XmlError::FromStr(Box::new(e)))?))
            },
            "HarmonicType" => {
                reader.skip_to_open()?;
                reader.open_tag_named("HType")?;
                reader.skip_to_text()?;
                let val = reader.text()?;
                reader.close_tag_named("HType")?;
                Ok(NoteProperty::HarmonicType(val.into()))
            },
            "LeftHandTapped" => {
                reader.skip_to_open()?;
                reader.open_tag_named("Enable")?;
                reader.skip_over_close_empty()?;
                Ok(NoteProperty::LeftHandTapped(true))
            },
            "Tapped" => {
                reader.skip_to_open()?;
                reader.open_tag_named("Enable")?;
                reader.skip_over_close_empty()?;
                Ok(NoteProperty::Tapped(true))
            },
            "HopoOrigin" => {
                reader.skip_to_open()?;
                reader.open_tag_named("Enable")?;
                reader.skip_over_close_empty()?;
                Ok(NoteProperty::HopoOrigin(true))
            },
            "HopoDestination" => {
                reader.skip_to_open()?;
                reader.open_tag_named("Enable")?;
                reader.skip_over_close_empty()?;
                Ok(NoteProperty::HopoDestination(true))
            },
            "Midi" => {
                reader.skip_to_open()?;
                reader.open_tag_named("Number")?;
                reader.skip_to_text()?;
                let val = reader.text()?;
                reader.close_tag_named("Number")?;
                Ok(NoteProperty::Midi(val.parse::<u8>().map_err(|e| XmlError::FromStr(Box::new(e)))?))
            },
            "Slide" => {
                reader.skip_to_open()?;
                reader.open_tag_named("Flags")?;
                reader.skip_to_text()?;
                let val = reader.text()?;
                reader.close_tag_named("Flags")?;
                Ok(NoteProperty::Slide(val.parse::<u8>().map_err(|e| XmlError::FromStr(Box::new(e)))?))
            },
            "HopoOrigin" => {
                reader.skip_to_open()?;
                reader.open_tag_named("Enable")?;
                reader.skip_over_close_empty()?;
                Ok(NoteProperty::HopoOrigin(true))
            },
            "Muted" => {
                reader.skip_to_open()?;
                reader.open_tag_named("Enable")?;
                reader.skip_over_close_empty()?;
                Ok(NoteProperty::Muted(true))
            },
            "PalmMuted" => {
                reader.skip_to_open()?;
                reader.open_tag_named("Enable")?;
                reader.skip_over_close_empty()?;
                Ok(NoteProperty::PalmMuted(true))
            },
            "String" => {
                reader.skip_to_open()?;
                reader.open_tag_named("String")?;
                reader.skip_to_text()?;
                let val = reader.text()?;
                reader.close_tag_named("String")?;
                Ok(NoteProperty::String(val.parse::<u8>().map_err(|e| XmlError::FromStr(Box::new(e)))?))
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





