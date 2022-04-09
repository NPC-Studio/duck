use super::*;
use crate::parse::*;

impl Solver {
    pub fn process_statements(&mut self, stmts: &[Stmt]) -> Result<(), Vec<TypeError>> {
        let mut errors = vec![];
        let mut functions = vec![];
        for stmt in stmts.iter() {
            if let Err(e) = self.visit_stmt(stmt, &mut functions, &mut errors) {
                errors.push(e);
            }
        }
        for expr in functions {
            let function = match expr.inner() {
                ExprType::Function(function) => function,
                _ => unreachable!(),
            };
            if let Err(e) = self.process_function(&expr, function) {
                errors.push(e);
            }
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

// Goal construction
impl Solver {
    fn visit_stmt(
        &mut self,
        stmt: &Stmt,
        functions: &mut Vec<Expr>,
        errors: &mut Vec<TypeError>,
    ) -> Result<(), TypeError> {
        match stmt.inner() {
            StmtType::Assignment(Assignment { left, right, op }) => {
                self.visit_expr(right, functions, errors)?;
                let right_ty = self.canonize(right)?;
                if let AssignmentOp::Identity(_) = op {
                    match left.inner() {
                        ExprType::Identifier(iden) => {
                            if self.local_scope().contains(&iden.lexeme) {
                                self.local_scope_mut()
                                    .apply_field(
                                        &iden.lexeme,
                                        Field {
                                            ty: right_ty,
                                            location: stmt.location(),
                                            op: RecordOp::Write,
                                        },
                                    )?
                                    .apply(self)?;
                            } else {
                                self.self_scope_mut()
                                    .apply_field(
                                        &iden.lexeme,
                                        Field {
                                            ty: right_ty,
                                            location: stmt.location(),
                                            op: RecordOp::Write,
                                        },
                                    )?
                                    .apply(self)?;
                            }
                        }
                        ExprType::Access(Access::Current { right: iden }) => {
                            self.self_scope_mut()
                                .apply_field(
                                    &iden.lexeme,
                                    Field {
                                        ty: right_ty,
                                        location: stmt.location(),
                                        op: RecordOp::Write,
                                    },
                                )?
                                .apply(self)?;
                        }
                        ExprType::Access(Access::Dot {
                            left: struct_expr,
                            right: iden,
                        }) => {
                            let mut record = Record::inferred();
                            let right_ty = self.canonize(right)?;
                            record
                                .apply_field(
                                    &iden.lexeme,
                                    Field {
                                        ty: right_ty,
                                        location: stmt.location(),
                                        op: RecordOp::Write,
                                    },
                                )?
                                .apply(self)?;
                            self.expr_eq_ty(struct_expr, Ty::Record(record))?;
                        }
                        _ => {
                            self.visit_expr(left, functions, errors)?;
                        }
                    }
                }
                return Ok(());
            }
            StmtType::LocalVariableSeries(LocalVariableSeries { declarations }) => {
                for initializer in declarations.iter() {
                    let ty = match initializer {
                        OptionalInitilization::Uninitialized(_) => Ty::Null,
                        OptionalInitilization::Initialized(_) => {
                            self.canonize(initializer.assignment_value().unwrap())?
                        }
                    };
                    self.local_scope_mut()
                        .apply_field(
                            initializer.name(),
                            Field {
                                ty,
                                location: stmt.location(),
                                op: RecordOp::Write,
                            },
                        )?
                        .apply(self)?;
                }
            }

            StmtType::Return(Return { value }) => {
                if let Some(value) = value {
                    let ty = self.canonize(value)?;
                    self.var_eq_ty(Var::Return, ty)?;
                } else {
                    self.var_eq_ty(Var::Return, Ty::Undefined)?;
                }
            }

            // todo: constrain types required for these things
            _ => {}
        }

        stmt.visit_child_stmts(|stmt| {
            if let Err(err) = self.visit_stmt(stmt, functions, errors) {
                errors.push(err);
            }
        });
        stmt.visit_child_exprs(|expr| {
            if let Err(err) = self.visit_expr(expr, functions, errors) {
                errors.push(err);
            }
        });
        Ok(())
    }

    fn visit_expr(
        &mut self,
        expr: &Expr,
        functions: &mut Vec<Expr>,
        errors: &mut Vec<TypeError>,
    ) -> Result<(), TypeError> {
        if let ExprType::Function(function) = expr.inner() {
            let expr_ty = self.canonize(expr)?;
            if let Some(name) = &function.name {
                self.self_scope_mut()
                    .apply_field(
                        &name.lexeme,
                        Field {
                            ty: expr_ty,
                            location: expr.location(),
                            op: RecordOp::Write,
                        },
                    )?
                    .apply(self)?;
            }
            functions.push(expr.clone());
            return Ok(());
        }
        expr.visit_child_stmts(|stmt| {
            if let Err(err) = self.visit_stmt(stmt, functions, errors) {
                errors.push(err);
            }
        });
        expr.visit_child_exprs(|expr| {
            if let Err(err) = self.visit_expr(expr, functions, errors) {
                errors.push(err);
            }
        });
        match expr.inner() {
            ExprType::Function(_) => unreachable!(),
            ExprType::Logical(Logical { left, right, .. }) => {
                self.expr_eq_ty(left, Ty::Bool)?;
                self.expr_eq_ty(right, Ty::Bool)?;
                self.expr_eq_ty(expr, Ty::Bool)?;
            }
            ExprType::Equality(Equality { left, right, .. }) => {
                self.expr_eq_expr(right, left)?;
                self.expr_eq_ty(expr, Ty::Bool)?;
            }
            ExprType::Evaluation(Evaluation { left, right, .. }) => {
                self.expr_eq_expr(right, left)?;
                self.expr_eq_expr(expr, left)?;
            }
            ExprType::NullCoalecence(NullCoalecence { .. }) => {
                todo!();
            }
            ExprType::Ternary(Ternary {
                condition,
                true_value,
                false_value,
            }) => {
                self.expr_eq_ty(condition, Ty::Bool)?;
                self.expr_eq_expr(false_value, true_value)?;
                self.expr_eq_expr(expr, true_value)?;
            }
            ExprType::Unary(Unary { op, right }) => match op {
                UnaryOp::Increment(_)
                | UnaryOp::Decrement(_)
                | UnaryOp::Positive(_)
                | UnaryOp::Negative(_)
                | UnaryOp::BitwiseNot(_) => {
                    self.expr_eq_ty(right, Ty::Real)?;
                    self.expr_eq_ty(expr, Ty::Real)?;
                }
                UnaryOp::Not(_) => {
                    self.expr_eq_ty(right, Ty::Bool)?;
                    self.expr_eq_ty(expr, Ty::Bool)?;
                }
            },
            ExprType::Postfix(Postfix { left, .. }) => {
                self.expr_eq_ty(left, Ty::Real)?;
                self.expr_eq_ty(expr, Ty::Real)?;
            }
            ExprType::Access(access) => {
                match access {
                    Access::Global { .. } => {}
                    Access::Current { right } => {
                        let ty = self.canonize(expr)?;
                        self.self_scope_mut()
                            .apply_field(
                                &right.lexeme,
                                Field {
                                    ty,
                                    location: expr.location(),
                                    op: RecordOp::Read,
                                },
                            )?
                            .apply(self)?;
                    }
                    Access::Other { .. } => {}
                    Access::Dot { left, right } => {
                        let mut record = Record::inferred();
                        let ty = self.canonize(expr)?;
                        record
                            .apply_field(
                                &right.lexeme,
                                Field {
                                    ty,
                                    location: expr.location(),
                                    op: RecordOp::Read,
                                },
                            )?
                            .apply(self)?;
                        self.expr_eq_ty(left, Ty::Record(record))?;
                    }
                    Access::Array {
                        left,
                        index_one,
                        index_two,
                        ..
                    } => {
                        let ty = self.canonize(expr)?;

                        // our indexes must be real
                        self.expr_eq_ty(index_one, Ty::Real)?;
                        if let Some(index_two) = index_two {
                            self.expr_eq_ty(index_two, Ty::Real)?;
                        }

                        // the left must be an array of the member
                        self.expr_eq_ty(left, Ty::Array(Box::new(ty)))?;
                    }
                    Access::Struct { .. } => {}
                    Access::Map { .. } => {}
                    Access::Grid { .. } => {}
                    Access::List { .. } => {}
                }
            }
            ExprType::Call(crate::parse::Call { left, arguments, .. }) => {
                let left_ty = self.canonize(left)?;
                let mut parameters = vec![];
                for arg in arguments.iter() {
                    parameters.push(self.canonize(arg)?);
                }
                self.expr_eq_ty(
                    expr,
                    Ty::Call(super::Call {
                        parameters: parameters.clone(),
                        target: Box::new(left_ty),
                    }),
                )?;
            }
            ExprType::Grouping(Grouping { inner, .. }) => {
                self.expr_eq_expr(expr, inner)?;
            }

            ExprType::Identifier(iden) => {
                let ty = self.canonize(expr)?;
                let scope = if self.local_scope().contains(&iden.lexeme) {
                    self.local_scope_mut()
                } else {
                    self.self_scope_mut()
                };
                scope
                    .apply_field(
                        &iden.lexeme,
                        Field {
                            ty,
                            location: expr.location(),
                            op: RecordOp::Read,
                        },
                    )?
                    .apply(self)?;
            }
            ExprType::Literal(literal) => {
                let ty = match literal {
                    Literal::True | Literal::False => Ty::Bool,
                    Literal::Undefined => Ty::Undefined,
                    Literal::Noone => Ty::Noone,
                    Literal::String(_) => Ty::Str,
                    Literal::Real(_) | Literal::Hex(_) => Ty::Real,
                    Literal::Misc(_) => Ty::Any, // todo
                    Literal::Array(exprs) => {
                        // Infer the type based on the first member
                        let app = if let Some(ty) = exprs.first().map(|expr| self.canonize(expr)) {
                            let ty = ty?;
                            Ty::Array(Box::new(ty))
                        } else {
                            Ty::Array(Box::new(Ty::Any))
                        };
                        self.expr_eq_ty(expr, app)?;
                        return Ok(());
                    }
                    Literal::Struct(declarations) => {
                        let mut record = Record::extendable();
                        for declaration in declarations {
                            let ty = self.canonize(&declaration.1)?;
                            record
                                .apply_field(
                                    &declaration.0.lexeme,
                                    Field {
                                        ty,
                                        location: expr.location(),
                                        op: RecordOp::Write,
                                    },
                                )?
                                .apply(self)?;
                        }
                        self.expr_eq_ty(expr, Ty::Record(record))?;
                        return Ok(());
                    }
                };
                self.expr_eq_ty(expr, ty)?;
            }
        }

        Ok(())
    }

    fn process_function(&mut self, expr: &Expr, function: &crate::parse::Function) -> Result<(), TypeError> {
        let expr_ty = self.canonize(expr)?;
        println!(
            "\n--- Entering function ({}: {})... ---\n",
            if let Some(name) = &function.name {
                &name.lexeme
            } else {
                "anon"
            },
            Printer::ty(&expr_ty)
        );
        let binding = if let Some(constructor) = function.constructor.as_ref() {
            Binding::Constructor(
                self.enter_new_self_scope(),
                match constructor {
                    Constructor::WithInheritance(call) => Some(
                        call.inner()
                            .as_call()
                            .and_then(|call| call.left.inner().as_identifier())
                            .cloned()
                            .unwrap(),
                    ),
                    _ => None,
                },
            )
        } else {
            Binding::Method(self.current_self_var())
        };

        let local_var = self.enter_new_local_scope();
        let mut parameters = vec![];
        for param in function.parameters.iter() {
            let ty = match param {
                OptionalInitilization::Uninitialized(_) => Ty::Var(self.var_for_expr(param.name_expr())),
                OptionalInitilization::Initialized(_) => self.canonize(param.assignment_value().unwrap())?,
            };
            self.local_scope_mut()
                .apply_field(
                    param.name(),
                    Field {
                        ty: ty.clone(),
                        location: param.name_expr().location(),
                        op: RecordOp::Write,
                    },
                )?
                .apply(self)?;
            parameters.push(ty);
        }
        if let Err(errs) = &mut self.process_statements(function.body_stmts()) {
            return Err(errs.pop().unwrap()); // todo
        }

        self.depart_local_scope();
        let return_type = if function.constructor.is_some() {
            let ret = self.resolve_var(&self.current_self_var())?;
            self.depart_self_scope();
            Box::new(ret)
        } else {
            Box::new(self.subs.remove(&Var::Return).unwrap_or(Ty::Undefined))
        };
        println!("\n--- Exiting function... ---\n");
        self.expr_eq_ty(
            expr,
            Ty::Function(super::Function {
                binding: Some(binding),
                local_var,
                parameters,
                return_type,
            }),
        )
    }
}

// Utilities
// TODO: kill these
impl Solver {
    pub fn expr_eq_expr(&mut self, target: &Expr, expr: &Expr) -> Result<(), TypeError> {
        let ty = self.canonize(expr)?;
        self.expr_eq_ty(target, ty)
    }

    pub fn expr_eq_ty(&mut self, expr: &Expr, mut ty: Ty) -> Result<(), TypeError> {
        let mut o_ty = self.canonize(expr)?;
        self.unify_tys(&mut o_ty, &mut ty)
    }

    pub fn var_eq_ty(&mut self, var: Var, mut ty: Ty) -> Result<(), TypeError> {
        self.unify_var(&var, &mut ty)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Goal {
    Eq(Var, Ty),
}
