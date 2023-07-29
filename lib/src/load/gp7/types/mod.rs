use std::{borrow::Cow, num::ParseIntError};
use strong_xml::{XmlRead, XmlReader, XmlResult, XmlError};


mod gpif;
pub use gpif::GPIF;

mod rhythms;
pub use rhythms::{Rhythms, Rhythm, PrimaryTuplet};

#[allow(unused)]
mod notes;
pub use notes::{Notes, Note, NoteProperty};

#[allow(unused)]
mod parser_ext;
use parser_ext::ReaderExt;

trait PropertyParser<'a>: Sized {
    fn parse_property(typ: &str, reader: &mut XmlReader<'a>) -> XmlResult<Self>;
}

// TODO: equality is not the same Vecs in different order
#[derive(PartialEq, Debug)]
pub struct Properties<T> {
    properties: Vec<T>
}

impl<'a, T> XmlRead<'a> for Properties<T>
where T: PropertyParser<'a> + std::fmt::Debug
{
    fn from_reader(reader: &mut XmlReader<'a>) -> XmlResult<Self> {
        let mut properties = Vec::<T>::new();
        let mut reader = ReaderExt::new(reader);

        while reader.until_end_tag("Properties")? {
            reader.skip_to_open()?; // This might be the last property
            reader.open_tag_named("Property")?;
            let name = reader.attr_named("name")?;
            reader.skip_to_open()?;

            let prop = <T as PropertyParser>::parse_property(name.as_ref(), reader.as_mut())?;
            properties.push(prop);

            reader.skip_to_close()?;
            reader.close_tag_named("Property")?;
        }
        Ok(Self { properties} )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_log;
//         let xml = r#"<Property name="ConcertPitch">
//           <Pitch>
//             <Step>B</Step>
//             <Accidental/>
//             <Octave>2</Octave>
//           </Pitch>
//         </Property>
//         <Property name="Fret">
//           <Fret>2</Fret>
//         </Property>
//         <Property name="Midi">
//           <Number>35</Number>
//         </Property>
//         <Property name="PalmMuted">
//           <Enable/>
//         </Property>
//         <Property name="String">
//           <String>1</String>
//         </Property>
//         <Property name="TransposedPitch">
//           <Pitch>
//             <Step>B</Step>
//             <Accidental/>
//             <Octave>3</Octave>
//           </Pitch>
//         </Property>
//       </Properties>
// "#;

    #[test_log::test]
    fn test_properties() {
        #[derive(PartialEq, Debug)]
        enum NoteProperty {
            ConcertPitch(u8),
            Fret(u8),
            Midi(u8),
            PalmMuted(u8),
            String(u8),
            TransposedPitch(u8)
        }

        impl<'a> PropertyParser<'a> for NoteProperty {
            fn parse_property(typ: &str, reader: &mut XmlReader<'a>)
                              -> XmlResult<Self> {
                let mut reader = ReaderExt::new(reader);
                log::debug!("Parsing property {:?}", reader.peek());
                reader.skip_over_close()?;
                match typ {
                    "ConcertPitch" => Ok(NoteProperty::ConcertPitch(47u8)),
                    "Fret" => Ok(NoteProperty::Fret(0u8)),
                    "Midi" => Ok(NoteProperty::Fret(0u8)),
                    "PalmMuted" => Ok(NoteProperty::Fret(0u8)),
                    "String" => Ok(NoteProperty::Fret(0u8)),
                    "TransposedPitch" => Ok(NoteProperty::Fret(0u8)),
                    _ => Err(XmlError::UnrecognizedSymbol { symbol: typ.into() })
                }
            }
        }

        let xml = r#"<Property name="ConcertPitch">
          <x>0</x>
        </Property><Property name="Fret">

          <x>0</x>
        </Property>
        <Property name="Midi">
          <x>0</x>
        </Property>
        <Property name="PalmMuted">
          <x>0</x>
        </Property>
        <Property name="String">
          <x>0</x>
        </Property>
        <Property name="TransposedPitch">
          <x>0</x>
        </Property>
      </Properties>
"#;
        let mut reader = XmlReader::new(&xml);
        let props = Properties::<NoteProperty>::from_reader(&mut reader);
        println!("props: {:?}", props);
        assert!(props.is_ok());

        let props = props.unwrap();
        assert_eq!(props.properties.len(), 6);
        assert_eq!(props.properties[0], NoteProperty::ConcertPitch(47u8));
        assert_eq!(props.properties[1], NoteProperty::Fret(47u8));
        assert_eq!(props.properties[2], NoteProperty::Midi(47u8));
        assert_eq!(props.properties[3], NoteProperty::PalmMuted(47u8));
        assert_eq!(props.properties[4], NoteProperty::String(47u8));
        assert_eq!(props.properties[5], NoteProperty::TransposedPitch(47u8));
    }
}



#[derive(PartialEq, Default, Debug)]
pub struct IdVec<T: Default> {
    pub vec: Vec<T>
}

impl<T: Default> IdVec<T> {
    pub fn len(&self) -> usize { self.vec.len()}
}

impl<T> std::str::FromStr for IdVec<T>
where T: std::str::FromStr<Err = ParseIntError> + Default
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
where T: std::str::FromStr<Err = ParseIntError> + Default
{
    fn from_reader(reader: &mut XmlReader<'a>) -> XmlResult<Self> {
        let mut reader = ReaderExt::new(reader);

        let tag = reader.open_tag()?;
        reader.skip_to_text()?;
        let text = reader.text()?;
        let v = <IdVec<T> as std::str::FromStr>::from_str(text.as_ref())
            .map_err(|e| XmlError::FromStr(Box::new(e)))?;
        reader.skip_to_close()?;
        reader.close_tag_named(tag)?;

        Ok(v)
    }
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
    pub bars: Vec<Bar<'a>>,
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
