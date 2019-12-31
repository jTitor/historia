/*!
 * Defines the Workspace type.
 */
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use super::{WorkspaceFile, WorkspaceMetadata};
use crate::error::ConversionError;
use crate::io::{helpers, Export};
use crate::model::{new_workspace_node, Notebook, WorkspaceNode};

/**
 * Contains one or more Notebook instances.
 */
#[derive(Debug)]
pub struct Workspace {
    notebooks: Vec<WorkspaceNode<Notebook>>,
    metadata: WorkspaceMetadata,
}

impl Workspace {
    fn import<P: AsRef<Path>>(path: P) -> Result<WorkspaceNode<Workspace>, ConversionError> {
        let path_string: String = path.as_ref().to_string_lossy().to_owned().into();
        let source = helpers::open_file_for_read(path)?;
        //First, try to deserialize a WorkspaceFile from 'source'.
        let workspace_file = helpers::open_workspace_file(&source)?;
        let path_buf = PathBuf::from(path_string);

        //The workspace must be initialized here first,
        //since we can't set the parent field
        //for its notebooks until the workspace itself exists.
        let mut workspace = Workspace {
            notebooks: vec![],
            metadata: workspace_file.metadata,
        };
        let workspace_node = new_workspace_node(workspace);

        let mut notebooks: Vec<WorkspaceNode<Notebook>> = vec![];
        let mut notebook_errors: Vec<ConversionError> = vec![];
        //For each path in the WorkspaceFile's path list:
        for path in workspace_file.notebook_paths {
            let mut converted_path: PathBuf = path_buf.clone();
            converted_path.push(path);

            //  * Try to open the specified file.
            match Notebook::import(converted_path, Rc::downgrade(&workspace_node)) {
                //      * If the file failed, mark it as a failed file and continue.
                //          * TODO: We need some way to deal with failed notebooks.
                //          * TODO: We should get the reason a file failed; as seen below,
                //          a file can open but fail to import.
                Err(error) => notebook_errors.push(error),
                //      * Else, try to import the opened file as a Notebook.
                //          * If the import failed, mark it as a failed file and continue.
                //      * Add the new Notebook to a list of opened Notebooks.
                Ok(opened_notebook) => notebooks.push(opened_notebook),
            }
        }

        workspace_node.borrow_mut().notebooks = notebooks;

        //Return the opened Notebooks and loaded WorkspaceMetadata.
        //TODO: this needs to return both the Workspace and its
        //warnings. Maybe move to a separate WorkspaceImportResult type?
        Ok(workspace_node)
    }
}

impl Export for Workspace {
    fn export(&self, destination: &File) -> Result<(), ConversionError> {
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
