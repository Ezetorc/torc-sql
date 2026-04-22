use std::{
    fs::{self as FileSystem, File},
    path::Path,
};

use crate::{abstract_syntax_tree::query::Query, interpreter::error::InterpreterError};

pub struct Interpreter;

impl Interpreter {
    pub const TABLES_FOLDER_NAME: &str = "tables";
    pub const DATABASES_FOLDER_NAME: &str = "databases";
    pub const ROOT_FOLDER: &str = "src/data";

    pub fn execute_queries(queries: Vec<Query>) -> Result<(), InterpreterError> {
        for query in queries {
            match query {
                Query::CreateDatabase { name } => Self::handle_create_database(name)?,
                Query::CreateTable { name } => Self::handle_create_table(name)?,
            }
        }

        Ok(())
    }

    pub(crate) fn create_folder(path: impl AsRef<Path>) -> Result<(), InterpreterError> {
        FileSystem::create_dir_all(path)
            .map_err(|error| InterpreterError::CreationFailed(error.to_string()))
    }

    pub(crate) fn create_file(path: impl AsRef<Path>) -> Result<File, InterpreterError> {
        File::create(path).map_err(|error| InterpreterError::CreationFailed(error.to_string()))
    }

    pub(crate) fn write_file(
        path: impl AsRef<Path>,
        content: &str,
    ) -> Result<(), InterpreterError> {
        FileSystem::write(path, content).map_err(|error| {
            InterpreterError::CreationFailed(format!("Error writing file: {}", error))
        })
    }
}
