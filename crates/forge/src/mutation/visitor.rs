use solar_parse::ast::{
    visit::Visit, Expr, ExprKind, Ident, IndexKind, LitKind, VariableDefinition,
};

use std::ops::ControlFlow;

use crate::mutation::{
    mutant::{
        create_assignement_mutation, create_binary_op_mutation, create_delegatecall_mutation,
        create_delete_mutation, create_unary_mutation, Mutant, MutationType,
    },
    mutators::mutator_registry::MutatorRegistry,
};

#[derive(Debug, Clone)]
pub enum AssignVarTypes {
    Literal(LitKind),
    Identifier(String), /* not using Ident as the symbol is slow to convert as to_str() <--
                         * maybe will have to switch back if validating more aggressively */
}

/// A visitor which collect all expression which will need to be mutated
pub struct MutantVisitor {
    pub mutation_to_conduct: Vec<Mutant>,
}

impl<'ast> Visit<'ast> for MutantVisitor {
    type BreakValue = ();

    fn visit_variable_definition(
        &mut self,
        var: &'ast VariableDefinition<'ast>,
    ) -> ControlFlow<Self::BreakValue> {
        let registry = MutatorRegistry::new();

        match &var.initializer {
            None => {}
            Some(exp) => match &exp.kind {
                ExprKind::Lit(val, _) => {
                    self.mutation_to_conduct.extend(create_assignement_mutation(
                        exp.span,
                        AssignVarTypes::Literal(val.kind.clone()),
                    ))
                }
                ExprKind::Unary(op, var) => {
                    self.mutation_to_conduct.extend(create_unary_mutation(op.span, op.kind, var))
                }
                _ => {}
            },
        }

        // @todo this is taken from the Visit trait -> commented line (original trait
        // implementation) infinitely recurse and don't see why rn
        // <Self as solar_parse::ast::visit::Visit<'ast>>::visit_variable_definition(self, var)

        let VariableDefinition {
            span,
            ty,
            visibility: _,
            mutability: _,
            data_location: _,
            override_: _,
            indexed: _,
            name,
            initializer,
        } = var;
        self.visit_span(span)?;
        self.visit_ty(ty)?;
        if let Some(name) = name {
            self.visit_ident(name)?;
        }
        if let Some(initializer) = initializer {
            self.visit_expr(initializer)?;
        }

        ControlFlow::Continue(())
    }

    fn visit_expr(&mut self, expr: &'ast Expr<'ast>) -> ControlFlow<Self::BreakValue> {
        match &expr.kind {
            // Array skipped for now (swap could be mutating it, cf above for rational)
            ExprKind::Assign(_, bin_op, rhs) => {
                match &rhs.kind {
                    ExprKind::Lit(kind, _) => {
                        self.mutation_to_conduct.extend(create_assignement_mutation(
                            rhs.span,
                            AssignVarTypes::Literal(kind.kind.clone()),
                        ))
                    }

                    ExprKind::Ident(val) => {
                        self.mutation_to_conduct.extend(create_assignement_mutation(
                            rhs.span,
                            AssignVarTypes::Identifier(val.to_string()),
                        ))
                    }

                    _ => {}
                }

                // if let ExprKind::Lit(kind, _) = &rhs.kind {
                //     self.mutation_to_conduct
                //         .extend(create_assignement_mutation(rhs.span,
                // kind.kind.clone())); }

                if let Some(op) = &bin_op {
                    self.mutation_to_conduct.extend(create_binary_op_mutation(op.span, op.kind));
                }
            }
            ExprKind::Binary(_, op, _) => {
                self.mutation_to_conduct.extend(create_binary_op_mutation(op.span, op.kind));
            }
            ExprKind::Call(expr, args) => {
                if let ExprKind::Member(_, ident) = &expr.kind {
                    if ident.to_string() == "delegatecall" {
                        self.mutation_to_conduct.push(create_delegatecall_mutation(ident.span));
                    }
                }
            }
            // CallOptions
            ExprKind::Delete(_) => self.mutation_to_conduct.push(create_delete_mutation(expr.span)),
            // Indent
            // Index -> mutable? 0 it? idx should be a regular expression?
            // Lit -> global/constant are using Lit as initializer
            // Member
            // New
            // Payable -> compilation error
            // Ternary -> swap them?
            // Tuple -> swap if same type?
            // TypeCall -> compilation error
            // Type -> compilation error, most likely
            ExprKind::Unary(op, var) => {
                self.mutation_to_conduct.extend(create_unary_mutation(expr.span, op.kind, var));
            }
            _ => {}
        };
        // @todo same as todo above, this should be working:
        // <Self as solar_parse::ast::visit::Visit<'ast>>::visit_expr(self, expr)

        let Expr { span, kind } = expr;
        self.visit_span(span)?;
        match kind {
            ExprKind::Array(exprs) => {
                for expr in exprs.iter() {
                    self.visit_expr(expr)?;
                }
            }
            ExprKind::Assign(lhs, _op, rhs) => {
                self.visit_expr(lhs)?;
                self.visit_expr(rhs)?;
            }
            ExprKind::Binary(lhs, _op, rhs) => {
                self.visit_expr(lhs)?;
                self.visit_expr(rhs)?;
            }
            ExprKind::Call(lhs, args) => {
                self.visit_expr(lhs)?;
                self.visit_call_args(args)?;
            }
            ExprKind::CallOptions(lhs, args) => {
                self.visit_expr(lhs)?;
                self.visit_named_args(args)?;
            }
            ExprKind::Delete(expr) => {
                self.visit_expr(expr)?;
            }
            ExprKind::Ident(ident) => {
                self.visit_ident(ident)?;
            }
            ExprKind::Index(lhs, kind) => {
                self.visit_expr(lhs)?;
                match kind {
                    IndexKind::Index(expr) => {
                        if let Some(expr) = expr {
                            self.visit_expr(expr)?;
                        }
                    }
                    IndexKind::Range(start, end) => {
                        if let Some(start) = start {
                            self.visit_expr(start)?;
                        }
                        if let Some(end) = end {
                            self.visit_expr(end)?;
                        }
                    }
                }
            }
            ExprKind::Lit(lit, _sub) => {
                self.visit_lit(lit)?;
            }
            ExprKind::Member(expr, member) => {
                self.visit_expr(expr)?;
                self.visit_ident(member)?;
            }
            ExprKind::New(ty) => {
                self.visit_ty(ty)?;
            }
            ExprKind::Payable(args) => {
                self.visit_call_args(args)?;
            }
            ExprKind::Ternary(cond, true_, false_) => {
                self.visit_expr(cond)?;
                self.visit_expr(true_)?;
                self.visit_expr(false_)?;
            }
            ExprKind::Tuple(exprs) => {
                for expr in exprs.iter().flatten() {
                    self.visit_expr(expr)?;
                }
            }
            ExprKind::TypeCall(ty) => {
                self.visit_ty(ty)?;
            }
            ExprKind::Type(ty) => {
                self.visit_ty(ty)?;
            }
            ExprKind::Unary(_op, expr) => {
                self.visit_expr(expr)?;
            }
        }
        ControlFlow::Continue(())
    }
}
