use crate::torc_sql::TorcSql;

mod abstract_syntax_tree;
mod cli;
mod interpreter;
mod parser;
mod torc_sql;

fn main() {
    if let Err(error) = TorcSql::execute_from_file("src/data/demo.sql") {
        print!("{}", error);
    }
}
