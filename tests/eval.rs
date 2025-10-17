use satukitan_rs::Interpreter;
use satukitan_rs::value::Value;

#[test]
fn interpreter_persists_bindings() {
    let mut interpreter = Interpreter::new();
    let result = interpreter
        .eval_str("gakas x ra\nx")
        .expect("evaluation should succeed");
    assert!(matches!(result, Value::Number(2)));
}

#[test]
fn define_function_and_call() {
    let mut interpreter = Interpreter::new();
    let source = r#"
        gakasdenu add-two (x y) (
            ritas x y
        )
        add-two ra ru
    "#;
    let result = interpreter
        .eval_str(source)
        .expect("function call should succeed");
    assert!(matches!(result, Value::Number(3)));
}

#[test]
fn list_operations() {
    let mut interpreter = Interpreter::new();
    let result = interpreter
        .eval_str("fanitas [ro ra ru]")
        .expect("fanitas should sort list");
    match result {
        Value::List(items) => {
            let numbers: Vec<i64> = items
                .into_iter()
                .map(|value| match value {
                    Value::Number(n) => n,
                    other => panic!("expected number, got {:?}", other),
                })
                .collect();
            assert_eq!(numbers, vec![1, 2, 3]);
        }
        other => panic!("expected list result, got {:?}", other),
    }
}

#[test]
fn conditional_branches() {
    let mut interpreter = Interpreter::new();
    let result_true = interpreter
        .eval_str("nobu me (ra) (ru)")
        .expect("nobu should evaluate true branch");
    assert!(matches!(result_true, Value::Number(2)));

    let result_false = interpreter
        .eval_str("nobu ga (ra) (ru)")
        .expect("nobu should evaluate false branch");
    assert!(matches!(result_false, Value::Number(1)));
}

#[test]
fn recursive_function_fibonacci() {
    let mut interpreter = Interpreter::new();
    let source = r#"
        gakasdenu fibo (n) (
            nobu (ditasgata n ru)
                (n)
                (ritas (fibo (matyes n ru)) (fibo (matyes n ra)))
        )
        fibo rya
    "#;
    let result = interpreter
        .eval_str(source)
        .expect("recursive function should evaluate");
    assert!(matches!(result, Value::Number(8)));
}
