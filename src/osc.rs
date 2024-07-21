use bevy::prelude::*;

use crate::{ApplicationState, ModeState};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct OscSet;

pub(super) struct OscPlugin;

impl Plugin for OscPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::Loading), load_oscs.in_set(OscSet));
        app.add_systems(OnEnter(ModeState::NotInGame), unload_oscs.in_set(OscSet));
    }
}

#[derive(Component)]
enum OscState {
    Active,
    Inactive,
}

#[derive(Component)]
enum OscType {
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

fn load_oscs(mut commands: Commands, server: Res<AssetServer>) {
    let origin_x = 0.;
    let origin_y = 0.;

    let sine_sprite: Handle<Image> = server.load("sine_tile.png");
    let sine = OscBundle {
        tag: OscTag,
        osc_type: OscType::Sine,
        state: OscState::Inactive,
        sprite: SpriteBundle {
            texture: sine_sprite,
            transform: Transform::from_translation(Vec3::new(origin_x - 64., origin_y, 0.)),
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
            transform: Transform::from_translation(Vec3::new(origin_x + 64., origin_y, 0.)),
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
            transform: Transform::from_translation(Vec3::new(origin_x, origin_y + 64., 0.)),
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
            transform: Transform::from_translation(Vec3::new(origin_x, origin_y, 0.)),
            ..default()
        },
    };
    commands.spawn(sawtooth);
}

fn unload_oscs(mut commands: Commands, query: Query<Entity, With<OscTag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
