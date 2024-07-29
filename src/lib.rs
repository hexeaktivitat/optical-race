use bevy::prelude::*;
// use bevy_console::ConsoleCommand;
// use clap::Parser;
use input::{InputPlugin, InputSet};
use led::{LedPlugin, LedSet};
use loading::LoadingPlugin;
use menu::{MenuPlugin, MenuSet, PauseSet};
use osc::{OscPlugin, OscSet};
// use player::{PlayerPlugin, PlayerSet};
use pot::{PotPlugin, PotSet};
use track::{TrackPlugin, TrackSet};

mod input;
mod loading;
mod menu;
mod osc;
// mod player;
mod led;
mod pot;
mod track;

pub struct OpticalRacePlugin;

impl Plugin for OpticalRacePlugin {
    fn build(&self, app: &mut App) {
        // state setup
        app.insert_state(ApplicationState::Menu)
            .init_state::<ModeState>()
            .init_state::<PauseState>();

        app.configure_sets(
            OnEnter(ApplicationState::InGame),
            (
                OscSet
                    // .run_if(in_state(ApplicationState::Loading))
                    // .run_if(in_state(ApplicationState::Menu))
                    .run_if(in_state(ApplicationState::InGame)),
                // .run_if(in_state(ApplicationState::Freeform)),
                PotSet.run_if(in_state(ApplicationState::InGame)), // .run_if(in_state(ApplicationState::Freeform)),
            ),
        );
        // app.configure_sets(
        //     OnEnter(ApplicationState::Freeform),
        //     (
        //         OscSet,
        //         // .run_if(in_state(ApplicationState::Loading))
        //         // .run_if(in_state(ApplicationState::Menu))
        //         // .run_if(in_state(ApplicationState::Freeform)),
        //         // .run_if(in_state(ApplicationState::Freeform)),
        //         PotSet, // .run_if(in_state(ApplicationState::Freeform)), // .run_if(in_state(ApplicationState::Freeform)),
        //     ),
        // );

        app.configure_sets(OnEnter(ApplicationState::Menu), MenuSet);
        // app.configure_sets(OnEnter(ApplicationState::Loading), LoadingSet);
        app.configure_sets(
            Update,
            (
                // PlayerSet
                //     .run_if(in_state(ApplicationState::InGame))
                //     .run_if(in_state(PauseState::Unpaused))
                //     .run_if(in_state(ModeState::Singleplayer)),
                InputSet,
                PauseSet.run_if(in_state(PauseState::Paused)),
            ),
        );

        app.configure_sets(Update, (MenuSet.run_if(in_state(ApplicationState::Menu)),));

        app.configure_sets(
            FixedUpdate,
            (
                LedSet
                    .run_if(in_state(ApplicationState::InGame))
                    // .run_if(in_state(ApplicationState::Freeform))
                    .run_if(in_state(PauseState::Unpaused)),
                TrackSet
                    .run_if(in_state(ApplicationState::InGame))
                    // .run_if(in_state(ApplicationState::Freeform))
                    .run_if(in_state(PauseState::Unpaused)),
            ),
        );
        // app.configure_sets(
        //     FixedUpdate,
        //     (
        //         LedSet
        //             // .run_if(in_state(ApplicationState::InGame))
        //             .run_if(in_state(ApplicationState::Freeform))
        //             .run_if(in_state(PauseState::Unpaused)),
        //         TrackSet
        //             // .run_if(in_state(ApplicationState::InGame))
        //             .run_if(in_state(ApplicationState::Freeform))
        //             .run_if(in_state(PauseState::Unpaused)),
        //     ),
        // );

        app.add_systems(
            Update,
            score_display.run_if(in_state(ApplicationState::InGame)),
        );

        // resources
        // app.insert_resource(ResourceStruct {})
        // app.insert_resource(Time::<Fixed>::from_hz(64.0));
        app.insert_resource(Score {
            value: 0,
            updated: true,
        });

        // plugins
        app.add_plugins((
            // PlayerPlugin,
            MenuPlugin,
            InputPlugin,
            LoadingPlugin,
            OscPlugin,
            PotPlugin,
            LedPlugin,
            TrackPlugin,
        ));

        // systems
        app.add_systems(OnEnter(ApplicationState::Exit), exit_game);

        // console comands
    }
}

#[derive(Resource)]
pub(crate) struct Score {
    pub(crate) value: u64,
    pub(crate) updated: bool,
}

#[derive(Component)]
struct ScoreDispTag;

fn score_display(
    mut commands: Commands,
    mut score: ResMut<Score>,
    query: Query<Entity, With<ScoreDispTag>>,
    state: Res<State<ModeState>>,
) {
    if score.updated || state.get() != &ModeState::Freeform {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        let score_disp = format!("SCORE: {}", score.value);
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    score_disp,
                    TextStyle {
                        font_size: 32.,
                        ..default()
                    },
                ),
                transform: Transform::from_translation(Vec3::new(0., 228., 104.)),
                text_anchor: bevy::sprite::Anchor::CenterRight,
                sprite_source: bevy::sprite::SpriteSource,
                ..default()
            },
            ScoreDispTag,
        ));
        score.updated = false;
    } else {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn exit_game(mut commands: Commands, window: Query<Entity, With<Window>>) {
    for game_app in window.iter() {
        commands.entity(game_app).despawn();
    }
}

// console commands

// #[derive(Parser, ConsoleCommand)]
// #[command(name = "echo")]
// struct EchoCommand {
//     msg: String,
// }

// fn echo_command(mut log: ConsoleCommand<EchoCommand>) {
//     if let Some(Ok(EchoCommand { msg })) = log.take() {
//         log.reply(msg);
//     }
// }

// game states

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApplicationState {
    Loading,
    Menu,
    InGame,
    Exit,
    Freeform,
}

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ModeState {
    #[default]
    NotInGame,
    Singleplayer,
    Freeform,
}

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum PauseState {
    #[default]
    Unpaused,
    Paused,
}
