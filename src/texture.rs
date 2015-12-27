use std::path::{Path};
use glium::texture::{SrgbTexture2d, RawImage2d};
use image;
use {Graphics};

/// A texture loaded for use in drawing.
pub struct Texture {
    texture: SrgbTexture2d
}

impl Texture {
    pub fn load(graphics: &Graphics, path: &str) -> Texture {
        let image = image::open(&Path::new(path)).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let raw_image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        let texture = SrgbTexture2d::new(graphics.glium_display(), raw_image).unwrap();

        Texture {
            texture: texture,
        }
    }

    /// Gets the dimensions of the texture in pixels.
    pub fn get_dimensions(&self) -> [u32; 2] {
        [self.texture.get_width(), self.texture.get_height().unwrap()]
    }

    pub fn glium_texture(&self) -> &SrgbTexture2d {
        &self.texture
    }
}
