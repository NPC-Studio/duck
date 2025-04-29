use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    FileId,
    lint::{EarlyStmtPass, Lint, LintLevel},
    parse::{Assignment, AssignmentOp, Stmt, StmtKind},
};

#[derive(Debug, PartialEq)]
pub struct NullCoalescenceEqual;
impl Lint for NullCoalescenceEqual {
    fn explanation() -> &'static str {
        "Null coalescence assignment is broken in YYC and we no longer trust it."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "null_coalescence_equal"
    }
}

impl EarlyStmtPass for NullCoalescenceEqual {
    fn visit_stmt_early(stmt: &Stmt, config: &crate::Config, reports: &mut Vec<Diagnostic<FileId>>) {
        if let StmtKind::Assignment(Assignment {
            op: AssignmentOp::NullCoalecenceEqual(_),
            left,
            right,
        }) = stmt.kind()
        {
            reports.push(Self::diagnostic(config).with_message("Use of `??=`").with_labels(vec![
                Label::primary(stmt.file_id(), stmt.span()).with_message(format!(
                    "this should be written as `{left} = {left} == undefined ? {right} : undefined;"
                )),
            ]));
        }
    }
}
