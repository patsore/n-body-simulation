use std::sync::{Arc, Mutex};
use std::time::Instant;
use image::GenericImageView;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::keyboard::{Key, NamedKey};
use wgpu_test::State;
use wgpu_test::Simulation;
#[tokio::main]
async fn main() {
    env_logger::init();
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let window = winit::window::WindowBuilder::default().build(&event_loop).unwrap();
    let out_window_id = window.id();
    let mut state =  Arc::new(Mutex::new(State::new(Arc::new(window)).await));

    let mut simulation = Simulation::new(state.clone());

    event_loop.run(|event, elfw| {
        match event {
            Event::WindowEvent { window_id, event }
            if window_id == out_window_id
            => {
                match event {
                    WindowEvent::Resized(size) => {
                        if size.width != 0 && size.height != 0 {
                            state.lock().unwrap().resize(size);
                        }
                    }
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                        event: KeyEvent {
                            logical_key: Key::Named(NamedKey::Escape),
                            ..
                        },
                        ..
                    } => {
                        elfw.exit()
                    }
                    WindowEvent::ScaleFactorChanged { .. } => {}
                    WindowEvent::RedrawRequested => {
                        state.lock().unwrap().window.request_redraw();
                        simulation.update();
                    }
                    _ => {}
                }
                state.lock().unwrap().update();
                state.lock().unwrap().render();
            }
            _ => {}
        }
    }).unwrap();
}