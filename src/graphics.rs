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

pub struct JamkitGraphics {
    display: GlutinFacade,
    program: glium::Program,
    is_closed: bool
}

impl JamkitGraphics {
    pub fn init() -> JamkitGraphics {
        let display = glium::glutin::WindowBuilder::new()
            .build_glium().unwrap();

        let program = glium::Program::from_source(
            &display,
            include_str!("shaders/vertex.glsl"), include_str!("shaders/fragment.glsl"),
            None).unwrap();

        JamkitGraphics {
            display: display,
            program: program,
            is_closed: false
        }
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    pub fn poll_events(&mut self) {
        for ev in self.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => self.is_closed = true,
                _ => ()
            }
        }
    }

    pub fn glium_display(&self) -> &GlutinFacade {
        &self.display
    }

    pub fn glium_program(&self) -> &glium::Program {
        &self.program
    }
}
