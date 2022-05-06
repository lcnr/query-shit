use std::collections::HashMap;

use indexmap::IndexSet;

mod display;

#[derive(Debug)]
pub enum Query {
    All(Vec<Query>),
    Any(Vec<Query>),
    Without(&'static str),
    ReadComponent(&'static str),
    WriteComponent(&'static str),
}

#[derive(Debug)]
pub enum Formula {
    And(Vec<Formula>),
    Or(Vec<Formula>),
    Neg(Box<Formula>),
    Var(usize),
}

impl Formula {
    pub fn holds(&self, assignment: &[bool]) -> bool {
        match self {
            Formula::And(inner) => inner.iter().all(|f| f.holds(assignment)),
            Formula::Or(inner) => inner.iter().any(|f| f.holds(assignment)),
            Formula::Neg(inner) => !inner.holds(assignment),
            &Formula::Var(var) => assignment[var],
        }
    }

    pub fn compute_assignment(&self, variables: &IndexSet<Variable>) -> Option<Vec<bool>> {
        for assign in 0..(1 << variables.len()) {
            let mut assignment = Vec::new();
            for i in 0..variables.len() {
                assignment.push(assign & (1 << i) != 0);
            }

            if self.holds(&assignment) {
                return Some(assignment);
            }
        }
        None
    }
}

#[derive(Default)]
struct Context {
    components: HashMap<&'static str, ForComponent>,
    variables: IndexSet<Variable>,
}

impl Context {
    fn to_var(&mut self, component: &'static str, mutable: bool) -> usize {
        let entry = self.components.entry(component).or_default();
        let index = if mutable {
            let index = entry.num_mut;
            entry.num_mut += 1;
            index
        } else {
            let index = entry.num_immut;
            entry.num_immut += 1;
            index
        };
        self.variables
            .insert_full(Variable {
                component,
                index,
                mutable,
            })
            .0
    }

    fn get_var(&self, var: Variable) -> usize {
        self.variables.get_index_of(&var).unwrap()
    }

    fn query_to_formula(&mut self, query: &Query) -> Formula {
        match query {
            Query::All(queries) => Formula::And(
                queries
                    .iter()
                    .map(|query| self.query_to_formula(query))
                    .collect(),
            ),
            Query::Any(queries) => Formula::Or(
                queries
                    .iter()
                    .map(|query| self.query_to_formula(query))
                    .collect(),
            ),
            Query::Without(component) => {
                Formula::Neg(Box::new(Formula::Var(self.to_var(component, false))))
            }
            Query::ReadComponent(component) => Formula::Var(self.to_var(component, false)),
            Query::WriteComponent(component) => Formula::Var(self.to_var(component, true)),
        }
    }

    // The formula itself is mostly alright, but well, it doesn't detect conflicts yet.
    fn finalize(mut self, formula: Formula) -> (Formula, IndexSet<Variable>) {
        let mut formulas = vec![formula];
        for (component, data) in std::mem::take(&mut self.components) {
            for i in 0..data.num_mut {
                let mut others = Vec::new();
                for j in 0..data.num_immut {
                    others.push(Formula::Var(self.get_var(Variable {
                        component,
                        index: j,
                        mutable: false,
                    })));
                }

                for j in 0..data.num_mut {
                    if j != i {
                        others.push(Formula::Var(self.get_var(Variable {
                            component,
                            index: j,
                            mutable: true,
                        })));
                    }
                }

                let this = Formula::Var(self.get_var(Variable {
                    component,
                    index: i,
                    mutable: true,
                }));

                formulas.push(Formula::Or(vec![
                    Formula::Neg(Box::new(this)),
                    Formula::Neg(Box::new(Formula::Or(others))),
                ]));
            }
        }

        (Formula::And(formulas), self.variables)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Variable {
    component: &'static str,
    index: usize,
    mutable: bool,
}

#[derive(Default)]
struct ForComponent {
    num_immut: usize,
    num_mut: usize,
}

pub fn to_formula(queries: Vec<Query>) -> (Formula, IndexSet<Variable>) {
    let mut ctx = Context::default();

    // Cheater move, just and all the queries:
    let mut and = Vec::new();
    for query in &queries {
        and.push(ctx.query_to_formula(query));
    }

    ctx.finalize(Formula::And(and))
}
