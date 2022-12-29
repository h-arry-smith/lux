#[derive(Debug)]
pub enum AstNode {
    Assign(Box<AstNode>, Box<AstNode>),
    Ident(String),
    Numeric(f64),
}
