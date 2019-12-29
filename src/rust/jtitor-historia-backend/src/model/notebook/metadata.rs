/*!
 * Defines the NotebookMetadata type.
 */
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotebookMetadata {
    pub name: String,
}