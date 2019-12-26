/*!
 * Defines the Note type.
 */
use std::fs::File;
use std::io::{BufReader, BufWriter};

use failure::Error;
use serde::{Serialize, Deserialize};

use crate::model::Section;
use crate::io::{Export, Import};

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
#[derive(Serialize, Deserialize, Debug)]
pub struct NoteMetadata {
    name: String,
    //TODO: Most likely this will need to
    //become a two-way lookup table. 
    tags: Vec<String>,
}

impl Import<Note> for Note {
    fn import(&self, source: &mut File) -> Result<Note, Error> {
        //Very simple: just deserialize from 'source'.
        let file_reader = BufReader::new(source);
        
        let read_result: serde_json::Result<Note> = serde_json::from_reader(file_reader);

        Ok(read_result?)
    }
}

impl Export for Note {
    fn export(&self, destination: &mut File) -> Result<(), Error> {
        //Very simple: just serialize to 'destination'.
        let file_writer = BufWriter::new(destination);
        
        let _serde_writer_result = serde_json::to_writer(file_writer, self)?;

        Ok(())
    }
}