/*!
 * Defines the Section type.
 */

/**
 * Represents a region of notes with a consistent
 * format.
 */
pub struct Section {
    content: String,
    format: SectionFormat,
    //TODO: Should this have metadata like a
    //section title?
}

pub enum SectionFormat {
    ///Plain Unicode text.
    RawText,
    ///Markdown text data.
    Markdown,
    ///LaTeX text data.
    Latex,
}