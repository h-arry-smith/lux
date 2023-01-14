#[derive(Debug)]
pub enum AstNode {
    Apply(Box<AstNode>, Box<AstNode>),
    Ident(String),
    Literal(f64),
    Percentage(f64),
    Query(Vec<AstNode>),
    Select(Box<AstNode>, Vec<AstNode>),
    FixtureID(usize),
    QRange(Box<AstNode>, Box<AstNode>),
    Static(Box<AstNode>),
    Fade(Box<AstNode>, Box<AstNode>, Box<AstNode>),
    Time(f64),
    DelayBlock(Box<AstNode>, Vec<AstNode>),
}
