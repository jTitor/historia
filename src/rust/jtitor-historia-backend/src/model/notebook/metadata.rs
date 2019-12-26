/*!
 * Defines the NotebookMetadata type.
 */
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NotebookMetadata {
    name: String,
}