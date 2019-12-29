/*!
 * Defines the NotebookFile type.
 */
use std::fs::File;

use failure::Error;
use serde::{Serialize, Deserialize};

use super::NotebookMetadata;

#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookFile {
    pub note_paths: Vec<String>,
    pub metadata: NotebookMetadata,
}

impl NotebookFile {
    pub fn from_file(file: &File) -> Result<NotebookFile, Error> {
        //Should be deserializable from the given file.
        unimplemented!()
    }
}