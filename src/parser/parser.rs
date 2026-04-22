use sqlparser::ast::Statement;

use crate::abstract_syntax_tree::query::Query;

pub struct Parser;

impl Parser {
    pub fn parse(statements: Vec<Statement>) -> Vec<Query> {
        let mut queries: Vec<Query> = Vec::new();

        for statement in statements {
            let new_query: Option<Query> = Self::parse_statement(statement);

            if let Some(query) = new_query {
                queries.push(query);
            }
        }

        queries
    }

    fn parse_statement(statement: Statement) -> Option<Query> {
        match statement {
            Statement::CreateDatabase { db_name, .. } => Some(Query::CreateDatabase {
                name: db_name.to_string(),
            }),
            Statement::CreateTable(create_table) => Some(Query::CreateTable {
                name: create_table.name.to_string(),
            }),
            _ => None,
        }
    }
}
