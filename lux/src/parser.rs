use pest::{error::Error, Parser};

use crate::ast::*;

#[derive(Parser)]
#[grammar = "lux.pest"]
pub struct LuxParser;

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let pairs = LuxParser::parse(Rule::program, source)?;
    Ok(parse_statements(pairs))
}

fn parse_statements(pairs: pest::iterators::Pairs<Rule>) -> Vec<AstNode> {
    let mut statements = Vec::new();
    for pair in pairs {
        // TODO: This allow shouldn't be needed for ever, but if we never have
        //       any other top level rules, we can actually make stmt silent.
        #[allow(clippy::single_match)]
        let node = match pair.as_rule() {
            Rule::stmt => parse_statement(pair.into_inner().next().unwrap()),
            Rule::EOI => break,
            _ => panic!("expected a statement, got: {}", pair.as_str()),
        };

        statements.push(node);
    }

    statements
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::assign => {
            let mut pair = pair.into_inner();
            let ident = parse_identifier(pair.next().unwrap());
            let generator = parse_generator(pair.next().unwrap());
            AstNode::Assign(Box::new(ident), Box::new(generator))
        }
        Rule::select => {
            let mut pair = pair.into_inner();
            let query = parse_query(pair.next().unwrap());
            let statements = parse_statements(pair);

            AstNode::Select(Box::new(query), statements)
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

fn parse_generator(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::numeric => parse_numeric(pair),
        _ => panic!("Unexpected value: {}", pair.as_str()),
    }
}

fn parse_numeric(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let number = pair.as_str().parse::<f64>().expect("not a valid float");
    AstNode::Numeric(number)
}

fn parse_query(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let number = pair.as_str().parse::<usize>().expect("not a valid id");
    AstNode::Query(number)
}
