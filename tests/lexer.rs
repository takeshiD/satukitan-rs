use satukitan_rs::lexer::{identifier, inline_space0, inline_space1};

#[test]
fn inline_space0_accepts_mixed_whitespace() {
    let (rest, matched) = inline_space0(" \t\tfoo").expect("should parse whitespace");
    assert_eq!(matched, " \t\t");
    assert_eq!(rest, "foo");
}

#[test]
fn inline_space1_requires_at_least_one_char() {
    let (rest, matched) = inline_space1("   bar").expect("should parse mandatory whitespace");
    assert_eq!(matched, "   ");
    assert_eq!(rest, "bar");
    assert!(inline_space1("baz").is_err());
}

#[test]
fn identifier_supports_extended_tokens() {
    let (rest, ident) = identifier("ritas-extra  ").expect("should parse identifier");
    assert_eq!(ident, "ritas-extra");
    assert_eq!(rest, "  ");

    let (rest_hash, ident_hash) =
        identifier("#ta rest").expect("should parse hash-prefixed literal");
    assert_eq!(ident_hash, "#ta");
    assert_eq!(rest_hash, " rest");
}

#[test]
fn identifier_rejects_invalid_start() {
    assert!(identifier("9start").is_err());
    assert!(identifier("-dash").is_err());
}
