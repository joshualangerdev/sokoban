use std::collections::HashMap;

use specs::{Join, ReadStorage, System, Write};

use crate::{Box, BoxSpot, Gameplay, GameplayState, Position};

pub struct GameStateSystem {}

impl<'a> System<'a> for GameStateSystem {
    type SystemData = (
        Write<'a, Gameplay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gameplay, position, boxes, box_spots) = data;

        let boxes_by_position: HashMap<(u8, u8), &Box> = (&position, &boxes)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect::<HashMap<_, _>>();

        for (_box_spot, position) in (&box_spots, &position).join() {
            if boxes_by_position.contains_key(&(position.x, position.y)) {
                // continue
            } else {
                gameplay.state = GameplayState::Playing;
                return;
            }
        }

        gameplay.state = GameplayState::Won;
    }
}
