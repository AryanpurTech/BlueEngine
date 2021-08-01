use crate::definitions::{Renderer};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
struct TextData {
    content: &'static str,
    position: (f32, f32),
    color: &'static [f32; 4],
    scale: f32,
}
pub struct Text {
    font: fontdue::Font,
    char_cache: BTreeMap<char, (fontdue::Metrics, Vec<u8>)>,
    size: f32,
}

impl Text {
    pub fn new(font: Vec<u8>, cache_on_size: f32) -> anyhow::Result<Self> {
        let font =
            fontdue::Font::from_bytes(font.as_slice(), fontdue::FontSettings::default()).unwrap();
        let mut char_cache = BTreeMap::<char, (fontdue::Metrics, Vec<u8>)>::new();

        let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=/\\?|<>'\"{}[],.~`";
        for i in characters.chars() {
            char_cache.insert(i, font.rasterize(i, cache_on_size)); // slap these as bmp textures
        }

        Ok(Self {
            font,
            char_cache,
            size: cache_on_size,
        })
    }

    pub fn draw(
        &mut self,
        content: &str,
        position: (u8, u8),
        renderer: &mut Renderer,
        window_size: winit::dpi::PhysicalSize<u32>,
        camera: crate::utils::camera::Camera,
    ) -> anyhow::Result<()> {
        //let mut chars = Vec::<Vertex>::new();
        for i in content.char_indices() {
            let character: (fontdue::Metrics, Vec<u8>);
            match self.char_cache.get(&i.1) {
                Some(char) => character = char.clone(),
                None => character = self.font.rasterize(i.1, self.size),
            }

            let mut character_shape = super::objects::square(Some("text"),renderer, window_size, camera)?;
            character_shape.resize(character.0.width as f32, character.0.height as f32);
        }
        Ok(())
    }
}
