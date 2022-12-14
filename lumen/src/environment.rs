use crate::{
    fixture::Fixture,
    fixture_set::FixtureSet,
    query::QueryResult,
    timecode::{FrameRate, Source},
    track::Track,
};

// FIXME: Remove public access to fixtures
pub struct Environment {
    pub fixtures: FixtureSet,
    timecode_sources: Vec<Source>,
    active_track: Option<Track>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            fixtures: FixtureSet::new(),
            // TODO: Number and frame rate of sources should be configurable.
            timecode_sources: vec![Source::new(FrameRate::Thirty); 8],
            active_track: None,
        }
    }

    // TODO: This works for moving only in a forward direction, but we should
    //       also be tracking some sort of history, and rewind to it if the tick
    //       is in the past.
    pub fn tick(&mut self) {
        if let Some(ref mut track) = self.active_track {
            let time = self.timecode_sources[track.tc()].time();
            // NOTE: Because every action can have N selections, and each
            //       seleciton can have N applicators, we end up with these
            //       nested for loops. Though this shouldn't be a problem as
            //       in general, N is low.
            for action in track.actions_to_apply(time) {
                for selection in action.selections.iter() {
                    let ids = selection.query.evaluate(&self.fixtures);
                    for applicator in selection.applicators.iter() {
                        for (_, fixture) in self.query_fixtures(&ids) {
                            fixture.set(applicator.parameter, applicator.generator);
                        }
                    }
                }
            }
        }
    }

    pub fn set_track(&mut self, track: Track) {
        self.active_track = Some(track);
    }

    pub fn query_fixtures<'q>(
        &'q mut self,
        result: &'q QueryResult,
    ) -> impl Iterator<Item = (&usize, &mut Fixture)> {
        self.fixtures
            .iter_mut()
            .filter(|(_, f)| result.contains(&f.id()))
    }

    pub fn timecode(&self, id: usize) -> &Source {
        assert!(id < self.timecode_sources.len());
        &self.timecode_sources[id]
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
