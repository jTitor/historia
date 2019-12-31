/*!
 * Defines helper methods for file I/O.
 */
use std::fs::File;
use std::io::{Read, Write};

use failure::Error;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::ConversionError;
use crate::model::{NotebookFile, WorkspaceFile};

fn open_from_file<T>(
    file: &File,
    constructor: fn(&File) -> Result<T, Error>,
) -> Result<T, ConversionError> {
    match constructor(file) {
        Ok(result) => {
            return Ok(result);
        }
        Err(error) => {
            return Err(ConversionError::FileOpenFailed {
                path: String::from(""),
            });
        }
    }
}

//Import helper functions
pub fn open_workspace_file(file: &File) -> Result<WorkspaceFile, ConversionError> {
    open_from_file(file, WorkspaceFile::from_file)
}

pub fn open_notebook_file(file: &File) -> Result<NotebookFile, ConversionError> {
    open_from_file(file, NotebookFile::from_file)
}

pub fn read_json<R: Read, D: DeserializeOwned>(reader: R) -> Result<D, ConversionError> {
    //"Note that counter to intuition,
    //this function is usually slower than
    //reading a file completely into memory and
    //then applying from_str or from_slice on it.
    //See issue #160."
    match serde_json::from_reader(reader) {
        Ok(result) => Ok(result),
        Err(error) => Err(ConversionError::SerializationFailed {
            reason: String::from(
                "unknown; possibly a value in the input could not be converted to a valid value or the file doesn't represent the requested type",
            ),
        }),
    }
}

//Export helper functions
pub fn write_json<W: Write, S: Serialize>(writer: W, data: &S) -> Result<(), ConversionError> {
    match serde_json::to_writer(writer, data) {
        Ok(_) => Ok(()),
        Err(error) => Err(ConversionError::SerializationFailed {
            reason: String::from(
                "unknown; possibly a key in the output dictionary is not a string",
            ),
        }),
    }
}
