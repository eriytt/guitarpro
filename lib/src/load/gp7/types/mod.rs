use std::{borrow::Cow, num::ParseIntError};
use strong_xml::{XmlRead, XmlReader, XmlResult, XmlError};


mod gpif;
pub use gpif::GPIF;

mod rhythms;
pub use rhythms::Rhythms;

#[allow(unused)]
mod notes;
pub use notes::Notes;

#[allow(unused)]
mod parser_ext;
use parser_ext::ReaderExt;

// reader::next()
// Reads the next token (might be empty)
//
// reader::peek()
// Same as read but does not advance
//
// reader::read_text(end_tag)
// Reads element text, advancing. Ignores ElementEnd::Open and Attributes and records the text.
// The end_tag must match the first encountered. Only raw text (or CDATA) is allowed inside the tag.
// Does not advance past potential newlines after end?
//
// reader.read_till_element_start(end_tag)
// Error on ElementEnd, Attribute, and text (including CDATA). Newlines are allowed (?).
// If end_tag doesn't match the start_tag, skips over the tag (read_to_end)(?). end_tag is effectively
// the start tag to search for.
//
// reader.find_attribute()
// Get one attribute, consuming it. Must be called after an ElementStart.
// Does not consume the ElementEnd.
//
// reader.find_element_start(Option<end_tag>)
// Forward to ElementStart without consuming it. Attributes and ElementEnd::Open are errors.
// ElementEnd::Close are ok if the end_tag matches. Anything else will just be forwarded over.
// Caveat: If you hit a matching ElementEnd, you will not get the ElementStart returned.
//
// reader.read_to_end(end_tag)
// Seems to be made for skipping over the rest of a tag when you're after the ElementStart, also
// nested tags. However Text is not allowed anywhere, so this seems broken.
// Hmm, text seems to be allowd after all

// XmlRead::from_reader()
// When called, you're before the ElementStart. 

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

        log::debug!("Start reading properties: {:?}", reader.peek());

        while reader.until_end_tag("Properties")? {
            reader.skip_to_open()?; // This might be the last property
            log::debug!("Read open");
            reader.open_tag_named("Property")?;
            log::debug!("Read name attribute");
            let name = reader.attr_named("name")?;
            log::debug!("Read skip to open (name={})", name);
            reader.skip_to_open()?;

            log::debug!("Parse property");
            let prop = <T as PropertyParser>::parse_property(name.as_ref(), reader.as_mut())?;
            log::debug!("Read property {:?}", &prop);
            properties.push(prop);

            reader.skip_to_close()?;
            reader.close_tag_named("Property")?;

            log::debug!("Closed");
            log::debug!("Skip to next property: {:?}", reader.peek());

            log::debug!("Loop: {:?}", reader.peek());
        }
        Ok(Self { properties} )
    }

    // fn from_reader(reader: &mut XmlReader<'a>) -> XmlResult<Self> {
    //     let mut properties = Vec::<T>::new();

    //     reader.log_current("At before start tag:");
    //     // Forward to the next Property
    //     while let Some(tag) = reader.open_tag()? {

    //         // let w = reader.peek().unwrap().unwrap();
    //         log::debug!("At start tag {}", tag);
    //         reader.log_current("At start tag:");
    //         //log::debug!("At start tag {}: {:?}", tag, reader.peek().unwrap().unwrap());
    //         reader.next();

    //         // Get attribute "name" value;
    //         let typ = match reader.find_attribute()? {
    //             Some((k, v)) => {
    //                 log::debug!("Attribute: {}={}", k, v);
    //                 match k {
    //                     "name" => Ok(v),
    //                     _ => Err(XmlError::UnexpectedToken {
    //                         token: format!("Unsupported property attribute key: {}", k) })
    //                 }
    //             },
    //             None => Err(XmlError::MissingField { name: "Property".into(), field: "name".into()})
    //         }?;

    //         //log::debug!("After attribute: {:?}", reader.peek().unwrap().unwrap());

    //         // Forward to child, checking attribute
    //         if let Some(attr) = reader.find_attribute()? {
    //             return Err(XmlError::UnexpectedToken {
    //                 token: format!("Unexpected property attribute {:?}", attr)
    //             })
    //         }

    //         //log::debug!("After attribute check: {:?}", reader.peek().unwrap().unwrap());
    //         reader.next().unwrap()?;

    //         // Get the child start tag
    //         let start_tag = reader.find_element_start(None)?.unwrap();
    //         // log::debug!("At property?: {:?}", reader.peek().unwrap().

    //         log::debug!("Parsing property {}(tag = {}): {:?}", typ, start_tag, reader.peek().unwrap().unwrap());

    //         let prop = <T as PropertyParser>::parse_property(typ.as_ref(), start_tag, reader)?;
    //         properties.push(prop);

    //         log::debug!("After property parse: {:?}", reader.peek().unwrap().unwrap());
    //         reader.next().unwrap()?;

    //         // log::debug!("Got tag: {:?}", reader.find_element_start(Some("Property"))?);
    //         // log::debug!("At next property?: {:?}", reader.peek().unwrap().unwrap());

    //         // log::debug!("Got tag again: {:?}", reader.find_element_start(Some("Property"))?); 
    //         // log::debug!("At next property again?: {:?}", reader.peek().unwrap().unwrap());
    //     }
    //     Err(XmlError::UnexpectedEof)
    // }
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
