use crate::{despawn_screen, GameState, HEIGHT, WIDTH};
use bevy::prelude::*;
use rand::Rng;

const ROWS: i8 = 10;
const COLS: i8 = 10;
const SPRITE_WIDTH: f32 = 16.0;
const SPRITE_HEIGHT: f32 = 16.0;
const SCALE_X: f32 = WIDTH / (COLS as f32 * 2.0 * SPRITE_WIDTH);
const SCALE_Y: f32 = HEIGHT / (ROWS as f32 * 2.0 * SPRITE_HEIGHT);

pub struct GamePlugin;

#[derive(Component)]
struct GameScreen;

#[derive(Component)]
struct Score(usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}
#[derive(Component)]
struct SnakeBody {
    direction: Direction,
    is_tail: bool,
}

#[derive(Component)]
struct Food;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FixedTime::new_from_secs(0.1))
            .add_systems(
                OnTransition {
                    from: GameState::Menu,
                    to: GameState::Game,
                },
                (init_background, init_score, init_snake, init_food),
            )
            .add_systems(
                OnTransition {
                    from: GameState::Game,
                    to: GameState::Menu,
                },
                despawn_screen::<GameScreen>,
            )
            .add_systems(
                Update,
                (update_score, handle_keys, handle_eat, check_for_death)
                    .run_if(in_state(GameState::Game)),
            )
            .add_systems(
                FixedUpdate,
                (handle_direction, handle_sprites, handle_movement)
                    .run_if(in_state(GameState::Game)),
            );
    }
}

fn init_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("texture/snake_spritesheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(32.0, 32.0),
        3,
        2,
        None,
        Some(Vec2::new(16.0, 0.0)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    for row in 0..ROWS {
        for col in 0..COLS {
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(5),
                    transform: Transform::from_xyz(
                        -WIDTH / 2.0 + SCALE_X * SPRITE_WIDTH * (col as f32 * 2.0 + 1.0),
                        HEIGHT / 2.0 - SCALE_Y * SPRITE_HEIGHT * (row as f32 * 2.0 + 1.0),
                        -1.0,
                    )
                    .with_scale(Vec3::new(SCALE_X, SCALE_Y, 1.0)),
                    ..default()
                },
                GameScreen,
            ));
        }
    }
}

fn init_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "SCORE: ",
                TextStyle {
                    font: asset_server.load("fonts/yoster.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/yoster.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            ..default()
        }),
        GameScreen,
        Score(0),
    ));
}

fn init_snake(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("texture/snake_spritesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 7, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(4),
            transform: Transform::from_xyz(
                -0.5 * SCALE_X * SPRITE_WIDTH,
                0.5 * SCALE_Y * SPRITE_HEIGHT,
                0.0,
            )
            .with_scale(Vec3::new(SCALE_X, SCALE_Y, 1.0)),
            ..default()
        },
        SnakeHead {
            direction: Direction::Right,
        },
        GameScreen,
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(1),
            transform: Transform::from_xyz(
                -1.5 * SCALE_X * SPRITE_WIDTH,
                0.5 * SCALE_Y * SPRITE_HEIGHT,
                0.0,
            )
            .with_scale(Vec3::new(SCALE_X, SCALE_Y, 1.0)),
            ..default()
        },
        SnakeBody {
            direction: Direction::Right,
            is_tail: false,
        },
        GameScreen,
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(18),
            transform: Transform::from_xyz(
                -2.5 * SCALE_X * SPRITE_WIDTH,
                0.5 * SCALE_Y * SPRITE_HEIGHT,
                0.0,
            )
            .with_scale(Vec3::new(SCALE_X, SCALE_Y, 1.0)),
            ..default()
        },
        SnakeBody {
            direction: Direction::Right,
            is_tail: true,
        },
        GameScreen,
    ));
}

fn init_food(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut rng = rand::thread_rng();
    let texture_handle = asset_server.load("texture/snake_spritesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 7, 4, None, None);
    let x = rng.gen_range(-ROWS, ROWS);
    let y = rng.gen_range(-COLS, COLS);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        {
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(21),
                transform: Transform::from_xyz(
                    WIDTH / (ROWS * 2) as f32 * x as f32 + 0.5 * SCALE_X * SPRITE_WIDTH as f32,
                    HEIGHT / (COLS * 2) as f32 * y as f32 + 0.5 * SCALE_Y * SPRITE_HEIGHT as f32,
                    0.0,
                )
                .with_scale(Vec3::new(SCALE_X, SCALE_Y, 1.0)),
                ..default()
            }
        },
        Food,
        GameScreen,
    ));
}

fn handle_eat(
    mut commands: Commands,
    head_query: Query<&Transform, With<SnakeHead>>,
    mut body_query: Query<
        (
            &mut SnakeBody,
            &mut Transform,
            &mut TextureAtlasSprite,
            &mut Handle<TextureAtlas>,
        ),
        Without<SnakeHead>,
    >,
    mut food_query: Query<
        (&mut Transform, &mut TextureAtlasSprite),
        (With<Food>, Without<SnakeHead>, Without<SnakeBody>),
    >,
    mut score_query: Query<&mut Score>,
) {
    let head_transform = head_query.single();
    let (mut food_transform, mut sprite) = food_query.single_mut();
    let mut score = score_query.single_mut();
    if head_transform.translation == food_transform.translation {
        // Grow bigger
        let (mut tail, tail_transfrom, mut tail_sprite, texture_atlas_handle) =
            body_query.iter_mut().last().unwrap();
        let new_part_pos = tail_transfrom.translation
            + match tail.direction {
                Direction::Right => Vec3::new(-SCALE_X * SPRITE_WIDTH, 0.0, 0.0),
                Direction::Left => Vec3::new(SCALE_X * SPRITE_WIDTH, 0.0, 0.0),
                Direction::Up => Vec3::new(0.0, -SCALE_Y * SPRITE_HEIGHT, 0.0),
                Direction::Down => Vec3::new(0.0, SCALE_Y * SPRITE_HEIGHT, 0.0),
            };

        let old_tail_sprite = tail_sprite.index;
        tail_sprite.index = match tail.direction {
            Direction::Right | Direction::Left => 1,
            Direction::Up | Direction::Down => 9,
        };
        tail.is_tail = false;

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(old_tail_sprite),
                transform: Transform::from_xyz(new_part_pos.x, new_part_pos.y, new_part_pos.z)
                    .with_scale(Vec3::new(SCALE_X, SCALE_Y, 1.0)),
                ..default()
            },
            SnakeBody {
                direction: Direction::Right,
                is_tail: true,
            },
            GameScreen,
        ));

        score.0 += 1;

        // Move food
        let mut rng = rand::thread_rng();
        let new_index = rng.gen_range(21, 24);
        let x = rng.gen_range(-ROWS, ROWS);
        let y = rng.gen_range(-COLS, COLS);
        food_transform.translation = Vec3::new(
            WIDTH / (ROWS * 2) as f32 * x as f32 + 0.5 * SCALE_X * SPRITE_WIDTH as f32,
            HEIGHT / (COLS * 2) as f32 * y as f32 + 0.5 * SCALE_Y * SPRITE_HEIGHT as f32,
            0.0,
        );
        sprite.index = new_index;
    }
}

fn handle_direction(
    mut head_query: Query<&mut SnakeHead>,
    mut body_query: Query<&mut SnakeBody>,
    keys: Res<Input<KeyCode>>,
) {
    let mut head = head_query.single_mut();
    let mut prev_dir = head.direction.clone();
    if keys.pressed(KeyCode::Right) && head.direction != Direction::Left {
        head.direction = Direction::Right;
    } else if keys.pressed(KeyCode::Left) && head.direction != Direction::Right {
        head.direction = Direction::Left;
    } else if keys.pressed(KeyCode::Up) && head.direction != Direction::Down {
        head.direction = Direction::Up;
    } else if keys.pressed(KeyCode::Down) && head.direction != Direction::Up {
        head.direction = Direction::Down;
    }

    for mut part in body_query.iter_mut() {
        let new_dir = prev_dir;
        prev_dir = part.direction.clone();
        part.direction = new_dir;
    }
}

fn handle_movement(
    mut head_query: Query<(&SnakeHead, &mut Transform), Without<SnakeBody>>,
    mut body_query: Query<&mut Transform, (With<SnakeBody>, Without<SnakeHead>)>,
) {
    let (head, mut transform) = head_query.single_mut();
    let mut prev_pos = transform.translation.clone();
    match head.direction {
        Direction::Right => transform.translation.x += SCALE_X * SPRITE_WIDTH,
        Direction::Left => transform.translation.x -= SCALE_X * SPRITE_WIDTH,
        Direction::Up => transform.translation.y += SCALE_Y * SPRITE_HEIGHT,
        Direction::Down => transform.translation.y -= SCALE_Y * SPRITE_HEIGHT,
    };
    for mut part_transform in body_query.iter_mut() {
        let new_pos = part_transform.translation.clone();
        part_transform.translation = prev_pos;
        prev_pos = new_pos;
    }
}

fn handle_sprites(
    mut head_query: Query<(&SnakeHead, &mut TextureAtlasSprite), Without<SnakeBody>>,
    mut body_query: Query<(&SnakeBody, &mut TextureAtlasSprite), Without<SnakeHead>>,
) {
    let (head, mut head_sprite) = head_query.single_mut();
    let mut prev_dir = head.direction;
    head_sprite.index = match head.direction {
        Direction::Right => 4,
        Direction::Left => 10,
        Direction::Up => 3,
        Direction::Down => 11,
    };

    for (part, mut part_sprite) in body_query.iter_mut() {
        if prev_dir != part.direction && !part.is_tail {
            part_sprite.index = match (part.direction, prev_dir) {
                (Direction::Left, Direction::Up) | (Direction::Down, Direction::Right) => 7,
                (Direction::Left, Direction::Down) | (Direction::Up, Direction::Right) => 0,
                (Direction::Right, Direction::Up) | (Direction::Down, Direction::Left) => 16,
                (Direction::Right, Direction::Down) | (Direction::Up, Direction::Left) => 2,
                _ => 0,
            }
        } else if part.is_tail {
            part_sprite.index = match prev_dir {
                Direction::Right => 18,
                Direction::Left => 24,
                Direction::Up => 17,
                Direction::Down => 25,
            }
        } else {
            part_sprite.index = match part.direction {
                Direction::Right | Direction::Left => 1,
                Direction::Up | Direction::Down => 9,
            };
        }

        prev_dir = part.direction;
    }
}

fn check_for_death(
    head_query: Query<&Transform, (With<SnakeHead>, Without<SnakeBody>)>,
    body_query: Query<&Transform, (With<SnakeBody>, Without<SnakeHead>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let head_transform = head_query.single();
    let position_x = head_transform.translation.x;
    let position_y = head_transform.translation.y;
    if (position_x < -WIDTH / 2.0)
        | (position_x > WIDTH / 2.0)
        | (position_y > HEIGHT / 2.0)
        | (position_y < -HEIGHT / 2.0)
    {
        game_state.set(GameState::Menu);
    }
    for body_part_transform in body_query.iter() {
        if head_transform.translation == body_part_transform.translation {
            game_state.set(GameState::Menu);
        }
    }
}

fn handle_keys(input: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if input.pressed(KeyCode::Escape) {
        game_state.set(GameState::Pause);
    }
}

fn update_score(mut query: Query<(&mut Text, &Score)>) {
    let (mut text, score) = query.single_mut();
    text.sections[1].value = format!("{:0>3}", score.0);
}
