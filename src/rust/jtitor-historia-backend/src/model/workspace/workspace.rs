/*!
 * Defines the Workspace type.
 */
use std::fs::File;
use std::io::BufWriter;

use serde::{Deserialize, Serialize};

use super::{WorkspaceFile, WorkspaceMetadata};
use crate::error::ConversionError;
use crate::io::{helpers, Export, Import};
use crate::model::Notebook;

/**
 * Contains one or more Notebook instances.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Workspace {
    notebooks: Vec<Notebook>,
    metadata: WorkspaceMetadata,
}

impl Import<Workspace> for Workspace {
    fn import(&self, source: &mut File) -> Result<Workspace, ConversionError> {
        //First, try to deserialize a WorkspaceFile from 'source'.
        let workspace_file = helpers::open_workspace_file(source)?;

        let mut notebooks: Vec<Notebook> = vec![];
        //For each path in the WorkspaceFile's path list:
        for path in workspace_file.notebook_paths {
            //  * Try to open the specified file.
            //      * If the file failed, mark it as a failed file and continue.
            //          * TODO: We need some way to deal with failed notebooks.
            //          * TODO: We should get the reason a file failed; as seen below,
            //          a file can open but fail to import.
            //      * Else, try to import the opened file as a Notebook.
            //          * If the import failed, mark it as a failed file and continue.
            //      * Add the new Notebook to a list of opened Notebooks.
            unimplemented!();
        }

        //Return the opened Notebooks and loaded WorkspaceMetadata.
        //TODO: this needs to return both the Workspace and its
        //warnings. Maybe move to a separate WorkspaceImportResult type?
        Ok(Workspace {
            notebooks: notebooks,
            metadata: workspace_file.metadata,
        })
    }
}

impl Export for Workspace {
    fn export(&self, destination: &mut File) -> Result<(), ConversionError> {
        //The metadata can be directly serialized.
        let metadata = self.metadata.clone();

        let mut notebook_paths: Vec<String> = vec![];
        for notebook in self.notebooks.iter() {
            //The notebooks are a little harder. For each notebook:
            //  * Determine its file path.
            //  * Open the notebook's file path.
            //  * Call Notebook.export(), using the newly opened file as destination.
            //  * Add the path to a list of successfully-written paths.
            //  * If any of the prior steps failed:
            //      * Add the path to a list of failed paths and continue.

            //TODO:
            //This indicates that we actually need a way to rollback changes;
            //if a write fails, we need to rollback all of the writes.
            //The simplest way is probably to move the original content to a temp,
            //save the new content in place, then delete the temps
            //when all writes are done.
            //Then, if a write fails, we can move the temps back to the original paths
            //and abort.
            unimplemented!();
        }

        //Write the output to the destination file.
        let output = WorkspaceFile {
            notebook_paths: notebook_paths,
            metadata: metadata,
        };

        let file_writer = BufWriter::new(destination);
        helpers::write_json(file_writer, &output)
    }
}
