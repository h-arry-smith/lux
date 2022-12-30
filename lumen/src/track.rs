use std::collections::BTreeMap;

use crate::{action::Action, history::HistoryID, timecode::time::Time};

#[derive(Clone)]
pub struct Tracks {
    tracks: Vec<Track>,
}

impl Tracks {
    pub fn new() -> Self {
        Self { tracks: Vec::new() }
    }

    pub fn push(&mut self, value: Track) {
        self.tracks.push(value)
    }

    // TODO: All built tracks live here, but should be able to be marked active
    //       and inactive.
    pub fn active(&self) -> impl Iterator<Item = &Track> {
        self.tracks.iter()
    }

    pub fn active_mut(&mut self) -> impl Iterator<Item = &mut Track> {
        self.tracks.iter_mut()
    }
}

impl Default for Tracks {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
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

    pub fn unrun_actions_at_time(&self, time: Time) -> BTreeMap<Time, Vec<&TrackAction>> {
        let mut unrun: BTreeMap<Time, Vec<&TrackAction>> = BTreeMap::new();

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

    pub fn set_action_history_for_time(&mut self, time: Time, history_id: HistoryID) {
        for action in self.actions.iter_mut() {
            if action.time == time {
                action.set_history(history_id)
            }
        }
    }

    pub fn get_closest_action_to_time_with_history(&self, time: Time) -> Option<&TrackAction> {
        let mut closest_action = None;

        for track_action in &self.actions {
            if track_action.time <= time && track_action.has_history() {
                closest_action = Some(track_action)
            }

            if track_action.time > time {
                break;
            }
        }

        closest_action
    }

    pub fn clear_history(&mut self) {
        for action in self.actions.iter_mut() {
            action.clear_history();
        }
    }

    pub fn clear_history_after_time(&mut self, time: Time) {
        for action in self.actions.iter_mut().filter(|action| action.time >= time) {
            action.clear_history();
        }
    }
}

impl Default for Track {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
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

    pub fn clear_history(&mut self) {
        self.history = None
    }

    pub fn history(&self) -> usize {
        self.history.unwrap()
    }

    pub fn time(&self) -> &Time {
        &self.time
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
