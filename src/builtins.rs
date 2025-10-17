use crate::env::Environment;
use crate::error::SatukitanError;
use crate::value::{Arity, Value};

pub fn install(env: &mut Environment) {
    env.define_builtin("ritas", Arity::AtLeast(2), builtin_add);
    env.define_builtin("matyes", Arity::AtLeast(2), builtin_sub);
    env.define_builtin("nitas", Arity::AtLeast(2), builtin_mul);
    env.define_builtin("teses", Arity::AtLeast(2), builtin_and);
    env.define_builtin("kenus", Arity::AtLeast(2), builtin_or);
    env.define_builtin("ditas", Arity::Exact(2), builtin_lt);
    env.define_builtin("fityes", Arity::Exact(2), builtin_gt);
    env.define_builtin("gatas", Arity::AtLeast(2), builtin_eq);
    env.define_builtin("ditasgata", Arity::Exact(2), builtin_le);
    env.define_builtin("fityesgata", Arity::Exact(2), builtin_ge);
    env.define_builtin("fanitas", Arity::Exact(1), builtin_sort);
    env.define_builtin("rakas", Arity::Exact(1), builtin_length);
    env.define_builtin("sipus", Arity::Any, builtin_print);
}

fn builtin_add(args: &[Value]) -> Result<Value, SatukitanError> {
    ensure_at_least("ritas", args, 2)?;
    let mut sum = 0i64;
    for value in args {
        sum += expect_number("ritas", value)?;
    }
    Ok(Value::Number(sum))
}

fn builtin_sub(args: &[Value]) -> Result<Value, SatukitanError> {
    ensure_at_least("matyes", args, 2)?;
    let mut iter = args.iter();
    let first = expect_number("matyes", iter.next().unwrap())?;
    let mut result = first;
    for value in iter {
        result -= expect_number("matyes", value)?;
    }
    Ok(Value::Number(result))
}

fn builtin_mul(args: &[Value]) -> Result<Value, SatukitanError> {
    ensure_at_least("nitas", args, 2)?;
    let mut product = 1i64;
    for value in args {
        product *= expect_number("nitas", value)?;
    }
    Ok(Value::Number(product))
}

fn builtin_and(args: &[Value]) -> Result<Value, SatukitanError> {
    ensure_at_least("teses", args, 2)?;
    let mut result = true;
    for value in args {
        result &= expect_bool("teses", value)?;
    }
    Ok(Value::Bool(result))
}

fn builtin_or(args: &[Value]) -> Result<Value, SatukitanError> {
    ensure_at_least("kenus", args, 2)?;
    let mut result = false;
    for value in args {
        result |= expect_bool("kenus", value)?;
    }
    Ok(Value::Bool(result))
}

fn builtin_lt(args: &[Value]) -> Result<Value, SatukitanError> {
    ensure_exact("ditas", args, 2)?;
    let a = expect_number("ditas", &args[0])?;
    let b = expect_number("ditas", &args[1])?;
    Ok(Value::Bool(a < b))
}

fn builtin_gt(args: &[Value]) -> Result<Value, SatukitanError> {
    ensure_exact("fityes", args, 2)?;
    let a = expect_number("fityes", &args[0])?;
    let b = expect_number("fityes", &args[1])?;
    Ok(Value::Bool(a > b))
}

fn builtin_le(args: &[Value]) -> Result<Value, SatukitanError> {
    ensure_exact("ditasgata", args, 2)?;
    let a = expect_number("ditasgata", &args[0])?;
    let b = expect_number("ditasgata", &args[1])?;
    Ok(Value::Bool(a <= b))
}

fn builtin_ge(args: &[Value]) -> Result<Value, SatukitanError> {
    ensure_exact("fityesgata", args, 2)?;
    let a = expect_number("fityesgata", &args[0])?;
    let b = expect_number("fityesgata", &args[1])?;
    Ok(Value::Bool(a >= b))
}

fn builtin_eq(args: &[Value]) -> Result<Value, SatukitanError> {
    ensure_at_least("gatas", args, 2)?;
    let first = &args[0];
    let is_equal = args.iter().all(|value| first.structural_eq(value));
    Ok(Value::Bool(is_equal))
}

fn builtin_sort(args: &[Value]) -> Result<Value, SatukitanError> {
    ensure_exact("fanitas", args, 1)?;
    let items = expect_list("fanitas", &args[0])?;
    let mut numbers: Vec<i64> = items
        .iter()
        .map(|value| expect_number("fanitas", value))
        .collect::<Result<_, _>>()?;
    numbers.sort_unstable();
    Ok(Value::List(
        numbers.into_iter().map(Value::Number).collect(),
    ))
}

fn builtin_length(args: &[Value]) -> Result<Value, SatukitanError> {
    ensure_exact("rakas", args, 1)?;
    let items = expect_list("rakas", &args[0])?;
    Ok(Value::Number(items.len() as i64))
}

fn builtin_print(args: &[Value]) -> Result<Value, SatukitanError> {
    if args.is_empty() {
        println!();
    } else {
        let output = args
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", output);
    }
    Ok(Value::Nil)
}

fn ensure_exact(name: &str, args: &[Value], expected: usize) -> Result<(), SatukitanError> {
    if args.len() == expected {
        Ok(())
    } else {
        Err(SatukitanError::arity_exact(
            name.to_string(),
            expected,
            args.len(),
        ))
    }
}

fn ensure_at_least(name: &str, args: &[Value], expected: usize) -> Result<(), SatukitanError> {
    if args.len() >= expected {
        Ok(())
    } else {
        Err(SatukitanError::arity_at_least(
            name.to_string(),
            expected,
            args.len(),
        ))
    }
}

fn expect_number(name: &str, value: &Value) -> Result<i64, SatukitanError> {
    match value {
        Value::Number(n) => Ok(*n),
        other => Err(SatukitanError::Eval(format!(
            "{name}: expected number, found {}",
            other.type_name()
        ))),
    }
}

fn expect_bool(name: &str, value: &Value) -> Result<bool, SatukitanError> {
    match value {
        Value::Bool(b) => Ok(*b),
        other => Err(SatukitanError::Eval(format!(
            "{name}: expected boolean, found {}",
            other.type_name()
        ))),
    }
}

fn expect_list<'a>(name: &str, value: &'a Value) -> Result<&'a [Value], SatukitanError> {
    match value {
        Value::List(items) => Ok(items.as_slice()),
        other => Err(SatukitanError::Eval(format!(
            "{name}: expected list, found {}",
            other.type_name()
        ))),
    }
}
