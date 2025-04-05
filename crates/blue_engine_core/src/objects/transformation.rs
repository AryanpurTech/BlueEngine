use crate::{
    Matrix4, PipelineData, Quaternion, Renderer, StringBuffer, TextureData, TextureMode, Textures,
    Vector3, Vector4,
};

use super::Object;

/// Defines how the rotation axis is
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotateAxis {
    #[doc(hidden)]
    X,
    #[doc(hidden)]
    Y,
    #[doc(hidden)]
    Z,
}
unsafe impl Send for RotateAxis {}
unsafe impl Sync for RotateAxis {}

/// Defines how the rotation amount is
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RotateAmount {
    #[doc(hidden)]
    Radians(f32),
    #[doc(hidden)]
    Degrees(f32),
}
unsafe impl Send for RotateAmount {}
unsafe impl Sync for RotateAmount {}

impl Object {
    /// Sets the name of the object
    pub fn set_name(&mut self, name: impl StringBuffer) -> &mut Self {
        self.name = name.as_arc();

        self
    }

    /// Scales an object. e.g. 2.0 doubles the size and 0.5 halves
    pub fn set_scale(&mut self, scale: impl Into<Vector3>) -> &mut Self {
        let scale = scale.into();
        self.size *= scale;

        let transformation_matrix = self.scale_matrix;
        let result = transformation_matrix * Matrix4::from_scale(scale);
        self.scale_matrix = result;
        self.inverse_matrices();

        self.changed = true;
        self
    }

    /// Resizes an object in pixels which are relative to the window
    pub fn resize(&mut self, size: impl Into<Vector3>) -> &mut Self {
        let size = size.into();
        self.size = size;
        self.scale_matrix = Matrix4::IDENTITY;

        self.set_scale(size)
    }

    /// Sets the rotation of the object in the axis you specify
    ///
    /// This function does NOT normalize the rotation.
    pub fn set_rotation(&mut self, rotation: impl Into<Vector3>) -> &mut Self {
        let rotation = rotation.into();
        self.rotation = rotation;
        self.rotation_quaternion = Quaternion::from_rotation_x(rotation.x)
            * Quaternion::from_rotation_y(rotation.y)
            * Quaternion::from_rotation_z(rotation.z);
        self.inverse_matrices();

        self.changed = true;
        self
    }

    /// Rotates the object in the axis you specify
    pub fn rotate(&mut self, amount: RotateAmount, axis: RotateAxis) -> &mut Self {
        let amount_radians = match amount {
            RotateAmount::Radians(amount) => amount,
            RotateAmount::Degrees(amount) => amount.to_radians(),
        };

        let axis = match axis {
            RotateAxis::X => {
                self.rotation.x += amount_radians;
                Quaternion::from_rotation_x(amount_radians)
            }
            RotateAxis::Y => {
                self.rotation.y += amount_radians;
                Quaternion::from_rotation_y(amount_radians)
            }
            RotateAxis::Z => {
                self.rotation.z += amount_radians;
                Quaternion::from_rotation_z(amount_radians)
            }
        };

        self.rotation_quaternion *= axis;
        self.inverse_matrices();

        self.changed = true;
        self
    }

    /// Moves the object by the amount you specify in the axis you specify
    #[deprecated]
    pub fn set_translation(&mut self, new_pos: impl Into<Vector3>) -> &mut Self {
        self.position -= new_pos.into();
        self.translation_matrix *= Matrix4::from_translation(self.position);

        self.inverse_matrices();
        self.changed = true;
        self
    }

    /// Moves the object by the amount you specify in the axis you specify
    pub fn translate(&mut self, new_pos: impl Into<Vector3>) -> &mut Self {
        self.position -= new_pos.into();
        self.translation_matrix *= Matrix4::from_translation(self.position);

        self.inverse_matrices();
        self.changed = true;
        self
    }
    /// Moves the object by the amount you specify in the axis you specify

    /// Sets the position of the object in 3D space relative to the window
    pub fn set_position(&mut self, new_pos: impl Into<Vector3>) -> &mut Self {
        let new_pos = new_pos.into();
        self.position = new_pos;
        self.translation_matrix = Matrix4::IDENTITY;

        self.translate(new_pos)
    }

    /// Changes the color of the object. If textures exist, the color of textures will change
    pub fn set_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) -> &mut Self {
        self.color = Vector4::new(red, green, blue, alpha);
        self.changed = true;
        self
    }

    /// Changes the render order of the Object.
    ///
    /// Objects with higher number get rendered later and appear "on top" when occupying the same space
    pub fn set_render_order(&mut self, render_order: usize) -> &mut Self {
        self.render_order = render_order;

        self
    }

    /// Replaces the object's texture with provided one
    ///
    /// This function previously served the role of [crate::Object::set_texture_raw]
    pub fn set_texture(
        &mut self,
        name: impl StringBuffer,
        texture_data: TextureData,
        texture_mode: TextureMode,
        renderer: &mut Renderer,
    ) -> Result<&mut Self, crate::error::Error> {
        let texture = renderer.build_texture(name, texture_data, texture_mode)?;
        Ok(self.set_texture_raw(texture))
    }

    /// Replaces the object's texture with provided one
    pub fn set_texture_raw(&mut self, texture: Textures) -> &mut Self {
        self.pipeline.texture = PipelineData::Data(texture);
        self.changed = true;

        self
    }

    /// This will flag object as changed and altered, leading to rebuilding parts, or entirety on next frame.
    /// Best used if you directly altered fields of the object. The functions normally flag the object as
    /// changed on every call anyways. But this function is to manually flag it yourself.
    pub fn flag_as_changed(&mut self, is_changed: bool) {
        self.changed = is_changed;
    }

    /// Sets if the object will be rendered or not
    pub fn set_visibility(&mut self, is_visible: bool) {
        self.is_visible = is_visible;
    }

    /// build an inverse of the transformation matrix to be sent to the gpu for lighting and other things.
    pub fn inverse_matrices(&mut self) {
        self.inverse_transformation_matrix = Matrix4::transpose(&Matrix4::inverse(
            &(self.translation_matrix
                * Matrix4::from_quat(self.rotation_quaternion)
                * self.scale_matrix),
        ));
    }
}
