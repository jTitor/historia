/*!
 * Defines the Workspace type.
 */
use serde::{Serialize, Deserialize};

use crate::model::Notebook;

/**
 * Contains one or more Notebook instances.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    notebooks: Vec<Notebook>,
    metadata: WorkspaceMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceMetadata {
    name: String,
}