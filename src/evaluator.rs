use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::Expr;
use crate::env::Environment;
use crate::error::SatukitanError;
use crate::value::{FunctionValue, Value};

pub fn eval_program(
    program: &[Expr],
    env: Rc<RefCell<Environment>>,
) -> Result<Value, SatukitanError> {
    let mut last = Value::Nil;
    for expr in program {
        last = eval_expr(expr, env.clone())?;
    }
    Ok(last)
}

pub fn eval_expr(expr: &Expr, env: Rc<RefCell<Environment>>) -> Result<Value, SatukitanError> {
    match expr {
        Expr::Number(value) => Ok(Value::Number(*value)),
        Expr::Bool(value) => Ok(Value::Bool(*value)),
        Expr::String(value) => Ok(Value::String(value.clone())),
        Expr::Symbol(name) => env
            .borrow()
            .get(name)
            .ok_or_else(|| SatukitanError::UndefinedSymbol(name.clone())),
        Expr::ListLiteral(items) => {
            let mut values = Vec::with_capacity(items.len());
            for item in items {
                values.push(eval_expr(item, env.clone())?);
            }
            Ok(Value::List(values))
        }
        Expr::List(items) => eval_list(items, env),
        Expr::Call { func, args } => eval_call(func, args, env),
    }
}

fn eval_list(items: &[Expr], env: Rc<RefCell<Environment>>) -> Result<Value, SatukitanError> {
    if items.is_empty() {
        return Ok(Value::List(Vec::new()));
    }

    if let Expr::Symbol(name) = &items[0] {
        eval_symbolic_application(name, &items[1..], env)
    } else {
        eval_block(items, env)
    }
}

fn eval_call(
    func: &Expr,
    args: &[Expr],
    env: Rc<RefCell<Environment>>,
) -> Result<Value, SatukitanError> {
    if let Expr::Symbol(name) = func {
        eval_symbolic_application(name, args, env)
    } else {
        let callable = eval_expr(func, env.clone())?;
        let evaluated_args = eval_arguments(args, env)?;
        apply_callable(callable, evaluated_args)
    }
}

fn eval_symbolic_application(
    name: &str,
    args: &[Expr],
    env: Rc<RefCell<Environment>>,
) -> Result<Value, SatukitanError> {
    match name {
        "nobu" => eval_nobu(args, env),
        "gakas" => eval_gakas(args, env),
        "gakasdenu" => eval_gakasdenu(args, env),
        _ => {
            let callable = env
                .borrow()
                .get(name)
                .ok_or_else(|| SatukitanError::UndefinedSymbol(name.to_string()))?;
            let evaluated_args = eval_arguments(args, env)?;
            apply_callable(callable, evaluated_args)
        }
    }
}

fn eval_nobu(args: &[Expr], env: Rc<RefCell<Environment>>) -> Result<Value, SatukitanError> {
    if args.len() != 3 {
        return Err(SatukitanError::arity_exact(
            "nobu".to_string(),
            3,
            args.len(),
        ));
    }

    let condition = eval_expr(&args[0], env.clone())?;
    let branch = match condition {
        Value::Bool(true) => &args[1],
        Value::Bool(false) => &args[2],
        other => return Err(SatukitanError::type_mismatch("boolean", other.type_name())),
    };

    eval_expr(branch, env)
}

fn eval_gakas(args: &[Expr], env: Rc<RefCell<Environment>>) -> Result<Value, SatukitanError> {
    if args.len() != 2 {
        return Err(SatukitanError::arity_exact(
            "gakas".to_string(),
            2,
            args.len(),
        ));
    }

    let name = match &args[0] {
        Expr::Symbol(name) => name.clone(),
        _ => {
            return Err(SatukitanError::Eval(
                "gakas: first argument must be symbol".into(),
            ));
        }
    };

    let value = eval_expr(&args[1], env.clone())?;
    {
        let mut env_mut = env.borrow_mut();
        env_mut.define(name.clone(), value.clone());
    }
    Ok(value)
}

fn eval_gakasdenu(args: &[Expr], env: Rc<RefCell<Environment>>) -> Result<Value, SatukitanError> {
    if args.len() != 3 {
        return Err(SatukitanError::arity_exact(
            "gakasdenu".to_string(),
            3,
            args.len(),
        ));
    }

    let name = match &args[0] {
        Expr::Symbol(name) => name.clone(),
        _ => {
            return Err(SatukitanError::Eval(
                "gakasdenu: function name must be symbol".into(),
            ));
        }
    };

    let params = extract_params(&args[1])?;
    let body = extract_body(&args[2]);

    let function = Rc::new(FunctionValue::new(params, body, env.clone()));
    let value = Value::Function(function.clone());

    {
        let mut env_mut = env.borrow_mut();
        env_mut.define(name, value.clone());
    }

    Ok(value)
}

fn extract_params(expr: &Expr) -> Result<Vec<String>, SatukitanError> {
    match expr {
        Expr::List(items) => {
            let mut params = Vec::with_capacity(items.len());
            for item in items {
                match item {
                    Expr::Symbol(name) => params.push(name.clone()),
                    _ => {
                        return Err(SatukitanError::Eval(
                            "gakasdenu: parameter list must contain symbols only".into(),
                        ));
                    }
                }
            }
            Ok(params)
        }
        _ => Err(SatukitanError::Eval(
            "gakasdenu: second argument must be parameter list".into(),
        )),
    }
}

fn extract_body(expr: &Expr) -> Vec<Expr> {
    match expr {
        Expr::List(items) => {
            if items.iter().all(|item| matches!(item, Expr::List(_))) {
                items.clone()
            } else {
                vec![Expr::List(items.clone())]
            }
        }
        other => vec![other.clone()],
    }
}

fn eval_block(items: &[Expr], env: Rc<RefCell<Environment>>) -> Result<Value, SatukitanError> {
    let mut last = Value::Nil;
    for expr in items {
        last = eval_expr(expr, env.clone())?;
    }
    Ok(last)
}

fn eval_arguments(
    args: &[Expr],
    env: Rc<RefCell<Environment>>,
) -> Result<Vec<Value>, SatukitanError> {
    let mut values = Vec::with_capacity(args.len());
    for expr in args {
        values.push(eval_expr(expr, env.clone())?);
    }
    Ok(values)
}

fn apply_callable(func: Value, args: Vec<Value>) -> Result<Value, SatukitanError> {
    match func {
        Value::Builtin(builtin) => builtin.call(&args),
        Value::Function(function) => apply_function(function, args),
        other => Err(SatukitanError::Eval(format!(
            "attempted to call non-callable value of type {}",
            other.type_name()
        ))),
    }
}

fn apply_function(function: Rc<FunctionValue>, args: Vec<Value>) -> Result<Value, SatukitanError> {
    if function.params.len() != args.len() {
        return Err(SatukitanError::arity_exact(
            "lambda".to_string(),
            function.params.len(),
            args.len(),
        ));
    }

    let child_env = Rc::new(RefCell::new(Environment::with_parent(function.env.clone())));

    for (param, value) in function.params.iter().zip(args.into_iter()) {
        child_env.borrow_mut().define(param.clone(), value);
    }

    eval_block(&function.body, child_env)
}
