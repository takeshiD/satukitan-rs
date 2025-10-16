use satukitan_rs::ast::Expr;
use satukitan_rs::parser::parse_program;

#[test]
fn parse_simple_call() {
    let program = parse_program("ritas ra ru").expect("parse failed");
    assert_eq!(program.len(), 1);
    match &program[0] {
        Expr::Call { func, args } => {
            match func.as_ref() {
                Expr::Symbol(name) => assert_eq!(name, "ritas"),
                other => panic!("expected function symbol, got {:?}", other),
            }
            assert_eq!(args.len(), 2);
            assert!(matches!(args[0], Expr::Number(2)));
            assert!(matches!(args[1], Expr::Number(1)));
        }
        other => panic!("unexpected expression: {:?}", other),
    }
}

#[test]
fn parse_list_literal() {
    let program = parse_program("[ra ru rya]").expect("parse failed");
    assert_eq!(program.len(), 1);
    match &program[0] {
        Expr::ListLiteral(items) => {
            assert!(matches!(items[0], Expr::Number(2)));
            assert!(matches!(items[1], Expr::Number(1)));
            assert!(matches!(items[2], Expr::Number(6)));
        }
        other => panic!("expected list literal, got {:?}", other),
    }
}

#[test]
fn parse_ignores_comments() {
    let program = parse_program("ritas ra ru  # add\n# comment line\n").expect("parse failed");
    assert_eq!(program.len(), 1);
}

#[test]
fn parse_gakas_two_args() {
    let program = parse_program("gakas x ra").expect("parse failed");
    match &program[0] {
        Expr::Call { args, .. } => {
            assert_eq!(args.len(), 2);
            assert!(matches!(args[0], Expr::Symbol(ref name) if name == "x"));
            assert!(matches!(args[1], Expr::Number(2)));
        }
        other => panic!("expected call, got {:?}", other),
    }
}
