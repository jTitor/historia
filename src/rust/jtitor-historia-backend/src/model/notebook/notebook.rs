/*!
 * Defines the Notebook type.
 */
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};

use failure::Error;

use super::{NotebookFile, NotebookMetadata};
use crate::error::ConversionError;
use crate::io::{helpers, Export};
use crate::model::{new_workspace_node, Note, WeakWorkspaceNode, Workspace, WorkspaceNode};

#[derive(Debug)]
pub struct Notebook {
    notes: Vec<WorkspaceNode<Note>>,
    metadata: NotebookMetadata,
    parent: WeakWorkspaceNode<Workspace>,
}

impl Notebook {
    /**
     * Writes this notebook's notes to disk,
     * rolling back all changes if any note fails to write.
     */
    fn write_notes_or_rollback(&self) -> Result<Vec<String>, ConversionError> {
        let mut note_paths_written: Vec<String> = vec![];

        for note in self.notes.iter() {
            //The notes are a little harder. For each note:
            //  * Determine its file path.
            //  * Open the note's file path.
            //  * Call Note.export(), using the newly opened file as destination.
            //  * Add the path to a list of successfully-written paths.
            //  * If any of the prior steps failed:
            //      * Add the path to a list of failed paths and continue.

            //TODO: Instead of immediately writing to destination files,
            //may want to generate changes here and then perform
            //a rollback-able commit. See notes/writing-fs-changes.md.
            unimplemented!();
        }

        Ok(note_paths_written)
    }

    pub fn import<P: AsRef<Path>>(
        path: P,
        parent: WeakWorkspaceNode<Workspace>,
    ) -> Result<WorkspaceNode<Notebook>, ConversionError> {
        let path_string: String = path.as_ref().to_string_lossy().to_owned().into();
        let source = helpers::open_file_for_read(path)?;
        //First, try to deserialize a NotebookFile from 'source'.
        let notebook_file = helpers::open_notebook_file(&source)?;
        let path_buf = PathBuf::from(path_string);

        //The notebook must be initialized here first,
        //since we can't set the parent field
        //for its notes until the notebook itself exists.
        let mut notebook = Notebook {
            notes: vec![],
            metadata: notebook_file.metadata,
            parent: parent,
        };
        let notebook_node = new_workspace_node(notebook);

        let mut notes: Vec<WorkspaceNode<Note>> = vec![];
        let mut note_errors: Vec<ConversionError> = vec![];
        //For each path in the NotebookFile's path list:
        for path in notebook_file.note_paths {
            let mut converted_path: PathBuf = path_buf.clone();
            converted_path.push(path);

            match Note::import(converted_path, Rc::downgrade(&notebook_node)) {
                //  * Try to open the specified file.
                //      * If the file failed, mark it as a failed file and continue.
                //          * TODO: as with Workspace.import(),
                //          we need a way to deal with failed note imports.
                Err(error) => note_errors.push(error),
                //      * Else, try to import the opened file as a Note.
                //          * If the import failed, mark it as a failed file and continue.
                //      * Add the new Note to a list of opened Notes.
                Ok(opened_note) => notes.push(opened_note),
            }
        }

        notebook_node.borrow_mut().notes = notes;

        //Return the opened Notes and loaded NotebookMetadata.
        //TODO: this needs to return both the Notebook and its
        //warnings. Maybe move to a separate NotebookImportResult type?
        Ok(notebook_node)
    }
}

impl Export for Notebook {
    fn export(&self, destination: &File) -> Result<(), ConversionError> {
        //The metadata can be directly serialized.
        let metadata = self.metadata.clone();
        //TODO: Instead of immediately writing to destination files,
        //may want to generate changes here and then perform
        //a rollback-able commit. See notes/writing-fs-changes.md.
        let note_paths = self.write_notes_or_rollback()?;
        //Write the output to the destination file.
        let output = NotebookFile {
            note_paths: note_paths,
            metadata: metadata,
        };

        let file_writer = BufWriter::new(destination);
        //TODO: We also need to handle this.
        //If this can't be written, we need to rollback all changes.
        helpers::write_json(file_writer, &output)
    }
}
