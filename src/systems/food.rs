extern crate rand;

use rand::Rng;
use rand::seq::SliceRandom;
use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Entities, System, SystemData, WriteStorage, Write, ReadStorage, WriteExpect};
use amethyst::core::math::Vector3;
use amethyst::renderer::SpriteRender;
use amethyst::ui::UiText;

use crate::states::{Player, ScoreText, Food, Body, BodyPart, WIDTH, SPRITE_WIDTH, HEIGHT, SCALE};
#[derive(SystemDesc)]
pub struct FoodSystem;

impl<'s> System<'s> for FoodSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Food>,
        Write<'s, Player>,
        Write<'s, ScoreText>,
        WriteStorage<'s, Body>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, UiText>,
        Entities<'s>,

    );

    fn run(&mut self, (mut transforms, foods, mut player, mut score_text, mut bodies, mut sprites, mut ui_text, entities): Self::SystemData) {
        if player.is_alive {
            let food_ids: [usize; 3] = [12, 13, 14];
            let head_transform = transforms.get_mut(player.snake[0].part).unwrap().clone();
            let mut did_eat = false;
            for (_, transform, sprite) in (&foods, &mut transforms, &mut sprites).join() {
                if can_eat(&head_transform, transform) {
                    let mut rng = rand::thread_rng();
                    let x = rng.gen_range(1.0, WIDTH / SPRITE_WIDTH).floor();
                    let y = rng.gen_range(1.0, HEIGHT / SPRITE_WIDTH).floor();
                    sprite.sprite_number = *food_ids.choose(&mut rng).unwrap();
                    transform.set_translation_xyz((x - 0.5) * SPRITE_WIDTH + SPRITE_WIDTH, (y - 0.5) * SPRITE_WIDTH, 0.0);
                    did_eat = true;
                }
            }

            if did_eat {
                if let Some(text) = ui_text.get_mut(score_text.entity[0]) {
                    score_text.score += 1;
                    text.text = format!("{:0>3}", score_text.score);
                }                
                let mut transform = Transform::default();
                transform.set_translation_xyz(
                    head_transform.translation().x - player.vel.0 * SPRITE_WIDTH * 0.5,
                    head_transform.translation().y - player.vel.1 * SPRITE_WIDTH * 0.5,
                    0.0);
                transform.set_scale(Vector3::new(SCALE, SCALE, 1.0));
                let sprite_render = sprites.get(player.snake[1].part).unwrap().clone();
                let new_body = entities
                                .build_entity()
                                .with(transform, &mut transforms)
                                .with(Body, &mut bodies)
                                .with(sprite_render, &mut sprites)
                                .build();

                let new_dir = player.snake[1].dir;
                player.snake.insert(1, BodyPart{part: new_body, dir: new_dir});
            }
        }
    }
}

fn can_eat(snake: &Transform, food: &Transform) -> bool {
    if ((snake.translation().x - food.translation().x).powf(2.0) + (snake.translation().y - food.translation().y).powf(2.0)).sqrt() <= 1.0  {
        println!("Eaten!");
        return true
    }
    false
}
