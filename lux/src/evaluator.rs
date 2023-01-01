// Evaluation Strategy

// Eventually, evaluation of a single lux file entry point (main.lux) should
// create the whole environment for the show, with Candela doing only the
// editing related work. However Candela will handle other tasks such as I/O,
// persistance, live update, and peripheries. Lux & Lumen are tightly coupled,
// Candela not so much.

// In the future, main.lux may be auto generated by candela, to handle the busy
// work of linking pieces together for the user, but this is a long way off.

// For now, we will start with a much simpler evaluation strategy, a single file
// will create an Lumen Environment that can be handed off and executed.

use std::fmt::Display;

use crate::ast::AstNode;
use lumen::{
    action::{Action, Apply, ApplyGroup},
    parameter::Param,
    timecode::time::Time,
    value::{
        generator::{BoxedGenerator, Static},
        Values,
    },
    Environment, Query, QueryBuilder, Step,
};

type EvaluationResult = Result<(), EvaluationError>;

pub struct Evaluator<'a> {
    pub env: &'a mut Environment,
    global_action: Action,
    apply_groups: Vec<ApplyGroup>,
}

impl<'a> Evaluator<'a> {
    pub fn new(env: &'a mut Environment) -> Self {
        Self {
            env,
            global_action: Action::new(),
            apply_groups: Vec::new(),
        }
    }

    pub fn evaluate(&mut self, program: Vec<AstNode>) -> EvaluationResult {
        self.add_global_apply_group();

        for node in program.iter() {
            self.evaluate_statement(node)?;
        }

        for apply_group in self.apply_groups.drain(0..) {
            self.global_action.add_group(apply_group);
        }

        dbg!(&self.global_action);

        // NOTE: This is obviously temporary, but we just apply the global action at time 0
        self.env
            .fixtures
            .apply_action(&self.global_action, Time::at(0, 0, 0, 0));

        Ok(())
    }

    fn add_global_apply_group(&mut self) {
        let query = QueryBuilder::new().build();
        self.apply_groups.push(ApplyGroup::new(query));
    }

    fn evaluate_statement(&mut self, node: &AstNode) -> EvaluationResult {
        match node {
            AstNode::Apply(identifier, value) => {
                self.evaluate_apply(identifier, value)?;
            }
            AstNode::Select(query, statements) => {
                self.evaluate_select(query, statements)?;
            }
            _ => {
                return self.evaluation_error(format!("Expected a statement but got: {:?}", node));
            }
        }
        Ok(())
    }

    fn evaluate_apply(&mut self, identifier: &AstNode, generator: &AstNode) -> EvaluationResult {
        let identifier = self.evaluate_identifier(identifier)?;
        let generator = self.evaluate_generator(generator)?;

        let apply = Apply::new(identifier, generator);

        self.current_apply_group().add_apply(apply);

        Ok(())
    }

    fn evaluate_identifier(&mut self, identifier: &AstNode) -> Result<Param, EvaluationError> {
        if let AstNode::Ident(identifier_string) = identifier {
            match Param::from_string(identifier_string) {
                Some(param) => Ok(param),
                None => self.evaluation_error(format!(
                    "Expected a valid parameter identifier but got: {}",
                    identifier_string
                )),
            }
        } else {
            self.evaluation_error(format!(
                "Expected a valid parameter identifier but got: {:?}",
                identifier
            ))
        }
    }

    fn evaluate_generator(
        &mut self,
        generator: &AstNode,
    ) -> Result<BoxedGenerator, EvaluationError> {
        let generator = match generator {
            AstNode::Numeric(number) => Box::new(Static::new(Values::make_literal(*number))),
            _ => {
                return self.evaluation_error(format!(
                    "Expected a valid generator but got: {:?}",
                    generator
                ))
            }
        };

        Ok(generator)
    }

    fn evaluate_select(&mut self, query: &AstNode, statements: &Vec<AstNode>) -> EvaluationResult {
        let query = self.evaluate_query(query)?;
        self.open_apply_group(query);

        for statement in statements {
            self.evaluate_statement(statement)?;
        }

        Ok(())
    }

    fn evaluate_query(&mut self, query: &AstNode) -> Result<Query, EvaluationError> {
        let mut query_steps = Vec::new();
        // TODO: Maybe we could pass a reference here, but maybe it makes sense for
        //       a query to own it's subquery?
        if !self.current_apply_group().query.steps.is_empty() {
            query_steps.push(Step::SubQuery(self.current_apply_group().query.clone()));
        }

        if let AstNode::Query(steps) = query {
            for step in steps {
                query_steps.push(self.evaluate_query_step(step)?);
            }
        } else {
            return self.evaluation_error("expected a query".to_string());
        }

        Ok(Query::new(query_steps))
    }

    fn evaluate_query_step(&mut self, step: &AstNode) -> Result<Step, EvaluationError> {
        match step {
            AstNode::FixtureID(id) => Ok(Step::Id(*id)),
            AstNode::QRange(start, end) => self.evaluate_query_range(start, end),
            _ => self.evaluation_error(format!("expected a valid query step but got: {:?}", step)),
        }
    }

    fn evaluate_query_range(
        &mut self,
        start: &AstNode,
        end: &AstNode,
    ) -> Result<Step, EvaluationError> {
        if let AstNode::FixtureID(start) = start {
            if let AstNode::FixtureID(end) = end {
                Ok(Step::Range(*start, *end))
            } else {
                self.evaluation_error("end value of range must be a fixture id".to_string())
            }
        } else {
            self.evaluation_error("start value of range must be a fixture id".to_string())
        }
    }

    fn open_apply_group(&mut self, query: Query) {
        self.apply_groups.push(ApplyGroup::new(query))
    }

    fn close_apply_group(&mut self) {
        let apply_group = self
            .apply_groups
            .pop()
            .expect("trying to close an apply group when one doesn't exist");
        self.global_action.add_group(apply_group);
    }

    fn current_apply_group(&mut self) -> &mut ApplyGroup {
        self.apply_groups
            .last_mut()
            .expect("trying to get a mut ref to the current apply group but there isn't one")
    }

    fn evaluation_error<T>(&self, text: String) -> Result<T, EvaluationError> {
        Err(EvaluationError(text))
    }
}

pub struct EvaluationError(String);
impl Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
