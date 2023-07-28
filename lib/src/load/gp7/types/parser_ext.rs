use std::borrow::Cow;
use log;

use strong_xml::{XmlReader, XmlResult, XmlError};
use strong_xml::xmlparser::Token;
use strong_xml::utils::xml_unescape;

// pub trait ReaderExt {
//     fn log_current(&mut self, s: &str);
//     fn open_tag(& mut self) -> XmlResult<Cow<'_, str>>;
//     fn open_tag_named(&mut self, name: &str) -> XmlResult<()>;
//     fn attr(&mut self) -> XmlResult<(Cow<'_, str>, Cow<'_, str>)>;
//     fn attr_named(&mut self, name: &str) -> XmlResult<Cow<'_, str>>;
//     fn skip_to_open(&mut self) -> XmlResult<()>;
//     fn my_find_attribute(&mut self) -> XmlResult<Option<(&'_ str, Cow<'_, str>)>>;
// }

pub struct ReaderExt<'a, 'b> {
    reader: &'a mut XmlReader<'b>
}

// impl<'a> std::convert::From<&mut XmlReader<'a>> for ReaderExt<'a> {
//     fn from(value: &mut XmlReader<'a>) -> Self {
//         Self { reader: value }
//     }
// }
use strong_xml::xmlparser::ElementEnd;
use strong_xml::xmlparser::Error;
// use strong_xml::xmlparser::Token;
// use strong_xml::xmlparser::Tokenizer;

impl<'a, 'b> ReaderExt<'a, 'b> {
    pub fn new(reader: &'a mut XmlReader<'b>) -> Self {
        Self { reader: reader }
    }

    pub fn as_mut(&mut self) -> &mut XmlReader<'b> {
        self.reader
    }

    pub fn next(&mut self) -> Option<Result<Token<'a>, Error>> {
        self.reader.next()
    }

    #[inline]
    pub fn peek(&mut self) -> Option<&Result<Token<'a>, Error>> {
        self.reader.peek()
    }

    pub fn until_end_tag(&mut self, name: &str) -> XmlResult<bool> {
        while let Some(token) = self.peek() {
            log::debug!("until end: {:?}", token);
            match token {
                Err(e) => { return Err(XmlError::Parser(*e)); }
                Ok(Token::ElementStart {..}) => { return Ok(true); },
                Ok(Token::ElementEnd { end: ElementEnd::Close(_, tag), .. }) => {
                    log::debug!("until end found end: {} == {}?", name, tag.as_str());
                    return if tag.as_str() == name {
                        Ok(false)
                    } else {
                        Err(XmlError::UnexpectedToken { token: format!("{:?}", token) })
                    }
                }
                _ => {self.reader.next();}
            }
        }

        Err(XmlError::UnexpectedEof)
    }

    pub fn open_tag(&mut self) -> XmlResult<&'b str> {
        if let Some(token) = self.reader.next() {
            match token {
                Err(e) => { return Err(XmlError::Parser(e)); }
                Ok(Token::ElementStart { span, .. }) => {
                    return Ok(&span.as_str()[1..]);
                },
                _ => return Err(XmlError::UnexpectedToken { token: format!("{:?}", token) })
            }
        }

        Err(XmlError::UnexpectedEof)
    }

    pub fn open_tag_named(&mut self, name: &str) -> XmlResult<()> {
        let tag = self.open_tag()?;
        if tag == name {
            Ok(())
        } else {
            Err(XmlError::UnexpectedToken { token: tag.to_string() })
        }
    }

    pub fn close_tag(&mut self) -> XmlResult<&'b str> {
        if let Some(token) = self.reader.next() {
            match token {
                Err(e) => { return Err(XmlError::Parser(e)); }
                Ok(Token::ElementEnd {
                    end: ElementEnd::Close(_, span),
                    ..
                }) => { log::debug!("Close tag: {:?}", span); return Ok(span.as_str()); },
                _ => return Err(XmlError::UnexpectedToken { token: format!("{:?}", token) })
            }
        }

        Err(XmlError::UnexpectedEof)
    }

    pub fn close_tag_named(&mut self, name: &str) -> XmlResult<()> {
        let tag = self.close_tag()?;
        log::debug!("Close tag named: {}", tag);
        if tag == name {
            log::debug!("Yup, that's we're looking for");
            Ok(())
        } else {
            log::debug!("{:?} is not equal to {:?}", tag, name);
            Err(XmlError::UnexpectedToken { token: tag.to_string() })
        }
    }

    pub fn attr(&mut self) -> XmlResult<(Cow<'b, str>, Cow<'b, str>)> {
        if let Some(token) = self.reader.next() {
            match token {
                Err(e) => { return Err(XmlError::Parser(e)); }
                 Ok(Token::Attribute { span, value, .. }) => {
                     let value = value.as_str();
                     let span = span.as_str(); // key="value"
                     let key = &span[0..span.len() - value.len() - 3]; // remove `="`, value and `"`
                     //self.next();
                     return Ok((Cow::Borrowed(key), Cow::Borrowed(value)));
                }
                _ => return Err(XmlError::UnexpectedToken { token: format!("{:?}", token) })
            }
        }

        Err(XmlError::UnexpectedEof)
    }

    pub fn attr_named(&mut self, name: &str) -> XmlResult<Cow<'b, str>> {
        let (k, v) = self.attr()?;
        if k == name {
            Ok(v)
        } else {
            Err(XmlError::UnexpectedToken { token: format!("{}=\"{}\"", k, v)})
        }
    }

    pub fn text(&mut self) -> XmlResult<Cow<'b, str>> {
        if let Some(token) = self.reader.next() {
            match token {
                Err(e) => { return Err(XmlError::Parser(e)); }
                Ok(Token::Text { text }) => {
                    return Ok(Cow::Borrowed(text.as_str()));
                }
                Ok(Token::Cdata { text, .. }) => {
                    return Ok(Cow::Borrowed(text.as_str()));
                }
                _ => { return Err(XmlError::UnexpectedToken { token: format!("{:?}", token) }); }
            }
        }

        Err(XmlError::UnexpectedEof)
    }

    pub fn skip_to_text(&mut self) -> XmlResult<()> {
        while let Some(token) = self.reader.peek() {
            match token {
                Err(e) => { return Err(XmlError::Parser(*e)); }
                Ok(Token::Text {..} | Token::Cdata {..}) => { return Ok(()); },
                _ => {self.reader.next();}
            }
        }

        Err(XmlError::UnexpectedEof)
    }

    pub fn skip_to_open(&mut self) -> XmlResult<()> {
        while let Some(token) = self.reader.peek() {
            match token {
                Err(e) => { return Err(XmlError::Parser(*e)); }
                Ok(Token::ElementStart { .. }) => {
                    return Ok(());
                },
                _ => {self.reader.next();}
            }
        }

        Err(XmlError::UnexpectedEof)
    }

    pub fn skip_to_close(&mut self) -> XmlResult<()> {
        while let Some(token) = self.reader.peek() {
            log::debug!("next {:?}", token);
            match token {
                Err(e) => { return Err(XmlError::Parser(*e)); }
                Ok(Token::ElementEnd { end: ElementEnd::Close(_, _), .. }) => {
                    return Ok(());
                },
                _ => {self.reader.next();}
            }
        }

        Err(XmlError::UnexpectedEof)
    }

    pub fn skip_over_close_empty(&mut self) -> XmlResult<()> {
        while let Some(token) = self.reader.next() {
            match token {
                Err(e) => { return Err(XmlError::Parser(e)); }
                Ok(Token::ElementEnd {
                    end: ElementEnd::Empty,
                    ..
                }) => { return Ok(()); },
                _ => ()
            }
        }

        Err(XmlError::UnexpectedEof)
    }

    pub fn skip_over_close(&mut self) -> XmlResult<()> {
        while let Some(token) = self.reader.next() {
            match token {
                Err(e) => { return Err(XmlError::Parser(e)); }
                Ok(Token::ElementEnd {
                    end: ElementEnd::Close(_, _),
                    ..
                }) => { return Ok(()); },
                _ => ()
            }
        }

        Err(XmlError::UnexpectedEof)
    }


    // pub fn my_find_attribute(&mut self) -> XmlResult<Option<(&'a str, Cow<'a, str>)>> {
    //     unimplemented!()
    //     // if let Some(token) = self.peek() {
    //     //     match token {
    //     //         Ok(Token::Attribute { span, value, .. }) => {
    //     //             let value = value.as_str();
    //     //             let span = span.as_str(); // key="value"
    //     //             let key = &span[0..span.len() - value.len() - 3]; // remove `="`, value and `"`
    //     //             let value = Cow::Borrowed(value);
    //     //             self.next();
    //     //             return Ok(Some((key, value)));
    //     //         }
    //     //         Ok(Token::ElementEnd {
    //     //             end: ElementEnd::Open,
    //     //             ..
    //     //         })
    //     //         | Ok(Token::ElementEnd {
    //     //             end: ElementEnd::Empty,
    //     //             ..
    //     //         }) => return Ok(None),
    //     //         Ok(token) => {
    //     //             return Err(XmlError::UnexpectedToken {
    //     //                 token: format!("{:?}", token),
    //     //             })
    //     //         }
    //     //         Err(_) => {
    //     //             // we have call .peek() above, and it's safe to use unwrap
    //     //             self.next().unwrap()?;
    //     //         }
    //     //     }
    //     // }

    //     // Err(XmlError::UnexpectedEof)
    // }

    // pub fn log_current(&mut self, s: &str) {
    //     log::debug!("{}: {:?}", s, self.reader.peek().unwrap().unwrap());
    // }

}
