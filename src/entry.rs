use crate::{EntryId, FixedDateTime};

#[derive(Debug, Eq, PartialEq)]
pub struct Entry {
    pub author_name: String,
    pub categories: Vec<String>,
    pub content: String,
    pub draft: bool,
    pub edit_url: String,
    pub edited: FixedDateTime,
    pub id: EntryId,
    pub published: FixedDateTime,
    pub title: String,
    pub updated: FixedDateTime,
    pub url: String,
}
