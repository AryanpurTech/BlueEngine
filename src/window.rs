/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::{
    header::{Engine, Renderer, WindowDescriptor},
    CameraContainer, ObjectStorage, Window,
};

use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
    event_loop::EventLoop,
    window::WindowAttributes,
};

impl Engine {
    /// Creates a new window in current thread using default settings.
    pub fn new() -> eyre::Result<Self> {
        Self::new_inner(
            WindowDescriptor::default(),
            #[cfg(target_os = "android")]
            None,
        )
    }

    /// Creates a new window in current thread using provided settings.
    pub fn new_config(settings: WindowDescriptor) -> eyre::Result<Self> {
        Self::new_inner(
            settings,
            #[cfg(target_os = "android")]
            None,
        )
    }

    /// Creates a new window for android
    #[cfg(target_os = "android")]
    pub fn new_android(
        settings: WindowDescriptor,
        app: winit::platform::android::activity::AndroidApp,
    ) -> eyre::Result<Self> {
        Self::new_inner(settings, Some(app))
    }

    /// Creates a new window in current thread.
    #[allow(unreachable_code)]
    pub(crate) fn new_inner(
        settings: WindowDescriptor,
        #[cfg(target_os = "android")] android_app: Option<
            winit::platform::android::activity::AndroidApp,
        >,
    ) -> eyre::Result<Self> {
        #[cfg(feature = "debug")]
        env_logger::init();
        // Dimensions of the window, as width and height
        // and then are set as a logical size that the window can accept
        #[cfg(not(target_os = "android"))]
        let dimension = winit::dpi::PhysicalSize {
            width: settings.width,   // Which sets the width of the window
            height: settings.height, // And sets the height of the window
        };

        // Here the size is finally made according to the dimensions we set earlier
        #[cfg(not(target_os = "android"))]
        let size = winit::dpi::Size::Physical(dimension);

        // And we will create a new window and set all the options we stored
        #[cfg(not(target_os = "android"))]
        let default_attributes = WindowAttributes::default()
            .with_inner_size(size) // sets the width and height of window
            .with_title(String::from(settings.title)) // sets title of the window
            .with_decorations(settings.decorations) // sets if the window should have borders
            .with_resizable(settings.resizable); // sets the window to be resizable

        // will create the main event loop of the window.
        // and will contain all the callbacks and button press
        // also will allow graphics API
        #[cfg(target_os = "android")]
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
                .with_android_app(if android_app.is_some() {
                    android_app.unwrap()
                } else {
                    panic!("No android app")
                })
                .build()?
        } else {
            EventLoop::new()?
        };

        let window_size = winit::dpi::PhysicalSize::new(settings.width, settings.height);

        // The renderer init on current window
        let mut renderer = pollster::block_on(Renderer::new(window_size, settings.clone()))?;
        let camera = CameraContainer::new(window_size, &mut renderer)?;

        Ok(Self {
            window: Window::new(default_attributes),
            event_loop_control_flow: settings.control_flow,
            renderer,
            objects: ObjectStorage::new(),
            camera,
            signals: crate::SignalStorage::new(),
            update_loop: None,
            input_events: crate::utils::winit_input_helper::WinitInputHelper::new(),
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
                // Core
                &mut Renderer,
                &mut Window,
                &mut ObjectStorage,
                &crate::utils::winit_input_helper::WinitInputHelper,
                &mut CameraContainer,
                &mut crate::SignalStorage,
            ),
    ) -> eyre::Result<()> {
        self.update_loop = Some(Box::new(update_function));

        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(self.event_loop_control_flow);
        event_loop.run_app(self)?;

        Ok(())
    }
}

impl ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            self.window.window = Some(std::sync::Arc::new(
                event_loop
                    .create_window(self.window.default_attributes.clone())
                    .unwrap(),
            ));

            if self.renderer.surface.is_none() {
                let surface = self
                    .renderer
                    .instance
                    .create_surface(self.window.window.as_ref().unwrap().clone())
                    .unwrap();
                surface.configure(&self.renderer.device, &self.renderer.config);

                self.renderer.depth_buffer = Renderer::build_depth_buffer(
                    "Depth Buffer",
                    &self.renderer.device,
                    &self.renderer.config,
                );
                self.renderer.surface = Some(surface);
            }
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        self.input_events.process_device_event(&event);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Self {
            ref mut camera,
            ref mut renderer,
            ref mut window,
            ref mut objects,
            input_events,
            signals,
            update_loop,
            ..
        } = self;

        let mut _device_event: winit::event::DeviceEvent =
            DeviceEvent::MouseMotion { delta: (0.0, 0.0) };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                std::process::exit(0);
            }

            WindowEvent::Resized(size) => {
                renderer.resize(size);
                camera
                    .set_resolution(size)
                    .expect("Couldn't set the resize to camera");
                camera
                    .update_view_projection(renderer)
                    .expect("Couldn't set the resize to camera in renderer");
            }

            WindowEvent::RedrawRequested => {
                input_events.end_step_time();

                if window.should_close {
                    event_loop.exit();
                }

                if let Some((mut encoder, view, frame)) = renderer
                    .pre_render(objects, window.as_ref().unwrap().inner_size(), camera)
                    .expect("Couldn't get pre render data")
                {
                    if let Some(update_function) = update_loop {
                        update_function(renderer, window, objects, input_events, camera, signals);
                    }

                    signals.events.iter_mut().for_each(|i| {
                        i.1.frame(
                            renderer,
                            window,
                            objects,
                            camera,
                            input_events,
                            &mut encoder,
                            &view,
                        );
                    });

                    for camera_value in camera.values_mut() {
                        camera_value
                            .update_view_projection(renderer)
                            .expect("Couldn't update camera");
                    }
                    objects.iter_mut().for_each(|i| {
                        if i.1.changed {
                            i.1.update(renderer).expect("Couldn't update objects");
                        }
                    });

                    match renderer.render(encoder, frame) {
                        Ok(_) => {}
                        // Recreate the swap_chain if lost
                        Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            event_loop.exit();
                        }
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }

                _device_event = DeviceEvent::MouseMotion { delta: (0.0, 0.0) };
                if let Some(window_inner) = &window.window {
                    window_inner.request_redraw();
                }
            }
            _ => {}
        }

        input_events.process_window_event(&event);

        if event == WindowEvent::RedrawRequested {
            input_events.step();
        }
    }
}

impl Window {
    /// create a new window
    pub fn new(default_attributes: winit::window::WindowAttributes) -> Self {
        Self {
            window: None,
            default_attributes,
            should_close: false,
        }
    }

    /// close the engine window
    #[allow(unused)]
    #[allow(dead_code)]
    pub fn close_engine(&mut self) {
        self.should_close = true;
    }
}
