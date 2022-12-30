#[derive(Debug)]
pub enum AstNode {
    Apply(Box<AstNode>, Box<AstNode>),
    Ident(String),
    Numeric(f64),
    Query(Vec<AstNode>),
    Select(Box<AstNode>, Vec<AstNode>),
    FixtureID(usize),
    QRange(Box<AstNode>, Box<AstNode>),
}
