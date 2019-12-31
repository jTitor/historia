/*!
 * Defines the Section type.
 */
use serde::{Serialize, Deserialize};

/**
 * Represents a region of notes with a consistent
 * format.
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Section {
    content: String,
    format: SectionFormat,
    //TODO: Should this have metadata like a
    //section title?
}

/**
 * TODO
 */
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum SectionFormat {
    ///Plain Unicode text.
    RawText,
    ///Markdown text data.
    Markdown,
    ///LaTeX text data.
    Latex,
}