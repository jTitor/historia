/*!
 * Defines the NotebookFile type.
 */
use serde::{Serialize, Deserialize};

use super::NotebookMetadata;

#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookFile {
    note_paths: Vec<String>,
    metadata: NotebookMetadata,
}