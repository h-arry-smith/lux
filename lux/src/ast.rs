#[derive(Debug)]
pub enum AstNode {
    Assign(Box<AstNode>, Box<AstNode>),
    Ident(String),
    IntegerLiteral(i64),
}
