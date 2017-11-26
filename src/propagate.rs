use std::collections::HashSet;
use std::fmt::Debug;
use std::borrow::Borrow;
use std::marker::PhantomData;

use var::{VarSet, VarId, DomainUpdate, Variable};

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct PropId {
    id: usize,
}

pub trait Propagate {
    type Variable: Variable;

    fn propagate(
        &mut self,
        vars: &mut VarSet<Self::Variable>,
        update: DomainUpdate,
    ) -> Result<HashSet<DomainUpdate>, ()>;

    fn initial_propagation(
        &mut self,
        vars: &mut VarSet<Self::Variable>,
    ) -> Result<HashSet<DomainUpdate>, ()>;

    fn boxed_clone(&self) -> Box<Propagate<Variable = Self::Variable>>;

    fn set_id(&mut self, id: PropId);
}

impl<V> Clone for Box<Propagate<Variable = V>>
where
    V: Variable,
{
    fn clone(&self) -> Box<Propagate<Variable = V>> {
        self.boxed_clone()
    }
}

#[derive(Clone)]
pub struct PropSet<V>
where
    V: Variable,
{
    propagators: Vec<Box<Propagate<Variable = V>>>,
    prop_ids: Vec<PropId>,
}

impl<V> PropSet<V>
where
    V: Variable,
{
    pub fn new() -> PropSet<V> {
        PropSet {
            propagators: vec![],
            prop_ids: vec![],
        }
    }

    pub fn add_propagator<P>(&mut self, mut propagator: P) -> PropId
    where
        P: Propagate<Variable = V> + 'static,
    {
        let prop_id = PropId { id: self.propagators.len() };
        self.prop_ids.push(prop_id);
        propagator.set_id(prop_id);
        self.propagators.push(Box::new(propagator));
        prop_id
    }

    pub fn propagator(&self, prop_id: PropId) -> &Box<Propagate<Variable = V>> {
        &self.propagators[prop_id.id]
    }

    pub fn propagator_mut(&mut self, prop_id: PropId) -> &mut Box<Propagate<Variable = V>> {
        &mut self.propagators[prop_id.id]
    }

    pub fn prop_ids(&self) -> &Vec<PropId> {
        &self.prop_ids
    }
}

#[derive(Clone, Debug)]
pub struct AllDifferent<V> {
    id: PropId,
    var_ids: HashSet<VarId>,
    phantom: PhantomData<V>,
}

impl<V> AllDifferent<V> {
    pub fn new<I, Q>(var_ids: I) -> AllDifferent<V>
    where
        I: IntoIterator<Item = Q>,
        Q: Borrow<VarId>,
    {
        let mut hashset = HashSet::new();
        hashset.extend(var_ids.into_iter().map(|id| *id.borrow()));
        AllDifferent {
            var_ids: hashset,
            id: PropId { id: 0 },
            phantom: PhantomData::default(),
        }
    }
}

impl<V> Propagate for AllDifferent<V>
where
    V: Variable + Clone + 'static,
{
    type Variable = V;

    fn propagate(
        &mut self,
        vars: &mut VarSet<V>,
        update: DomainUpdate,
    ) -> Result<HashSet<DomainUpdate>, ()> {
        let mut domain_updates = HashSet::new();
        if let DomainUpdate::Fixed(fixed_id) = update {
            if self.var_ids.contains(&fixed_id) {
                // self.var_ids.remove(&fixed_id);
                let value = vars.var(fixed_id).value().unwrap().clone();
                for &var_id in &self.var_ids {
                    if var_id != fixed_id {
                        domain_updates.insert(vars.var_mut(var_id).remove(&value)?);
                    }
                }
            }
        };
        Ok(domain_updates)
    }

    fn initial_propagation(&mut self, vars: &mut VarSet<V>) -> Result<HashSet<DomainUpdate>, ()> {
        for &var_id in &self.var_ids {
            vars.subscribe(var_id, self.id);
        }

        let mut domain_updates = HashSet::new();
        for &var_id in &self.var_ids {
            if let Some(value) = vars.var(var_id).value().cloned() {
                for &other in &self.var_ids {
                    if var_id != other {
                        domain_updates.insert(vars.var_mut(other).remove(&value)?);
                    }
                }
            }
        }
        Ok(domain_updates)
    }

    fn boxed_clone(&self) -> Box<Propagate<Variable = V>> {
        Box::new((*self).clone())
    }

    fn set_id(&mut self, id: PropId) {
        self.id = id;
    }
}
