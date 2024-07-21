// use bevy::{input::keyboard::KeyCode, prelude::*};

// use crate::ApplicationState;

// pub(super) struct PlayerPlugin;

// #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
// pub(super) struct PlayerSet;

// impl Plugin for PlayerPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(
//             OnEnter(ApplicationState::Loading),
//             (player_setup).in_set(PlayerSet),
//         )
//         .add_systems(Update, (player_input).in_set(PlayerSet));
//     }
// }

// #[derive(Component)]
// struct PlayerTag;

// #[derive(Bundle)]
// struct PlayerBundle {
//     tag: PlayerTag,
//     sprite: SpriteBundle,
// }

// // player specific systems

// fn player_setup(
//     mut commands: Commands,
//     server: Res<AssetServer>,
//     previous_player: Query<Entity, With<PlayerTag>>,
//     mut next_state: ResMut<NextState<ApplicationState>>,
// ) {
//     for entity in previous_player.iter() {
//         commands.entity(entity).despawn();
//     }
//     let player_sprite: Handle<Image> = server.load("default.png");
//     let player = PlayerBundle {
//         tag: PlayerTag,
//         sprite: SpriteBundle {
//             texture: player_sprite,
//             transform: Transform::from_xyz(0., 0., 100.),
//             ..default()
//         },
//     };
//     commands.spawn(player);
//     next_state.set(ApplicationState::InGame);
// }

// fn player_input(
//     keys: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
//     mut query: Query<(&mut Transform, Entity), With<PlayerTag>>,
// ) {
//     for (mut position, _entity) in query.iter_mut() {
//         let translate = 250. * time.delta_seconds();

//         for key in keys.get_pressed() {
//             match key {
//                 KeyCode::ArrowDown | KeyCode::KeyS => position.translation.y -= translate,
//                 KeyCode::ArrowLeft | KeyCode::KeyA => position.translation.x -= translate,
//                 KeyCode::ArrowRight | KeyCode::KeyD => position.translation.x += translate,
//                 KeyCode::ArrowUp | KeyCode::KeyW => position.translation.y += translate,
//                 _ => {}
//             }
//         }
//     }
// }
