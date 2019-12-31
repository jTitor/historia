/*!
 * Defines the Export trait.
 */
use std::fs::File;

use crate::error::ConversionError;

/**
 * Trait for types that can be exported to disk.
 * 
 * Types implementing Export specify a method for writing to an arbitrary File.
 */
pub trait Export {
    fn export(&self, destination: &File) -> Result<(), ConversionError>;
}