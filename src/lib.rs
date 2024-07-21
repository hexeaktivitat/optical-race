use bevy::prelude::*;
// use bevy_console::ConsoleCommand;
// use clap::Parser;
use input::{InputPlugin, InputSet};
use loading::LoadingPlugin;
use menu::{MenuPlugin, MenuSet, PauseSet};
use osc::{OscPlugin, OscSet};
// use player::{PlayerPlugin, PlayerSet};
use pot::{PotPlugin, PotSet};

mod input;
mod loading;
mod menu;
mod osc;
// mod player;
mod pot;

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
                PotSet.run_if(in_state(ApplicationState::InGame)),
            ),
        );
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

        // resources
        // app.insert_resource(ResourceStruct {})
        app.insert_resource(Time::<Fixed>::from_hz(60.0));

        // plugins
        app.add_plugins((
            // PlayerPlugin,
            MenuPlugin,
            InputPlugin,
            LoadingPlugin,
            OscPlugin,
            PotPlugin,
        ));

        // systems
        app.add_systems(OnEnter(ApplicationState::Exit), exit_game);

        // console comands
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
}

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ModeState {
    #[default]
    NotInGame,
    Singleplayer,
}

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum PauseState {
    #[default]
    Unpaused,
    Paused,
}
