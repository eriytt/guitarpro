use std::borrow::Cow;
use log;

use strong_xml::{XmlReader, XmlResult, XmlError};
use strong_xml::xmlparser::Token;
use strong_xml::utils::xml_unescape;

pub struct ReaderExt<'a, 'b> {
    reader: &'a mut XmlReader<'b>
}
use strong_xml::xmlparser::ElementEnd;
use strong_xml::xmlparser::Error;

impl<'a, 'b> ReaderExt<'a, 'b> {
    pub fn new(reader: &'a mut XmlReader<'b>) -> Self {
        Self { reader }
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
            match token {
                Err(e) => { return Err(XmlError::Parser(*e)); }
                Ok(Token::ElementStart {..}) => { return Ok(true); },
                Ok(Token::ElementEnd { end: ElementEnd::Close(_, tag), .. }) => {
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
                }) => { return Ok(span.as_str()); },
                _ => return Err(XmlError::UnexpectedToken { token: format!("{:?}", token) })
            }
        }

        Err(XmlError::UnexpectedEof)
    }

    pub fn close_tag_named(&mut self, name: &str) -> XmlResult<()> {
        let tag = self.close_tag()?;
        if tag == name {
            Ok(())
        } else {
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

    pub fn simple_tag_text(&mut self, tag: &str) -> XmlResult<Cow<'b, str>> {
        self.skip_to_open()?;
        let t = self.open_tag_named(tag)?;
        self.skip_to_text()?;
        let t = self.text()?;
        self.close_tag_named(tag)?;
        Ok(t)
    }

    pub fn empty_tag(&mut self) -> XmlResult<Cow<'b, str>> {
        self.skip_to_open()?;
        let t = self.open_tag()?;
        self.skip_over_close_empty()?;
        Ok(Cow::Borrowed(t))
    }

}
