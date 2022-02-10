// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

//! Parse XML dumps exported from Mediawiki.
//!
//! This module parses [XML dumps](https://www.mediawiki.org/wiki/Help:Export) exported from Mediawiki, providing each page from the dump through an iterator. This is useful for parsing the [dumps from Wikipedia and other Wikimedia projects](https://dumps.wikimedia.org).
//!
//! # Caution
//!
//! If you need to parse any wiki text extracted from a dump, please use the crate Parse Wiki Text ([crates.io](https://crates.io/crates/parse_wiki_text), [Github](https://github.com/portstrom/parse_wiki_text)). Correctly parsing wiki text requires dealing with an astonishing amount of difficult and counterintuitive cases. Parse Wiki Text automatically deals with all these cases, giving you an unambiguous tree of parsed elements that is easy to work with.
//!
//! # Limitations
//!
//! This module only parses dumps containing only one revision of each page. This is what you get from the page `Special:Export` when enabling the option “Include only the current revision, not the full history”, as well as what you get from the Wikimedia dumps with file names ending with `-pages-articles.xml.bz2`.
//!
//! This module ignores the `siteinfo` element, every child element of the `page` element except `ns`, `revision` and `title`, and every element inside the `revision` element except `format`, `model` and `text`.
//!
//! Until there is a real use case that justifies going beyond these limitations, they will remain in order to avoid premature design driven by imagined requirements.
//!
//! # Examples
//!
//! Parse a bzip2 compressed file and distinguish ordinary articles from other pages. A running example with complete error handling is available in the `examples` folder.
//!
//! ```rust,no_run
//! extern crate bzip2;
//! extern crate parse_mediawiki_dump;
//!
//! fn main() {
//!     let file = std::fs::File::open("example.xml.bz2").unwrap();
//!     let file = std::io::BufReader::new(file);
//!     let file = bzip2::bufread::BzDecoder::new(file);
//!     let file = std::io::BufReader::new(file);
//!     for result in parse_mediawiki_dump::parse(file) {
//!         match result {
//!             Err(error) => {
//!                 eprintln!("Error: {}", error);
//!                 break;
//!             }
//!             Ok(page) => if page.namespace == 0 && match &page.format {
//!                 None => false,
//!                 Some(format) => format == "text/x-wiki"
//!             } && match &page.model {
//!                 None => false,
//!                 Some(model) => model == "wikitext"
//!             } {
//!                 println!(
//!                     "The page {title:?} is an ordinary article with byte length {length}.",
//!                     title = page.title,
//!                     length = page.text.len()
//!                 );
//!             } else {
//!                 println!("The page {:?} has something special to it.", page.title);
//!             }
//!         }
//!     }
//! }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate quick_xml;

use quick_xml::{events::Event, Reader};
use std::io::BufRead;

enum PageChildElement {
    Ns,
    Revision,
    Title,
    Unknown,
}

enum RevisionChildElement {
    Format,
    Model,
    Text,
    Unknown,
}

#[derive(Debug)]
/// The error type for `Parser`.
pub enum Error {
    /// Format not matching expectations.
    ///
    /// Indicates the position in the stream.
    Format(usize),

    /// The source contains a feature not supported by the parser.
    ///
    /// In particular, this means a `page` element contains more than one `revision` element.
    NotSupported(usize),

    /// Error from the XML reader.
    XmlReader(quick_xml::Error),
}

/// Parsed page.
///
/// Parsed from the `page` element.
///
/// Although the `format` and `model` elements are defined as mandatory in the [schema](https://www.mediawiki.org/xml/export-0.10.xsd), previous versions of the schema don't contain them. Therefore the corresponding fields can be `None`.
#[derive(Debug)]
pub struct Page {
    /// The format of the revision if any.
    ///
    /// Parsed from the text content of the `format` element in the `revision` element. `None` if the element is not present.
    ///
    /// For ordinary articles the format is `text/x-wiki`.
    pub format: Option<String>,

    /// The model of the revision if any.
    ///
    /// Parsed from the text content of the `model` element in the `revision` element. `None` if the element is not present.
    ///
    /// For ordinary articles the model is `wikitext`.
    pub model: Option<String>,

    /// The namespace of the page.
    ///
    /// Parsed from the text content of the `ns` element in the `page` element.
    ///
    /// For ordinary articles the namespace is 0.
    pub namespace: u32,

    /// The text of the revision.
    ///
    /// Parsed from the text content of the `text` element in the `revision` element.
    pub text: String,

    /// The title of the page.
    ///
    /// Parsed from the text content of the `title` element in the `page` element.
    pub title: String,
}

/// Parser working as an iterator over pages.
pub struct Parser<R: BufRead> {
    buffer: Vec<u8>,
    namespace_buffer: Vec<u8>,
    reader: Reader<R>,
    started: bool,
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Format(position) => write!(formatter, "Invalid format at position {}", position),
            Error::NotSupported(position) => write!(
                formatter,
                "The element at position {} is not supported",
                position
            ),
            Error::XmlReader(error) => error.fmt(formatter),
        }
    }
}

impl From<quick_xml::Error> for Error {
    fn from(value: quick_xml::Error) -> Self {
        Error::XmlReader(value)
    }
}

impl<R: BufRead> Iterator for Parser<R> {
    type Item = Result<Page, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match next(self) {
            Err(error) => Err(error),
            Ok(item) => Ok(item?),
        })
    }
}

fn match_namespace(namespace: Option<&[u8]>) -> bool {
    match namespace {
        None => false,
        Some(namespace) => namespace == b"http://www.mediawiki.org/xml/export-0.10/" as &[u8],
    }
}

fn next(parser: &mut Parser<impl BufRead>) -> Result<Option<Page>, Error> {
    if !parser.started {
        loop {
            parser.buffer.clear();
            if let (namespace, Event::Start(event)) = parser
                .reader
                .read_namespaced_event(&mut parser.buffer, &mut parser.namespace_buffer)?
            {
                if match_namespace(namespace) && event.local_name() == b"mediawiki" {
                    break;
                }
                return Err(Error::Format(parser.reader.buffer_position()));
            }
        }
        parser.started = true;
    }
    loop {
        parser.buffer.clear();
        if !match parser
            .reader
            .read_namespaced_event(&mut parser.buffer, &mut parser.namespace_buffer)?
        {
            (_, Event::End(_)) => return Ok(None),
            (namespace, Event::Start(event)) => {
                match_namespace(namespace) && event.local_name() == b"page"
            }
            _ => continue,
        } {
            skip_element(parser)?;
            continue;
        }
        let mut format = None;
        let mut model = None;
        let mut namespace = None;
        let mut text = None;
        let mut title = None;
        loop {
            parser.buffer.clear();
            match match parser
                .reader
                .read_namespaced_event(&mut parser.buffer, &mut parser.namespace_buffer)?
            {
                (_, Event::End(_)) => {
                    return match (namespace, text, title) {
                        (Some(namespace), Some(text), Some(title)) => Ok(Some(Page {
                            format,
                            model,
                            namespace,
                            text,
                            title,
                        })),
                        _ => Err(Error::Format(parser.reader.buffer_position())),
                    }
                }
                (namespace, Event::Start(event)) => {
                    if match_namespace(namespace) {
                        match event.local_name() {
                            b"ns" => PageChildElement::Ns,
                            b"revision" => PageChildElement::Revision,
                            b"title" => PageChildElement::Title,
                            _ => PageChildElement::Unknown,
                        }
                    } else {
                        PageChildElement::Unknown
                    }
                }
                _ => continue,
            } {
                PageChildElement::Ns => match parse_text(parser, &namespace)?.parse() {
                    Err(_) => return Err(Error::Format(parser.reader.buffer_position())),
                    Ok(value) => {
                        namespace = Some(value);
                        continue;
                    }
                },
                PageChildElement::Revision => {
                    if text.is_some() {
                        return Err(Error::NotSupported(parser.reader.buffer_position()));
                    }
                    loop {
                        parser.buffer.clear();
                        match match parser.reader.read_namespaced_event(
                            &mut parser.buffer,
                            &mut parser.namespace_buffer,
                        )? {
                            (_, Event::End(_)) => match text {
                                None => return Err(Error::Format(parser.reader.buffer_position())),
                                Some(_) => break,
                            },
                            (namespace, Event::Start(event)) => if match_namespace(namespace) {
                                match event.local_name() {
                                    b"format" => RevisionChildElement::Format,
                                    b"model" => RevisionChildElement::Model,
                                    b"text" => RevisionChildElement::Text,
                                    _ => RevisionChildElement::Unknown,
                                }
                            } else {
                                RevisionChildElement::Unknown
                            },
                            _ => continue,
                        } {
                            RevisionChildElement::Format => {
                                format = Some(parse_text(parser, &format)?)
                            }
                            RevisionChildElement::Model => {
                                model = Some(parse_text(parser, &model)?)
                            }
                            RevisionChildElement::Text => text = Some(parse_text(parser, &text)?),
                            RevisionChildElement::Unknown => skip_element(parser)?,
                        }
                    }
                    continue;
                }
                PageChildElement::Title => {
                    title = Some(parse_text(parser, &title)?);
                    continue;
                }
                PageChildElement::Unknown => skip_element(parser)?,
            }
        }
    }
}

/// Creates a parser for a stream.
///
/// The stream is parsed as an XML dump exported from Mediawiki. The parser is an iterator over the pages in the dump.
pub fn parse<R: BufRead>(source: R) -> Parser<R> {
    let mut reader = Reader::from_reader(source);
    reader.expand_empty_elements(true);
    Parser {
        buffer: vec![],
        namespace_buffer: vec![],
        reader,
        started: false,
    }
}

fn parse_text(
    parser: &mut Parser<impl BufRead>,
    output: &Option<impl Sized>,
) -> Result<String, Error> {
    if output.is_some() {
        return Err(Error::Format(parser.reader.buffer_position()));
    }
    parser.buffer.clear();
    let text = match parser
        .reader
        .read_namespaced_event(&mut parser.buffer, &mut parser.namespace_buffer)?
        .1
    {
        Event::Text(text) => text.unescape_and_decode(&parser.reader)?,
        Event::End { .. } => return Ok(String::new()),
        _ => return Err(Error::Format(parser.reader.buffer_position())),
    };
    parser.buffer.clear();
    if let Event::End(_) = parser
        .reader
        .read_namespaced_event(&mut parser.buffer, &mut parser.namespace_buffer)?
        .1
    {
        Ok(text)
    } else {
        Err(Error::Format(parser.reader.buffer_position()))
    }
}

fn skip_element(parser: &mut Parser<impl BufRead>) -> Result<(), quick_xml::Error> {
    let mut level = 0;
    loop {
        parser.buffer.clear();
        match parser
            .reader
            .read_namespaced_event(&mut parser.buffer, &mut parser.namespace_buffer)?
            .1
        {
            Event::End(_) => {
                if level == 0 {
                    return Ok(());
                }
                level -= 1;
            }
            Event::Start(_) => level += 1,
            _ => {}
        }
    }
}
