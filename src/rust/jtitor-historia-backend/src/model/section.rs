/*!
 * Defines the Section type.
 */
use serde::{Serialize, Deserialize};

/**
 * Represents a region of notes with a consistent
 * format.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
    content: String,
    format: SectionFormat,
    //TODO: Should this have metadata like a
    //section title?
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SectionFormat {
    ///Plain Unicode text.
    RawText,
    ///Markdown text data.
    Markdown,
    ///LaTeX text data.
    Latex,
}