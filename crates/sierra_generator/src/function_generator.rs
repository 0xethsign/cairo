#[cfg(test)]
#[path = "function_generator_test.rs"]
mod test;

use defs::ids::{FreeFunctionId, GenericFunctionId};

use crate::db::SierraGenGroup;
use crate::expr_generator::generate_expression_code;
use crate::expr_generator_context::ExprGeneratorContext;
use crate::pre_sierra;
use crate::utils::{return_statement, simple_statement};

pub fn generate_function_code(
    db: &dyn SierraGenGroup,
    function_id: FreeFunctionId,
    function_semantic: semantic::FreeFunction,
) -> pre_sierra::Function {
    let mut context = ExprGeneratorContext::new(db, function_id);

    // Generate a label for the function's body.
    let (label, label_id) = context.new_label();

    let mut statements: Vec<pre_sierra::Statement> = vec![label];

    // Generate the function's body.
    let (body_statements, res) = generate_expression_code(&mut context, function_semantic.body);
    statements.extend(body_statements);

    // Copy the result to the top of the stack before returning.
    statements.push(simple_statement(
        context.store_temp_libfunc_id(),
        &[res.clone()],
        &[res.clone()],
    ));

    statements.push(return_statement(vec![res]));

    pre_sierra::Function {
        id: db.intern_function(db.intern_concrete_function(semantic::ConcreteFunctionLongId {
            generic_function: GenericFunctionId::Free(function_id),
            // TODO(lior): Add generic arguments.
            generic_args: vec![],
        })),
        body: statements,
        entry_point: label_id,
    }
}
