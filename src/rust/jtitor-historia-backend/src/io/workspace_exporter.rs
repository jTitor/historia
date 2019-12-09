/*!
 * Defines the WorkspaceExporter type.
*/
use std::fs::File;
use std::io::BufWriter;
use failure::Error;
use serde_json;

use crate::model::Workspace;

pub struct WorkspaceExporter {}

impl WorkspaceExporter {
    /**
     * Writes the given workspace to a new
     * workspace file.
     */
    pub fn export(&self, workspace: &Workspace, file: &mut File) -> Result<(), Error> {
        let file_writer = BufWriter::new(file);
        
        let _serde_writer_result = serde_json::to_writer(file_writer, workspace)?;

        Ok(())
    }
}
