use std::slice::Iter;

use crate::{parameter::Param, query::Query, timecode::time::Time, value::Generator};

// A Track consists of many Actions associated at a paticular Time.
//
// Each Action consists of many Selections.
//
// A Selection has a Query and a set of Applicators to apply to the fixtures of
// that query
//
// An Applicator has the Param and Generator to set the Fixture.

pub struct Track {
    tc_source_id: usize,
    actions: Vec<Action>,
}

impl Track {
    pub fn new(tc_source_id: usize) -> Self {
        Self {
            tc_source_id,
            actions: Vec::new(),
        }
    }

    // TODO: If time is before the previous tick received, then it should not
    //       return any actions.
    pub fn actions_to_apply(&self, time: Time) -> Iter<&Action> {
        todo!();
        // TODO: Return any action between this tick and the last tick
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn tc(&self) -> usize {
        self.tc_source_id
    }
}

pub struct Action {
    time: Time,
    pub selections: Vec<Selection>,
}

impl Action {
    pub fn new(time: Time) -> Self {
        Self {
            time,
            selections: Vec::new(),
        }
    }

    pub fn add_selection(&mut self, selection: Selection) {
        self.selections.push(selection);
    }
}

pub struct Selection {
    pub query: Query,
    pub applicators: Vec<Applicator>,
}

impl Selection {
    pub fn new(query: Query) -> Self {
        Self {
            query,
            applicators: Vec::new(),
        }
    }

    pub fn add_applicator(&mut self, applicator: Applicator) {
        self.applicators.push(applicator);
    }
}

pub struct Applicator {
    pub parameter: Param,
    pub generator: Box<dyn Generator>,
}

impl Applicator {
    pub fn new(parameter: Param, generator: Box<dyn Generator>) -> Self {
        Self {
            parameter,
            generator,
        }
    }
}
