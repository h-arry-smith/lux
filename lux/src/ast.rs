#[derive(Debug, Clone)]
pub enum AstNode {
    Apply(Box<AstNode>, Box<AstNode>),
    Parameter(String),
    Ident(String),
    Literal(f64),
    Percentage(f64),
    Query(Vec<AstNode>),
    QRange(Box<AstNode>, Box<AstNode>),
    QCommand(Box<AstNode>),
    Select(Box<AstNode>, Vec<AstNode>),
    FixtureID(usize),
    Static(Box<AstNode>),
    Fade(Box<AstNode>, Box<AstNode>, Box<AstNode>),
    Time(f64),
    DelayBlock(Box<AstNode>, Vec<AstNode>),
    PresetBlock(Box<AstNode>, Vec<AstNode>),
    Preset(Box<AstNode>),
}
