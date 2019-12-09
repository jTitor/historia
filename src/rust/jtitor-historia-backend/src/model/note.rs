/*!
 * Defines the Note type.
*/
use serde::{Serialize, Deserialize};

use crate::model::Section;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct NoteMetadata {
    name: String,
    //TODO: Most likely this will need to
    //become a two-way lookup table. 
    tags: Vec<String>,
}