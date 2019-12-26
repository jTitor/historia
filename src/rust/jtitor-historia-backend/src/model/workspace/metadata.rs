/*!
 * Defines the WorkspaceMetadata type.
 */
use serde::{Serialize, Deserialize};

/**
 * TODO
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceMetadata {
    name: String,
    //TODO: version number?
}