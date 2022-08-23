use crate::MessageFormat;
use crate::{ast, types::VariableType};
use std::collections::HashMap;

// 'vars - lifetime of variables map
// 'msgs - lifetime of messages map
// 'msgsv - lifetime of message values
// VARSV - variable value type
// MSGSV - messages value type
pub struct Scope<'b, 'mf, 'vars, 'varsv> {
    pub mf: &'mf MessageFormat<'b>,
    pub variables: Option<&'vars Variables<'varsv>>,
}

impl<'b, 'mf, 'vars, 'varsv> Scope<'b, 'mf, 'vars, 'varsv> {
    pub fn new(mf: &'mf MessageFormat<'b>, variables: Option<&'vars Variables<'varsv>>) -> Self {
        Self { mf, variables }
    }
}

pub struct Variables<'v>(pub HashMap<String, Box<dyn VariableType<'v> + 'v>>);

impl<'v> Variables<'v> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert<T: VariableType<'v> + 'v>(&mut self, key: String, value: T) {
        self.0.insert(key, Box::new(value));
    }

    pub fn get(&self, key: &str) -> Option<&dyn VariableType<'v>> {
        let candidate = self.0.get(key)?;
        let v: &dyn VariableType = candidate.as_ref();
        Some(v)
    }
}
