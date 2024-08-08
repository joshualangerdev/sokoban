use ggez::{
    graphics::{self, Color, DrawParam, Image},
    Context,
};
use glam::Vec2;
use specs::{Join, Read, ReadStorage, System};

use crate::components::{Position, Renderable};
use crate::{constants::*, Gameplay};

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

// Implementation of the Rendering System
impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Read<'a, Gameplay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, positions, renderables) = data;

        graphics::clear(
            self.context,
            graphics::Color {
                r: 0.95,
                g: 0.95,
                b: 0.95,
                a: 1.0,
            },
        );
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        for (position, renderable) in rendering_data.iter() {
            let image =
                Image::new(self.context, renderable.path.clone()).expect("expected an image");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        self.draw_text(&gameplay.state.to_string(), 525.0, 80.0);
        self.draw_text(&gameplay.moves_count.to_string(), 525.0, 100.0);

        graphics::present(self.context).expect("expect display present");
    }
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let dest = Vec2::new(x, y);
        let color = Some(Color::new(0.0, 0.0, 0.0, 1.0));
        let dimensions = Vec2::new(0.0, 20.0);

        graphics::queue_text(self.context, &text, dimensions, color);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(dest),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("Expected drawing queued text");
    }
}
