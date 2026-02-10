use fastrand::Rng;

use crate::generator::Point2D;

#[derive(Clone, Debug)]
pub struct Road {
    pub start: Point2D,
    pub end: Point2D,
}

pub fn generate_roads(rng: &mut Rng) -> Vec<Road> {
    vec![
        Road {
            start: Point2D { x: 100.0, y: 100.0 },
            end: Point2D { x: 600.0, y: 300.0 },
        },
        Road {
            start: Point2D { x: 200.0, y: 500.0 },
            end: Point2D { x: 700.0, y: 120.0 },
        },
    ]
}
