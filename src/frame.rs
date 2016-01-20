use cgmath;
use glium;
use glium::{Surface, Blend};
use glium::draw_parameters::{BlendingFunction, LinearBlendingFactor};
use {Graphics, Texture};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

/// A single frame to be displayed on-screen. Contains functions for drawing to that frame.
pub struct Frame<'a> {
    graphics: &'a Graphics,
    viewport_matrix: [[f32; 4]; 4],
    frame: glium::Frame
}

impl<'a> Frame<'a> {
    /// Starts a new frame for the jamkit target.
    pub fn start(graphics: &Graphics) -> Frame {
        // Clear our frame so we don't have lingering data
        let mut frame = graphics.glium_display().draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);

        // Calculate this frame's viewport
        let (w, h) = frame.get_dimensions();
        let matrix = cgmath::ortho::<f32>(
            0.0, w as f32,
            h as f32, 0.0,
            1.0, -1.0).into();

        Frame {
            graphics: graphics,
            frame: frame,
            viewport_matrix: matrix
        }
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        self.frame.get_dimensions()
    }

    /// Draws the source rectangle in the texture on the frame as the destination rectangle.
    /// If None is provided for source, the full texture is drawn.
    pub fn draw(&mut self, texture: &Texture, source: Option<[i32; 4]>, destination: [i32; 4]) {
        self.draw_many(texture, &[DrawData{source: source, destination: destination}]);
    }

    /// Performs the same action as `draw`, but many in a batched call.
    pub fn draw_many(&mut self, texture: &Texture, data: &[DrawData]) {
        let mut vertices: Vec<Vertex> = Vec::with_capacity(data.len() * 6);

        for entry in data {
            // Calculate this quad's vertices
            let src = get_texcoords(texture, entry.source);
            let dest = [
                entry.destination[0] as f32, entry.destination[1] as f32,
                entry.destination[2] as f32, entry.destination[3] as f32];

            vertices.push(Vertex {position: [dest[0], dest[1]], tex_coords: [src[0], src[3]]});
            vertices.push(Vertex {position: [dest[0], dest[3]], tex_coords: [src[0], src[1]]});
            vertices.push(Vertex {position: [dest[2], dest[1]], tex_coords: [src[2], src[3]]});

            vertices.push(Vertex {position: [dest[2], dest[3]], tex_coords: [src[2], src[1]]});
            vertices.push(Vertex {position: [dest[2], dest[1]], tex_coords: [src[2], src[3]]});
            vertices.push(Vertex {position: [dest[0], dest[3]], tex_coords: [src[0], src[1]]});
        }

        let vertex_buffer = glium::VertexBuffer::dynamic(self.graphics.glium_display(), &vertices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let uniforms = uniform! {
            matrix: self.viewport_matrix.clone(),
            tex: texture.glium_texture().sampled()
                .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        };

        let params = glium::DrawParameters {
            blend: Blend {
                color: BlendingFunction::Addition {
                    source: LinearBlendingFactor::SourceAlpha,
                    destination: LinearBlendingFactor::OneMinusSourceAlpha
                },
                .. Default::default()
            },
            .. Default::default()
        };

        self.frame.draw(
            &vertex_buffer, &indices,
            self.graphics.glium_program(),
            &uniforms, &params
        ).unwrap();
    }

    /// Finishes this frame, causing the back-buffer to be flipped. This may block for vsync.
    pub fn finish(self) {
        self.frame.finish().unwrap();
    }
}

/// A source-destination combination for use in `draw_many`.
pub struct DrawData {
    pub source: Option<[i32; 4]>,
    pub destination: [i32; 4]
}

fn get_texcoords(texture: &Texture, src: Option<[i32; 4]>) -> [f32; 4] {
    if let Some(val) = src {
        let size = texture.get_dimensions();

        [val[0] as f32 / size[0] as f32, val[1] as f32 / size[1] as f32,
         val[2] as f32 / size[0] as f32, val[3] as f32 / size[1] as f32]
    } else {
        [0.0, 0.0, 1.0, 1.0]
    }
}
