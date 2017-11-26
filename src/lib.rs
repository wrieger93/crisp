pub mod var;
pub mod propagate;
pub mod solve;

use std::borrow::Borrow;

use var::{VarSet, Variable, VarId};
use propagate::{PropSet, PropId, Propagate};
use solve::Solver;

pub struct Model<V>
where
    V: Variable,
{
    var_set: VarSet<V>,
    prop_set: PropSet<V>,
}

impl<V> Model<V>
where
    V: Variable,
{
    pub fn new() -> Model<V> {
        Model {
            var_set: VarSet::new(),
            prop_set: PropSet::new(),
        }
    }

    pub fn create_var<I, Q>(&mut self, values: I) -> VarId
    where
        I: IntoIterator<Item = Q>,
        Q: Borrow<V::Value>,
    {
        self.var_set.create_var(values)
    }

    pub fn create_var_array<I, Q>(&mut self, values: I, len: usize) -> Vec<VarId>
    where
        I: IntoIterator<Item = Q>,
        Q: Borrow<V::Value>,
    {
        let mut var_ids = vec![];
        let domain_vec = values
            .into_iter()
            .map(|q| q.borrow().clone())
            .collect::<Vec<_>>();
        for _ in 0..len {
            var_ids.push(self.create_var(&domain_vec));
        }
        var_ids
    }

    pub fn create_var_matrix<I, Q>(
        &mut self,
        values: I,
        rows: usize,
        cols: usize,
    ) -> Vec<Vec<VarId>>
    where
        I: IntoIterator<Item = Q>,
        Q: Borrow<V::Value>,
    {
        let mut var_ids = vec![];
        let domain_vec = values
            .into_iter()
            .map(|q| q.borrow().clone())
            .collect::<Vec<_>>();
        for _ in 0..rows {
            var_ids.push(self.create_var_array(&domain_vec, cols));
        }
        var_ids
    }

    pub fn set(&mut self, var_id: VarId, value: &V::Value) {
        self.var_set.set(var_id, value)
    }

    pub fn all_different<I, Q>(&mut self, var_ids: I) -> PropId
    where
        I: IntoIterator<Item = Q>,
        Q: Borrow<VarId>,
        V: 'static,
    {
        self.prop_set.add_propagator(
            propagate::AllDifferent::new(var_ids),
        )
    }

    pub fn add_propagator<P>(&mut self, propagator: P) -> PropId
    where
        P: Propagate<Variable = V> + 'static,
    {
        self.prop_set.add_propagator(propagator)
    }

    pub fn solve(&self) -> Solver<V> {
        Solver::new(self.var_set.clone(), self.prop_set.clone())
    }
}
