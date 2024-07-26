use bevy::prelude::*;

use crate::{
    osc::{OscState, OscType},
    track::{Track, TrackTimer},
    ApplicationState, ModeState, Score,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct PotSet;

pub(super) struct PotPlugin;

impl Plugin for PotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::Loading), load_pots.in_set(PotSet));
        app.add_systems(Update, pot_input.in_set(PotSet));
        app.add_systems(FixedFirst, check_note.in_set(PotSet));
        app.add_systems(OnEnter(ModeState::NotInGame), unload_pots.in_set(PotSet));

        app.add_event::<PotActiveEvent>();
        app.add_event::<CheckNoteEvent>();
    }
}

#[derive(Bundle)]
struct PotBundle {
    tag: PotTag,
    pot_type: PotType,
    state: PotState,
    sprite: SpriteBundle,
}

pub(crate) fn fetch_pot_tex(pot_type: PotType) -> String {
    match pot_type {
        PotType::PotJ => "pot_j_on.png",
        PotType::PotI => "pot_i_on.png",
        PotType::PotK => "pot_k_on.png",
        PotType::PotO => "pot_o_on.png",
        PotType::PotL => "pot_l_on.png",
    }
    .into()
}

fn load_pots(mut commands: Commands, server: Res<AssetServer>) {
    let origin_x = 0.;
    let origin_y = 0.;

    let potj_tex: Handle<Image> = server.load("pot_j_off.png");
    let potj = PotBundle {
        tag: PotTag,
        pot_type: PotType::PotJ,
        state: PotState::Inactive,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(origin_x, origin_y, 0.)),
            texture: potj_tex,
            ..default()
        },
    };
    commands.spawn(potj);

    let poti_tex: Handle<Image> = server.load("pot_i_off.png");
    let poti = PotBundle {
        tag: PotTag,
        pot_type: PotType::PotI,
        state: PotState::Inactive,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(origin_x + 16., origin_y + 32., 0.)),
            texture: poti_tex,
            ..default()
        },
    };
    commands.spawn(poti);

    let potk_tex: Handle<Image> = server.load("pot_k_off.png");
    let potk = PotBundle {
        tag: PotTag,
        pot_type: PotType::PotK,
        state: PotState::Inactive,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(origin_x + 32., origin_y, 0.)),
            texture: potk_tex,
            ..default()
        },
    };
    commands.spawn(potk);

    let poto_tex: Handle<Image> = server.load("pot_o_off.png");
    let poto = PotBundle {
        tag: PotTag,
        pot_type: PotType::PotO,
        state: PotState::Inactive,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(origin_x + 48., origin_y + 32., 0.)),
            texture: poto_tex,
            ..default()
        },
    };
    commands.spawn(poto);

    let potl_tex: Handle<Image> = server.load("pot_l_off.png");
    let potl = PotBundle {
        tag: PotTag,
        pot_type: PotType::PotL,
        state: PotState::Inactive,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(origin_x + 64., origin_y, 0.)),
            texture: potl_tex,
            ..default()
        },
    };
    commands.spawn(potl);
}

fn pot_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &PotType, &mut PotState, &mut Handle<Image>), With<PotTag>>,
    server: Res<AssetServer>,
    mut _ev_activate_pot: EventWriter<PotActiveEvent>,
    mut ev_check_note: EventWriter<CheckNoteEvent>,
) {
    for (_entity, pot_type, mut state, mut texture) in query.iter_mut() {
        for key in keys.get_just_pressed() {
            match key {
                KeyCode::KeyJ => {
                    if pot_type == &PotType::PotJ {
                        *state = PotState::Active;
                        *texture = server.load("pot_j_on.png");
                        ev_check_note.send(CheckNoteEvent);
                    }
                }
                KeyCode::KeyI => {
                    if pot_type == &PotType::PotI {
                        *state = PotState::Active;
                        *texture = server.load("pot_i_on.png");
                        ev_check_note.send(CheckNoteEvent);
                    }
                }
                KeyCode::KeyK => {
                    if pot_type == &PotType::PotK {
                        *state = PotState::Active;
                        *texture = server.load("pot_k_on.png");
                        ev_check_note.send(CheckNoteEvent);
                    }
                }
                KeyCode::KeyO => {
                    if pot_type == &PotType::PotO {
                        *state = PotState::Active;
                        *texture = server.load("pot_o_on.png");
                        ev_check_note.send(CheckNoteEvent);
                    }
                }
                KeyCode::KeyL => {
                    if pot_type == &PotType::PotL {
                        *state = PotState::Active;
                        *texture = server.load("pot_l_on.png");
                        ev_check_note.send(CheckNoteEvent);
                    }
                }
                _ => {}
            }
        }
        for key in keys.get_just_released() {
            match key {
                KeyCode::KeyJ => {
                    if pot_type == &PotType::PotJ {
                        *state = PotState::Inactive;
                        *texture = server.load("pot_j_off.png");
                    }
                }
                KeyCode::KeyI => {
                    if pot_type == &PotType::PotI {
                        *state = PotState::Inactive;
                        *texture = server.load("pot_i_off.png");
                    }
                }
                KeyCode::KeyK => {
                    if pot_type == &PotType::PotK {
                        *state = PotState::Inactive;
                        *texture = server.load("pot_k_off.png");
                    }
                }
                KeyCode::KeyO => {
                    if pot_type == &PotType::PotO {
                        *state = PotState::Inactive;
                        *texture = server.load("pot_o_off.png");
                    }
                }
                KeyCode::KeyL => {
                    if pot_type == &PotType::PotL {
                        *state = PotState::Inactive;
                        *texture = server.load("pot_l_off.png");
                    }
                }
                _ => {}
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Event)]
struct PotActiveEvent(PotType);

fn _activate_pot(
    mut ev_activate_pot: EventReader<PotActiveEvent>,
    mut _query: Query<(&PotType, &mut PotState, &mut Handle<Image>), With<PotTag>>,
) {
    for ev in ev_activate_pot.read() {
        match ev.0 {
            PotType::PotJ => todo!(),
            PotType::PotI => todo!(),
            PotType::PotK => todo!(),
            PotType::PotO => todo!(),
            PotType::PotL => todo!(),
        }
    }
}

#[allow(dead_code)]
#[derive(Event)]
pub(crate) struct CheckNoteEvent;

#[allow(clippy::comparison_chain)]
fn check_note(
    mut ev_check_note: EventReader<CheckNoteEvent>,
    track: ResMut<Track>,
    timer_query: Query<&TrackTimer>,
    pot_active_query: Query<(&PotState, &PotType)>,
    osc_active_query: Query<(&OscState, &OscType)>,
    mut score: ResMut<Score>,
) {
    for _ev in ev_check_note.read() {
        if !track.seq.is_empty() {
            let bpm = track.bpm;
            for track_timer in timer_query.iter() {
                let current_time = track.seq[track.pos].time + (track.iteration * 8);
                let current_frame = (track_timer.timer.elapsed_secs_f64() / bpm).floor() as u64;
                println!("frame: {} time: {}", current_frame, current_time);
                if current_frame == current_time {
                    for (p_state, p_type) in pot_active_query.iter() {
                        if track.seq[track.pos].note.pot == *p_type && *p_state == PotState::Active
                        {
                            for (o_state, o_type) in osc_active_query.iter() {
                                if track.seq[track.pos].note.s1 == *o_type
                                    && *o_state == OscState::Active
                                // && (current_frame == head.time
                                //     || current_frame <= head.time + 5
                                //     || current_frame >= (head.time - 5).clamp(5, 65536))
                                {
                                    println!("success");
                                    score.value += 1;
                                    score.updated = true;
                                }
                            }
                        }
                    }
                    // track.pos += 1;
                    // if track.pos > 7 {
                    //     track.pos = 0;
                    //     track.iteration += 1;
                    // }
                } else if current_frame > current_time {
                    println!("too late");
                } else {
                    println!("invalid timing");
                }
            }
        }
    }
}

fn unload_pots(mut commands: Commands, query: Query<Entity, With<PotTag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
struct PotTag;

#[derive(Component, PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum PotType {
    PotJ,
    PotI,
    PotK,
    PotO,
    PotL,
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
enum PotState {
    Active,
    Inactive,
}
