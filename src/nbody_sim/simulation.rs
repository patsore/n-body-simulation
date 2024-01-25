use std::sync::{Arc, Mutex};
use cgmath::{InnerSpace, Vector2};
use crate::nbody_sim::Body;
use crate::State;
use std::borrow::BorrowMut;
use rand::Rng;
use std::f32::consts::TAU;

pub struct Simulation {
    bodies: Vec<Body>,
    state: Arc<Mutex<State>>,
}

const T: f32 = 0.05;

impl Simulation {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        let mut bodies = create_spiral_cluster(50, 0.5);

        Simulation {
            bodies,
            state,
        }
    }

    fn render(&mut self) {
        let circle_bodies = self.bodies.clone().iter().map(Body::to_circle).collect::<Vec<_>>();
        self.state.lock().unwrap().update_circles(circle_bodies);
    }

    pub fn update(&mut self) {
        let bodies_len = self.bodies.len();
        for body_from_i in 0..bodies_len - 1 {
            for body_other in body_from_i + 1..bodies_len {
                let body_to_vec = self.bodies[body_from_i].position - self.bodies[body_other].position;
                // println!("pos0: {:?}, pos1: {:?}", self.bodies[0].position, self.bodies[1].position);
                let a = self.bodies[body_from_i].compute_acceleration_to_other_body(&self.bodies[body_other]);
                // println!("a: {a}");
                self.bodies[body_from_i].acceleration += body_to_vec * -1.0 * a;
                self.bodies[body_other].acceleration += body_to_vec * a;
            }
        }

        for body in self.bodies.iter_mut() {
            body.update(T);
        }
        self.render();
    }
}

fn create_spiral_cluster(num_bodies: usize, average_spacing: f32) -> Vec<Body> {
    let mut bodies = Vec::with_capacity(num_bodies);

    let mut angle: f32 = 0.0;
    let mut radius: f32 = 0.0;

    for _ in 0..num_bodies {
        let x = radius * angle.cos();
        let y = radius * angle.sin();

        let body = Body::new_sp(Vector2::new(x, y), 0.05, Vector2::new(0.0, 0.0));
        bodies.push(body);

        angle += 20.0; // You can adjust this value for tighter or looser spirals
        radius += average_spacing;
    }

    bodies
}