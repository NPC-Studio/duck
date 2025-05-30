use codespan_reporting::diagnostic::Diagnostic;

use crate::{
    FileId,
    driver::Ctx,
    lint::{LateExprPass, Lint, LintLevel},
    parse::Expr,
};

#[derive(Debug, PartialEq)]
pub struct NonConstantDefaultParameter;
impl Lint for NonConstantDefaultParameter {
    fn explanation() -> &'static str {
        "Expressive default parameters are not supported in most languages due to their instability and tendency to hide important logic execution from the caller."
    }

    fn default_level() -> LintLevel {
        LintLevel::Warn
    }

    fn tag() -> &'static str {
        "non_constant_default_parameter"
    }
}

impl NonConstantDefaultParameter {
    fn _is_constant(_expresion_box: &Expr) -> bool {
        // match expresion_box.kind() {
        //     ExprType::Access(Access::Dot { left, .. }) => left
        //         .kind()
        //         .as_identifier()
        //         .map_or(false, |iden| global_scope.find_enum(&iden.lexeme).is_some()),
        //     ExprType::Unary(Unary {
        //         op: UnaryOp::Positive(_),
        //         right,
        //     })
        //     | ExprType::Unary(Unary {
        //         op: UnaryOp::Negative(_),
        //         right,
        //     }) => Self::is_constant(right, global_scope),
        //     ExprType::Evaluation(Evaluation { left, right, .. }) => {
        //         Self::is_constant(left, global_scope) && Self::is_constant(right, global_scope)
        //     }
        //     ExprType::Literal(_) | ExprType::Identifier(_) => true,
        //     _ => false,
        // }
        false
    }
}

impl LateExprPass for NonConstantDefaultParameter {
    fn visit_expr_late(_expr: &Expr, _config: &crate::Config, _ctx: &Ctx, _reports: &mut Vec<Diagnostic<FileId>>) {
        // if let ExprType::Function(Function { parameters, .. }) = expr.kind() {
        //     for param in parameters {
        //         if let Some(default_expr) = param.assignment_value() {
        //             let constant = Self::is_constant(default_expr, global_scope);
        //             if !constant {
        //                 reports.push(
        //                     Self::diagnostic(config)
        //                         .with_message("Non constant default parameter")
        //                         .with_labels(vec![
        //                             Label::primary(default_expr.file_id(), default_expr.span())
        //                                 .with_message("this parameter's default value is not
        // constant"),                         ]),
        //                 );
        //             }
        //         }
        //     }
        // }
    }
}
