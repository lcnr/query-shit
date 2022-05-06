use super::*;
use std::fmt::{self, Write};

#[derive(Clone, Copy)]
pub struct DisplayFormula<'a> {
    formula: &'a Formula,
    variables: &'a IndexSet<Variable>,
}

impl DisplayFormula<'_> {
    fn recurse(&self, n: &Formula, prec: usize) -> String {
        use Formula::*;
        let (p, c, children) = match n {
            And(ref nodes) => (10, '∧', nodes),
            Or(ref nodes) => (8, '∨', nodes),
            Implies(ref lhs, ref rhs) => {
                return format!(
                    "({} -> {})",
                    self.recurse(lhs, usize::MAX),
                    self.recurse(rhs, usize::MAX)
                )
            }
            Neg(ref node) => return format!("¬{}", self.recurse(node, 4)),
            &Var(id) => {
                let var = self.variables.get_index(id).unwrap();
                return format!("{}({},{})", var.component, var.index, var.mutable);
            }
        };

        let mut s = String::new();
        if prec < p {
            s.push('(');
        }
        let (fst, rest) = children.split_first().unwrap();
        write!(s, "{}", self.recurse(fst, p)).unwrap();
        for r in rest {
            write!(s, " {} {}", c, self.recurse(r, p)).unwrap();
        }

        if prec < p {
            s.push(')');
        }
        s
    }
}

impl fmt::Display for DisplayFormula<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.recurse(&self.formula, usize::MAX))
    }
}

impl Formula {
    pub fn display<'a>(&'a self, variables: &'a IndexSet<Variable>) -> DisplayFormula<'a> {
        DisplayFormula {
            formula: self,
            variables,
        }
    }
}
