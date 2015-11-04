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

    pub fn draw(&mut self, texture: &Texture, source: Option<[i32; 4]>, destination: [i32; 4]) {
        self.draw_many(texture, &vec![DrawData{source: source, destination: destination}]);
    }

    pub fn draw_many(&mut self, texture: &Texture, data: &[DrawData]) {
        let mut vertices = Vec::<Vertex>::new();

        for entry in data {
            // Calculate this quad's vertices
            let src = get_texcords(texture, entry.source);
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
            blend: {
                let mut blend: Blend = Default::default();
                blend.color = BlendingFunction::Addition {
                    source: LinearBlendingFactor::SourceAlpha,
                    destination: LinearBlendingFactor::OneMinusSourceAlpha
                };
                blend
            },
            .. Default::default()
        };

        self.frame.draw(
            &vertex_buffer, &indices,
            self.graphics.glium_program(),
            &uniforms, &params
        ).unwrap();
    }

    pub fn finish(self) {
        self.frame.finish().unwrap();
    }
}

pub struct DrawData {
    pub source: Option<[i32; 4]>,
    pub destination: [i32; 4]
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
