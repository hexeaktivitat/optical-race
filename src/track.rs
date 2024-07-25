use bevy::{prelude::*, time::Stopwatch};

use crate::{osc::OscType, pot::PotType, ApplicationState};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct TrackSet;

pub(super) struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::Loading), load_track);
        app.add_systems(
            FixedUpdate,
            (tick_track_timer, advance_iteration).in_set(TrackSet),
        );
        app.insert_resource(Track {
            bpm: 0.333,
            iteration: 0,
            pos: 0,
            seq: SAMPLE_SEQ_TWO.to_vec(),
        });
        app.add_event::<AdvanceIterationEvent>();
    }
}

#[derive(Component)]
pub(crate) struct TrackTimer {
    pub(crate) timer: Stopwatch,
}

fn tick_track_timer(mut query: Query<&mut TrackTimer>, time: Res<Time>, mut track: ResMut<Track>) {
    for mut track_timer in query.iter_mut() {
        track_timer.timer.tick(time.delta());
        let current_frame = (track_timer.timer.elapsed_secs_f64() / track.bpm).floor() as u64;
        let current_time = track.seq[track.pos].time + (8 * track.iteration);
        if current_frame > current_time {
            track.pos += 1;
            if track.pos > 7 {
                track.pos = 0;
                track.iteration += 1;
            }
        }
    }
}

fn load_track(
    mut commands: Commands,
    _server: Res<AssetServer>,
    query: Query<Entity, With<TrackTimer>>,
    mut track: ResMut<Track>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    if track.pos != 0 {
        track.pos = 0;
    }
    if track.iteration != 0 {
        track.iteration = 0;
    }
    commands.spawn(TrackTimer {
        timer: Stopwatch::new(),
    });
}

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
    pub(crate) iteration: u64,
    pub(crate) pos: usize,
    pub(crate) seq: Vec<Seq>,
}

#[allow(dead_code)]
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

#[derive(Event)]
pub(crate) struct AdvanceIterationEvent;

fn advance_iteration(
    mut ev_advance_iter: EventReader<AdvanceIterationEvent>,
    mut track: ResMut<Track>,
) {
    for _ev in ev_advance_iter.read() {
        track.iteration += 1;
    }
}

const SAMPLE_SEQ_TWO: [Seq; 8] = [
    Seq {
        time: 1,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
    Seq {
        time: 2,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotO,
        },
    },
    Seq {
        time: 3,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotI,
        },
    },
    Seq {
        time: 4,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotL,
        },
    },
    Seq {
        time: 5,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
    Seq {
        time: 6,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotK,
        },
    },
    Seq {
        time: 7,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotO,
        },
    },
    Seq {
        time: 8,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotK,
        },
    },
];

const _SAMPLE_SEQ: [Seq; 8] = [
    Seq {
        time: 5,
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
        time: 65,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
    Seq {
        time: 105,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
    Seq {
        time: 155,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
    Seq {
        time: 205,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
    Seq {
        time: 255,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
    Seq {
        time: 305,
        note: Note {
            s1: OscType::Sine,
            s2: None,
            pot: PotType::PotJ,
        },
    },
];
