/*
 * Blue Engine copyright 2021 © Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::{definitions::{}, objects};
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
        engine: &mut crate::definitions::Engine,
        camera: crate::utils::camera::Camera,
    ) -> anyhow::Result<()> {
        //let mut chars = Vec::<Vertex>::new();
        for i in content.char_indices() {
            let character: (fontdue::Metrics, Vec<u8>);
            match self.char_cache.get(&i.1) {
                Some(char) => character = char.clone(),
                None => character = self.font.rasterize(i.1, self.size),
            }

            let character_shape_index = objects::square(Some("text"), engine, camera)?;
            let character_shape = engine.get_object(character_shape_index)?;
            character_shape.resize(character.0.width as f32, character.0.height as f32, 0.0);
        }
        Ok(())
    }
}
