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

use crate::group_parameters::GROUP_PARAMETERS;
use std::{collections::HashMap, fmt::Display, time::Duration};

use crate::ast::AstNode;
use lumen::{
    action::{Action, Apply, ApplyGroup},
    parameter::Param,
    timecode::time::Time,
    track::Track,
    value::{
        generator::{BoxedGenerator, CurrentValue, Delay, Fade, Static},
        Values,
    },
    Environment, Query, QueryBuilder, Step,
};

type EvaluationResult = Result<(), EvaluationError>;

pub struct Evaluator<'a> {
    pub env: &'a mut Environment,
    global_action: Action,
    apply_groups: Vec<ApplyGroup>,
    parent_apply_group: Vec<usize>,
    delay_time: Option<Duration>,
    presets: HashMap<String, Vec<AstNode>>,
}

impl<'b, 'a> Evaluator<'a> {
    pub fn new(env: &'a mut Environment) -> Self {
        Self {
            env,
            global_action: Action::new(),
            apply_groups: Vec::new(),
            parent_apply_group: Vec::new(),
            delay_time: None,
            presets: HashMap::new(),
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

        let mut track = Track::new();
        track.add_action(Time::at(0, 0, 0, 0), self.global_action.clone());

        self.env.reset();
        self.env.add_track(track);

        Ok(())
    }

    fn add_global_apply_group(&mut self) {
        let query = QueryBuilder::new().all().build();
        self.apply_groups.push(ApplyGroup::new(query));
        // self.parent_apply_group.push(self.apply_groups.len() - 1);
    }

    fn evaluate_statement(&mut self, node: &AstNode) -> EvaluationResult {
        match node {
            AstNode::Apply(identifier, value) => {
                self.evaluate_apply(identifier, value)?;
            }
            AstNode::Select(query, statements) => {
                self.evaluate_select(query, statements)?;
            }
            AstNode::DelayBlock(time, statements) => {
                self.evaluate_delay_block(time, statements)?;
            }
            AstNode::PresetBlock(identifier, statements) => {
                self.evaluate_preset_block(identifier, statements)?;
            }
            AstNode::Preset(identifier) => {
                self.evaluate_preset(identifier)?;
            }
            _ => {
                return self.evaluation_error(format!("Expected a statement but got: {:?}", node));
            }
        }
        Ok(())
    }

    fn evaluate_apply(&mut self, identifier: &AstNode, generator: &AstNode) -> EvaluationResult {
        match generator {
            AstNode::GeneratorGroup(generator_group) => {
                self.evaluate_apply_generator_group(identifier, generator_group)
            }
            _ => self.evaluate_apply_single_generator(identifier, generator),
        }
    }

    fn evaluate_apply_generator_group(
        &mut self,
        identifier: &AstNode,
        generator_group: &[AstNode],
    ) -> EvaluationResult {
        let group_parameter = self.evaluate_group_parameter(identifier)?;

        // check param is a group param and get list of param values to use
        let group_parameters = GROUP_PARAMETERS.get(group_parameter).unwrap();
        // generate list of generators from the group
        let mut generators = Vec::new();
        for unevaluated_generator in generator_group.iter() {
            let evaluated_generator = self.evaluate_generator(unevaluated_generator)?;
            generators.push(evaluated_generator);
        }

        // check provided generator group is exact length
        if generators.len() != group_parameters.len() {
            return self.evaluation_error(format!(
                "expected a group of len {} but got one of len {}",
                group_parameters.len(),
                generators.len()
            ));
        }

        // for each param, generator group pair, add to the parent apply group
        for (param, generator) in group_parameters.iter().zip(generators.into_iter()) {
            self.parent_apply_group()
                .add_apply(Apply::new(*param, generator))
        }

        Ok(())
    }

    fn evaluate_apply_single_generator(
        &mut self,
        identifier: &AstNode,
        generator: &AstNode,
    ) -> EvaluationResult {
        let identifier = self.evaluate_parameter(identifier)?;
        let mut generator = self.evaluate_generator(generator)?;

        if let Some(delay_time) = self.delay_time {
            generator = Box::new(Delay::new(delay_time, generator));
        }

        let apply = Apply::new(identifier, generator);

        self.parent_apply_group().add_apply(apply);

        Ok(())
    }

    fn evaluate_parameter(&mut self, parameter: &AstNode) -> Result<Param, EvaluationError> {
        if let AstNode::Parameter(parameter_string) = parameter {
            match Param::from_string(parameter_string) {
                Some(param) => Ok(param),
                None => self.evaluation_error(format!(
                    "Expected a valid parameter identifier but got: {}",
                    parameter_string
                )),
            }
        } else {
            self.evaluation_error(format!("Expected a parameter but got: {:?}", parameter))
        }
    }

    fn evaluate_group_parameter(
        &mut self,
        parameter: &'b AstNode,
    ) -> Result<&'b str, EvaluationError> {
        if let AstNode::Parameter(parameter_string) = parameter {
            if GROUP_PARAMETERS.contains_key(parameter_string.as_str()) {
                Ok(parameter_string)
            } else {
                self.evaluation_error(format!(
                    "Expected a valid group parameter identifier but got: {}",
                    parameter_string
                ))
            }
        } else {
            self.evaluation_error(format!("Expected a parameter but got {:?}", parameter))
        }
    }

    fn evaluate_generator(&self, generator: &AstNode) -> Result<BoxedGenerator, EvaluationError> {
        let generator = match generator {
            AstNode::Static(value) => self.evaluate_static(value)?,
            AstNode::Fade(start, end, time) => self.evaluate_fade(start, end, time)?,
            AstNode::CurrentValue => Box::new(CurrentValue::new()),
            _ => {
                return self.evaluation_error(format!(
                    "Expected a valid generator but got: {:?}",
                    generator
                ))
            }
        };

        Ok(generator)
    }

    fn evaluate_static(&self, value: &AstNode) -> Result<BoxedGenerator, EvaluationError> {
        let value = self.evaluate_value(value)?;
        Ok(Box::new(Static::new(value)))
    }

    fn evaluate_value(&self, value: &AstNode) -> Result<Values, EvaluationError> {
        let value = match value {
            AstNode::Literal(value) => Values::make_literal(*value),
            AstNode::Percentage(value) => Values::make_percentage(*value),
            _ => {
                return self.evaluation_error(format!(
                    "Expected a valid static value but got: {:?}",
                    value
                ))
            }
        };

        Ok(value)
    }

    fn evaluate_fade(
        &self,
        start: &AstNode,
        end: &AstNode,
        time: &AstNode,
    ) -> Result<BoxedGenerator, EvaluationError> {
        let start = self.evaluate_generator(start)?;
        let end = self.evaluate_generator(end)?;
        let time = self.evaluate_time(time)?;

        Ok(Box::new(Fade::new(start, end, time)))
    }

    fn evaluate_time(&self, time: &AstNode) -> Result<Duration, EvaluationError> {
        match time {
            AstNode::Time(seconds) => Ok(Duration::from_secs_f64(*seconds)),
            _ => self.evaluation_error(format!("Expected a time but got: {:?}", time)),
        }
    }

    fn evaluate_select(&mut self, query: &AstNode, statements: &Vec<AstNode>) -> EvaluationResult {
        let query = self.evaluate_query(query)?;
        self.open_apply_group(query);

        for statement in statements {
            self.evaluate_statement(statement)?;
        }

        self.close_apply_group();

        Ok(())
    }

    fn evaluate_query(&mut self, query: &AstNode) -> Result<Query, EvaluationError> {
        let mut query_steps = Vec::new();

        if let Some(parent_group_index) = self.parent_apply_group.last() {
            let parent_group = self.apply_groups.get(*parent_group_index).unwrap();
            if !parent_group.query.steps.is_empty() {
                query_steps.push(Step::SubQuery(parent_group.query.clone()));
            }
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
            AstNode::QCommand(ident) => self.evaluate_query_command(ident),
            _ => self.evaluation_error(format!("expected a valid query step but got: {:?}", step)),
        }
    }

    fn evaluate_query_range(
        &mut self,
        start: &AstNode,
        end: &AstNode,
    ) -> Result<Step, EvaluationError> {
        // TODO: this should be two fixture id evaluations steps with error returns
        //       not this nested monstrosity.
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

    fn evaluate_query_command(&mut self, identifier: &AstNode) -> Result<Step, EvaluationError> {
        let identifier = self.evaluate_identifier(identifier)?;

        match identifier.as_str() {
            "even" => Ok(Step::Even),
            "odd" => Ok(Step::Odd),
            _ => self.evaluation_error(format!("{} is not a valid query command", identifier)),
        }
    }

    fn evaluate_delay_block(
        &mut self,
        time: &AstNode,
        statements: &Vec<AstNode>,
    ) -> EvaluationResult {
        // Nested delay blocks are not supported
        if self.delay_time.is_some() {
            return self.evaluation_error("can not nest delay blocks".to_string());
        }

        self.delay_time = Some(self.evaluate_time(time)?);

        for statement in statements {
            self.evaluate_statement(statement)?;
        }

        self.delay_time = None;
        Ok(())
    }

    fn evaluate_preset_block(
        &mut self,
        identifier: &AstNode,
        statements: &[AstNode],
    ) -> EvaluationResult {
        let identifier = self.evaluate_identifier(identifier)?;

        self.presets.insert(identifier, statements.to_vec());

        Ok(())
    }

    fn evaluate_preset(&mut self, identifier: &AstNode) -> EvaluationResult {
        let identifier = self.evaluate_identifier(identifier)?;

        match self.presets.get(&identifier) {
            Some(statements) => {
                for node in statements.clone() {
                    self.evaluate_statement(&node)?;
                }
                Ok(())
            }
            None => self.evaluation_error(format!("could not find preset: {}", identifier)),
        }
    }

    fn evaluate_identifier(&mut self, identifier: &AstNode) -> Result<String, EvaluationError> {
        match identifier {
            AstNode::Ident(string) => Ok(string.clone()),
            _ => self.evaluation_error(format!("expected a identifier, got: {:?}", identifier)),
        }
    }

    fn open_apply_group(&mut self, query: Query) {
        self.apply_groups.push(ApplyGroup::new(query));
        self.parent_apply_group.push(self.apply_groups.len() - 1);
    }

    fn close_apply_group(&mut self) {
        self.parent_apply_group.pop();
    }

    fn parent_apply_group(&mut self) -> &mut ApplyGroup {
        if let Some(parent_group_index) = self.parent_apply_group.last() {
            self.apply_groups
                .get_mut(*parent_group_index)
                .expect("tried to apply to a parent apply group that doesn't exist")
        } else {
            self.apply_groups
                .first_mut()
                .expect("no global apply group to apply too")
        }
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
