use std::path::{Path};
use glium::texture::{Texture2d};
use image;
use {Graphics};

pub struct Texture {
    texture: Texture2d
}

impl Texture {
    pub fn load(graphics: &Graphics, path: &str) -> Texture {
        let image = image::open(&Path::new(path)).unwrap();
        let texture = Texture2d::new(graphics.glium_display(), image).unwrap();

        Texture {
            texture: texture,
        }
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.texture.get_width(), self.texture.get_height().unwrap())
    }

    pub fn glium_texture(&self) -> &Texture2d {
        &self.texture
    }
}
