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

use cgmath;
use cgmath::{FixedArray};
use glium;
use glium::{Surface};
use {Graphics, Texture};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

pub struct Frame<'a> {
    graphics: &'a Graphics,
    viewport_matrix: [[f32; 4]; 4],
    frame: glium::Frame
}

impl<'a> Frame<'a> {
    pub fn start(graphics: &Graphics) -> Frame {
        // Clear our frame so we don't have lingering data
        let mut frame = graphics.glium_display().draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);

        // Calculate this frame's viewport
        let (w, h) = frame.get_dimensions();
        let matrix = cgmath::ortho::<f32>(
            0.0, w as f32,
            h as f32, 0.0,
            1.0, -1.0).into_fixed();

        Frame {
            graphics: graphics,
            frame: frame,
            viewport_matrix: matrix
        }
    }

    pub fn draw(&mut self, texture: &Texture, src: Option<[i32; 4]>, destination: [i32; 4]) {
        // Calculate this quad's vertices
        let src = get_texcords(texture, src);
        let dest = [destination[0] as f32, destination[1] as f32, destination[2] as f32, destination[3] as f32];
        let shape = vec![
            Vertex {position: [dest[0], dest[1]], tex_coords: [src[0], src[3]]},
            Vertex {position: [dest[0], dest[3]], tex_coords: [src[0], src[1]]},
            Vertex {position: [dest[2], dest[1]], tex_coords: [src[2], src[3]]},

            Vertex {position: [dest[2], dest[3]], tex_coords: [src[2], src[1]]},
            Vertex {position: [dest[2], dest[1]], tex_coords: [src[2], src[3]]},
            Vertex {position: [dest[0], dest[3]], tex_coords: [src[1], src[1]]}
        ];

        let vertex_buffer = glium::VertexBuffer::new(self.graphics.glium_display(), shape);
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let uniforms = uniform! {
            matrix: self.viewport_matrix.clone(),
            tex: texture.glium_texture(),
        };

        self.frame.draw(
            &vertex_buffer, &indices,
            self.graphics.glium_program(),
            &uniforms, &Default::default()).unwrap();
    }

    pub fn finish(self) {
        self.frame.finish().unwrap();
    }
}

fn get_texcords(texture: &Texture, src: Option<[i32; 4]>) -> [f32; 4] {
    match src {
        Some(val) => {
            let (w, h) = texture.get_dimensions();

            [val[0] as f32 / w as f32, val[1] as f32 / h as f32,
             val[2] as f32 / w as f32, val[3] as f32 / h as f32]
        },
        None => [0.0, 0.0, 1.0, 1.0]
    }
}
