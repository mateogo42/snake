use crate::{despawn_screen, GameState};
use bevy::{app::AppExit, prelude::*};

pub struct MenuPlugin;

#[derive(Component)]
struct MenuScreen;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup)
            .add_systems(Update, handle_actions.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), despawn_screen::<MenuScreen>);
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
                ..default()
            },
            MenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "S N A K E",
                    TextStyle {
                        font: asset_server.load("fonts/yoster.ttf"),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
            parent.spawn(TextBundle::from_section(
                "Press SPACE to start playing",
                TextStyle {
                    font: asset_server.load("fonts/yoster.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn handle_actions(
    input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }

    if input.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Game);
        println!("Playing")
    }
}
