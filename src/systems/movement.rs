use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{System, SystemData, World, Write, WriteStorage};
use amethyst::core::timing::Stopwatch;

use std::time::{Duration, Instant};
use crate::states::{HEIGHT, Player, Direction, WIDTH, SPRITE_WIDTH};

#[derive(SystemDesc)]
pub struct MoveSystem {
    max_elapsed_time: Duration,
    elapsed_time: Stopwatch
}

impl Default for MoveSystem {
    fn default() -> Self {
        MoveSystem {
            elapsed_time: Stopwatch::new(),
            max_elapsed_time: Duration::from_millis(80),            
        }
    }
}

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Write<'s, Player>,
    );

    fn setup(&mut self, _world: &mut World) {
        self.elapsed_time = Stopwatch::Started(Duration::from_millis(0), Instant::now());
    }

    fn run(&mut self, (mut transforms, mut player): Self::SystemData) {
        if player.is_alive {
            if self.elapsed_time.elapsed() >= self.max_elapsed_time {
                let head_transform = transforms.get_mut(player.snake[0].part).unwrap();
                let mut prev_pos = (head_transform.translation().x, head_transform.translation().y);
                head_transform.prepend_translation_x(player.vel.0 * SPRITE_WIDTH);
                head_transform.prepend_translation_y(player.vel.1 * SPRITE_WIDTH);
                player.snake[0].dir = if player.vel == (1.0, 0.0) {
                    Direction::Right
                } else if player.vel == (-1.0, 0.0) {
                    Direction::Left
                } else if player.vel == (0.0, 1.0) {
                    Direction::Up
                } else if player.vel == (0.0, -1.0) {
                    Direction::Down
                } else {
                    player.snake[0].dir
                };
    
                let mut prev_dir = player.snake[0].dir;
    
                check_bounds(head_transform);
    
                for i in 1..player.snake.len() {
                    let cur_transform = transforms.get_mut(player.snake[i].part).unwrap();
                    let cur_pos = (cur_transform.translation().x, cur_transform.translation().y);
                    let cur_dir = player.snake[i].dir;
    
                    player.snake[i].dir = get_new_dir(prev_dir, cur_dir);
    
                    cur_transform.set_translation_x(prev_pos.0);
                    cur_transform.set_translation_y(prev_pos.1);
                    prev_pos = cur_pos;
                    prev_dir = player.snake[i].dir;
                    
                }
                self.elapsed_time.restart();
            }
        }                
    }
}

fn check_bounds(part: &mut Transform) {
    if part.translation().x > WIDTH {
        part.set_translation_x(SPRITE_WIDTH / 2.0);
    } else if part.translation().x < 0.0 {
        part.set_translation_x(WIDTH - SPRITE_WIDTH / 2.0);
    }

    if part.translation().y > HEIGHT {
        part.set_translation_y(SPRITE_WIDTH / 2.0);
    } else if part.translation().y < 0.0{
        part.set_translation_y(HEIGHT - SPRITE_WIDTH / 2.0);
    }
}

fn get_new_dir(prev: Direction, cur: Direction) -> Direction {
    match cur {
        Direction::Up | Direction::LeftUp | Direction::RightUp => match prev {
                Direction::Right | Direction::RightDown | Direction::RightUp => Direction::UpRight,
                Direction::Left | Direction::LeftDown | Direction::LeftUp => Direction::UpLeft,
                _ => Direction::Up
            },
        Direction::Down | Direction::LeftDown | Direction::RightDown => match prev {
                Direction::Right | Direction::RightDown | Direction::RightUp => Direction::DownRight,
                Direction::Left | Direction::LeftDown | Direction::LeftUp => Direction::DownLeft,
                _ => Direction::Down
            },
        Direction::Right | Direction::DownRight | Direction::UpRight => match prev {
                Direction::Up | Direction::UpRight | Direction::UpLeft => Direction::RightUp,
                Direction::Down | Direction::DownRight | Direction::DownLeft => Direction::RightDown,
                _ => Direction::Right
            },
        Direction::Left | Direction::UpLeft | Direction::DownLeft => match prev {
                Direction::Up | Direction::UpLeft | Direction::UpRight => Direction::LeftUp,
                Direction::Down | Direction::DownLeft | Direction::DownRight => Direction::LeftDown,
                _ => Direction::Left
            },
    }
}
