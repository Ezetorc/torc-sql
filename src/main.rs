use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser as SqlParser;
use std::fs;

use crate::parser::parser::Parser;

mod abstract_syntax_tree;
mod cli;
mod parser;

fn main() {
    if let Err(error) = run() {
        print!("{}", error);
    }
}

fn run() -> Result<(), String> {
    let content: String = fs::read_to_string("src/data/demo.sql").expect("File not found");
    let dialect: GenericDialect = GenericDialect {};
    let result = SqlParser::parse_sql(&dialect, &content);

    match result {
        Ok(statements) => {
            let result = Parser::parse(statements);

            println!("result: {:?}", result);
            Ok(())
        }
        Err(error) => return Err(error.to_string()),
    }
}
