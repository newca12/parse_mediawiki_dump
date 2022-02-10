// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

extern crate bzip2;
extern crate parse_mediawiki_dump;

fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        eprintln!("invalid use");
        std::process::exit(1);
    }
    let path = args.nth(1).unwrap();
    let file = match std::fs::File::open(&path) {
        Err(error) => {
            eprintln!("Failed to open input file: {}", error);
            std::process::exit(1);
        }
        Ok(file) => std::io::BufReader::new(file),
    };
    if path.ends_with(".bz2") {
        parse(std::io::BufReader::new(bzip2::bufread::BzDecoder::new(
            file,
        )));
    } else {
        parse(file);
    }
}

fn parse(source: impl std::io::BufRead) {
    for result in parse_mediawiki_dump::parse(source) {
        match result {
            Err(error) => {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
            Ok(page) => eprintln!("{:#?}", page),
        }
    }
}
