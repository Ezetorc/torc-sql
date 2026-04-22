use crate::abstract_syntax_tree::{column_attribute::ColumnAttribute, column_data_type::ColumnDataType};

pub struct Column {
    name: String,
    data_type: ColumnDataType,
    attributes: Vec<ColumnAttribute>,
}
