/*!
 * Defines the WorkspaceImporter type.
 */
use std::fs::File;
use std::io::BufReader;
use failure::Error;
use serde_json;

use crate::model::Workspace;

/**
 * Opens a fs::File and returns a Workspace.
 */
pub struct WorkspaceImporter {}

impl WorkspaceImporter {
    pub fn import(file: &File) -> Result<Workspace, Error> {
        let file_reader = BufReader::new(file);
        
        let read_result: serde_json::Result<Workspace> = serde_json::from_reader(file_reader);

        Ok(read_result?)
    }
}