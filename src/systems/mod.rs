pub use self::movement::MoveSystem;
pub use self::direction::DirectionSystem;
pub use self::food::FoodSystem;
pub use self::death::DeathSystem;

mod movement;
mod direction;
mod food;
mod death;

use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::prelude::{DispatcherBuilder, World};

pub struct GameplaySystemBundle;


impl<'a, 'b> SystemBundle<'a, 'b> for GameplaySystemBundle{
    fn build(self, _world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> amethyst::Result<()> {
        builder.add(DirectionSystem, "direction_system", &["input_system"]);
        builder.add(MoveSystem::default(), "move_system", &[]);
        builder.add(FoodSystem, "food_system", &[]);
        builder.add(DeathSystem, "death_system", &[]);
        Ok(())        
    }
}