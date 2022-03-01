use crate::prelude::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox};

/// A return statement, with an optional return value.
#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    /// The value, if any, that this statement returns.
    pub value: Option<ExpressionBox>,
}
impl Return {
    /// Creates a new return statement with an optional value.
    pub fn new(value: Option<ExpressionBox>) -> Self {
        Self { value }
    }
}
impl From<Return> for Statement {
    fn from(ret: Return) -> Self {
        Self::Return(ret)
    }
}
impl IntoStatementBox for Return {}
impl ParseVisitor for Return {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        if let Some(value) = &self.value {
            expression_visitor(value);
        }
    }

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}
