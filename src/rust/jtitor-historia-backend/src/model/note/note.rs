/*!
 * Defines the Note type.
 */
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::rc::Weak;
use std::path::Path;

use serde::{Serialize, Deserialize};

use crate::error::ConversionError;
use crate::model::{Section, Notebook, NoteFile, WorkspaceNode, WeakWorkspaceNode, new_workspace_node};
use crate::io::{helpers, Export};

/**
 * Represents a single note.
 * Contains one or more Section instances,
 * as well as search metadata such as a title
 * and tags.
 */
#[derive(Debug)]
pub struct Note {
    sections: Vec<Section>,
    metadata: NoteMetadata,
    parent: WeakWorkspaceNode<Notebook>,
}

/**
 * TODO
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteMetadata {
    name: String,
    //TODO: Most likely this will need to
    //become a two-way lookup table. 
    tags: Vec<String>,
}

impl Note {
    pub fn import<P: AsRef<Path>>(path: P, parent: WeakWorkspaceNode<Notebook>) -> Result<WorkspaceNode<Note>, ConversionError> {
        //Very simple: just deserialize from the file at 'path'.
        let source = helpers::open_file_for_read(path)?;
        let file_reader = BufReader::new(source);
        
        let input: NoteFile = helpers::read_json(file_reader)?;

        let result = Note {
            sections: input.sections,
            metadata: input.metadata,
            parent: parent,
        };

        Ok(new_workspace_node(result))
    }

    fn generate_output_file(&self) -> NoteFile {
        NoteFile {
            sections: self.sections.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

impl Export for Note {
    fn export(&self, destination: &File) -> Result<(), ConversionError> {
        //Very simple: just serialize to 'destination'.
        let file_writer = BufWriter::new(destination);
        
        let output = self.generate_output_file();

        helpers::write_json(file_writer, &output)
    }
}