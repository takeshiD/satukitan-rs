use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::SatukitanError;
use crate::value::{BuiltinFunction, Value};

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn define(&mut self, name: impl Into<String>, value: Value) {
        self.values.insert(name.into(), value);
    }

    pub fn define_builtin(
        &mut self,
        name: &'static str,
        func: fn(&[Value]) -> Result<Value, SatukitanError>,
    ) {
        self.define(name, Value::Builtin(BuiltinFunction::new(name, func)));
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), SatukitanError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().assign(name, value)
        } else {
            Err(SatukitanError::UndefinedSymbol(name.to_string()))
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get(name)
        } else {
            None
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
