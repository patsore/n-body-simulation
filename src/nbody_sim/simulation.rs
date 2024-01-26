

use cgmath::{MetricSpace, Vector2, Zero};
use crate::nbody_sim::Body;





use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::drawing::Circle;

pub struct Simulation {
    bodies: Vec<Body>,
}
const T: f32 = 0.001;

impl Simulation {
    pub fn new(num_bodies: usize, spacing: f32) -> Self {
        let bodies = create_spiral_cluster(num_bodies, spacing);

        Simulation {
            bodies,
        }
    }

    pub fn get_bodies_as_circles(&mut self) -> Vec<Circle> {
        let circle_bodies = self.bodies.clone().iter().map(Body::to_circle).collect::<Vec<_>>();
        return circle_bodies;
    }

    pub fn update(&mut self) {
        let bodies_len = self.bodies.len();

        let precomputed_a = (0..bodies_len - 1).into_par_iter().map(|i_from| {
            let i_from_position = self.bodies[i_from].position;
            let i_from_mass = self.bodies[i_from].mass;
            (i_from + 1..bodies_len).into_par_iter().map(|i_to| {
                let i_to_position = self.bodies[i_to].position;
                let i_to_mass = self.bodies[i_to].mass;
                let distance = i_from_position.distance2(i_to_position);
                // println!("distance: {distance} {dx} {dy}");
                let body_to_vec = i_from_position - i_to_position;
                if distance.floor() == 1.0 {
                    return Vector2::<f32>::zero();
                };

                let a = (crate::nbody_sim::body::G * i_from_mass * i_to_mass) / distance;
                return a * body_to_vec;
            }).collect::<Vec<Vector2<f32>>>()
        }).collect::<Vec<_>>();

        for body_from_i in 0..bodies_len - 1 {
            let precomputed_a_vec = &precomputed_a[body_from_i];
            for body_other in body_from_i + 1..bodies_len {
                let a_vec = precomputed_a_vec[body_other - body_from_i - 1];
                self.bodies[body_from_i].acceleration += a_vec * -1.0;
                self.bodies[body_other].acceleration += a_vec;
            }
        }

        for body in self.bodies.iter_mut() {
            body.update(T);
        }
    }
}

fn create_spiral_cluster(num_bodies: usize, average_spacing: f32) -> Vec<Body> {
    let mut bodies = Vec::with_capacity(num_bodies);

    let mut angle: f32 = 0.0;
    let mut radius: f32 = 0.0;

    for _ in 0..num_bodies {
        let x = radius * angle.cos();
        let y = radius * angle.sin();

        let body = Body::new_sp(Vector2::new(x, y), 5.0, Vector2::new(0.0, 0.0), 100.0);
        bodies.push(body);

        angle += 20.0; // You can adjust this value for tighter or looser spirals
        radius += average_spacing;
    }

    bodies
}