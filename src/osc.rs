use bevy::prelude::*;

use crate::{ApplicationState, ModeState};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct OscSet;

pub(super) struct OscPlugin;

impl Plugin for OscPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::Loading), load_oscs.in_set(OscSet));
        app.add_systems(Update, osc_inputs.in_set(OscSet));
        app.add_systems(OnEnter(ModeState::NotInGame), unload_oscs.in_set(OscSet));
    }
}

fn load_oscs(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut next_app_state: ResMut<NextState<ApplicationState>>,
    mut next_game_state: ResMut<NextState<ModeState>>,
) {
    let origin_x = -128.;
    let origin_y = 0.;

    let sine_sprite: Handle<Image> = server.load("sine_tile.png");
    let sine = OscBundle {
        tag: OscTag,
        osc_type: OscType::Sine,
        state: OscState::Inactive,
        sprite: SpriteBundle {
            texture: sine_sprite,
            transform: Transform::from_translation(Vec3::new(origin_x - 32., origin_y, 0.)),
            ..default()
        },
    };
    commands.spawn(sine);

    let triangle_sprite: Handle<Image> = server.load("triangle_tile.png");
    let triangle = OscBundle {
        tag: OscTag,
        osc_type: OscType::Triangle,
        state: OscState::Inactive,
        sprite: SpriteBundle {
            texture: triangle_sprite,
            transform: Transform::from_translation(Vec3::new(origin_x, origin_y + 32., 0.)),
            ..default()
        },
    };
    commands.spawn(triangle);

    let square_sprite: Handle<Image> = server.load("square_tile.png");
    let square = OscBundle {
        tag: OscTag,
        osc_type: OscType::Square,
        state: OscState::Inactive,
        sprite: SpriteBundle {
            texture: square_sprite,
            transform: Transform::from_translation(Vec3::new(origin_x, origin_y, 0.)),
            ..default()
        },
    };
    commands.spawn(square);

    let sawtooth_sprite: Handle<Image> = server.load("saw_tile.png");
    let sawtooth = OscBundle {
        tag: OscTag,
        osc_type: OscType::Sawtooth,
        state: OscState::Inactive,
        sprite: SpriteBundle {
            texture: sawtooth_sprite,
            transform: Transform::from_translation(Vec3::new(origin_x + 32., origin_y, 0.)),
            ..default()
        },
    };
    commands.spawn(sawtooth);

    next_app_state.set(ApplicationState::InGame);
    next_game_state.set(ModeState::Singleplayer);
}

fn osc_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &OscType, &mut OscState, &mut Handle<Image>), With<OscTag>>,
    server: Res<AssetServer>,
) {
    for (_entity, osc_type, mut state, mut texture) in query.iter_mut() {
        for key in keys.get_pressed() {
            match key {
                KeyCode::KeyD => {
                    if osc_type == &OscType::Sawtooth {
                        *state = OscState::Active;
                        *texture = server.load("saw_tile_on.png");
                    }
                }
                KeyCode::KeyA => {
                    if osc_type == &OscType::Sine {
                        *state = OscState::Active;
                        *texture = server.load("sine_tile_on.png");
                    }
                }
                KeyCode::KeyS => {
                    if osc_type == &OscType::Square {
                        *state = OscState::Active;
                        *texture = server.load("square_tile_on.png");
                    }
                }
                KeyCode::KeyW => {
                    if osc_type == &OscType::Triangle {
                        *state = OscState::Active;
                        *texture = server.load("triangle_tile_on.png");
                    }
                }
                _ => {}
            }
        }
        for key in keys.get_just_released() {
            match key {
                KeyCode::KeyD => {
                    if osc_type == &OscType::Sawtooth {
                        *state = OscState::Inactive;
                        *texture = server.load("saw_tile.png");
                    }
                }
                KeyCode::KeyA => {
                    if osc_type == &OscType::Sine {
                        *state = OscState::Inactive;
                        *texture = server.load("sine_tile.png");
                    }
                }
                KeyCode::KeyS => {
                    if osc_type == &OscType::Square {
                        *state = OscState::Inactive;
                        *texture = server.load("square_tile.png");
                    }
                }
                KeyCode::KeyW => {
                    if osc_type == &OscType::Triangle {
                        *state = OscState::Inactive;
                        *texture = server.load("triangle_tile.png");
                    }
                }
                _ => {}
            }
        }
    }
}

fn unload_oscs(mut commands: Commands, query: Query<Entity, With<OscTag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component, PartialEq)]
pub(crate) enum OscState {
    Active,
    Inactive,
}

#[derive(Component, PartialEq, Clone)]
pub(crate) enum OscType {
    Sine,
    Triangle,
    Square,
    Sawtooth,
}

#[derive(Component)]
struct OscTag;

#[derive(Bundle)]
struct OscBundle {
    tag: OscTag,
    osc_type: OscType,
    state: OscState,
    sprite: SpriteBundle,
}
