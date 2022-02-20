use crate::{Lint, LintCategory, LintReport, Position};

#[derive(Debug, PartialEq)]
pub struct AndKeyword;
impl Lint for AndKeyword {
    fn generate_report(position: Position) -> LintReport {
        LintReport {
			display_name: "Use of `and`",
			tag: "and_keyword",
			explanation: "GML supports both `and` and `&&` to refer to logical and -- `&&` is more consistent with other languages and is preferred.",
			suggestions: vec!["Use `&&` instead of `and`"],
			category: LintCategory::Style,
			position,
		}
    }
}
