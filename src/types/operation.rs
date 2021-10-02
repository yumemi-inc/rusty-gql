use graphql_parser::schema::Field;

use super::{argument::GraphQLArgument, gql_type::GraphQLGenericType, GraphQLDirective};

#[derive(Debug)]
pub struct GraphQLOperationSchema {
    pub name: String,
    pub args: Vec<GraphQLArgument>,
    pub description: Option<String>,
    pub directives: Vec<GraphQLDirective>,
    pub return_type: GraphQLGenericType,
}

impl GraphQLOperationSchema {
    pub fn parse<'a>(field: Field<'a, &'a str>) -> Self {
        let args: Vec<GraphQLArgument> = field
            .arguments
            .into_iter()
            .map(|arg| GraphQLArgument::parse(arg))
            .collect();
        let directives: Vec<GraphQLDirective> = field
            .directives
            .into_iter()
            .map(|dir| GraphQLDirective::parse(dir))
            .collect();
        let return_type = GraphQLGenericType::parse(field.field_type);

        GraphQLOperationSchema {
            name: field.name.to_string(),
            description: field.description,
            args,
            directives,
            return_type,
        }
    }
}
