/// Allows all events to be fetched directly, making it easier to add custom additions to the engine.
pub trait EnginePlugin {
    fn update_events(
        &mut self,
        _renderer: &mut crate::Renderer,
        _window: &crate::Window,
        _objects: &mut std::collections::HashMap<&'static str, crate::Object>,
        _events: &crate::Event<()>,
        _input: &crate::InputHelper,
        _camera: &mut crate::Camera,
    );

    fn update(
        &mut self,
        _renderer: &mut crate::Renderer,
        _window: &crate::Window,
        _objects: &mut std::collections::HashMap<&'static str, crate::Object>,
        _camera: &mut crate::Camera,
        _input: &crate::InputHelper,
        _plugin_data_storage: &mut std::collections::HashMap<&'static str, Box<dyn std::any::Any>>,
        _encoder: &mut crate::CommandEncoder,
        _view: &crate::TextureView,
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotateAxis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone)]
pub enum TextureData {
    Bytes(Vec<u8>),
    Image(image::DynamicImage),
    Path(&'static str),
}

/// Defines how the borders of texture would look like
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureMode {
    /// Expands the texture to fit the object
    Clamp,
    /// Repeats the texture instead of stretching
    Repeat,
    /// Repeats the texture, but mirrors it on edges
    MirrorRepeat,
}

/// Defines file format of the texture to load
pub enum TextureFormat {
    PNG,
    BMP,
    JPEG,
    PNM,
}

/// This function helps in converting pixel value to the value that is between -1 and +1
pub fn normalize(value: f32, max: u32) -> f32 {
    let mut result = value / max as f32;

    if value == max as f32 {
        result = 0.0;
    } else if result < max as f32 / 2.0 {
    }

    if result > -1.0 {
        return result as f32;
    } else {
        return -1.0;
    }
}

/// Returns
pub fn percentage(amount: f32, of: f32) -> f32 {
    let result = amount / of;

    return result;
}
