use bevy::{prelude::*, time::Stopwatch};

#[allow(unused_imports)]
use crate::{
    osc::{fetch_osc_tex, OscType},
    pot::{fetch_pot_tex, PotActiveEvent, PotType},
    ApplicationState,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct TrackSet;

pub(super) struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::Loading), load_track);
        app.add_systems(OnEnter(ApplicationState::Freeform), load_track);
        // app.add_systems(Update, start_playback.in_set(TrackSet));
        app.add_systems(
            FixedUpdate,
            (tick_track_timer, advance_iteration, start_playback).in_set(TrackSet),
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

#[derive(Component)]
struct StartDelayTimer {
    timer: Stopwatch,
}

fn tick_track_timer(
    mut query: Query<&mut TrackTimer>,
    time: Res<Time>,
    mut track: ResMut<Track>,
    mut ev_activate_pot: EventWriter<PotActiveEvent>,
) {
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
            ev_activate_pot.send(PotActiveEvent(track.seq[track.pos].note.pot));
        }
    }
}

#[derive(Component)]
enum TrackPos {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Bundle)]
struct TrackOscBundle {
    tag: TrackOscTag,
    sprite: SpriteBundle,
    pos: TrackPos,
}

#[derive(Component)]
struct TrackOscTag;

#[derive(Bundle)]
struct TrackPotBundle {
    tag: TrackPotTag,
    sprite: SpriteBundle,
    pos: TrackPos,
}

fn start_playback(
    mut commands: Commands,
    server: Res<AssetServer>,
    time: Res<Time<Virtual>>,
    track: Res<Track>,
    mut query: Query<&mut StartDelayTimer>,
) {
    for mut delay_timer in query.iter_mut() {
        if delay_timer.timer.elapsed_secs_f64() < track.bpm * 24. {
            delay_timer.timer.tick(time.delta());
        } else if delay_timer.timer.elapsed_secs_f64() >= track.bpm * 24. {
            commands.spawn(AudioBundle {
                source: server.load("tj_01.ogg"),
                ..default()
            });
            delay_timer.timer.reset();
            delay_timer.timer.pause();
        }
    }
}

#[derive(Component)]
struct TrackPotTag;

fn load_track(
    mut commands: Commands,
    server: Res<AssetServer>,
    query: Query<Entity, With<TrackTimer>>,
    mut track: ResMut<Track>,
) {
    let origin_x = -150.;
    let origin_y = 150.;
    let osc_layer = 0.;
    let pot_layer = 5.;
    let offset = 32.;

    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    if track.pos != 0 {
        track.pos = 0;
    }
    if track.iteration != 0 {
        track.iteration = 0;
    }

    let track_osc_a = (TrackOscBundle {
        tag: TrackOscTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(origin_x, origin_y, osc_layer)),

            texture: server.load(fetch_osc_tex(track.seq[0].note.s1)),
            ..default()
        },
        pos: TrackPos::A,
    },);
    let track_pot_a = TrackPotBundle {
        tag: TrackPotTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                origin_x,
                origin_y + offset,
                pot_layer,
            )),
            texture: server.load(fetch_pot_tex(track.seq[0].note.pot)),
            ..default()
        },
        pos: TrackPos::A,
    };
    commands.spawn(track_osc_a);
    commands.spawn(track_pot_a);

    let track_osc_b = (TrackOscBundle {
        tag: TrackOscTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                origin_x + offset * 1.,
                origin_y,
                osc_layer,
            )),

            texture: server.load(fetch_osc_tex(track.seq[1].note.s1)),
            ..default()
        },
        pos: TrackPos::B,
    },);
    let track_pot_b = TrackPotBundle {
        tag: TrackPotTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                origin_x + offset * 1.,
                origin_y + offset,
                pot_layer,
            )),
            texture: server.load(fetch_pot_tex(track.seq[1].note.pot)),
            ..default()
        },
        pos: TrackPos::B,
    };
    commands.spawn(track_osc_b);
    commands.spawn(track_pot_b);

    let track_osc_c = (TrackOscBundle {
        tag: TrackOscTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                origin_x + offset * 2.,
                origin_y,
                osc_layer,
            )),

            texture: server.load(fetch_osc_tex(track.seq[2].note.s1)),
            ..default()
        },
        pos: TrackPos::C,
    },);
    let track_pot_c = TrackPotBundle {
        tag: TrackPotTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                origin_x + offset * 2.,
                origin_y + offset,
                pot_layer,
            )),
            texture: server.load(fetch_pot_tex(track.seq[2].note.pot)),
            ..default()
        },
        pos: TrackPos::C,
    };
    commands.spawn(track_osc_c);
    commands.spawn(track_pot_c);

    let track_osc_d = (TrackOscBundle {
        tag: TrackOscTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                origin_x + offset * 3.,
                origin_y,
                osc_layer,
            )),

            texture: server.load(fetch_osc_tex(track.seq[3].note.s1)),
            ..default()
        },
        pos: TrackPos::D,
    },);
    let track_pot_d = TrackPotBundle {
        tag: TrackPotTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                origin_x + offset * 3.,
                origin_y + offset,
                pot_layer,
            )),
            texture: server.load(fetch_pot_tex(track.seq[3].note.pot)),
            ..default()
        },
        pos: TrackPos::D,
    };
    commands.spawn(track_osc_d);
    commands.spawn(track_pot_d);

    let track_osc_e = (TrackOscBundle {
        tag: TrackOscTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                origin_x + offset * 4.,
                origin_y,
                osc_layer,
            )),

            texture: server.load(fetch_osc_tex(track.seq[4].note.s1)),
            ..default()
        },
        pos: TrackPos::E,
    },);
    let track_pot_e = TrackPotBundle {
        tag: TrackPotTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                origin_x + offset * 4.,
                origin_y + offset,
                pot_layer,
            )),
            texture: server.load(fetch_pot_tex(track.seq[4].note.pot)),
            ..default()
        },
        pos: TrackPos::E,
    };
    commands.spawn(track_osc_e);
    commands.spawn(track_pot_e);

    let track_osc_f = (TrackOscBundle {
        tag: TrackOscTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                origin_x + offset * 5.,
                origin_y,
                osc_layer,
            )),

            texture: server.load(fetch_osc_tex(track.seq[5].note.s1)),
            ..default()
        },
        pos: TrackPos::F,
    },);
    let track_pot_f = TrackPotBundle {
        tag: TrackPotTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                origin_x + offset * 5.,
                origin_y + offset,
                pot_layer,
            )),
            texture: server.load(fetch_pot_tex(track.seq[5].note.pot)),
            ..default()
        },
        pos: TrackPos::F,
    };
    commands.spawn(track_osc_f);
    commands.spawn(track_pot_f);

    let track_osc_g = (TrackOscBundle {
        tag: TrackOscTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                origin_x + offset * 6.,
                origin_y,
                osc_layer,
            )),

            texture: server.load(fetch_osc_tex(track.seq[6].note.s1)),
            ..default()
        },
        pos: TrackPos::G,
    },);
    let track_pot_g = TrackPotBundle {
        tag: TrackPotTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                origin_x + offset * 6.,
                origin_y + offset,
                pot_layer,
            )),
            texture: server.load(fetch_pot_tex(track.seq[6].note.pot)),
            ..default()
        },
        pos: TrackPos::G,
    };
    commands.spawn(track_osc_g);
    commands.spawn(track_pot_g);

    let track_osc_h = (TrackOscBundle {
        tag: TrackOscTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                origin_x + offset * 7.,
                origin_y,
                osc_layer,
            )),

            texture: server.load(fetch_osc_tex(track.seq[7].note.s1)),
            ..default()
        },
        pos: TrackPos::H,
    },);
    let track_pot_h = TrackPotBundle {
        tag: TrackPotTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                origin_x + offset * 7.,
                origin_y + offset,
                pot_layer,
            )),
            texture: server.load(fetch_pot_tex(track.seq[7].note.pot)),
            ..default()
        },
        pos: TrackPos::H,
    };
    commands.spawn(track_osc_h);
    commands.spawn(track_pot_h);

    commands.spawn(TrackTimer {
        timer: Stopwatch::new(),
    });
    commands.spawn(StartDelayTimer {
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
            s1: OscType::Triangle,
            s2: None,
            pot: PotType::PotI,
        },
    },
    Seq {
        time: 4,
        note: Note {
            s1: OscType::Sawtooth,
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
            s1: OscType::Square,
            s2: None,
            pot: PotType::PotK,
        },
    },
    Seq {
        time: 7,
        note: Note {
            s1: OscType::Square,
            s2: None,
            pot: PotType::PotO,
        },
    },
    Seq {
        time: 8,
        note: Note {
            s1: OscType::Sawtooth,
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
