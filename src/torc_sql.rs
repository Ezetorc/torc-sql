use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::{Parser as SqlParser, ParserError};
use std::fs as FileSystem;
use std::path::Path;

use crate::abstract_syntax_tree::query::Query;
use crate::interpreter::error::InterpreterError;
use crate::interpreter::interpreter::Interpreter;
use crate::parser::parser::Parser;

pub struct TorcSql;

impl TorcSql {
    pub fn execute_from_file(path: impl AsRef<Path>) -> Result<(), String> {
        let content: String = match FileSystem::read_to_string(path) {
            Ok(content) => content,
            Err(error) => return Err(format!("Error reading file: {}", error)),
        };

        let dialect: GenericDialect = GenericDialect {};
        let result: Result<Vec<Statement>, ParserError> = SqlParser::parse_sql(&dialect, &content);

        match result {
            Ok(statements) => {
                let queries: Vec<Query> = Parser::parse(statements);

                let result: Result<(), InterpreterError> = Interpreter::execute_queries(queries);

                match result {
                    Ok(()) => return Ok(()),
                    Err(error) => return Err(error.to_string()),
                }
            }
            Err(error) => return Err(error.to_string()),
        }
    }
}
