extern crate rand;

use rand::Rng;

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    prelude::*,
    core::Transform,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, Texture, SpriteSheetFormat},
    ecs::prelude::{Component, DenseVecStorage, NullStorage, Entity},
    core::math::Vector3
};

pub const HEIGHT: f32 = 640.0;
pub const WIDTH: f32 = 640.0;
pub const SNAKE_VELOCITY: f32 = 1.0;
pub const SCALE: f32 = 1.0;
pub const SPRITE_WIDTH: f32 = 32.0 * SCALE;

#[derive(Default)]
pub struct Snake {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Direction {
    Up,
    UpRight,
    UpLeft,
    Down,
    DownRight,
    DownLeft,
    Right,
    RightUp,
    RightDown,
    Left,
    LeftUp,
    LeftDown
}

pub struct BodyPart {
    pub part: Entity,
    pub dir: Direction,
}

#[derive(Default)]
pub struct Head;

#[derive(Default)]
pub struct Tail;

#[derive(Default)]
pub struct Body;

#[derive(Default)]
pub struct Player {
    pub snake: Vec<BodyPart>,
    pub vel: (f32, f32)
}

impl Player {
    fn new(head: Entity, body: Entity, tail:Entity) -> Self {
        Player {
            snake: vec![BodyPart{part: head, dir: Direction::Right}, 
                        BodyPart{part: body, dir: Direction::Right}, 
                        BodyPart{part: tail, dir: Direction::Right}],
            vel: (SNAKE_VELOCITY, 0.0)
        }
    }
}

#[derive(Default)]
pub struct Food;

impl Component for Head {
    type Storage = NullStorage<Self>;
}

impl Component for Tail {
    type Storage = NullStorage<Self>;
}

impl Component for Body {
    type Storage = NullStorage<Self>;
}

impl Component for Food {
    type Storage = DenseVecStorage<Self>;
}



impl SimpleState for Snake {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        world.register::<Head>();
        world.register::<Body>();
        world.register::<Tail>();
        world.register::<Food>();
        initialise_camera(world);
        initialise_food(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_snake(world, self.sprite_sheet_handle.clone().unwrap());
        println!("Game started!");
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(WIDTH * 0.5, HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(WIDTH, HEIGHT))
        .with(transform)
        .build();
}

fn initialise_snake(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut head_transform = Transform::default();
    let mut tail_transform = Transform::default();
    let mut body_transform = Transform::default();


    let head_sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 4
    };
    let body_sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 1
    };
    let tail_sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 11
    };

    head_transform.set_translation_xyz((WIDTH * 0.5).floor() + SPRITE_WIDTH * 0.5, (HEIGHT * 0.5).floor() + SPRITE_WIDTH * 0.5, 0.0);
    body_transform.set_translation_xyz(head_transform.translation().x - SPRITE_WIDTH * 0.5, head_transform.translation().y, 0.0);
    tail_transform.set_translation_xyz(body_transform.translation().x - SPRITE_WIDTH * 0.5, body_transform.translation().y, 0.0);

    head_transform.set_scale(Vector3::new(SCALE, SCALE, 1.0));
    body_transform.set_scale(Vector3::new(SCALE, SCALE, 1.0));
    tail_transform.set_scale(Vector3::new(SCALE, SCALE, 1.0));

    let head = world
        .create_entity()
        .with(Head::default())
        .with(head_sprite_render)
        .with(head_transform)
        .build();

    let body = world
        .create_entity()
        .with(Body::default())
        .with(body_sprite_render)
        .with(body_transform)
        .build();

    let tail = world
        .create_entity()
        .with(Tail::default())
        .with(tail_sprite_render)
        .with(tail_transform)
        .build();

    world.insert(Player::new(head, body, tail));
}

fn initialise_food(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(1.0, WIDTH / SPRITE_WIDTH).floor();
    let y = rng.gen_range(1.0, HEIGHT / SPRITE_WIDTH).floor() ;
    transform.set_translation_xyz((x - 0.5) * SPRITE_WIDTH + SPRITE_WIDTH, (y - 0.5) * SPRITE_WIDTH, 0.0);
    transform.set_scale(Vector3::new(SCALE, SCALE, 1.0));

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 12
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Food::default())
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/snake_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/snake_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store
    )
}
