use fastrand::Rng;

use crate::vec2::Vec2;

#[derive(Clone, Debug)]
pub struct Road {
    pub start: Vec2,
    pub end: Vec2,
}

pub fn generate_roads(rng: &mut Rng) -> Vec<Road> {
    vec![
        Road {
            start: Vec2 { x: 100.0, y: 100.0 },
            end: Vec2 { x: 600.0, y: 300.0 },
        },
        Road {
            start: Vec2 { x: 200.0, y: 500.0 },
            end: Vec2 { x: 700.0, y: 120.0 },
        },
    ]
}
