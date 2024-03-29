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
            Rule::preset_block => parse_preset_block(pair.into_inner()),
            Rule::EOI => break,
            _ => panic!("expected a statement, got: {}", pair.as_str()),
        };

        statements.push(node);
    }

    statements
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::apply => {
            let mut pair = pair.into_inner();
            let ident = parse_parameter(pair.next().unwrap());
            let generator = parse_group_or_generator(pair.next().unwrap());
            AstNode::Apply(Box::new(ident), Box::new(generator))
        }
        Rule::select => {
            let mut pair = pair.into_inner();
            let query = parse_query(pair.next().unwrap());
            let statements = parse_statements(pair);
            AstNode::Select(Box::new(query), statements)
        }
        Rule::preset => {
            let mut pair = pair.into_inner();
            let ident = parse_identifier(pair.next().unwrap());
            AstNode::Preset(Box::new(ident))
        }
        Rule::delay_block => parse_delay_block(pair.into_inner()),
        Rule::inline_delay => parse_inline_delay(pair.into_inner()),
        _ => panic!("Unexpected statement: {}", pair.as_str()),
    }
}

fn parse_delay_block(mut pairs: pest::iterators::Pairs<Rule>) -> AstNode {
    let time = parse_time(pairs.next().unwrap());
    let statements = parse_statements(pairs);

    AstNode::DelayBlock(Box::new(time), statements)
}

fn parse_inline_delay(mut pairs: pest::iterators::Pairs<Rule>) -> AstNode {
    let time = parse_time(pairs.next().unwrap());
    let statement = parse_statement(pairs.next().unwrap().into_inner().next().unwrap());

    AstNode::DelayBlock(Box::new(time), vec![statement])
}

fn parse_preset_block(mut pairs: pest::iterators::Pairs<Rule>) -> AstNode {
    let ident = parse_identifier(pairs.next().unwrap());
    let statements = parse_statements(pairs);

    AstNode::PresetBlock(Box::new(ident), statements)
}

fn parse_parameter(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::param => AstNode::Parameter(pair.as_str().to_owned()),
        _ => panic!("Expected an identifier, but got: {}", pair.as_str()),
    }
}

fn parse_identifier(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::ident => AstNode::Ident(pair.as_str().to_owned()),
        _ => panic!("Expected an identifier, but got: {}", pair.as_str()),
    }
}

fn parse_group_or_generator(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::group => parse_generator_group(pair.into_inner()),
        Rule::generator => parse_generator(pair),
        _ => panic!("Expected generator or group, but got {}", pair.as_str()),
    }
}

fn parse_generator_group(mut pairs: pest::iterators::Pairs<Rule>) -> AstNode {
    // Check if the group has a specific identifier
    let identifier = match pairs.peek().unwrap().as_rule() {
        Rule::ident => Some(Box::new(parse_identifier(pairs.next().unwrap()))),
        _ => None,
    };

    let generators = pairs.map(parse_generator).collect();

    AstNode::GeneratorGroup(identifier, generators)
}

fn parse_generator(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::static_value => parse_static_value(pair),
        Rule::fade => parse_fade(pair.into_inner()),
        _ => panic!("Unexpected generator: {}", pair.as_str()),
    }
}

fn parse_static_value(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let pair = pair.into_inner().next().unwrap();

    let value = match pair.as_rule() {
        Rule::literal => parse_literal(pair),
        Rule::percentage => parse_percentage(pair),
        Rule::current_value => return AstNode::CurrentValue,
        _ => panic!("Unexpected value for static generator: {}", pair.as_str()),
    };

    AstNode::Static(Box::new(value))
}

fn parse_fade(mut pairs: pest::iterators::Pairs<Rule>) -> AstNode {
    let start = parse_static_value(pairs.next().unwrap());
    let end = parse_static_value(pairs.next().unwrap());

    let time = match pairs.next() {
        Some(time) => parse_time(time),
        None => AstNode::Time(3.0),
    };

    AstNode::Fade(Box::new(start), Box::new(end), Box::new(time))
}

fn parse_time(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let seconds = pair
        .as_str()
        .strip_suffix('s')
        .expect("time did not end with s")
        .parse::<f64>()
        .expect("not a valid time");
    AstNode::Time(seconds)
}

fn parse_literal(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let number = pair.as_str().parse::<f64>().expect("not a valid float");
    AstNode::Literal(number)
}

fn parse_percentage(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let str = pair
        .as_str()
        .strip_suffix('%')
        .expect("percentage value did not end with %");

    let number = str.parse::<f64>().expect("not a valid float");
    AstNode::Percentage(number)
}

fn parse_query(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let mut query_nodes = Vec::new();

    for pair in pair.into_inner() {
        query_nodes.push(parse_query_step(pair))
    }

    AstNode::Query(query_nodes)
}

fn parse_query_step(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::id => parse_query_id(pair),
        Rule::qrange => parse_query_range(pair.into_inner()),
        Rule::qcommand => parse_qcommand(pair.into_inner()),
        _ => panic!("Invalid query: {}", pair.as_str()),
    }
}

fn parse_query_id(pair: pest::iterators::Pair<Rule>) -> AstNode {
    let number = pair.as_str().parse::<usize>().expect("not a valid id");
    AstNode::FixtureID(number)
}

fn parse_query_range(mut pairs: pest::iterators::Pairs<Rule>) -> AstNode {
    let start = pairs.next().unwrap();
    let end = pairs.next().unwrap();

    let start = parse_query_id(start);
    let end = parse_query_id(end);

    AstNode::QRange(Box::new(start), Box::new(end))
}

fn parse_qcommand(mut pairs: pest::iterators::Pairs<Rule>) -> AstNode {
    let command_ident = parse_identifier(pairs.next().unwrap());

    AstNode::QCommand(Box::new(command_ident))
}
