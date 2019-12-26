/*!
 * Defines the Export trait.
 */
use std::fs::File;

use failure::Error;

/**
 * Trait for types that can be exported to disk.
 * 
 * Types implementing Export specify a method for writing to an arbitrary File.
 */
pub trait Export {
    fn export(&self, destination: &mut File) -> Result<(), Error>;
}