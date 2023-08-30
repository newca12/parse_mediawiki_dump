//! Defines a schema for elements from the wikipedia xml dump
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(from = "i32")]
/// Wikipedia namespace
///  see: <https://en.wikipedia.org/wiki/Wikipedia:Namespace>
pub enum Namespace {
    /// Can be used to link directly to a file, rather than to the file description page.
    Media,
    /// The Special: namespace consists of pages (called special pages) that are created by the
    /// software on demand, such as Special:RecentChanges
    Special,
    /// All encyclopedia articles, lists, disambiguation pages, and encyclopedia redirects.
    /// Sometimes referred to as "mainspace" or "Article".
    Main,
    /// Talk namespaces are used to discuss changes to pages
    Talk,
    /// Contains user pages and other pages created by individual users for their own personal use
    User,
    /// Used to leave messages for a particular user
    UserTalk,
    /// Consists of administration pages with information or discussion about Wikipedia
    Wikipedia,
    /// Talk page for Wikipedia pages
    WikipediaTalk,
    /// Administration pages in which all of Wikipedia's media content resides
    File,
    /// Talk page for File pages
    FileTalk,
    /// Contains text to be displayed in certain places in the interface
    MediaWiki,
    /// Talk page for MediaWiki page
    MediaWikiTalk,
    /// Used to store templates, which contain Wiki markup intended for inclusion on multiple pages
    Template,
    /// Talk page for template page
    TemplateTalk,
    /// Consists of "how-to" and information pages whose titles begin with the prefix Help:, such as Help:Link.
    Help,
    /// Talk page for help pages
    HelpTalk,
    /// Categories are normally found at the bottom of an article page. Clicking a category name
    /// brings up a category page listing the articles (or other pages) that have been added to
    /// that particular category
    Category,
    /// Talk page for category
    CategoryTalk,
    /// Portals serve as enhanced "main pages" for specific broad subjects.
    Portal,
    /// Talk page for portal
    PortalTalk,
    /// Drafts are pages in the Draft namespace where new articles may be created and developed,
    /// for a limited period of time.
    Draft,
    /// Talk page for draft pages
    DraftTalk,
    /// The TimedMediaHandler extension allows you to display audio and video files in wiki pages
    TimedText,
    /// Talk page for timed text
    TimedTextTalk,
    /// Where Wikipedia Lua source code is stored
    Module,
    /// Talk page for Module
    ModuleTalk,
    /// Depreciated by Wikipedia
    /// A JavaScript program and/or a CSS snippet that can be enabled simply by checking an option
    /// in a Wikipedia user's preferences
    Gadget,
    /// Depreciated by Wikipedia
    /// Talk page for gadgets
    GadgetTalk,
    /// Depreciated by Wikipedia
    /// Definition of gadgets
    GadgetDefinition,
    /// Depreciated by Wikipedia
    /// Talk page for gadgets
    GadgetDefinitionTalk,
    /// Unknown namespace code
    Unknown,
}

impl Namespace {
    pub fn to_int(&self) -> i32 {
        match self {
            Namespace::Media => -2,
            Namespace::Special => -1,
            Namespace::Main => 0,
            Namespace::Talk => 1,
            Namespace::User => 2,
            Namespace::UserTalk => 3,
            Namespace::Wikipedia => 4,
            Namespace::WikipediaTalk => 5,
            Namespace::File => 6,
            Namespace::FileTalk => 7,
            Namespace::MediaWiki => 8,
            Namespace::MediaWikiTalk => 9,
            Namespace::Template => 10,
            Namespace::TemplateTalk => 11,
            Namespace::Help => 12,
            Namespace::HelpTalk => 13,
            Namespace::Category => 14,
            Namespace::CategoryTalk => 15,
            Namespace::Portal => 100,
            Namespace::PortalTalk => 101,
            Namespace::Draft => 118,
            Namespace::DraftTalk => 119,
            Namespace::TimedText => 710,
            Namespace::TimedTextTalk => 711,
            Namespace::Module => 828,
            Namespace::ModuleTalk => 829,
            Namespace::Gadget => 2300,
            Namespace::GadgetTalk => 2301,
            Namespace::GadgetDefinition => 2302,
            Namespace::GadgetDefinitionTalk => 2303,
            Namespace::Unknown => -999,
        }
    }
}

impl From<i32> for Namespace {
    fn from(id: i32) -> Self {
        match id {
            -2 => Namespace::Media,
            -1 => Namespace::Special,
            0 => Namespace::Main,
            1 => Namespace::Talk,
            2 => Namespace::User,
            3 => Namespace::UserTalk,
            4 => Namespace::Wikipedia,
            5 => Namespace::WikipediaTalk,
            6 => Namespace::File,
            7 => Namespace::FileTalk,
            8 => Namespace::MediaWiki,
            9 => Namespace::MediaWikiTalk,
            10 => Namespace::Template,
            11 => Namespace::TemplateTalk,
            12 => Namespace::Help,
            13 => Namespace::HelpTalk,
            14 => Namespace::Category,
            15 => Namespace::CategoryTalk,
            100 => Namespace::Portal,
            101 => Namespace::PortalTalk,
            118 => Namespace::Draft,
            119 => Namespace::DraftTalk,
            710 => Namespace::TimedText,
            711 => Namespace::TimedTextTalk,
            828 => Namespace::Module,
            829 => Namespace::ModuleTalk,
            2300 => Namespace::Gadget,
            2301 => Namespace::GadgetTalk,
            2302 => Namespace::GadgetDefinition,
            2303 => Namespace::GadgetDefinitionTalk,
            _ => Namespace::Unknown,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Redirect {
    #[serde(rename = "@title")]
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct Page {
    pub title: String,
    pub ns: Namespace,
    pub id: u32,
    #[serde(rename = "revision")]
    pub revisions: Vec<Revision>,
    pub redirect: Option<()>,
}

#[derive(Debug, Deserialize)]
pub struct Contributer {
    pub username: Option<String>,
    pub ip: Option<String>,
    pub id: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Minor;

#[derive(Debug, Deserialize)]
pub struct Text {
    #[serde(rename = "@bytes")]
    pub bytes: i32,
    #[serde(rename = "$value")]
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Revision {
    pub id: u32,
    pub parentid: Option<u32>,
    pub timestamp: String,
    pub contributor: Contributer,
    pub minor: Option<Minor>,
    pub comment: Option<String>,
    pub model: String,
    pub format: String,
    pub sha1: String,
    pub text: Text,
}
