use std::collections::{BTreeSet, HashSet};
use std::borrow::Borrow;

use propagate::PropId;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct VarId {
    id: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum DomainUpdate {
    Unchanged(VarId),
    Reduced(VarId),
    Fixed(VarId),
}

impl DomainUpdate {
    pub fn var_id(&self) -> VarId {
        match self {
            &DomainUpdate::Unchanged(v) => v,
            &DomainUpdate::Reduced(v) => v,
            &DomainUpdate::Fixed(v) => v,
        }
    }
}

pub type VarResult<T> = Result<T, ()>;

pub trait Variable: Clone {
    type Value: Clone;

    fn with_domain<I, Q>(values: I) -> Self
    where
        I: IntoIterator<Item = Q>,
        Q: Borrow<Self::Value>;

    fn id(&self) -> VarId;
    fn set_id(&mut self, id: VarId);

    fn size(&self) -> usize;
    fn contains(&self, value: &Self::Value) -> bool;
    fn value(&self) -> Option<&Self::Value>;
    fn possibilities<'a>(&'a self) -> Box<Iterator<Item = &Self::Value> + 'a>;

    fn remove(&mut self, value: &Self::Value) -> VarResult<DomainUpdate>;
    fn instantiate(&mut self, value: &Self::Value) -> VarResult<DomainUpdate>;
}

#[derive(Clone, Debug)]
pub struct BTreeSetVar<T> {
    id: VarId,
    domain: BTreeSet<T>,
}

impl<T> Variable for BTreeSetVar<T>
where
    T: Clone + Ord,
{
    type Value = T;

    fn with_domain<I, Q>(values: I) -> BTreeSetVar<T>
    where
        I: IntoIterator<Item = Q>,
        Q: Borrow<T>,
    {
        let mut domain = BTreeSet::new();
        domain.extend(values.into_iter().map(|q| q.borrow().clone()));
        BTreeSetVar {
            id: VarId::default(),
            domain: domain,
        }
    }

    fn id(&self) -> VarId {
        self.id
    }

    fn set_id(&mut self, id: VarId) {
        self.id = id;
    }

    fn size(&self) -> usize {
        self.domain.len()
    }

    fn contains(&self, value: &Self::Value) -> bool {
        self.domain.contains(value)
    }

    fn value(&self) -> Option<&Self::Value> {
        if self.size() == 1 {
            self.domain.iter().nth(0)
        } else {
            None
        }
    }

    fn possibilities<'a>(&'a self) -> Box<Iterator<Item = &Self::Value> + 'a> {
        Box::new(self.domain.iter())
    }

    fn remove(&mut self, value: &Self::Value) -> VarResult<DomainUpdate> {
        if self.domain.remove(value) {
            match self.size() {
                0 => Err(()),
                1 => Ok(DomainUpdate::Fixed(self.id)),
                _ => Ok(DomainUpdate::Reduced(self.id)),
            }
        } else {
            Ok(DomainUpdate::Unchanged(self.id))
        }
    }

    fn instantiate(&mut self, value: &Self::Value) -> VarResult<DomainUpdate> {
        if self.contains(value) {
            if self.size() == 1 {
                Ok(DomainUpdate::Unchanged(self.id))
            } else {
                self.domain = BTreeSet::new();
                self.domain.insert(value.clone());
                Ok(DomainUpdate::Fixed(self.id))
            }
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct HashSetVar<T>
where
    T: Eq + ::std::hash::Hash,
{
    id: VarId,
    domain: HashSet<T>,
}

impl<T> Variable for HashSetVar<T>
where
    T: Clone + Eq + ::std::hash::Hash,
{
    type Value = T;

    fn with_domain<I, Q>(values: I) -> HashSetVar<T>
    where
        I: IntoIterator<Item = Q>,
        Q: Borrow<T>,
    {
        let mut domain = HashSet::new();
        domain.extend(values.into_iter().map(|q| q.borrow().clone()));
        HashSetVar {
            id: VarId::default(),
            domain: domain,
        }
    }

    fn id(&self) -> VarId {
        self.id
    }

    fn set_id(&mut self, id: VarId) {
        self.id = id;
    }

    fn size(&self) -> usize {
        self.domain.len()
    }

    fn contains(&self, value: &Self::Value) -> bool {
        self.domain.contains(value)
    }

    fn value(&self) -> Option<&Self::Value> {
        if self.size() == 1 {
            self.domain.iter().nth(0)
        } else {
            None
        }
    }

    fn possibilities<'a>(&'a self) -> Box<Iterator<Item = &Self::Value> + 'a> {
        Box::new(self.domain.iter())
    }

    fn remove(&mut self, value: &Self::Value) -> VarResult<DomainUpdate> {
        if self.domain.remove(value) {
            match self.size() {
                0 => Err(()),
                1 => Ok(DomainUpdate::Fixed(self.id)),
                _ => Ok(DomainUpdate::Reduced(self.id)),
            }
        } else {
            Ok(DomainUpdate::Unchanged(self.id))
        }
    }

    fn instantiate(&mut self, value: &Self::Value) -> VarResult<DomainUpdate> {
        if self.contains(value) {
            if self.size() == 1 {
                Ok(DomainUpdate::Unchanged(self.id))
            } else {
                self.domain = HashSet::new();
                self.domain.insert(value.clone());
                Ok(DomainUpdate::Fixed(self.id))
            }
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct VarSet<V>
where
    V: Variable,
{
    vars: Vec<V>,
    var_ids: Vec<VarId>,
    subscriptions: Vec<Vec<PropId>>,
}

impl<V> VarSet<V>
where
    V: Variable,
{
    pub fn new() -> VarSet<V> {
        VarSet {
            vars: vec![],
            var_ids: vec![],
            subscriptions: vec![],
        }
    }

    pub fn create_var<I, Q>(&mut self, values: I) -> VarId
    where
        I: IntoIterator<Item = Q>,
        Q: Borrow<V::Value>,
    {
        let var_id = VarId { id: self.vars.len() };
        let mut var = V::with_domain(values);
        var.set_id(var_id);
        self.vars.push(var);
        self.var_ids.push(var_id);
        self.subscriptions.push(vec![]);
        var_id
    }

    pub fn set(&mut self, var_id: VarId, value: &V::Value) {
        let mut new_var = V::with_domain(vec![value]);
        new_var.set_id(var_id);
        self.vars[var_id.id] = new_var;
    }

    pub fn var(&self, var_id: VarId) -> &V {
        &self.vars[var_id.id]
    }

    pub fn var_mut(&mut self, var_id: VarId) -> &mut V {
        &mut self.vars[var_id.id]
    }

    pub fn var_id(&self, num: usize) -> VarId {
        self.var_ids[num]
    }

    pub fn var_ids(&self) -> &Vec<VarId> {
        &self.var_ids
    }

    pub fn size(&self) -> usize {
        self.vars.len()
    }

    pub fn check(&self) -> bool {
        for var in &self.vars {
            if var.size() != 1 {
                return false;
            }
        }
        true
    }

    pub fn subscribe(&mut self, var_id: VarId, prop_id: PropId) {
        self.subscriptions[var_id.id].push(prop_id);
    }

    pub fn subscriptions(&self, var_id: VarId) -> &Vec<PropId> {
        &self.subscriptions[var_id.id]
    }
}
