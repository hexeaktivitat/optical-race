use bevy::prelude::*;

use crate::{led::LedPos, osc::OscType, pot::PotType, ApplicationState};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct TrackSet;

pub(super) struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::Loading), load_track);
        app.insert_resource(Track {
            seq: SAMPLE_SEQ.to_vec(),
        });
    }
}

fn load_track(mut commands: Commands, server: Res<AssetServer>) {}

#[derive(Bundle)]
struct TrackBundle {
    tag: TrackTag,
    audio: AudioBundle,
}

#[derive(Component)]
struct TrackTag;

#[derive(Resource)]
struct Track {
    seq: Vec<Seq>,
}

#[derive(Component, Clone)]
struct Note {
    s1: OscType,
    s2: Option<OscType>,
    pot: PotType,
}

#[derive(Component, Clone)]
struct Seq {
    time: u64, // multiplier for current bpm frame
    bpm: f64,
    note: Note,
}

const SAMPLE_SEQ: [Seq; 1] = [Seq {
    time: 5,
    bpm: 333.,
    note: Note {
        s1: OscType::Sine,
        s2: None,
        pot: PotType::PotJ,
    },
}];
