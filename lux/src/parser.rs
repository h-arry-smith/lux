use pest::{error::Error, Parser};

use crate::ast::*;

#[derive(Parser)]
#[grammar = "lux.pest"]
pub struct LuxParser;

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = Vec::new();

    let pairs = LuxParser::parse(Rule::program, source)?;
    for pair in pairs {
        // TODO: This allow shouldn't be needed for ever, but if we never have
        //       any other top level rules, we can actually make stmt silent.
        #[allow(clippy::single_match)]
        let node = match pair.as_rule() {
            Rule::stmt => parse_statement(pair.into_inner().next().unwrap()),
            _ => continue,
        };

        ast.push(node);
    }

    Ok(ast)
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::assign => {
            let mut pair = pair.into_inner();
            let ident = parse_identifier(pair.next().unwrap());
            let value = parse_value(pair.next().unwrap());
            AstNode::Assign(Box::new(ident), Box::new(value))
        }
        _ => panic!("Unexpected statement: {}", pair.as_str()),
    }
}

fn parse_identifier(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::ident => AstNode::Ident(pair.as_str().to_owned()),
        _ => panic!("Expected an identifier, but got: {}", pair.as_str()),
    }
}

fn parse_value(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::integer => parse_integer(pair),
        _ => panic!("Unexpected value: {}", pair.as_str()),
    }
}

fn parse_integer(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let integer = pair.as_str().parse::<i64>().expect("not a valid integer");
    AstNode::IntegerLiteral(integer)
}
