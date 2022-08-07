use crate::uniform_type::Array;

impl crate::LightManager {
    pub fn new() -> Self {
        Self {
            ambient_color: Array {
                data: [1f32, 1f32, 1f32, 1f32],
            },
        }
    }

    pub fn update(&self, objects: &mut Vec<crate::Object>) -> anyhow::Result<()> {
        for i in objects {
            let result = i.main_color * self.ambient_color;
            i.set_color(
                result.data[0],
                result.data[1],
                result.data[2],
                result.data[3],
            )?;
        }

        Ok(())
    }
}
