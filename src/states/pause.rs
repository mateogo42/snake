use crate::{despawn_screen, GameState};
use bevy::prelude::*;

pub struct PausePlugin;

#[derive(Component)]
struct PauseScreen;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Pause), setup)
            .add_systems(Update, handle_actions.run_if(in_state(GameState::Pause)))
            .add_systems(OnExit(GameState::Pause), despawn_screen::<PauseScreen>);
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.5)),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            },
            PauseScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PAUSED",
                TextStyle {
                    font: asset_server.load("fonts/yoster.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Press SPACE to resume playing",
                TextStyle {
                    font: asset_server.load("fonts/yoster.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn handle_actions(input: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Game);
    }
}
