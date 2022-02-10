<!--
Copyright 2018 Fredrik Portström <https://portstrom.com>
This is free software distributed under the terms specified in
the file LICENSE at the top-level directory of this distribution.
-->

# Parse Mediawiki dump

Parse XML dumps exported from Mediawiki.

![Parse Wiki Text](https://portstrom.com/parse_wiki_text.svg)

This module parses [XML dumps](https://www.mediawiki.org/wiki/Help:Export) exported from Mediawiki, providing each page from the dump through an iterator. This is useful for parsing the [dumps from Wikipedia and other Wikimedia projects](https://dumps.wikimedia.org).

# Caution

If you need to parse any wiki text extracted from a dump, please use the crate Parse Wiki Text ([crates.io](https://crates.io/crates/parse_wiki_text), [Github](https://github.com/portstrom/parse_wiki_text)). Correctly parsing wiki text requires dealing with an astonishing amount of difficult and counterintuitive cases. Parse Wiki Text automatically deals with all these cases, giving you an unambiguous tree of parsed elements that is easy to work with.

# Limitations

This module only parses dumps containing only one revision of each page. This is what you get from the page `Special:Export` when enabling the option “Include only the current revision, not the full history”, as well as what you get from the Wikimedia dumps with file names ending with `-pages-articles.xml.bz2`.

This module ignores the `siteinfo` element, every child element of the `page` element except `ns`, `revision` and `title`, and every element inside the `revision` element except `format`, `model` and `text`.

Until there is a real use case that justifies going beyond these limitations, they will remain in order to avoid premature design driven by imagined requirements.

# Examples

Parse a bzip2 compressed file and distinguish ordinary articles from other pages. A running example with complete error handling is available in the `examples` folder.

```rust
extern crate bzip2;
extern crate parse_mediawiki_dump;

fn main() {
    let file = std::fs::File::open("example.xml.bz2").unwrap();
    let file = std::io::BufReader::new(file);
    let file = bzip2::bufread::BzDecoder::new(file);
    let file = std::io::BufReader::new(file);
    for result in parse_mediawiki_dump::parse(file) {
        match result {
            Err(error) => {
                eprintln!("Error: {}", error);
                break;
            }
            Ok(page) => if page.namespace == 0 && match &page.format {
                None => false,
                Some(format) => format == "text/x-wiki"
            } && match &page.model {
                None => false,
                Some(model) => model == "wikitext"
            } {
                println!(
                    "The page {title:?} is an ordinary article with byte length {length}.",
                    title = page.title,
                    length = page.text.len()
                );
            } else {
                println!("The page {:?} has something special to it.", page.title);
            }
        }
    }
}
```
