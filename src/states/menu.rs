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
        let font = world.read_resource::<Loader>().load(
            "fonts/yoster.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

        let welcome_transform = UiTransform::new("welcome".to_string(), Anchor::TopMiddle, Anchor::TopMiddle, 0., 0., 0., 300., 30.);
        world
            .create_entity()
            .with(welcome_transform)
            .with(UiText::new(font, "Hola culero".to_string(), [1., 1., 1., 1.], 25.))
            .build();

    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();
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