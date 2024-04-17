use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    driver::Ctx,
    lint::{LateExprPass, Lint, LintLevel},
    parse::{Expr, ExprKind, Function},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct FunctionNameAsParameter;
impl Lint for FunctionNameAsParameter {
    fn explanation() -> &'static str {
        "This pattern leads to runtime bugs in v2024.2.0.163."
    }

    fn default_level() -> LintLevel {
        LintLevel::Deny
    }

    fn tag() -> &'static str {
        "fucntion_name_as_parameter"
    }
}

impl LateExprPass for FunctionNameAsParameter {
    fn visit_expr_late(expr: &Expr, config: &crate::Config, ctx: &Ctx, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Function(Function { parameters, .. }) = expr.kind() {
            for param in parameters {
                if ctx.global_function_names.iter().any(|v| v == param.name()) {
                    reports.push(
                        Self::diagnostic(config)
                            .with_message("Non constant default parameter")
                            .with_labels(vec![
                                Label::primary(expr.file_id(), param.name_identifier().span)
                                    .with_message("Parameter names cannot be the same as a global function's name"),
                            ]),
                    );
                }
            }
        }
    }
}
