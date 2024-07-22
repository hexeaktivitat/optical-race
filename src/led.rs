use std::time::Duration;

use bevy::prelude::*;

use crate::{ApplicationState, ModeState};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct LedSet;

pub(super) struct LedPlugin;

impl Plugin for LedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::Loading), load_leds.in_set(LedSet));
        app.add_systems(
            OnEnter(ApplicationState::InGame),
            init_led_timer.in_set(LedSet),
        );
        app.add_systems(FixedUpdate, tick_leds.in_set(LedSet));
        app.add_systems(OnEnter(ModeState::NotInGame), unload_leds.in_set(LedSet));
    }
}

#[derive(Component)]
struct LedTick {
    timer: Timer,
    next_led: LedPos,
}

fn init_led_timer(mut commands: Commands) {
    let timer = LedTick {
        timer: Timer::new(Duration::from_millis(333), TimerMode::Repeating),
        next_led: LedPos::A,
    };

    println!("timer spawned");
    commands.spawn(timer);
}

fn tick_leds(
    server: Res<AssetServer>,
    mut timer_query: Query<&mut LedTick>,
    mut query: Query<(&mut LedState, &LedPos, &mut Handle<Image>)>,
    time: Res<Time>,
) {
    for mut tick in timer_query.iter_mut() {
        tick.timer.tick(time.delta());
        if tick.timer.finished() {
            for (mut state, pos, mut tex) in query.iter_mut() {
                let on_tex = server.load("led_on.png");
                let off_tex = server.load("led_off.png");
                if *state == LedState::On {
                    *state = LedState::Off;
                    *tex = off_tex;
                }
                if *pos == tick.next_led {
                    *state = LedState::On;
                    *tex = on_tex;
                }
            }
            tick.next_led = match tick.next_led {
                LedPos::A => LedPos::B,
                LedPos::B => LedPos::C,
                LedPos::C => LedPos::D,
                LedPos::D => LedPos::E,
                LedPos::E => LedPos::F,
                LedPos::F => LedPos::G,
                LedPos::G => LedPos::H,
                LedPos::H => LedPos::A,
            };
        }
    }
}

#[derive(Bundle)]
struct LedBundle {
    state: LedState,
    pos: LedPos,
    sprite: SpriteBundle,
    tag: LedTag,
}

fn load_leds(mut commands: Commands, server: Res<AssetServer>) {
    let origin_x = -150.;
    let origin_y = 100.;
    let offset = 32.;

    let tex = server.load("led_off.png");
    let on_tex = server.load("led_on.png");

    let led_a = LedBundle {
        state: LedState::On,
        pos: LedPos::A,
        tag: LedTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(origin_x, origin_y, 0.)),

            texture: on_tex,
            ..default()
        },
    };
    commands.spawn(led_a);

    let led_b = LedBundle {
        state: LedState::Off,
        pos: LedPos::B,
        tag: LedTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(origin_x + offset * 1., origin_y, 0.)),

            texture: tex.clone(),
            ..default()
        },
    };
    commands.spawn(led_b);

    let led_c = LedBundle {
        state: LedState::Off,
        pos: LedPos::C,
        tag: LedTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(origin_x + offset * 2., origin_y, 0.)),

            texture: tex.clone(),
            ..default()
        },
    };
    commands.spawn(led_c);

    let led_d = LedBundle {
        state: LedState::Off,
        pos: LedPos::D,
        tag: LedTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(origin_x + offset * 3., origin_y, 0.)),

            texture: tex.clone(),
            ..default()
        },
    };
    commands.spawn(led_d);

    let led_e = LedBundle {
        state: LedState::Off,
        pos: LedPos::E,
        tag: LedTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(origin_x + offset * 4., origin_y, 0.)),

            texture: tex.clone(),
            ..default()
        },
    };
    commands.spawn(led_e);

    let led_f = LedBundle {
        state: LedState::Off,
        pos: LedPos::F,
        tag: LedTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(origin_x + offset * 5., origin_y, 0.)),

            texture: tex.clone(),
            ..default()
        },
    };
    commands.spawn(led_f);

    let led_g = LedBundle {
        state: LedState::Off,
        pos: LedPos::G,
        tag: LedTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(origin_x + offset * 6., origin_y, 0.)),

            texture: tex.clone(),
            ..default()
        },
    };
    commands.spawn(led_g);

    let led_h = LedBundle {
        state: LedState::Off,
        pos: LedPos::H,
        tag: LedTag,
        sprite: SpriteBundle {
            transform: Transform::from_translation(Vec3::new(origin_x + offset * 7., origin_y, 0.)),

            texture: tex,
            ..default()
        },
    };
    commands.spawn(led_h);
}

fn unload_leds(mut commands: Commands, query: Query<Entity, With<LedTag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
struct LedTag;

#[derive(Component, PartialEq, Eq)]
enum LedState {
    On,
    Off,
}

#[derive(Component, PartialEq, Debug)]
enum LedPos {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
