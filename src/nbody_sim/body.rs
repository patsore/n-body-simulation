
use cgmath::{MetricSpace, Vector2};
use crate::drawing::Circle;

#[derive(Copy, Clone)]
pub struct Body {
    pub position: Vector2<f32>,
    pub mass: f32,
    pub speed: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub density: f32,
}

pub(crate) const G: f32 = 50.0;

impl Body {
    pub fn new(position: Vector2<f32>, mass: f32, density: f32) -> Self {
        Body {
            position,
            mass,
            speed: Vector2 { x: 0.0, y: 0.0 },
            acceleration: Vector2 { x: 0.0, y: 0.0 },
            density,
        }
    }

    pub fn new_sp(position: Vector2<f32>, mass: f32, speed: Vector2<f32>, density: f32) -> Self {
        Body {
            position,
            mass,
            speed,
            acceleration: Vector2 { x: 0.0, y: 0.0 },
            density,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update velocity and position using Euler method
        self.speed += self.acceleration * dt;
        self.position += self.speed * dt;

        self.acceleration = Vector2 {
            x: 0.0,
            y: 0.0,
        };
    }

    pub fn to_circle(&self) -> Circle {
        Circle {
            world_pos: [self.position.x, self.position.y, 0.0],
            radius: self.mass / self.density,
            color: 0xFFFFFFFF,
        }
    }

    pub fn compute_acceleration_to_other_body(&self, other: &Body) -> f32 {
        // println!("{:?}, {:?}", self.position.x, other.position.x);
        let distance = self.position.distance2(other.position);
        // println!("distance: {distance} {dx} {dy}");
        if distance.floor() == 1.0 {
            return 0.0;
        }
        let a = (G * self.mass * other.mass) / distance;
        return a;
    }
}

