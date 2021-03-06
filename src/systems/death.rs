use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{System, SystemData, WriteStorage, Write };

use crate::states::Player;

#[derive(SystemDesc)]
pub struct DeathSystem;

impl<'s> System<'s> for DeathSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Write<'s, Player>
    );

    fn run(&mut self, (mut transforms, mut player): Self::SystemData) {
        if player.is_alive  {
            let mut head_transform = transforms.get_mut(player.snake[0].part).unwrap().clone();
            for i in 1..player.snake.len() {
                let cur_transform = transforms.get(player.snake[i].part).unwrap();
                if check_collision(&mut head_transform, cur_transform) {
                    player.is_alive = false;                  
                }
            }
        }
    }
}

fn check_collision(head: &mut Transform, body: &Transform) -> bool {
    let dist = ((head.translation().x - body.translation().x).powf(2.0) + (head.translation().y - body.translation().y).powf(2.0)).sqrt();

    dist <= 1.0
}
