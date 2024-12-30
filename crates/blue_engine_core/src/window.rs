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
    pub fn new() -> Result<Self, crate::error::Error> {
        Self::new_inner(
            WindowDescriptor::default(),
            #[cfg(target_os = "android")]
            None,
        )
    }

    /// Creates a new window in current thread using provided settings.
    pub fn new_config(settings: WindowDescriptor) -> Result<Self, crate::error::Error> {
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
    ) -> Result<Self, crate::error::Error> {
        Self::new_inner(settings, Some(app))
    }

    /// Creates a new window in current thread.
    #[allow(unreachable_code)]
    pub(crate) fn new_inner(
        settings: WindowDescriptor,
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
    ) -> Result<(), crate::error::Error> {
        self.update_loop = Some(Box::new(update_function));

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

impl ApplicationHandler for Engine {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let Self {
            ref mut window,
            ref mut renderer,
            ref mut objects,
            ref mut signals,
            ref mut camera,
            ..
        } = self;

        if window.is_none() {
            if let Ok(new_window) = event_loop.create_window(window.default_attributes.clone()) {
                let new_window = std::sync::Arc::new(new_window);

                if renderer.surface.is_none() {
                    if let Ok(surface) = renderer.instance.create_surface(new_window.clone()) {
                        let surface_capabilities = surface.get_capabilities(&renderer.adapter);
                        let tex_format = surface_capabilities
                            .formats
                            .iter()
                            .copied()
                            .find(|f| f.is_srgb())
                            .unwrap_or(surface_capabilities.formats[0]);

                        renderer.config.format = tex_format;
                        renderer.config.view_formats = vec![tex_format];

                        surface.configure(&renderer.device, &renderer.config);
                        renderer.depth_buffer = Renderer::build_depth_buffer(
                            "Depth Buffer",
                            &renderer.device,
                            &renderer.config,
                        );
                        renderer.surface = Some(surface);

                        renderer.build_default_data();
                        objects.iter_mut().for_each(|i| {
                            i.1.update(renderer);
                        });
                    }
                }

                new_window.set_min_inner_size(window.default_attributes.min_inner_size);
                new_window.set_max_inner_size(window.default_attributes.max_inner_size);
                if let Some(position) = window.default_attributes.position {
                    new_window.set_outer_position(position);
                }
                new_window.set_resizable(window.default_attributes.resizable);
                new_window.set_enabled_buttons(window.default_attributes.enabled_buttons);
                new_window.set_title(window.default_attributes.title.as_str());
                new_window.set_maximized(window.default_attributes.maximized);
                new_window.set_visible(window.default_attributes.visible);
                new_window.set_transparent(window.default_attributes.transparent);
                new_window.set_blur(window.default_attributes.blur);
                new_window.set_decorations(window.default_attributes.decorations);
                new_window.set_window_icon(window.default_attributes.window_icon.clone());
                new_window.set_theme(window.default_attributes.preferred_theme);
                new_window.set_resize_increments(window.default_attributes.resize_increments);
                new_window.set_window_level(window.default_attributes.window_level);
                new_window.set_cursor(window.default_attributes.cursor.clone());
                new_window.set_fullscreen(window.default_attributes.fullscreen.clone());

                window.window = Some(new_window);
            }

            signals.events.iter_mut().for_each(|i| {
                i.1.init(renderer, &self.window, objects, camera);
            });
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        let Self {
            ref mut camera,
            ref mut renderer,
            ref mut window,
            ref mut objects,
            input_events,
            signals,
            ..
        } = self;

        input_events.process_device_event(&event);
        signals.events.iter_mut().for_each(|i| {
            i.1.device_events(renderer, window, objects, &event, input_events, camera);
        });
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

        signals.events.iter_mut().for_each(|i| {
            i.1.window_events(renderer, window, objects, &event, input_events, camera);
        });

        let mut _device_event: winit::event::DeviceEvent =
            DeviceEvent::MouseMotion { delta: (0.0, 0.0) };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
                std::process::exit(0);
            }

            WindowEvent::Resized(size) => {
                renderer.resize(size);
                camera.set_resolution(size);
                camera.update_view_projection(renderer);
            }

            WindowEvent::RedrawRequested => {
                input_events.end_step_time();

                if window.should_close {
                    event_loop.exit();
                }

                if let Some(window_ref) = window.as_ref() {
                    if let Ok(Some((mut encoder, view, frame))) =
                        renderer.pre_render(objects, window_ref.inner_size(), camera)
                    {
                        if let Some(update_function) = update_loop {
                            update_function(
                                renderer,
                                window,
                                objects,
                                input_events,
                                camera,
                                signals,
                            );
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
                            camera_value.update_view_projection(renderer);
                        }
                        objects.iter_mut().for_each(|i| {
                            if i.1.changed {
                                i.1.update(renderer);
                            }
                        });

                        renderer.render(encoder, frame);
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
    pub fn close_engine(&mut self) {
        self.should_close = true;
    }

    // ====================================================== WINDOW SETTERS ====================================================== //
    //MARK: SETTERS

    /// see [winit::window::Window::set_min_inner_size]
    pub fn set_min_inner_size(&mut self, value: Option<winit::dpi::Size>) {
        if let Some(window) = self.window.as_mut() {
            window.set_min_inner_size(value);
        } else {
            self.default_attributes.min_inner_size = value;
        }
    }

    /// see [winit::window::Window::set_max_inner_size]
    pub fn set_max_inner_size(&mut self, value: Option<winit::dpi::Size>) {
        if let Some(window) = self.window.as_mut() {
            window.set_max_inner_size(value);
        } else {
            self.default_attributes.max_inner_size = value;
        }
    }

    /// see [winit::window::Window::set_outer_position]
    pub fn set_outer_position(&mut self, value: winit::dpi::Position) {
        if let Some(window) = self.window.as_mut() {
            window.set_outer_position(value);
        } else {
            self.default_attributes.position = Some(value);
        }
    }

    /// see [winit::window::Window::set_resizable]
    pub fn set_resizable(&mut self, value: bool) {
        if let Some(window) = self.window.as_mut() {
            window.set_resizable(value);
        } else {
            self.default_attributes.resizable = value;
        }
    }

    /// see [winit::window::Window::set_enabled_buttons]
    pub fn set_enabled_buttons(&mut self, value: winit::window::WindowButtons) {
        if let Some(window) = self.window.as_mut() {
            window.set_enabled_buttons(value);
        } else {
            self.default_attributes.enabled_buttons = value;
        }
    }

    /// see [winit::window::Window::set_title]
    pub fn set_title(&mut self, value: String) {
        if let Some(window) = self.window.as_mut() {
            window.set_title(value.as_str());
        } else {
            self.default_attributes.title = value;
        }
    }

    /// see [winit::window::Window::set_maximized]
    pub fn set_maximized(&mut self, value: bool) {
        if let Some(window) = self.window.as_mut() {
            window.set_maximized(value);
        } else {
            self.default_attributes.maximized = value;
        }
    }

    /// see [winit::window::Window::set_visible]
    pub fn set_visible(&mut self, value: bool) {
        if let Some(window) = self.window.as_mut() {
            window.set_visible(value);
        } else {
            self.default_attributes.visible = value;
        }
    }

    /// see [winit::window::Window::set_transparent]
    pub fn set_transparent(&mut self, value: bool) {
        if let Some(window) = self.window.as_mut() {
            window.set_transparent(value);
        } else {
            self.default_attributes.transparent = value;
        }
    }

    /// see [winit::window::Window::set_blur]
    pub fn set_blur(&mut self, value: bool) {
        if let Some(window) = self.window.as_mut() {
            window.set_blur(value);
        } else {
            self.default_attributes.blur = value;
        }
    }

    /// see [winit::window::Window::set_decorations]
    pub fn set_decorations(&mut self, value: bool) {
        if let Some(window) = self.window.as_mut() {
            window.set_decorations(value);
        } else {
            self.default_attributes.decorations = value;
        }
    }

    /// see [winit::window::Window::set_window_icon]
    pub fn set_window_icon(&mut self, value: Option<winit::window::Icon>) {
        if let Some(window) = self.window.as_mut() {
            window.set_window_icon(value);
        } else {
            self.default_attributes.window_icon = value;
        }
    }

    /// see [winit::window::Window::set_preferred_theme]
    pub fn set_preferred_theme(&mut self, value: Option<winit::window::Theme>) {
        if let Some(window) = self.window.as_mut() {
            window.set_theme(value);
        } else {
            self.default_attributes.preferred_theme = value;
        }
    }

    /// see [winit::window::Window::set_resize_increments]
    pub fn set_resize_increments(&mut self, value: Option<winit::dpi::Size>) {
        if let Some(window) = self.window.as_mut() {
            window.set_resize_increments(value);
        } else {
            self.default_attributes.resize_increments = value;
        }
    }

    /// see [winit::window::Window::set_content_protected]
    pub fn set_content_protected(&mut self, value: bool) {
        if let Some(window) = self.window.as_mut() {
            window.set_content_protected(value);
        } else {
            self.default_attributes.content_protected = value;
        }
    }

    /// see [winit::window::Window::set_window_level]
    pub fn set_window_level(&mut self, value: winit::window::WindowLevel) {
        if let Some(window) = self.window.as_mut() {
            window.set_window_level(value);
        } else {
            self.default_attributes.window_level = value;
        }
    }

    /// see [winit::window::Window::set_cursor]
    pub fn set_cursor(&mut self, value: winit::window::Cursor) {
        if let Some(window) = self.window.as_mut() {
            window.set_cursor(value);
        } else {
            self.default_attributes.cursor = value;
        }
    }

    /// see [winit::window::Window::set_fullscreen]
    pub fn set_fullscreen_borderless(&mut self, value: bool) {
        let full_screen_result = if value {
            Some(winit::window::Fullscreen::Borderless(None))
        } else {
            None
        };

        if let Some(window) = self.window.as_mut() {
            window.set_fullscreen(full_screen_result);
        } else {
            self.default_attributes.fullscreen = full_screen_result;
        }
    }

    /// see [winit::window::Window::set_fullscreen]
    ///
    /// **Does not work unless during update_loop**
    pub fn set_fullscreen_exclusive(&mut self, value: bool) {
        if let Some(window) = self.window.as_mut() {
            let full_screen_result = match value {
                true => match window.available_monitors().next() {
                    Some(monitor) => match monitor.video_modes().next() {
                        Some(vide_mode) => Some(winit::window::Fullscreen::Exclusive(vide_mode)),
                        None => None,
                    },
                    None => None,
                },
                false => None,
            };

            window.set_fullscreen(full_screen_result);
        } else {
            self.default_attributes.fullscreen = None;
        }
    }
}
