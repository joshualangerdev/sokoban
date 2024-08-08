use ggez::{
    conf,
    event::{self, EventHandler, KeyCode, KeyMods},
    Context, GameError, GameResult,
};
use specs::{RunNow, World, WorldExt};
use std::path;

// Modules
mod components;
mod constants;
mod entities;
mod map;
mod resources;
mod systems;

use crate::components::*;
use crate::map::*;
use crate::resources::*;
use crate::systems::*;

struct Game {
    world: World,
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        {
            let mut gs = GameStateSystem {};
            gs.run_now(&self.world);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        {
            let mut rs = RenderingSystem { context: ctx };
            rs.run_now(&self.world);
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        println!("Key pressed: {:?}", keycode);
        let mut iq = self.world.write_resource::<InputQueue>();
        iq.keys_pressed.push(keycode);
    }
}

fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
";
    load_map(world, MAP.to_string());
}

fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    // Context and event loop
    let context = ggez::ContextBuilder::new("rust_sokoban", "Sokoban")
        .window_setup(conf::WindowSetup::default().title("SOKOBAN!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (ctx, event_loop) = context.build()?;

    // game state
    let game = Game { world };
    // Run main loop
    event::run(ctx, event_loop, game)
}
