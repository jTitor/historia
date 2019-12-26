/*!
 * Defines the Workspace type.
 */
use std::fs::File;

use failure::Error;
use serde::{Deserialize, Serialize};

use super::WorkspaceMetadata;
use crate::io::{Export, Import};
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
    fn import(&self, source: &mut File) -> Result<Workspace, Error> {
        //First, try to deserialize a WorkspaceFile from 'source'.

        //For each path in the WorkspaceFile's path list:
        //  * Try to open the specified file.
        //      * If the file failed, mark it as a failed file and continue.
        //          * TODO: We need some way to deal with failed notebooks.
        //          * TODO: We should get the reason a file failed; as seen below,
        //          a file can open but fail to import.
        //      * Else, try to import the opened file as a Notebook.
        //          * If the import failed, mark it as a failed file and continue.
        //      * Add the new Notebook to a list of opened Notebooks.

        //Return the opened Notebooks and loaded WorkspaceMetadata.

        unimplemented!()
    }
}

impl Export for Workspace {
    fn export(&self, destination: &mut File) -> Result<(), Error> {
        //The metadata can be directly serialized.

        //The notebooks are a little harder. For each notebook:
        //  * Determine its file path.
        //  * Open the notebook's file path.
        //  * Call Notebook.export(), using the newly opened file as destination.
        //  * Add the path to a list of successfully-written paths.
        //  * If any of the prior steps failed:
        //      * Add the path to a list of failed paths and continue.

        //Report success.

        unimplemented!()
    }
}
