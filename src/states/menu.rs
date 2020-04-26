use amethyst::prelude::*;
use amethyst::input::{VirtualKeyCode, is_key_down};
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};
use amethyst::assets::{Loader};
use crate::states::*;


#[derive(Default)]
pub struct Menu;


impl SimpleState for Menu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        create_text(world);        

    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();
        create_text(world);
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Space) {
                return Trans::Push(Box::new(Snake::default()))
            }
        }
        Trans::None
    }
}


fn create_text(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "fonts/yoster.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    
    let welcome_transform = UiTransform::new("welcome".to_string(), Anchor::Middle, Anchor::Middle, 0., 100., 0., 500., 100.);
    let play_test_transform = UiTransform::new("play_text".to_string(), Anchor::Middle, Anchor::Middle, 0., -100., 0., 500., 25.);
    world
        .create_entity()
        .with(welcome_transform)
        .with(UiText::new(font.clone(), "S N A K E".to_string(), [1., 1., 1., 1.], 75.))
        .build();

    world
        .create_entity()
        .with(play_test_transform)
        .with(UiText::new(font, "Press space to start playing".to_string(), [1., 1., 1., 1.], 25.))
        .build();
}