// Copyright 2015 The Athena Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use glium;
use glium::{DisplayBuild};
use glium::backend::glutin_backend::GlutinFacade;

/// Represents a graphics context for Jamkit to draw into.
pub struct Graphics {
    // TODO: Consider renaming
    display: GlutinFacade,
    program: glium::Program
}

impl Graphics {
    /// Initializes a new `Graphics` object with given values.
    pub fn init(title: &str, width: u32, height: u32) -> Graphics {
        let display = glium::glutin::WindowBuilder::new()
            .with_title(String::from(title))
            .with_dimensions(width, height)
            .build_glium().unwrap();

        let program = glium::Program::from_source(
            &display,
            include_str!("shaders/vertex.glsl"), include_str!("shaders/fragment.glsl"),
            None).unwrap();

        Graphics {
            display: display,
            program: program
        }
    }

    /// Returns an iterator that polls for the next event in the window's events queue.
    pub fn poll_events(&mut self) -> PollEventsIter {
        PollEventsIter {
            iter: self.display.poll_events()
        }
    }

    pub fn glium_display(&self) -> &GlutinFacade {
        &self.display
    }

    pub fn glium_program(&self) -> &glium::Program {
        &self.program
    }
}

#[derive(Debug)]
pub enum KeyState {
    Pressed,
    Released
}

impl KeyState {
    pub fn is_pressed(&self) -> bool {
        match *self {
            KeyState::Pressed => true,
            KeyState::Released => false
        }
    }

    pub fn is_released(&self) -> bool {
        !self.is_pressed()
    }
}

fn map_state(state: glium::glutin::ElementState) -> KeyState {
    match state {
        glium::glutin::ElementState::Pressed => KeyState::Pressed,
        glium::glutin::ElementState::Released => KeyState::Released
    }
}

#[derive(Debug)]
pub enum Key {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Unknown
}

fn map_key(key_o: Option<glium::glutin::VirtualKeyCode>) -> Key {
    use glium::glutin::VirtualKeyCode as Gvkc;

    if let Some(key) = key_o {
        match key {
            Gvkc::A => Key::A,
            Gvkc::B => Key::B,
            Gvkc::C => Key::C,
            Gvkc::D => Key::D,
            Gvkc::E => Key::E,
            Gvkc::F => Key::F,
            Gvkc::G => Key::G,
            Gvkc::H => Key::H,
            Gvkc::I => Key::I,
            Gvkc::J => Key::J,
            Gvkc::K => Key::K,
            Gvkc::L => Key::L,
            Gvkc::M => Key::M,
            Gvkc::N => Key::N,
            Gvkc::O => Key::O,
            Gvkc::P => Key::P,
            Gvkc::Q => Key::Q,
            Gvkc::R => Key::R,
            Gvkc::S => Key::S,
            Gvkc::T => Key::T,
            Gvkc::U => Key::U,
            Gvkc::V => Key::V,
            Gvkc::W => Key::W,
            Gvkc::X => Key::X,
            Gvkc::Y => Key::Y,
            Gvkc::Z => Key::Z,
            _ => Key::Unknown
        }
    } else {
        Key::Unknown
    }
}

#[derive(Debug)]
pub enum Event {
    Closed,
    KeyboardInput(KeyState, Key),
    Unknown
}

pub struct PollEventsIter<'a> {
    iter: glium::backend::glutin_backend::PollEventsIter<'a>
}

impl<'a> Iterator for PollEventsIter<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        use glium::glutin::Event as GliumEvent;

        if let Some(event) = self.iter.next() {
            let retev = match event {
                GliumEvent::Closed => Event::Closed,
                GliumEvent::KeyboardInput(state, _, key) => Event::KeyboardInput(map_state(state), map_key(key)),
                _ => Event::Unknown
            };

            Some(retev)
        } else {
            None
        }
    }
}
