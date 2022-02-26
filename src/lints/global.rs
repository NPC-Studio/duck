use crate::{
    lint::EarlyExpressionPass,
    parsing::expression::{Expression, Scope},
    utils::Span,
    Duck, Lint, LintCategory, LintReport,
};

#[derive(Debug, PartialEq)]
pub struct Global;
impl Lint for Global {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
			display_name: "Use of `global`".into(),
            tag: Self::tag(),
			explanation: "While useful at times, global variables reduce saftey since they can be accessed or mutated anywhere.",
			suggestions: vec!["Scope this variable to an individual object".into()],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Strict
    }

    fn tag() -> &'static str {
        "global"
    }
}

impl EarlyExpressionPass for Global {
    fn visit_expression_early(
        _config: &crate::Config,
        expression: &Expression,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Expression::Access(Scope::Global, _) = expression {
            reports.push(Self::generate_report(span))
        }
    }
}
