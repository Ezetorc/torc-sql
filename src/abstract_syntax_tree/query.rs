#[derive(Debug)]
pub enum Query {
    CreateDatabase { name: String },
    CreateTable { name: String },
}
