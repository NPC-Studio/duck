use crate::{
    lint::EarlyStatementPass, parsing::statement::Statement, utils::Span, Duck, Lint, LintCategory,
    LintReport,
};

#[derive(Debug, PartialEq)]
pub struct WithLoop;
impl Lint for WithLoop {
    fn generate_report(span: Span) -> LintReport {
        LintReport {
            tag: Self::tag(),
			display_name: "Use of `with`".into(),
			explanation: "The `with` loop allows your code's context to suddenly change, both making it more difficult to read (as a given line of code is no longer promised to be executing in the scope expected from the file), but also making it more difficult to track down all of the places an object is modified.",
			suggestions: vec![
            "Use `instance_find` if looping over objects".into(),
            "Use direct dot reference `foo.bar` to manipulate single objects".into(),
        ],
			category: Self::category(),
			span,
		}
    }

    fn category() -> LintCategory {
        LintCategory::Strict
    }

    fn tag() -> &'static str {
        "with_loop"
    }
}

impl EarlyStatementPass for WithLoop {
    fn visit_statement_early(
        _config: &crate::Config,
        statement: &crate::parsing::statement::Statement,
        span: Span,
        reports: &mut Vec<LintReport>,
    ) {
        if let Statement::With(..) = statement {
            reports.push(Self::generate_report(span))
        }
    }
}
