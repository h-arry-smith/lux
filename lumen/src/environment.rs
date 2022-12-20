use std::collections::HashMap;

use crate::{fixture_set::FixtureSet, timecode::time::Time, track::Track};

// TODO: This struct splitting technique is succesful for dealing with mutable
//       borrows, but shouldn't live in this file, and all these interfaces need
//       tidying up.

struct Tracks {
    tracks: Vec<Track>,
}

impl Tracks {
    fn new() -> Self {
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

pub struct History {
    history: Vec<FixtureSet>,
}

impl History {
    fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
    // We return the most recent history ID for the reference of any history
    // generator that wants to know where to return to.
    // The returned history ID always refers to the point in the history before the
    // action was applied.

    // As a return to a point in history discards everything after it, we don't
    // have to worry about those ID's shifting.
    // This may be hopelessly naive, but for now we will use it and in the future
    // we may create some unique identifier for a history
    fn record(&mut self, fixture_set: FixtureSet) -> usize {
        self.history.push(fixture_set);
        self.history.len() - 1
    }

    fn revert(&mut self, history_index: usize) -> Option<FixtureSet> {
        if self.history.get(history_index).is_some() {
            // discard all other histories
            self.history = self.history.split_off(history_index);
            // restore the last history
            Some(self.history.pop().unwrap())
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }
}

pub struct Environment {
    pub fixtures: FixtureSet,
    pub history: History,
    tracks: Tracks,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            fixtures: FixtureSet::new(),
            history: History::new(),
            tracks: Tracks::new(),
        }
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track)
    }

    pub fn run_to_time(&mut self, time: Time) {
        let mut all_unrun_actions = HashMap::new();
        // for each active track
        for track in self.tracks.active() {
            // get all the unrun actions and merge them into time groups
            all_unrun_actions.extend(track.unrun_actions_at_time(time));
        }

        let mut histories = Vec::new();

        // apply the actions at each time generating a history
        for (time_frame, track_actions) in all_unrun_actions {
            let history_id = self.history.record(self.fixtures.clone());

            // collect a history id for each time group
            histories.push((time_frame, history_id));

            for track_action in track_actions {
                self.fixtures.apply_action(track_action.action());
            }
        }

        // apply those history id's to those time groups in all the active tracks
        for (time_frame, history_id) in histories {
            for track in self.tracks.active_mut() {
                track.set_action_history_for_time(time_frame, history_id);
            }
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
