use crate::ObjectStorage;

#[derive(Debug, Clone, Copy)]
pub struct Operation {
    pub translation: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub keyframes: Vec<(u64, Operation)>,
    pub time: std::time::Instant,
    pub target: u128,
    pub difference_translate: (f32, f32, f32),
    pub progressed_translation: (f32, f32, f32),
    pub difference_rotation: (f32, f32, f32),
    pub progressed_rotation: (f32, f32, f32),
    pub current_frame: usize,
    pub object: &'static str,
}

impl Animation {
    pub fn new(object: &'static str) -> Self {
        Self {
            keyframes: Vec::new(),
            time: std::time::Instant::now(),
            target: 0,
            difference_translate: (0f32, 0f32, 0f32),
            progressed_translation: (0f32, 0f32, 0f32),
            difference_rotation: (0f32, 0f32, 0f32),
            progressed_rotation: (0f32, 0f32, 0f32),
            current_frame: 0,
            object,
        }
    }

    pub fn animate(
        &mut self,
        objects: &mut ObjectStorage,
    ) {
        let elapsed = self.time.elapsed().as_millis();
        if elapsed <= self.target {
            let obj = objects.get_mut(self.object).unwrap();
            let x_rotation = obj.rotation.0;
            let y_rotation = obj.rotation.1;
            let z_rotation = obj.rotation.2;
            
            obj.rotate(x_rotation * -1f32, crate::RotateAxis::X);
            obj.rotate(y_rotation * -1f32, crate::RotateAxis::Y);
            obj.rotate(z_rotation * -1f32, crate::RotateAxis::Z);
            obj.position(
                self.progressed_translation.0 + (self.difference_translate.0 * elapsed as f32),
                self.progressed_translation.1 + (self.difference_translate.1 * elapsed as f32),
                self.progressed_translation.2 + (self.difference_translate.2 * elapsed as f32),
            );
            obj.rotate(((self.difference_rotation.0 *  elapsed as f32) + x_rotation).to_radians(),  crate::RotateAxis::X);
            //obj.rotate(self.difference_rotation.0 + x_rotation, crate::RotateAxis::X);
            //obj.rotate(self.difference_rotation.0 + x_rotation, crate::RotateAxis::X);
        } else {
            if self.current_frame != 0 {
                let target_translation = self.keyframes[self.current_frame - 1].1.translation;
                let target_rotation = self.keyframes[self.current_frame - 1].1.rotation;
                let obj = objects.get_mut(self.object).unwrap();
                
                obj.rotate(obj.rotation.0 * -1f32, crate::RotateAxis::X);
                obj.rotate(obj.rotation.1 * -1f32, crate::RotateAxis::Y);
                obj.rotate(obj.rotation.2 * -1f32, crate::RotateAxis::Z);

                obj.position(
                    target_translation.0 + self.progressed_translation.0,
                    target_translation.1 + self.progressed_translation.1,
                    target_translation.2 + self.progressed_translation.2,
                );
                
                obj.rotate((target_translation.0 + self.progressed_rotation.0).to_radians(), crate::RotateAxis::X);
                obj.rotate((target_translation.1 + self.progressed_rotation.1).to_radians(), crate::RotateAxis::Y);
                obj.rotate((target_translation.2 + self.progressed_rotation.2).to_radians(), crate::RotateAxis::Z);

                if self.current_frame < self.keyframes.len() {
                    self.progressed_translation = (
                        target_translation.0 + self.progressed_translation.0,
                        target_translation.1 + self.progressed_translation.1,
                        target_translation.2 + self.progressed_translation.2,
                    );

                    self.progressed_rotation = (
                        target_rotation.0 + self.progressed_rotation.0,
                        target_rotation.1 + self.progressed_rotation.1,
                        target_rotation.2 + self.progressed_rotation.2,
                    );
                }
            }

            if self.current_frame < self.keyframes.len() {
                let next_frame = self.keyframes[self.current_frame];
                self.target = std::time::Duration::from_secs(next_frame.0).as_millis();
                self.difference_translate = (
                    (next_frame.1.translation.0 - self.difference_translate.0) / self.target as f32,
                    (next_frame.1.translation.1 - self.difference_translate.1) / self.target as f32,
                    (next_frame.1.translation.2 - self.difference_translate.2) / self.target as f32,
                );
                self.difference_rotation = (
                    (next_frame.1.rotation.0 - self.difference_rotation.0) / self.target as f32,
                    (next_frame.1.rotation.1 - self.difference_rotation.1) / self.target as f32,
                    (next_frame.1.rotation.2 - self.difference_rotation.2) / self.target as f32,
                );
                self.time = std::time::Instant::now();
                self.current_frame += 1;
            }
        }
    }
}
