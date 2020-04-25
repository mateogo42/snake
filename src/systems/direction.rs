use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, Write, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;

use crate::states::{Player, Direction};

#[derive(SystemDesc)]
pub struct DirectionSystem;

impl<'s> System<'s> for DirectionSystem {
    type SystemData = (
        Write<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, SpriteRender>
    );

    fn run(&mut self, (mut player, input, mut sprites): Self::SystemData) {
        if player.snake.len() > 0 {
            let cur_dir = player.snake[0].dir;
            let movement = match cur_dir {
                Direction:: Up | Direction::Down => input.axis_value("horizontal"),
                Direction:: Left | Direction::Right => input.axis_value("vertical"),
                _ => None
            };
            
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    match cur_dir {
                        Direction:: Up | Direction::Down => {
                            player.vel = (mv_amount, 0.0);
                            
                        },
                        Direction:: Left | Direction::Right => {
                            player.vel = (0.0, mv_amount);
                        },
                        _ => ()
                    };                
                }

            }

            let mut head_sprite = sprites.get_mut(player.snake[0].part).unwrap();
            head_sprite.sprite_number = match player.snake[0].dir {
                Direction::Up => 3,
                Direction::Down => 8,
                Direction::Right => 4,
                Direction::Left => 7,
                _ => head_sprite.sprite_number,
            };

            for i in 1..player.snake.len() {
                if i == player.snake.len() - 1 {
                    let mut tail_sprite = sprites.get_mut(player.snake.last().unwrap().part.clone()).unwrap();
                    tail_sprite.sprite_number = match player.snake[i].dir {
                        Direction::Up | Direction::RightUp | Direction::LeftUp => 10,
                        Direction::Down | Direction::LeftDown | Direction::RightDown => 16,
                        Direction::Right | Direction::DownRight | Direction::UpRight => 11,
                        Direction::Left | Direction::UpLeft | Direction::DownLeft => 15,
                    };
                } else {
                    let mut body_sprite = sprites.get_mut(player.snake[i].part).unwrap();
                    body_sprite.sprite_number = match player.snake[i].dir {
                        Direction::Up | Direction::Down => 6,
                        Direction::Right | Direction::Left => 1,
                        Direction::UpRight | Direction::LeftDown => 0,
                        Direction::DownRight | Direction::LeftUp => 5,
                        Direction::RightUp | Direction::DownLeft => 9,
                        Direction::UpLeft | Direction::RightDown => 2,
                    };
                }
            }
        }
    }
}
