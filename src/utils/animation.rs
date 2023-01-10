use crate::{AnimationKeyframe, ObjectStorage};
use keyframe::{functions::Linear, AnimationSequence, Keyframe, AnimationSequenceError};

#[derive(Clone)]
pub struct Animation {
    pub keyframes: Vec<(f64, AnimationKeyframe)>,
    pub animation_sequence: AnimationSequence<AnimationKeyframe>,
    pub time: std::time::Instant,
    pub object: &'static str,
}

impl Animation {
    pub fn new(object: &'static str) -> Self {
        Self {
            keyframes: Vec::new(),
            time: std::time::Instant::now(),
            animation_sequence: AnimationSequence::new(),
            object,
        }
    }

    pub fn start(&mut self) -> Result<(), AnimationSequenceError> {
        for i in self.keyframes.iter() {
            self.animation_sequence
                .insert(Keyframe::new(i.1, i.0, Linear))?;
        }

        Ok(())
    }

    pub fn animate(&mut self, objects: &mut ObjectStorage) {
        let elapsed = self.time.elapsed().as_secs_f64();
        let obj = objects.get_mut(self.object).unwrap();
        self.animation_sequence.advance_to(elapsed);
        let frame_data = self.animation_sequence.now();

        if !self.animation_sequence.finished() {
            obj.position(
                frame_data.position.x,
                frame_data.position.y,
                frame_data.position.z,
            );
            
            obj.rotate(frame_data.rotation.x - obj.rotation.0, crate::RotateAxis::X);
            obj.rotate(frame_data.rotation.y - obj.rotation.1, crate::RotateAxis::Y);
            obj.rotate(frame_data.rotation.z - obj.rotation.2, crate::RotateAxis::Z);
        }
    }
}
