use crate::{parsing::expression::Expression, Duck, Lint, LintCategory, LintReport, Span};

#[derive(Debug, PartialEq)]
pub struct AnonymousConstructor;
impl Lint for AnonymousConstructor {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            display_name: "Use of an anonymous constructor".into(),
            tag: Self::tag(),
            explanation: "Constructors should be reserved for larger, higher scoped types.",
            suggestions: vec![
                "Change this to a named function".into(),
                "Change this to a function that returns a struct literal".into(),
            ],
            category: Self::category(),
            span,
        }
    }

    fn category() -> LintCategory {
        LintCategory::Style
    }

    fn tag() -> &'static str {
        "anonymous_constructor"
    }

    fn visit_expression(
        _duck: &Duck,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::FunctionDeclaration(None, _, Some(_), _, _) = expression {
            reports.push(Self::generate_report(span))
        }
    }
}
