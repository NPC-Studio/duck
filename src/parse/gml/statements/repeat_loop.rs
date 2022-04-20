use crate::parse::{Expr, IntoStmt, ParseVisitor, Stmt, StmtKind};

/// Representation of a repeat loop in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct RepeatLoop {
    /// The expression dictating the amount of ticks.
    pub tick_counts: Expr,
    /// The body of the loop.
    pub body: Stmt,
}
impl RepeatLoop {
    /// Creates a new repeat loop.
    pub fn new(tick_counts: Expr, body: Stmt) -> Self {
        Self { tick_counts, body }
    }
}
impl From<RepeatLoop> for StmtKind {
    fn from(repeat_loop: RepeatLoop) -> Self {
        Self::RepeatLoop(repeat_loop)
    }
}
impl IntoStmt for RepeatLoop {}
impl ParseVisitor for RepeatLoop {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.tick_counts);
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.tick_counts);
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut visitor: S) {
        visitor(&self.body);
    }
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, mut visitor: S) {
        visitor(&mut self.body);
    }
}
