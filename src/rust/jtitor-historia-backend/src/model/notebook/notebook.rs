/*!
 * Defines the Notebook type.
 */
use std::fs::File;
use std::io::BufWriter;

use failure::Error;
use serde::{Deserialize, Serialize};

use super::{NotebookFile, NotebookMetadata};
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
        let notebook_file = NotebookFile::from_file(source)?;

        let mut notes: Vec<Note> = vec![];
        //For each path in the NotebookFile's path list:
        for path in notebook_file.note_paths {
            //  * Try to open the specified file.
            //      * If the file failed, mark it as a failed file and continue.
            //          * TODO: as with Workspace.import(),
            //          we need a way to deal with failed note imports.
            //      * Else, try to import the opened file as a Note.
            //          * If the import failed, mark it as a failed file and continue.
            //      * Add the new Note to a list of opened Notes.
            unimplemented!();
        }

        //Return the opened Notes and loaded NotebookMetadata.
        Ok(Notebook {
            notes: notes,
            metadata: notebook_file.metadata,
        })
    }
}

impl Export for Notebook {
    fn export(&self, destination: &mut File) -> Result<(), Error> {
        //The metadata can be directly serialized.
        let metadata = self.metadata.clone();

        let mut note_paths: Vec<String> = vec![];
        for note in self.notes.iter() {
            //The notes are a little harder. For each note:
            //  * Determine its file path.
            //  * Open the note's file path.
            //  * Call Note.export(), using the newly opened file as destination.
            //  * Add the path to a list of successfully-written paths.
            //  * If any of the prior steps failed:
            //      * Add the path to a list of failed paths and continue.
            unimplemented!();
        }
        
        //Write the output to the destination file.
        let output = NotebookFile {
            note_paths: note_paths,
            metadata: metadata,
        };

        let file_writer = BufWriter::new(destination);
        serde_json::to_writer(file_writer, output)
    }
}
