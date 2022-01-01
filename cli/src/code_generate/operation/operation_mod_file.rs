use std::collections::BTreeMap;

use codegen::{Scope, Type};
use heck::ToSnakeCase;
use rusty_gql::{GqlField, OperationType};

use crate::code_generate::{use_gql_definitions, util::gql_value_ty_to_rust_ty, FileDefinition};

pub struct OperationModFile<'a> {
    pub operations: &'a BTreeMap<String, GqlField>,
    pub operation_type: OperationType,
    pub path: String,
    pub interface_names: &'a Vec<String>,
}

impl<'a> FileDefinition for OperationModFile<'a> {
    fn path(&self) -> String {
        self.path.to_string()
    }

    fn content(&self) -> String {
        let mut result = String::from("");

        for (operation_name, _) in self.operations.iter() {
            let file_name = operation_name.to_snake_case();
            result += format!("mod {};\n", file_name,).as_str();
        }

        result += "\n";
        result += &self.build_query_str();

        result
    }
}

impl<'a> OperationModFile<'a> {
    fn build_query_str(&self) -> String {
        let mut scope = Scope::new();
        let struct_name = self.operation_type.to_string();
        scope.new_struct(&struct_name).vis("pub");
        let imp = scope.new_impl(&struct_name);

        for (operation_name, method) in self.operations.iter() {
            let f = imp.new_fn(&operation_name.to_snake_case());
            let mut args_str = String::from("");
            for arg in &method.arguments {
                f.arg(
                    &arg.name.to_snake_case(),
                    gql_value_ty_to_rust_ty(&arg.meta_type),
                );
                args_str += format!("{},", &arg.name.to_snake_case()).as_str();
            }
            // remove last `,`
            args_str.pop();
            f.set_async(true);

            let is_interface_return_ty = self
                .interface_names
                .contains(&method.meta_type.name().to_string());
            if is_interface_return_ty {
                f.generic(&format!("T: {}", &method.meta_type.name()));
                f.ret(Type::new("T"));
            } else {
                f.ret(Type::new(&method.meta_type.name()));
            }

            let file_name = operation_name.to_snake_case();
            f.line(format!(
                "{file_name}::{method}({args}).await",
                file_name = file_name,
                method = method.name.to_snake_case(),
                args = args_str
            ));
        }

        format!("{}\n\n{}", use_gql_definitions(), scope.to_string())
    }
}
