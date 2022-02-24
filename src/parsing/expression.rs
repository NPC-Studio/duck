use super::statement::StatementBox;
use crate::utils::Span;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    FunctionDeclaration(
        Option<String>,
        Vec<Parameter>,
        Option<Constructor>,
        StatementBox,
        bool,
    ),
    Logical(ExpressionBox, LogicalOperator, ExpressionBox),
    Equality(ExpressionBox, EqualityOperator, ExpressionBox),
    Evaluation(ExpressionBox, EvaluationOperator, ExpressionBox),
    NullCoalecence(ExpressionBox, ExpressionBox),
    Ternary(ExpressionBox, ExpressionBox, ExpressionBox),
    Assignment(ExpressionBox, AssignmentOperator, ExpressionBox),
    Unary(UnaryOperator, ExpressionBox),
    Postfix(ExpressionBox, PostfixOperator),
    Access(Scope, ExpressionBox),
    Call(ExpressionBox, Vec<ExpressionBox>, bool),
    Grouping(ExpressionBox),
    Literal(Literal),
    Identifier(String),
}
impl Expression {
    pub fn into_box(self, span: Span) -> ExpressionBox {
        ExpressionBox(Box::new(self), span)
    }

    pub fn lazy_box(self) -> ExpressionBox {
        ExpressionBox(Box::new(self), Span::default())
    }

    pub fn visit_child_statements<S>(&self, mut statement_visitor: S)
    where
        S: FnMut(&StatementBox),
    {
        if let Expression::FunctionDeclaration(_, _, _, body, _) = self {
            statement_visitor(body);
        }
    }

    pub fn visit_child_expressions<E>(&self, mut expression_visitor: E)
    where
        E: FnMut(&ExpressionBox),
    {
        match self {
            Expression::FunctionDeclaration(_, parameters, constructor, _, _) => {
                for parameter in parameters.iter() {
                    if let Some(default_value) = &parameter.1 {
                        expression_visitor(default_value);
                    }
                }
                if let Some(Some(inheritance_call)) = constructor.as_ref().map(|c| &c.0) {
                    expression_visitor(inheritance_call);
                }
            }
            Expression::Logical(left, _, right)
            | Expression::Equality(left, _, right)
            | Expression::Evaluation(left, _, right)
            | Expression::Assignment(left, _, right)
            | Expression::NullCoalecence(left, right) => {
                expression_visitor(left);
                expression_visitor(right);
            }
            Expression::Ternary(condition, left, right) => {
                expression_visitor(condition);
                expression_visitor(left);
                expression_visitor(right);
            }
            Expression::Unary(_, right) => {
                expression_visitor(right);
            }
            Expression::Postfix(left, _) => {
                expression_visitor(left);
            }
            Expression::Access(scope, expression) => {
                expression_visitor(expression);
                match scope {
                    Scope::Dot(other) => {
                        expression_visitor(other);
                    }
                    Scope::Array(x, y, _) => {
                        expression_visitor(x);
                        if let Some(y) = y {
                            expression_visitor(y);
                        }
                    }
                    Scope::Map(key) => {
                        expression_visitor(key);
                    }
                    Scope::Grid(x, y) => {
                        expression_visitor(x);
                        expression_visitor(y);
                    }
                    Scope::List(index) => {
                        expression_visitor(index);
                    }
                    Scope::Struct(key) => {
                        expression_visitor(key);
                    }
                    Scope::Global | Scope::Current => {}
                }
            }
            Expression::Call(left, arguments, _) => {
                expression_visitor(left);
                for arg in arguments {
                    expression_visitor(arg);
                }
            }
            Expression::Grouping(expression) => {
                expression_visitor(expression);
            }
            Expression::Literal(_) | Expression::Identifier(_) => {}
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionBox(pub Box<Expression>, pub Span);
impl ExpressionBox {
    pub fn expression(&self) -> &Expression {
        self.0.as_ref()
    }
    pub fn span(&self) -> Span {
        self.1
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum EvaluationOperator {
    Plus,
    Minus,
    Slash,
    Star,
    Div,
    Modulo,
    And,
    Or,
    Xor,
    BitShiftLeft,
    BitShiftRight,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EqualityOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LogicalOperator {
    And,
    Or,
    Xor,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum AssignmentOperator {
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    XorEqual,
    OrEqual,
    AndEqual,
    NullCoalecenceEqual,
    ModEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Increment,
    Decrement,
    Not,
    Negative,
    BitwiseNot,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PostfixOperator {
    Increment,
    Decrement,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    True,
    False,
    PointerNull,
    PointerInvalid,
    Undefined,
    NaN,
    Infinity,
    Pi,
    String(String),
    Real(f64),
    Hex(String),
    Array(Vec<ExpressionBox>),
    Struct(Vec<(String, ExpressionBox)>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Scope {
    Global,
    Current,
    Dot(ExpressionBox),
    Array(ExpressionBox, Option<ExpressionBox>, bool),
    Map(ExpressionBox),
    Grid(ExpressionBox, ExpressionBox),
    List(ExpressionBox),
    Struct(ExpressionBox),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Constructor(pub Option<ExpressionBox>);

#[derive(Debug, PartialEq, Clone)]
pub struct Parameter(pub String, pub Option<ExpressionBox>);
