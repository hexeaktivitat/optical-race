use bevy::prelude::*;

use crate::{ApplicationState, ModeState};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct PotSet;

pub(super) struct PotPlugin;

impl Plugin for PotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::Loading), load_pots.in_set(PotSet));
        app.add_systems(Update, pot_input.in_set(PotSet));
        app.add_systems(OnEnter(ModeState::NotInGame), unload_pots.in_set(PotSet));

        app.add_event::<PotActiveEvent>();
    }
}

#[derive(Bundle)]
struct PotBundle {
    tag: PotTag,
    pot_type: PotType,
    state: PotState,
    sprite: SpriteBundle,
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
    mut ev_activate_pot: EventWriter<PotActiveEvent>,
) {
    for (_entity, pot_type, mut state, mut texture) in query.iter_mut() {
        for key in keys.get_pressed() {
            match key {
                KeyCode::KeyJ => {
                    if pot_type == &PotType::PotJ {
                        *state = PotState::Active;
                        *texture = server.load("pot_j_on.png");
                    }
                }
                KeyCode::KeyI => {
                    if pot_type == &PotType::PotI {
                        *state = PotState::Active;
                        *texture = server.load("pot_i_on.png");
                    }
                }
                KeyCode::KeyK => {
                    if pot_type == &PotType::PotK {
                        *state = PotState::Active;
                        *texture = server.load("pot_k_on.png");
                    }
                }
                KeyCode::KeyO => {
                    if pot_type == &PotType::PotO {
                        *state = PotState::Active;
                        *texture = server.load("pot_o_on.png");
                    }
                }
                KeyCode::KeyL => {
                    if pot_type == &PotType::PotL {
                        *state = PotState::Active;
                        *texture = server.load("pot_l_on.png");
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

#[derive(Event)]
struct PotActiveEvent(PotType);

fn activate_pot(mut ev_activate_pot: EventReader<PotActiveEvent>) {
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

fn unload_pots(mut commands: Commands, query: Query<Entity, With<PotTag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
struct PotTag;

#[derive(Component, PartialEq, Clone)]
pub(crate) enum PotType {
    PotJ,
    PotI,
    PotK,
    PotO,
    PotL,
}

#[derive(Component)]
enum PotState {
    Active,
    Inactive,
}
