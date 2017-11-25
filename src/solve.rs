use std::collections::{VecDeque, HashSet};

use var::{VarId, VarSet, DomainUpdate, Variable};
use propagate::PropSet;

#[derive(Debug)]
pub struct Solver<V> where V: Variable {
    state_stack: Vec<SearchState<V>>,
    initialized: bool,
}

#[derive(Debug)]
pub struct SearchState<V> where V: Variable {
    pub var_set: VarSet<V>,
    pub prop_set: PropSet<V>,
    pub instantiated_vars: HashSet<VarId>,
}

impl<V> Clone for SearchState<V> where V: Variable {
    fn clone(&self) -> SearchState<V> {
        SearchState {
            var_set: self.var_set.clone(),
            prop_set: self.prop_set.clone(),
            instantiated_vars: self.instantiated_vars.clone(),
        }
    }
}

impl<V> SearchState<V> where V: Variable {
    pub fn instantiate(
        self,
        var_id: VarId,
        value: &V::Value,
    ) -> (Option<SearchState<V>>, Option<SearchState<V>>) {
        let mut instantiated_state = self.clone();
        let mut removed_state = self;

        let mut instantiated_state_retval = None;
        let mut removed_state_retval = None;

        if let Ok(update) = removed_state.var_set.var_mut(var_id).remove(value) {
            match removed_state.propagate(update) {
                Ok(()) => {
                    removed_state_retval = Some(removed_state);
                }
                Err(()) => {}
            }
        }

        if let Ok(update) = instantiated_state.var_set.var_mut(var_id).instantiate(
            value,
        )
        {
            match instantiated_state.propagate(update) {
                Ok(()) => {
                    instantiated_state.instantiated_vars.insert(var_id);
                    instantiated_state_retval = Some(instantiated_state);
                }
                Err(()) => {}
            }
        }

        (instantiated_state_retval, removed_state_retval)
    }

    pub fn propagate(&mut self, update: DomainUpdate) -> Result<(), ()> {
        let mut queue = VecDeque::new();
        queue.push_back(update);
        while !queue.is_empty() {
            let latest_update = queue.pop_front().unwrap();
            // for prop_id in self.prop_set.prop_ids().clone() {
            for prop_id in self.var_set.subscriptions(latest_update.var_id()).clone() {
                let new_updates = self.prop_set.propagator_mut(prop_id).propagate(
                    &mut self.var_set,
                    latest_update,
                )?;
                for update in new_updates {
                    match update {
                        DomainUpdate::Unchanged(_) => {}
                        _ => {
                            queue.push_back(update);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn choose_var(&self) -> Option<VarId> {
        self.var_set
            .var_ids()
            .iter()
            .filter(|id| !self.instantiated_vars.contains(id))
            .min_by_key(|id| self.var_set.var(**id).size())
            .cloned()
    }

    pub fn choose_value(&self, var_id: VarId) -> Option<&V::Value> {
        self.var_set.var(var_id).possibilities().nth(0)
    }

    pub fn initial_propagation(&mut self) -> Result<(), ()> {
        let mut domain_updates = vec![];
        for &prop_id in &self.prop_set.prop_ids().clone() {
            let updates = self.prop_set.propagator_mut(prop_id).initial_propagation(
                &mut self.var_set,
            )?;
            for update in updates {
                domain_updates.push(update);
            }
        }
        for update in domain_updates {
            self.propagate(update)?;
        }
        Ok(())
    }
}

impl<V> Solver<V> where V: Variable {
    pub fn new(var_set: VarSet<V>, prop_set: PropSet<V>) -> Solver<V> {
        let state = SearchState {
            var_set: var_set,
            prop_set: prop_set,
            instantiated_vars: HashSet::new(),
        };
        Solver {
            state_stack: vec![state],
            initialized: false,
        }
    }
}

impl<V> Iterator for Solver<V> where V: Variable {
    type Item = VarSet<V>;

    fn next(&mut self) -> Option<VarSet<V>> {
        if !self.initialized {
            if let Err(_) = self.state_stack[0].initial_propagation() {
                return None;
            }
            self.initialized = true;
        }
        while let Some(current_state) = self.state_stack.pop() {
            if let Some(next_var) = current_state.choose_var() {
                if let Some(next_value) = current_state.choose_value(next_var).cloned() {
                    let (instantiated_state, removed_state) =
                        current_state.instantiate(next_var, &next_value);
                    if let Some(state) = removed_state {
                        self.state_stack.push(state);
                    }
                    if let Some(state) = instantiated_state {
                        self.state_stack.push(state);
                    }
                } else {
                    continue;
                }
            } else {
                return Some(current_state.var_set);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
}
