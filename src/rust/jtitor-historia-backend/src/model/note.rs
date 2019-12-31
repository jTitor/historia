/*!
 * Defines the Note type.
 */
use std::fs::File;
use std::io::{BufReader, BufWriter};

use serde::{Deserialize, Serialize};

use crate::error::ConversionError;
use crate::io::{helpers, Export, Import};
use crate::model::{Section, WorkspaceNode, WorkspaceNodeDescribe, NotebookNode};

/**
 * Represents a single note.
 * Contains one or more Section instances,
 * as well as search metadata such as a title
 * and tags.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    sections: Vec<Section>,
    metadata: NoteMetadata,
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

impl Import<Note> for Note {
    fn import(&self, source: &File) -> Result<Note, ConversionError> {
        //Very simple: just deserialize from 'source'.
        let file_reader = BufReader::new(source);
        helpers::read_json(file_reader)
    }
}

impl Export for Note {
    fn export(&self, destination: &File) -> Result<(), ConversionError> {
        //Very simple: just serialize to 'destination'.
        let file_writer = BufWriter::new(destination);
        helpers::write_json(file_writer, self)
    }
}

pub type NoteNode = WorkspaceNode<Note, NotebookNode>;
impl WorkspaceNodeDescribe for NoteNode {
    fn node_name<'a>(&self) -> &'a str {
        self.metadata.name
    }

    fn full_file_name<'a>(&self) -> &'a str {
    }

    fn full_container_name<'a>(&self) -> &'a str;

    fn is_container(&self) -> bool {
        false
    }
}