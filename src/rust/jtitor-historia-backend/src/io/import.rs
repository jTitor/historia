/*!
 * Defines the Import trait.
 */
use std::fs::File;

use failure::Error;

/**
 * Trait for types that can be imported from disk.
 * 
 * Types implementing Import specify a method for creating their
 * in-memory type from an arbitrary File.
 */
pub trait Import<T> {
    fn import(&self, source: &mut File) -> Result<T, Error>;
}
