/*!
 * TODO
 */
use serde::{Deserialize, Serialize};

use crate::model::{NoteMetadata, Section};

#[derive(Serialize, Deserialize, Debug)]
pub struct NoteFile {
    pub sections: Vec<Section>,
    pub metadata: NoteMetadata,
}
