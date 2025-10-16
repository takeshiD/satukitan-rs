use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::Program;
use crate::builtins;
use crate::env::Environment;
use crate::error::SatukitanError;
use crate::evaluator::eval_program;
use crate::parser::parse_program;
use crate::value::Value;

pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut root = Environment::new();
        builtins::install(&mut root);
        Self {
            env: Rc::new(RefCell::new(root)),
        }
    }

    pub fn eval_str(&mut self, source: &str) -> Result<Value, SatukitanError> {
        let program = parse_program(source)?;
        eval_program(&program, self.env.clone())
    }

    pub fn eval_program(&mut self, program: &Program) -> Result<Value, SatukitanError> {
        eval_program(program, self.env.clone())
    }

    pub fn environment(&self) -> Rc<RefCell<Environment>> {
        self.env.clone()
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
