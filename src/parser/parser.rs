use sqlparser::ast::Statement;

use crate::abstract_syntax_tree::query::Query;

pub struct Parser;

impl Parser {
    pub fn parse(statements: Vec<Statement>) -> Vec<Query> {
        let mut queries: Vec<Query> = Vec::new();

        for statement in statements {
            let new_query: Query = Self::parse_statement(statement);

            queries.push(new_query);
        }

        queries
    }

    fn parse_statement(statement: Statement) -> Query {
        match statement {
            Statement::CreateDatabase { db_name, .. } => Query::CreateDatabase {
                name: db_name.to_string(),
            },
            _ => unimplemented!(),
        }
    }
}
