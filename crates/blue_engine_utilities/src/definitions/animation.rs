#![cfg(feature = "animation")]

use crate::AnimationKeyframe;
use blue_engine::ObjectStorage;
use keyframe::{AnimationSequence, AnimationSequenceError, Keyframe, functions::Linear};
use std::sync::Arc;

#[derive(Clone)]
pub struct Animation {
    pub keyframes: Vec<(f64, AnimationKeyframe)>,
    pub animation_sequence: AnimationSequence<AnimationKeyframe>,
    pub time: std::time::Instant,
    pub object: Arc<str>,
}

impl Animation {
    pub fn new(object: impl AsRef<str>) -> Self {
        Self {
            keyframes: Vec::new(),
            time: std::time::Instant::now(),
            animation_sequence: AnimationSequence::new(),
            object: object.as_ref().into(),
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
        let obj = objects.get_mut(&self.object).unwrap();
        self.animation_sequence.advance_to(elapsed);
        let frame_data = self.animation_sequence.now();

        if !self.animation_sequence.finished() {
            obj.set_position([
                frame_data.position.x,
                frame_data.position.y,
                frame_data.position.z,
            ]);

            obj.set_rotation([frame_data.rotation.x - obj.rotation.x, 0f32, 0f32]);
            obj.set_rotation([0f32, frame_data.rotation.y - obj.rotation.y, 0f32]);
            obj.set_rotation([0f32, 0f32, frame_data.rotation.z - obj.rotation.z]);

            obj.resize([frame_data.size.x, frame_data.size.y, frame_data.size.z]);
        }
    }
}
