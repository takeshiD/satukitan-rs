use satukitan_rs::Interpreter;
use satukitan_rs::value::Value;
#[test]
fn nested_list_evaluates_sequentially() {
    let mut interpreter = Interpreter::new();
    let source = "sipus matyes ra ru";
    let result = interpreter
        .eval_str(source)
        .expect("evaluation should succeed");
    assert!(matches!(result, Value::Nil));
}
