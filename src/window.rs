/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::{
    header::{Camera, Engine, Renderer, WindowDescriptor},
    ObjectStorage,
};

use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

impl Engine {
    /// Creates a new window in current thread using default settings.
    pub fn new() -> anyhow::Result<Self> {
        Self::new_inner(WindowDescriptor::default())
    }

    /// Creates a new window in current thread using provided settings.
    pub fn new_config(settings: WindowDescriptor) -> anyhow::Result<Self> {
        Self::new_inner(settings)
    }

    /// Creates a new window in current thread.
    #[allow(unreachable_code)]
    pub(crate) fn new_inner(settings: WindowDescriptor) -> anyhow::Result<Self> {
        #[cfg(feature = "debug")]
        env_logger::init();
        // Dimentions of the window, as width and height
        // and then are set as a logical size that the window can accept
        #[cfg(not(feature = "android"))]
        let dimention = winit::dpi::PhysicalSize {
            width: settings.width,   // Which sets the width of the window
            height: settings.height, // And sets the height of the window
        };

        // Here the size is finally made according to the dimentions we set earlier
        #[cfg(not(feature = "android"))]
        let size = winit::dpi::Size::Physical(dimention);

        // And we will create a new window and set all the options we stored
        #[cfg(not(feature = "android"))]
        let new_window = WindowBuilder::new()
            .with_inner_size(size) // sets the width and height of window
            .with_title(String::from(settings.title)) // sets title of the window
            .with_decorations(settings.decorations) // sets if the window should have borders
            .with_resizable(settings.resizable); // sets the window to be resizable

        // will create the main event loop of the window.
        // and will contain all the callbacks and button press
        // also will allow graphics API
        let event_loop = EventLoop::new();

        // bind the loop to window
        #[cfg(not(feature = "android"))]
        let window = new_window.build(&event_loop)?;
        #[cfg(feature = "android")]
        let window = Window::new(&event_loop).unwrap();

        // The renderer init on current window
        let mut renderer = futures::executor::block_on(Renderer::new(
            &window,
            settings.power_preference,
            settings.backends,
        ))?;

        let camera = Camera::new(window.inner_size(), &mut renderer)?;

        Ok(Self {
            window,
            event_loop,
            renderer,
            objects: ObjectStorage::new(),
            camera,
            plugins: vec![],
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
                &mut Vec<Box<dyn crate::EnginePlugin>>,
            ),
    >(
        self,
        mut update_function: F,
    ) -> anyhow::Result<()> {
        let Self {
            event_loop,
            mut renderer,
            mut window,
            mut objects,
            mut camera,
            mut plugins,
        } = self;

        // and get input events to handle them later
        let mut input = winit_input_helper::WinitInputHelper::new();
        let mut _device_event: winit::event::DeviceEvent =
            DeviceEvent::MouseMotion { delta: (0.0, 0.0) };

        // The main loop
        event_loop.run(move |events, _, control_flow| {
            // updates the data on what events happened before the frame start
            input.update(&events);

            plugins.iter_mut().for_each(|i| {
                i.update_events(
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
                        *control_flow = ControlFlow::Exit;
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
                    _ => {}
                },
                #[cfg(feature = "android")]
                Event::Resumed => {
                    let surface = unsafe { renderer.instance.create_surface(&window) };
                    surface.configure(&renderer.device, &renderer.config);
                    dbg!(window.inner_size());
                    renderer.depth_buffer = Renderer::build_depth_buffer(
                        "Depth Buffer",
                        &renderer.device,
                        &renderer.config,
                    );
                    renderer.surface = Some(surface);
                }
                #[cfg(feature = "android")]
                Event::Suspended => {
                    renderer.surface = None;
                }

                Event::DeviceEvent { event, .. } => _device_event = event,
                Event::MainEventsCleared => {
                    let pre_render = renderer
                        .pre_render(&objects, &camera)
                        .expect("Couldn't get pre render data");
                    if pre_render.is_some() {
                        let (mut encoder, view, frame) = pre_render.unwrap();

                        #[cfg(not(feature = "gui"))]
                        update_function(
                            &mut renderer,
                            &mut window,
                            &mut objects,
                            &input,
                            &mut camera,
                            &mut plugins,
                        );

                        plugins.iter_mut().for_each(|i| {
                            i.update(
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

                        let ren = renderer.render(encoder, frame);

                        match ren {
                            Ok(_) => {}
                            // Recreate the swap_chain if lost
                            Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
                            // The system is out of memory, we should probably quit
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                *control_flow = ControlFlow::Exit
                            }
                            // All other errors (Outdated, Timeout) should be resolved by the next frame
                            Err(e) => eprintln!("{:?}", e),
                        }
                    }

                    _device_event = DeviceEvent::Text { codepoint: ' ' };
                    window.request_redraw();
                }
                _ => (),
            }
        });
        //logic(&mut renderer, WindowCallbackEvents::After, &window);

        Ok(())
    }
}
