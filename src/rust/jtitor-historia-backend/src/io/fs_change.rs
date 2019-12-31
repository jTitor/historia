/*!
 * Defines the FsChange type.
 */
use std::fs;
use std::path::PathBuf;

use failure::Error;

use crate::io::Export;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum FsChangeState {
    NotWritten,
    PerformingWrite,
    Written,
    PerformingRollback,
}

/**
 * Represents a change to the filesystem.
 */
pub struct FsChange<'a> {
    _path: PathBuf,
    _data: &'a dyn Export,
    _write_state: FsChangeState,
    //Paths to temp files used during commit/rollback stages.
    _new_path: PathBuf,
    _rollback_path: PathBuf,
}

const k_new_file_prefix: String = "~new~".into();
const k_rollback_file_prefix: String = "~old~".into();

fn add_prefix_to_path(path: &PathBuf, prefix: String) -> PathBuf {
    let file_name = path.file_name().unwrap_or(std::ffi::OsStr::new("")).to_str().unwrap_or("");
    let new_file_name = format!("{}{}", prefix, file_name);

    let new_path = path.with_file_name(new_file_name).to_str().unwrap_or("");

    new_path.into()
}

impl<'a> FsChange<'a> {
    //Properties.
    pub fn path(&self) -> &PathBuf {
        &self._path
    }

    pub fn write_state(&self) -> FsChangeState {
        self._write_state
    }

    pub fn write(&mut self) -> Result<(), Error> {
        self.validate_write()?;

        self.perform_write()?;

        Ok(())
    }

    pub fn new(path: PathBuf, data: &'a dyn Export) -> FsChange<'a> {
        FsChange {
            _path: path,
            _data: data,
            _write_state: FsChangeState::NotWritten,
            _new_path: add_prefix_to_path(&path, k_new_file_prefix),
            _rollback_path: add_prefix_to_path(&path, k_rollback_file_prefix),
        }
    }

    //Methods.
    pub fn delete_rollback_file(&mut self) -> Result<(), Error> {
        match fs::remove_file(self._rollback_path) {
            Ok(_) => Ok(()),
            Err(error) => Err(Error::from(error))
        }
    }

    pub fn rollback(&mut self) -> Result<(), Error> {
        self.validate_rollback()?;

        self.perform_rollback()?;

        Ok(())
    }

    //Private methods.
    fn validate_write(&self) -> Result<(), Error> {
        if self.write_state() != FsChangeState::NotWritten {
            let error = format_err!("Change is either already written or being written, so it can't be written again. Change state is '{:?}'", self.write_state());
            return Err(error);
        }

        Ok(())
    }

    fn perform_write(&mut self) -> Result<(), Error> {
        self._write_state = FsChangeState::PerformingWrite;

        //TODO: These should be gated such that if a step fails,
        //we're set to a WriteFailed/NotWritten state.
        self.write_new_file()?;

        self.move_target_file_to_rollback_path()?;

        self.move_new_file_to_target_path()?;

        self._write_state = FsChangeState::Written;

        Ok(())
    }

    fn validate_rollback(&self) -> Result<(), Error> {
        if self.write_state() != FsChangeState::Written {
            let error = format_err!("Change hasn't been written, so it can't be rolled back. Change state is '{:?}'", self.write_state());
            return Err(error);
        }

        if !self.rollback_file_exists() {
            let error = format_err!("No rollback file for {} exists, can't rollback this change", self.path().to_string_lossy());
            return Err(error);
        }

        Ok(())
    }

    fn perform_rollback(&mut self) -> Result<(), Error> {
        self._write_state = FsChangeState::PerformingRollback;

        self.move_target_file_to_new_path()?;
        self.move_rollback_file_to_target_path()?;
        //It's okay for this to fail; any subsequent write operations are free to overwrite this path
        self.delete_new_file();

        self._write_state = FsChangeState::NotWritten;

        Ok(())
    }

    //I/O operations.
    fn rollback_file_exists(&self) -> bool {
        self._rollback_path.is_file()
    }

    fn write_new_file(&self) -> Result<(), Error> {
        let file = fs::File::create(self._new_path)?;

        self.write_data(&file)
    }

    fn write_data(&self, to: &fs::File) -> Result<(), Error> {
        match self._data.export(to) {
            Ok(_) => Ok(()),
            Err(error) => Err(Error::from(error))
        }
    }

    fn delete_new_file(&self) {
        let _ = fs::remove_file(self._new_path);
    }

    fn move_target_file_to_rollback_path(&self) -> Result<(), Error> {
        self.perform_move(&self._path, &self._rollback_path)
    }

    fn move_new_file_to_target_path(&self) -> Result<(), Error> {
        self.perform_move(&self._new_path, &self._path)
    }

    fn move_target_file_to_new_path(&self) -> Result<(), Error> {
        self.perform_move(&self._path, &self._new_path)
    }

    fn move_rollback_file_to_target_path(&self) -> Result<(), Error> {
        self.perform_move(&self._rollback_path, &self._path)
    }

    fn perform_move(&self, from: &PathBuf, to: &PathBuf) -> Result<(), Error> {
        let _ = fs::copy(from, to)?;

        Ok(())
    }
}
