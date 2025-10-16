#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    String(String),
    Symbol(String),
    List(Vec<Expr>),
    ListLiteral(Vec<Expr>),
    Call { func: Box<Expr>, args: Vec<Expr> },
}

pub type Program = Vec<Expr>;

impl Expr {
    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Expr::Symbol(name) => Some(name.as_str()),
            _ => None,
        }
    }

    pub fn list_items(&self) -> Option<&[Expr]> {
        match self {
            Expr::List(items) | Expr::ListLiteral(items) => Some(items.as_slice()),
            _ => None,
        }
    }
}
