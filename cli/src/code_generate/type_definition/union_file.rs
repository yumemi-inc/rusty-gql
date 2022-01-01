use codegen::Scope;
use rusty_gql::GqlUnion;

use crate::code_generate::{use_gql_definitions, FileDefinition};

pub struct UnionFile<'a> {
    pub def: &'a GqlUnion,
    pub path: &'a str,
}

impl<'a> FileDefinition for UnionFile<'a> {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut scope = Scope::new();
        let enum_scope = scope.new_enum(&self.def.name).vis("pub");

        for value in &self.def.types {
            enum_scope.new_variant(&value);
        }

        format!("{}\n\n{}", use_gql_definitions(), scope.to_string())
    }
}
