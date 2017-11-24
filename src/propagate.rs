use std::collections::HashSet;
use std::fmt::Debug;
use std::borrow::Borrow;

use var::{VarSet, VarId, DomainUpdate, Variable};

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct PropId {
    id: usize,
}

pub trait Propagate: Debug {
    fn propagate(
        &mut self,
        vars: &mut VarSet,
        update: DomainUpdate,
    ) -> Result<HashSet<DomainUpdate>, ()>;

    fn initial_propagation(&mut self, vars: &mut VarSet) -> Result<HashSet<DomainUpdate>, ()>;

    fn boxed_clone(&self) -> Box<Propagate>;

    fn set_id(&mut self, id: PropId);
}

#[derive(Clone, Debug, Default)]
pub struct PropSet {
    propagators: Vec<Box<Propagate>>,
    prop_ids: Vec<PropId>,
}

impl PropSet {
    pub fn new() -> PropSet {
        PropSet::default()
    }

    pub fn add_propagator<P>(&mut self, mut propagator: P) -> PropId
    where
        P: Propagate + 'static,
    {
        let prop_id = PropId { id: self.propagators.len() };
        self.prop_ids.push(prop_id);
        propagator.set_id(prop_id);
        self.propagators.push(Box::new(propagator));
        prop_id
    }

    pub fn propagator(&self, prop_id: PropId) -> &Box<Propagate> {
        &self.propagators[prop_id.id]
    }

    pub fn propagator_mut(&mut self, prop_id: PropId) -> &mut Box<Propagate> {
        &mut self.propagators[prop_id.id]
    }

    pub fn prop_ids(&self) -> &Vec<PropId> {
        &self.prop_ids
    }
}

impl Clone for Box<Propagate> {
    fn clone(&self) -> Box<Propagate> {
        self.boxed_clone()
    }
}

#[derive(Clone, Debug)]
pub struct AllDifferent {
    var_ids: HashSet<VarId>,
    id: PropId,
}

impl AllDifferent {
    pub fn new<I, Q>(var_ids: I) -> AllDifferent
    where
        I: IntoIterator<Item = Q>,
        Q: Borrow<VarId>,
    {
        let mut hashset = HashSet::new();
        hashset.extend(var_ids.into_iter().map(|id| *id.borrow()));
        AllDifferent { var_ids: hashset, id: PropId { id: 0 }, }
    }
}

impl Propagate for AllDifferent {
    fn propagate(
        &mut self,
        vars: &mut VarSet,
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

    fn initial_propagation(&mut self, vars: &mut VarSet) -> Result<HashSet<DomainUpdate>, ()> {
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

    fn boxed_clone(&self) -> Box<Propagate> {
        Box::new(self.clone())
    }

    fn set_id(&mut self, id: PropId) {
        self.id = id;
    }
}
