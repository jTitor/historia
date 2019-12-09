/*!
 * Defines the Notebook type.
 */
use crate::model::Note;

pub struct Notebook {
    notes: Vec<Note>,
    metadata: NotebookMetadata,
}

pub struct NotebookMetadata {
    name: String,
}