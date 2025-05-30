use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    Config, FileId,
    lint::{EarlyExprPass, Lint, LintLevel},
    parse::{Equality, EqualityOp, Expr, ExprKind, Literal},
};

#[derive(Debug, PartialEq)]
pub struct BoolEquality;
impl Lint for BoolEquality {
    fn explanation() -> &'static str {
        "Comparing a bool with a bool literal is more verbose than neccesary."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "bool_equality"
    }
}

impl EarlyExprPass for BoolEquality {
    fn visit_expr_early(expr: &Expr, config: &Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let ExprKind::Equality(Equality {
            left,
            op: EqualityOp::Equal(token),
            right,
        }) = expr.kind()
        {
            if let Some(literal) = right.kind().as_literal() {
                reports.push(match literal {
                    Literal::True => Self::diagnostic(config)
                        .with_message("Equality check with `true`")
                        .with_labels(vec![
                            Label::primary(right.file_id(), right.span()).with_message("this can be omitted"),
                        ]),
                    Literal::False => Self::diagnostic(config)
                        .with_message("Equality check with `false`")
                        .with_labels(vec![
                            Label::primary(right.file_id(), token.span.start()..right.span().end())
                                .with_message("this can be omitted..."),
                            Label::secondary(left.file_id(), left.span().start()..left.span().start())
                                .with_message("...if you add a not operator here (`!`, `not`)"),
                        ]),
                    _ => return,
                });
            }
        }
    }
}
