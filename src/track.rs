use bevy::prelude::*;

use crate::{led::LedPos, osc::OscType, pot::PotType, ApplicationState};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct TrackSet;

pub(super) struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::Loading), load_track);
        app.insert_resource(Track {
            bpm: 0.333,
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
pub(crate) struct Track {
    pub(crate) bpm: f64,
    pub(crate) seq: Vec<Seq>,
}

#[derive(Component, Clone)]
pub(crate) struct Note {
    pub(crate) s1: OscType,
    pub(crate) s2: Option<OscType>,
    pub(crate) pot: PotType,
}

#[derive(Component, Clone)]
pub(crate) struct Seq {
    pub(crate) time: u64, // multiplier for current bpm frame
    pub(crate) note: Note,
}

const SAMPLE_SEQ: [Seq; 8] = [
    Seq {
        time: 5,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
    Seq {
        time: 15,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
    Seq {
        time: 25,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
    Seq {
        time: 35,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
    Seq {
        time: 45,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
    Seq {
        time: 55,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
    Seq {
        time: 65,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
    Seq {
        time: 75,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
];
