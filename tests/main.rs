// Copyright 2018 Fredrik Portström <https://portstrom.com>
// Copyright (c) 2023 Olivier ROLAND
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

use parse_mediawiki_dump_reboot::schema::Namespace;

extern crate parse_mediawiki_dump_reboot;

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
        parse_mediawiki_dump_reboot::parse(std::io::BufReader::new(std::io::Cursor::new(DUMP)));
    assert!(match parser.next() {
        Some(Ok(parse_mediawiki_dump_reboot::Page {
            format: Some(format),
            model: Some(model),
            namespace: Namespace::Main,
            text,
            title,
        })) => format == "beta" && model == "gamma" && text == "delta" && title == "alpha",
        _ => false,
    });
    assert!(match parser.next() {
        Some(Ok(parse_mediawiki_dump_reboot::Page {
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
