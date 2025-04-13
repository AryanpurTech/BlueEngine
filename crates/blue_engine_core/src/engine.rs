use winit::{event_loop::EventLoop, window::WindowAttributes};

use crate::{CameraContainer, ObjectStorage, Renderer, SignalStorage, Window};

/// Descriptor and settings for a window.
#[derive(Debug, Clone)]
pub struct EngineSettings {
    /// The width of the window
    pub width: u32,
    /// The height of the window
    pub height: u32,
    /// The title of the window
    pub title: &'static str,
    // winit
    /// Should the window contain the keys like minimize, maximize, or resize?
    pub decorations: bool,
    /// Should the window be resizable
    pub resizable: bool,
    /// Controls how the events are processed
    ///
    /// read more at [winit::event_loop::ControlFlow]
    pub control_flow: crate::winit::event_loop::ControlFlow,
    // wgpu
    /// Define how much power should the app ask for
    pub power_preference: crate::PowerPreference,
    /// The backend to use for the draw
    pub backends: crate::Backends,
    /// The features to be enabled on a backend
    ///
    /// read more at [wgpu::Features]
    pub features: crate::wgpu::Features,
    /// The presentation mode of renderer for things like VSync
    ///
    /// read more at [wgpu::PresentMode]
    pub present_mode: crate::wgpu::PresentMode,
    /// Limits to be required based on the generation of the GPU and the API.
    ///
    /// read more at [wgpu::Limits]
    pub limits: crate::wgpu::Limits,
    /// The alpha mode which specifies how the alpha channel of
    /// the textures should be handled during compositing.
    pub alpha_mode: crate::wgpu::CompositeAlphaMode,
    /// The desired frame latency.
    ///
    /// read more at [wgpu::SurfaceConfiguration::desired_maximum_frame_latency]
    pub desired_maximum_frame_latency: u32,
    /// How the memory should be utilized
    ///
    /// read more at [wgpu::MemoryHints]
    pub memory_hints: crate::wgpu::MemoryHints,
}
impl std::default::Default for EngineSettings {
    /// Will quickly create a window with default settings
    fn default() -> Self {
        let backends = crate::Backends::all();
        Self {
            width: 800,
            height: 600,
            title: "Blue Engine",
            // winit
            decorations: true,
            resizable: true,
            control_flow: crate::winit::event_loop::ControlFlow::Poll,
            // wgpu
            power_preference: crate::PowerPreference::LowPower,
            backends,
            features: if backends == wgpu::Backends::VULKAN {
                wgpu::Features::POLYGON_MODE_LINE | wgpu::Features::POLYGON_MODE_POINT
            } else if backends
                .contains(wgpu::Backends::VULKAN | wgpu::Backends::METAL | wgpu::Backends::DX12)
            {
                wgpu::Features::POLYGON_MODE_LINE
            } else {
                wgpu::Features::empty()
            },
            present_mode: crate::wgpu::PresentMode::AutoNoVsync,
            limits: crate::wgpu::Limits::default(),
            alpha_mode: crate::wgpu::CompositeAlphaMode::Auto,
            desired_maximum_frame_latency: 2,
            memory_hints: crate::MemoryHints::Performance,
        }
    }
}
unsafe impl Send for EngineSettings {}
unsafe impl Sync for EngineSettings {}

/// The engine is the main starting point of using the Blue Engine.
/// Everything that runs on Blue Engine will be under this struct.
/// The structure of engine is monolithic, but the underlying data and the way it works is not.
/// It gives a set of default data to work with,
/// but also allow you to go beyond that and work as low level as you wish to.
///
/// You can also use the Engine to build you own custom structure the way you wish for it to be.
/// Possibilities are endless!
///
/// To start using the Blue Engine, you can start by creating a new Engine like follows:
/// ```
/// use blue_engine::prelude::{Engine, EngineSettings};
///
/// fn main() {
///     let engine = Engine::new().expect("Couldn't create the engine");
/// }
/// ```
/// The EngineSettings simply holds what features you would like for your window.
/// If you are reading this on later version of
/// the engine, you might be able to even run the engine in headless mode
/// meaning there would not be a need for a window and the
/// renders would come as image files.
///
/// If you so wish to have a window, you would need to start a window update loop.
/// The update loop of window runs a frame every few millisecond,
/// and gives you details of what is happening during this time, like input events.
/// You can also modify existing parts of the engine during
/// this update loop, such as changing camera to look differently,
/// or creating a new object on the scene, or even changing window details!
///
/// The update loop is just a method of the Engine struct
/// that have one argument which is a callback function.
/// ```
///
/// ```
/// [THE DATA HERE IS WORK IN PROGRESS!]
pub struct Engine {
    /// The renderer does exactly what it is called.
    /// It works with the GPU to render frames according to the data you gave it.
    pub renderer: Renderer,
    /// The event_loop handles the events of the window and inputs.
    ///
    /// #### USED INTERNALLY
    pub event_loop_control_flow: crate::winit::event_loop::ControlFlow,
    /// The window handles everything about window and inputs.
    /// This includes ability to modify window and listen toinput devices for changes.
    ///
    /// ### The window is not available before update_loop.
    pub window: Window,
    /// The object system is a way to make it easier to work with the engine.
    /// Obviously you can work without it, but it's for those who
    /// do not have the know-how, or wish to handle all the work of rendering data manually.
    pub objects: ObjectStorage,
    /// The camera handles the way the scene looks when rendered.
    /// You can modify everything there is to camera through this.
    pub camera: CameraContainer,
    /// Handles all engine plugins
    pub signals: SignalStorage,

    /// holds the update_loop function
    ///
    /// #### USED INTERNALLY
    pub update_loop: Option<
        Box<
            dyn 'static
                + FnMut(
                    // Core
                    &mut Engine,
                ),
        >,
    >,

    /// input events
    ///
    /// #### USED INTERNALLY
    pub simple_input: crate::utils::winit_input_helper::WinitInputHelper,
}
unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}

impl Engine {
    /// Creates a new window in current thread using default settings.
    pub fn new() -> Result<Self, crate::error::Error> {
        Self::new_inner(
            EngineSettings::default(),
            #[cfg(target_os = "android")]
            None,
        )
    }

    /// Creates a new window in current thread using provided settings.
    pub fn new_config(settings: EngineSettings) -> Result<Self, crate::error::Error> {
        Self::new_inner(
            settings,
            #[cfg(target_os = "android")]
            None,
        )
    }

    /// Creates a new window for android
    #[cfg(target_os = "android")]
    pub fn new_android(
        settings: EngineSettings,
        app: winit::platform::android::activity::AndroidApp,
    ) -> Result<Self, crate::error::Error> {
        Self::new_inner(settings, Some(app))
    }

    /// Creates a new window in current thread.
    #[allow(unreachable_code)]
    pub(crate) fn new_inner(
        settings: EngineSettings,
        #[cfg(target_os = "android")] android_app: Option<
            winit::platform::android::activity::AndroidApp,
        >,
    ) -> Result<Self, crate::error::Error> {
        #[cfg(feature = "debug")]
        env_logger::init();
        // Dimensions of the window, as width and height
        // and then are set as a logical size that the window can accept
        #[cfg(not(target_os = "android"))]
        let dimension = winit::dpi::PhysicalSize {
            width: settings.width,   // Which sets the width of the window
            height: settings.height, // And sets the height of the window
        };

        // And we will create a new window and set all the options we stored
        #[cfg(not(target_os = "android"))]
        let default_attributes = WindowAttributes::default()
            .with_inner_size(dimension) // sets the width and height of window
            .with_title(String::from(settings.title)) // sets title of the window
            .with_decorations(settings.decorations) // sets if the window should have borders
            .with_resizable(settings.resizable); // sets the window to be resizable

        // The renderer init on current window
        let mut renderer = pollster::block_on(Renderer::new(dimension, settings.clone()))?;
        let camera = CameraContainer::new(dimension, &mut renderer);

        Ok(Self {
            window: Window::new(default_attributes),
            event_loop_control_flow: settings.control_flow,
            renderer,
            objects: ObjectStorage::new(),
            camera,
            signals: crate::SignalStorage::new(),
            update_loop: None,
            simple_input: crate::utils::winit_input_helper::WinitInputHelper::new(),
        })
    }

    /// Runs the block of code that you pass to it every frame. The update code is used
    /// to modify the engine on the fly thus creating interactive graphics and making things
    /// happy in the engine!
    ///
    /// Renderer, window, vec of objects, events, and camera are passed to the update code.
    #[allow(unreachable_code)]
    pub fn update_loop(
        &mut self,
        update_function: impl 'static
        + FnMut(
            // Coreall(target_os = "android", not(feature = "headless"))
            &mut Engine,
        ),
    ) -> Result<(), crate::error::Error> {
        self.update_loop = Some(Box::new(update_function));

        // will create the main event loop of the window.
        // and will contain all the callbacks and button press
        // also will allow graphics API
        #[cfg(all(target_os = "android", not(feature = "headless")))]
        let event_loop = if android_app.is_some() {
            use winit::platform::android::EventLoopBuilderExtAndroid;

            android_logger::init_once(
                android_logger::Config::default()
                    .with_max_level(log::LevelFilter::Trace) // Default comes from `log::max_level`, i.e. Off
                    .with_filter(
                        android_logger::FilterBuilder::new()
                            .filter_level(log::LevelFilter::Debug)
                            .filter_module("android_activity", log::LevelFilter::Trace)
                            .filter_module("winit", log::LevelFilter::Trace)
                            .build(),
                    ),
            );

            winit::event_loop::EventLoopBuilder::new()
                .with_android_app(if let Some(android_app) = android_app {
                    android_app
                } else {
                    panic!("No android app")
                })
                .build()?
        } else {
            EventLoop::new()?
        };

        #[cfg(not(target_os = "android"))]
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(self.event_loop_control_flow);
        event_loop.run_app(self)?;

        Ok(())
    }
}
