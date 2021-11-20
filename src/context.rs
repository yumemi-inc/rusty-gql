use crate::{
    error::GqlError, operation::ArcOperation, path::GraphQLPath, types::schema::ArcSchema,
};
use graphql_parser::{query::Field, schema::Directive};
use serde_json::to_string;

#[derive(Debug, Clone)]
pub struct ExecutionContext<'a> {
    pub schema: &'a ArcSchema,
    pub operation: &'a ArcOperation<'a>,
    pub current_field: Field<'a, String>,
    pub current_path: GraphQLPath,
    pub errors: Vec<GqlError>,
}

impl<'a> ExecutionContext<'a> {
    pub fn current_field(&mut self, field: Field<'a, String>) -> &mut ExecutionContext<'a> {
        self.current_field = field;
        self
    }

    pub fn is_skip(&self, directives: &'a [Directive<'a, String>]) -> bool {
        for dir in directives {
            let skip = match dir.name.as_str() {
                "skip" => true,
                "include" => false,
                _ => continue,
            };
            return skip;
        }
        false
    }
}

pub(crate) fn build_context<'a>(
    schema: &'a ArcSchema,
    operation: &'a ArcOperation<'a>,
) -> ExecutionContext<'a> {
    let operation_type = operation.operation_type.to_string();
    let root_fieldname = operation.root_field.name.to_string();
    let current_field = operation.root_field.clone();

    let current_path = GraphQLPath::default()
        .prev(None)
        .current_key(root_fieldname)
        .parent_name(operation_type);

    ExecutionContext {
        schema,
        operation,
        current_field,
        current_path,
        errors: vec![],
    }
}
