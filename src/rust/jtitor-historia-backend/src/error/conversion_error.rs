/*!
 * Defines error types related to
 * serialization/deserialization errors.
 */
use failure::{Fail, Error};

/**
 * TODO
 */
#[derive(Debug, Fail)]
pub enum ConversionError {
    /** The file for an object couldn't be opened. */
    #[fail(display = "source file couldn't be opened: {}", path)]
    FileOpenFailed { path: String },
    /**
     * The file for an object could be opened,
     * but it couldn't be converted to the desired type.
     */
    #[fail(display = "instance couldn't be created: {}", cause)]
    ConstructionFailed { cause: Error },
    /**
     * Multiple errors occured.
     */
    #[fail(display = "multiple conversion errors: {:?}", errors)]
    Multiple { errors: Vec<ConversionError> },
}