/*!
 * Defines the WorkspaceFile type.
 */
use std::fs::File;

use failure::Error;
use serde::{Serialize, Deserialize};

use super::WorkspaceMetadata;

/**
 * TODO
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceFile {
    pub notebook_paths: Vec<String>,
    pub metadata: WorkspaceMetadata,
}

impl WorkspaceFile {
    pub fn from_file(file: &File) -> Result<WorkspaceFile, Error> {
        //Should be deserializable from the given file.
        unimplemented!()
    }
}