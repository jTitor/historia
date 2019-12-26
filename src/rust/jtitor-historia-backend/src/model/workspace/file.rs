/*!
 * Defines the WorkspaceFile type.
 */
use serde::{Serialize, Deserialize};

use super::WorkspaceMetadata;

/**
 * TODO
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceFile {
    notebook_paths: Vec<String>,
    metadata: WorkspaceMetadata,
}