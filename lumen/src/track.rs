use std::collections::HashMap;

use crate::{action::Action, timecode::time::Time};

#[derive(Debug)]
pub struct Track {
    actions: Vec<TrackAction>,
    offset: Time,
}

impl Track {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            offset: Time::at(0, 0, 0, 0),
        }
    }

    pub fn actions(&self) -> &Vec<TrackAction> {
        &self.actions
    }

    pub fn set_offset(&mut self, time: Time) {
        self.offset = time;
    }

    pub fn add_action(&mut self, time: Time, action: Action) {
        self.actions.push(TrackAction::new(time, action));
        self.actions.sort();
    }

    pub fn unrun_actions_at_time(&self, time: Time) -> HashMap<Time, Vec<&TrackAction>> {
        let mut unrun: HashMap<Time, Vec<&TrackAction>> = HashMap::new();

        for action in self.actions.iter() {
            if action.time <= time && !action.has_history() {
                match unrun.get_mut(&action.time) {
                    Some(other_actions_at_this_time) => {
                        other_actions_at_this_time.push(action);
                    }
                    None => {
                        unrun.insert(action.time, vec![action]);
                    }
                }
            }
        }

        unrun
    }

    pub fn set_action_history_for_time(&mut self, time: Time, history_id: usize) {
        for action in self.actions.iter_mut() {
            if action.time == time {
                action.set_history(history_id)
            }
        }
    }
}

impl Default for Track {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct TrackAction {
    time: Time,
    action: Action,
    history: Option<usize>,
}

impl TrackAction {
    fn new(time: Time, action: Action) -> Self {
        Self {
            time,
            action,
            history: None,
        }
    }

    pub fn action(&self) -> &Action {
        &self.action
    }

    pub fn has_history(&self) -> bool {
        self.history.is_some()
    }

    fn clear_history(&mut self) {
        self.history = None
    }

    fn history(&self) -> usize {
        self.history.unwrap()
    }

    fn set_history(&mut self, history_id: usize) {
        self.history = Some(history_id)
    }
}

impl PartialEq for TrackAction {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for TrackAction {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for TrackAction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TrackAction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}
