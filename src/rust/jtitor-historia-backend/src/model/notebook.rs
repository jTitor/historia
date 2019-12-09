/*!
 * Defines the Notebook type.
 */
use serde::{Serialize, Deserialize};

use crate::model::Note;

#[derive(Serialize, Deserialize, Debug)]
pub struct Notebook {
    notes: Vec<Note>,
    metadata: NotebookMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookMetadata {
    name: String,
    //TODO: version number?
}