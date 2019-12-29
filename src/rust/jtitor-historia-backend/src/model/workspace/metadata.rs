/*!
 * Defines the WorkspaceMetadata type.
 */
use serde::{Serialize, Deserialize};

/**
 * TODO
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorkspaceMetadata {
    pub name: String,
    //TODO: version number?
}