// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

use parse_mediawiki_dump::schema::Namespace;

extern crate parse_mediawiki_dump;

const DUMP: &str = concat!(
    r#"<mediawiki xmlns="http://www.mediawiki.org/xml/export-0.10/">"#,
    "<page>",
    "<ns>0</ns>",
    "<title>alpha</title>",
    "<revision>",
    "<format>beta</format>",
    "<model>gamma</model>",
    "<text>delta</text>",
    "</revision>",
    "</page>",
    "<page>",
    "<ns>4</ns>",
    "<title>epsilon</title>",
    "<revision>",
    "<text>zeta</text>",
    "</revision>",
    "</page>",
    "</mediawiki>"
);

#[test]
fn main() {
    let mut parser =
        parse_mediawiki_dump::parse(std::io::BufReader::new(std::io::Cursor::new(DUMP)));
    assert!(match parser.next() {
        Some(Ok(parse_mediawiki_dump::Page {
            format: Some(format),
            model: Some(model),
            namespace: Namespace::Main,
            text,
            title,
        })) => format == "beta" && model == "gamma" && text == "delta" && title == "alpha",
        _ => false,
    });
    assert!(match parser.next() {
        Some(Ok(parse_mediawiki_dump::Page {
            format: None,
            model: None,
            namespace: Namespace::Wikipedia,
            text,
            title,
        })) => text == "zeta" && title == "epsilon",
        _ => false,
    });
    assert!(parser.next().is_none());
}
