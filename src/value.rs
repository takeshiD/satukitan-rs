use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::{ast::Expr, env::Environment, error::SatukitanError};

#[derive(Clone, Copy, Debug)]
pub enum Arity {
    Exact(usize),
    AtLeast(usize),
    Any,
}

#[derive(Clone)]
pub enum Value {
    Number(i64),
    Bool(bool),
    String(String),
    List(Vec<Value>),
    Function(Rc<FunctionValue>),
    Builtin(BuiltinFunction),
    Nil,
}

#[derive(Clone)]
pub struct FunctionValue {
    pub params: Vec<String>,
    pub body: Vec<Expr>,
    pub env: Rc<RefCell<Environment>>,
}

#[derive(Clone, Copy)]
pub struct BuiltinFunction {
    pub name: &'static str,
    func: fn(&[Value]) -> Result<Value, SatukitanError>,
    arity: Arity,
}

impl FunctionValue {
    pub fn new(params: Vec<String>, body: Vec<Expr>, env: Rc<RefCell<Environment>>) -> Self {
        Self { params, body, env }
    }
}

impl BuiltinFunction {
    pub fn new(
        name: &'static str,
        arity: Arity,
        func: fn(&[Value]) -> Result<Value, SatukitanError>,
    ) -> Self {
        Self { name, func, arity }
    }

    pub fn call(&self, args: &[Value]) -> Result<Value, SatukitanError> {
        (self.func)(args)
    }

    pub fn arity(&self) -> Arity {
        self.arity
    }
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::Bool(_) => "boolean",
            Value::String(_) => "string",
            Value::List(_) => "list",
            Value::Function(_) => "function",
            Value::Builtin(_) => "builtin",
            Value::Nil => "nil",
        }
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
    }

    pub fn as_number(&self) -> Result<i64, SatukitanError> {
        match self {
            Value::Number(n) => Ok(*n),
            other => Err(SatukitanError::type_mismatch("number", other.type_name())),
        }
    }

    pub fn as_bool(&self) -> Result<bool, SatukitanError> {
        match self {
            Value::Bool(b) => Ok(*b),
            other => Err(SatukitanError::type_mismatch("boolean", other.type_name())),
        }
    }

    pub fn into_list(self) -> Result<Vec<Value>, SatukitanError> {
        match self {
            Value::List(items) => Ok(items),
            other => Err(SatukitanError::type_mismatch("list", other.type_name())),
        }
    }

    pub fn structural_eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::List(a), Value::List(b)) => {
                a.len() == b.len() && a.iter().zip(b).all(|(lhs, rhs)| lhs.structural_eq(rhs))
            }
            (Value::Function(a), Value::Function(b)) => Rc::ptr_eq(a, b),
            (Value::Builtin(a), Value::Builtin(b)) => a.name == b.name,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", lexeme_for_number(*n)),
            Value::Bool(true) => write!(f, "me"),
            Value::Bool(false) => write!(f, "ga"),
            Value::String(s) => write!(f, "{}", s),
            Value::List(items) => {
                write!(f, "[")?;
                for (idx, item) in items.iter().enumerate() {
                    if idx > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Function(func) => {
                write!(f, "<lambda (")?;
                for (idx, param) in func.params.iter().enumerate() {
                    if idx > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ")>")
            }
            Value::Builtin(builtin) => write!(f, "<builtin {}>", builtin.name),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Function(func) => f
                .debug_tuple("Function")
                .field(&format!("params={:?}", func.params))
                .finish(),
            Value::Builtin(builtin) => f.debug_tuple("Builtin").field(&builtin.name).finish(),
            _ => fmt::Display::fmt(self, f),
        }
    }
}

impl fmt::Debug for FunctionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionValue")
            .field("params", &self.params)
            .field("body_len", &self.body.len())
            .finish()
    }
}

impl fmt::Debug for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BuiltinFunction")
            .field("name", &self.name)
            .finish()
    }
}

fn lexeme_for_number(value: i64) -> String {
    match value {
        0 => "rv".to_string(),
        1 => "ru".to_string(),
        2 => "ra".to_string(),
        3 => "ro".to_string(),
        4 => "re".to_string(),
        5 => "ri".to_string(),
        6 => "rya".to_string(),
        7 => "ryu".to_string(),
        8 => "ryo".to_string(),
        9 => "rye".to_string(),
        10 => "#ta".to_string(),
        other => other.to_string(),
    }
}

pub fn parse_number_token(token: &str) -> Option<i64> {
    match token {
        "rv" => Some(0),
        "ru" => Some(1),
        "ra" => Some(2),
        "ro" => Some(3),
        "re" => Some(4),
        "ri" => Some(5),
        "rya" => Some(6),
        "ryu" => Some(7),
        "ryo" => Some(8),
        "rye" => Some(9),
        "#ta" => Some(10),
        _ => None,
    }
}

pub fn bool_token(token: &str) -> Option<bool> {
    match token {
        "ga" => Some(false),
        "me" => Some(true),
        _ => None,
    }
}
