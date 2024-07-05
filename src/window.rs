/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::{
    header::{Camera, Engine, Renderer, WindowDescriptor},
    ObjectStorage, Window,
};

use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

impl Engine {
    /// Creates a new window in current thread using default settings.
    pub fn new() -> color_eyre::Result<Self> {
        Self::new_inner(
            WindowDescriptor::default(),
            #[cfg(target_os = "android")]
            None,
        )
    }

    /// Creates a new window in current thread using provided settings.
    pub fn new_config(settings: WindowDescriptor) -> color_eyre::Result<Self> {
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
    ) -> color_eyre::Result<Self> {
        Self::new_inner(settings, Some(app))
    }

    /// Creates a new window in current thread.
    #[allow(unreachable_code)]
    pub(crate) fn new_inner(
        settings: WindowDescriptor,
        #[cfg(target_os = "android")] android_app: Option<
            winit::platform::android::activity::AndroidApp,
        >,
    ) -> color_eyre::Result<Self> {
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
        let new_window = WindowBuilder::new()
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

        #[cfg(not(target_os = "android"))]
        let event_loop = EventLoop::new()?;

        event_loop.set_control_flow(settings.control_flow);

        // bind the loop to window
        #[cfg(not(feature = "android"))]
        let window = new_window.build(&event_loop)?;
        #[cfg(feature = "android")]
        let window = winit::window::Window::new(&event_loop)?;

        let window = std::sync::Arc::new(window);

        let window_inner_size = window.inner_size();

        // The renderer init on current window
        let mut renderer = futures::executor::block_on(Renderer::new(window.clone(), settings))?;

        let camera = Camera::new(window_inner_size, &mut renderer)?;

        Ok(Self {
            window: Window::new(window),
            event_loop,
            renderer,
            objects: ObjectStorage::new(),
            camera,
            signals: crate::SignalStorage::new(),
        })
    }

    /// Runs the block of code that you pass to it every frame. The update code is used
    /// to modify the engine on the fly thus creating interactive graphics and making things
    /// happy in the engine!
    ///
    /// Renderer, window, vec of objects, events, and camera are passed to the update code.
    #[allow(unreachable_code)]
    pub fn update_loop<
        F: 'static
            + FnMut(
                // Core
                &mut Renderer,
                &mut Window,
                &mut ObjectStorage,
                &winit_input_helper::WinitInputHelper,
                &mut Camera,
                &mut crate::SignalStorage,
            ),
    >(
        self,
        mut update_function: F,
    ) -> color_eyre::Result<()> {
        let Self {
            event_loop,
            mut renderer,
            mut window,
            mut objects,
            mut camera,
            mut signals,
        } = self;

        // and get input events to handle them later
        let mut input = winit_input_helper::WinitInputHelper::new();
        let mut _device_event: winit::event::DeviceEvent =
            DeviceEvent::MouseMotion { delta: (0.0, 0.0) };

        // The main loop
        event_loop.run(move |events, window_target| {
            input.update(&events);

            signals.events.iter_mut().for_each(|i| {
                i.1.events(
                    &mut renderer,
                    &window,
                    &mut objects,
                    &events,
                    &input,
                    &mut camera,
                );
            });

            match events {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested => {
                        window_target.exit();
                        std::process::exit(0);
                    }

                    WindowEvent::Resized(size) => {
                        renderer.resize(*size);
                        camera
                            .set_resolution(*size)
                            .expect("Couldn't set the resize to camera");
                        camera
                            .update_view_projection(&mut renderer)
                            .expect("Couldn't set the resize to camera in renderer");
                    }

                    WindowEvent::RedrawRequested => {
                        if window.should_close {
                            window_target.exit();
                        }

                        let pre_render = renderer
                            .pre_render(&objects, window.inner_size(), &camera)
                            .expect("Couldn't get pre render data");
                        if pre_render.is_some() {
                            let (mut encoder, view, frame) =
                                pre_render.expect("Couldn't get pre render data");

                            update_function(
                                &mut renderer,
                                &mut window,
                                &mut objects,
                                &input,
                                &mut camera,
                                &mut signals,
                            );

                            signals.events.iter_mut().for_each(|i| {
                                i.1.frame(
                                    &mut renderer,
                                    &window,
                                    &mut objects,
                                    &mut camera,
                                    &input,
                                    &mut encoder,
                                    &view,
                                );
                            });

                            camera
                                .update_view_projection(&mut renderer)
                                .expect("Couldn't update camera");
                            objects.iter_mut().for_each(|i| {
                                if i.1.changed {
                                    i.1.update(&mut renderer).expect("Couldn't update objects");
                                }
                            });

                            match renderer.render(encoder, frame) {
                                Ok(_) => {}
                                // Recreate the swap_chain if lost
                                Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
                                // The system is out of memory, we should probably quit
                                Err(wgpu::SurfaceError::OutOfMemory) => {
                                    window_target.exit();
                                }
                                // All other errors (Outdated, Timeout) should be resolved by the next frame
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }

                        _device_event = DeviceEvent::MouseMotion { delta: (0.0, 0.0) };
                        window.request_redraw();
                    }
                    _ => {}
                },

                Event::Resumed => {
                    // let surface = unsafe {
                    //     renderer
                    //         .instance
                    //         .create_surface_unsafe(
                    //             wgpu::SurfaceTargetUnsafe::from_window(&window.window)
                    //                 .expect("Couldn't create surface target"),
                    //         )
                    //         .expect("Couldn't create surface")
                    // };

                    let surface = renderer
                        .instance
                        .create_surface(window.window.clone())
                        .unwrap();
                    surface.configure(&renderer.device, &renderer.config);

                    renderer.depth_buffer = Renderer::build_depth_buffer(
                        "Depth Buffer",
                        &renderer.device,
                        &renderer.config,
                    );
                    renderer.surface = Some(surface);
                }

                Event::Suspended => {
                    renderer.surface = None;
                }
                Event::DeviceEvent { event, .. } => _device_event = event,

                _ => (),
            }
        })?;
        //logic(&mut renderer, WindowCallbackEvents::After, &window);

        Ok(())
    }
}

impl Window {
    /// create a new window
    pub fn new(window: std::sync::Arc<crate::winit::window::Window>) -> Self {
        Self {
            window,
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
