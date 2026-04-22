use crate::interpreter::{error::InterpreterError, interpreter::Interpreter};

impl Interpreter {
    pub(crate) fn handle_create_database(database_name: String) -> Result<(), InterpreterError> {
        let folder_path: String = format!(
            "{}/{}/{database_name}",
            Self::ROOT_FOLDER,
            Self::DATABASES_FOLDER_NAME
        );

        Self::create_folder(folder_path)
    }

    pub(crate) fn handle_create_table(table_name: String) -> Result<(), InterpreterError> {
        let folder_path: String = format!(
            "{}/{}/MyDatabase/{}",
            Self::ROOT_FOLDER,
            Self::DATABASES_FOLDER_NAME,
            Self::TABLES_FOLDER_NAME
        );

        Self::create_folder(&folder_path)?;

        let file_path: String = format!("{}/{}", folder_path, table_name);

        Self::create_file(&file_path)?;
        Self::write_file(&file_path, "Hola")?;

        Ok(())
    }
}
