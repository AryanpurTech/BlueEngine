/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::{
    header::{normalize, ObjectSettings, Renderer, TextureData},
    objects,
};
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
    char_cache: BTreeMap<char, (fontdue::Metrics, usize)>,
    size: f32,
}

impl Text {
    pub fn new(font: &[u8], cache_on_size: f32, renderer: &mut Renderer) -> anyhow::Result<Self> {
        let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
        let mut char_cache = BTreeMap::<char, (fontdue::Metrics, usize)>::new();

        let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=/\\?|<>'\"{}[],.~`";
        for i in characters.chars() {
            let character = font.rasterize(i, cache_on_size);

            let mut char_image =
                image::RgbaImage::new(character.0.width as u32, character.0.height as u32);

            let mut char_length: usize = 0;
            for pixel in char_image.pixels_mut() {
                //let pixel_value = percentage(character.1[char_length] as f32, 255f32);]
                let pixel_value = character.1[char_length];
                pixel.0 = [pixel_value, pixel_value, pixel_value, 0];

                char_length += 1;
            }

            let index = renderer.build_and_append_texture(
                "Character Cache",
                TextureData::Image(image::DynamicImage::ImageRgba8(char_image)),
                crate::header::TextureMode::Clamp,
                //crate::header::TextureFormat::PNM,
            )?;
            char_cache.insert(i, (character.0, index)); // slap these as bmp textures
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
        position: (isize, isize),
        engine: &mut crate::header::Engine,
    ) -> anyhow::Result<()> {
        let mut characters = Vec::<(fontdue::Metrics, usize)>::new();
        for i in content.char_indices() {
            let character: (fontdue::Metrics, usize);
            match self.char_cache.get(&i.1) {
                Some(char) => character = char.clone(),
                None => {
                    character = {
                        let character = self.font.rasterize(i.1, self.size);
                        let mut char_image = image::RgbaImage::new(
                            character.0.width as u32,
                            character.0.height as u32,
                        );

                        let mut char_length: usize = 0;
                        for pixel in char_image.pixels_mut() {
                            //let pixel_value = percentage(character.1[char_length] as f32, 255f32);]
                            let pixel_value = character.1[char_length];
                            pixel.0 = [pixel_value, pixel_value, pixel_value, 0];

                            char_length += 1;
                        }
                        let index = engine.renderer.build_and_append_texture(
                            "CharTex",
                            TextureData::Image(image::DynamicImage::ImageRgba8(char_image)),
                            crate::header::TextureMode::Clamp,
                            //crate::header::TextureFormat::BMP,
                        )?;
                        (character.0, index)
                    }
                }
            }

            characters.push(character);
        }

        let mut current_character_position_x = position.0;
        let mut average_height = 0;
        for i in characters.iter() {
            average_height += i.0.height;
        }
        average_height /= characters.len();

        for character in characters.iter() {
            let window_size = engine.window.inner_size();
            let character_shape_index = objects::two_dimensions::square(
                ObjectSettings {
                    name: Some("text"),
                    size: (character.0.width as f32, character.0.height as f32, 0f32),
                    texture_index: character.1,
                    position: (position.0 as f32, position.1 as f32, 0.0),
                    camera_effect: false,
                    ..Default::default()
                },
                engine,
            )?;

            let character_shape = engine.get_object(character_shape_index).unwrap();
            current_character_position_x += character.0.width as isize + 20;

            character_shape.translate(
                normalize((current_character_position_x) as f32, window_size.width),
                normalize(
                    (average_height as f32 + character.0.height as f32),
                    window_size.height,
                ),
                0.0,
            );
        }
        Ok(())
    }
}
