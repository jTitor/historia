/*!
 * Defines the Notebook type.
 */
use std::fs::File;

use failure::Error;
use serde::{Deserialize, Serialize};

use super::NotebookMetadata;
use crate::io::{Export, Import};
use crate::model::Note;

#[derive(Serialize, Deserialize, Debug)]
pub struct Notebook {
    notes: Vec<Note>,
    metadata: NotebookMetadata,
}

impl Import<Notebook> for Notebook {
    fn import(&self, source: &mut File) -> Result<Notebook, Error> {
        //First, try to deserialize a NotebookFile from 'source'.

        //For each path in the NotebookFile's path list:
        //  * Try to open the specified file.
        //      * If the file failed, mark it as a failed file and continue.
        //          * TODO: as with Workspace.import(),
        //          we need a way to deal with failed note imports.
        //      * Else, try to import the opened file as a Note.
        //          * If the import failed, mark it as a failed file and continue.
        //      * Add the new Note to a list of opened Notes.

        //Return the opened Notes and loaded NotebookMetadata.

        unimplemented!()
    }
}

impl Export for Notebook {
    fn export(&self, destination: &mut File) -> Result<(), Error> {
        //The metadata can be directly serialized.

        //The notes are a little harder. For each note:
        //  * Determine its file path.
        //  * Open the note's file path.
        //  * Call Note.export(), using the newly opened file as destination.
        //  * Add the path to a list of successfully-written paths.
        //  * If any of the prior steps failed:
        //      * Add the path to a list of failed paths and continue.

        //Report success.

        unimplemented!()
    }
}