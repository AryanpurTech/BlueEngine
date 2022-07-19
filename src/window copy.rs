/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::header::{Camera, Engine, Object, Renderer, WindowDescriptor};
use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

#[cfg(feature = "gui")]
use egui_winit_platform::{Platform, PlatformDescriptor};

#[cfg(feature = "gui")]
enum ExEvent {
    RequestRedraw,
}

#[cfg(feature = "gui")]
struct ExampleRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<ExEvent>>);
#[cfg(feature = "gui")]
impl epi::backend::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        self.0
            .lock()
            .unwrap()
            .send_event(ExEvent::RequestRedraw)
            .ok();
    }
}

impl Engine {
    /// Creates a new window in current thread.
    #[allow(unreachable_code)]
    pub fn new(settings: WindowDescriptor) -> anyhow::Result<Self> {
        env_logger::init();
        // Dimentions of the window, as width and height
        // and then are set as a logical size that the window can accept
        let dimention = winit::dpi::PhysicalSize {
            width: settings.width,   // Which sets the width of the window
            height: settings.height, // And sets the height of the window
        };

        // Here the size is finally made according to the dimentions we set earlier
        let size = winit::dpi::Size::Physical(dimention);

        // And we will create a new window and set all the options we stored
        let new_window = WindowBuilder::new()
            .with_inner_size(size) // sets the width and height of window
            .with_title(String::from(settings.title)) // sets title of the window
            .with_decorations(settings.decorations) // sets if the window should have borders
            .with_resizable(settings.resizable); // sets the window to be resizable

        // will create the main event loop of the window.
        // and will contain all the callbacks and button press
        // also will allow graphics API
        #[cfg(feature = "gui")]
        let event_loop = EventLoop::with_user_event();
        #[cfg(not(feature = "gui"))]
        let event_loop = EventLoop::new();

        // bind the loop to window
        let window = new_window.build(&event_loop).unwrap();

        // The renderer init on current window
        let mut renderer = futures::executor::block_on(Renderer::new(&window))?;

        let camera = Camera::new(window.inner_size(), &mut renderer)?;

        Ok(Self {
            window,
            event_loop,
            renderer,
            objects: Vec::new(),
            camera,
        })
    }

    /// Runs the block of code that you pass to it every frame. The update code is used
    /// to modify the engine on the fly thus creating interactive graphics and making things
    /// happy in the engine!
    ///
    /// Renderer, window, vec of objects, events, and camera are passed to the update code.
    #[allow(unreachable_code)]
    pub fn update_loop<F>(self, mut update_function: F) -> anyhow::Result<()>
    where
        F: 'static
            + FnMut(
                &mut Renderer,
                &Window,
                &mut Vec<Object>,
                (&winit::event::DeviceEvent, &WinitInputHelper),
                &mut Camera,
            ),
    {
        let Self {
            event_loop,
            mut renderer,
            window,
            mut objects,
            mut camera,
        } = self;

        // and get input events to handle them later
        let mut input = winit_input_helper::WinitInputHelper::new();
        let mut device_event: winit::event::DeviceEvent =
            DeviceEvent::MouseMotion { delta: (0.0, 0.0) };
        let mut current_window_size = window.inner_size();

        #[cfg(feature = "gui")]
        let mut platform = Platform::new(PlatformDescriptor {
            physical_width: current_window_size.width,
            physical_height: current_window_size.height,
            scale_factor: window.scale_factor(),
            font_definitions: egui::FontDefinitions::default(),
            style: Default::default(),
        });

        // The main loop
        event_loop.run(move |events, _, control_flow| {
            // updates the data on what events happened before the frame start
            input.update(&events);

            match events {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => {}
                },

                Event::DeviceEvent { event, .. } => device_event = event,
                Event::MainEventsCleared => {
                    let new_window_size = window.inner_size();
                    if new_window_size != current_window_size {
                        renderer.resize(new_window_size);
                        current_window_size = new_window_size;
                    }

                    update_function(
                        &mut renderer,
                        &window,
                        &mut objects,
                        (&device_event, &input),
                        &mut camera,
                    );
                    camera
                        .update_view_projection(&mut renderer)
                        .expect("Couldn't update camera");
                    objects.iter_mut().for_each(|i| {
                        if i.changed {
                            i.update(&mut renderer).expect("Couldn't update objects");
                        }
                    });

                    #[cfg(feature = "gui")]
                    platform.begin_frame();

                    egui::CentralPanel::default().show(&platform.context(), |ui| {
                        ui.heading("YOOOO");
                    });

                    let full_output = platform.end_frame(Some(&window));
                    let paint_jobs = platform.context().tessellate(full_output.shapes);
                    let tdelta: egui::TexturesDelta = full_output.textures_delta;
                    let screen_descriptor = egui_wgpu_backend::ScreenDescriptor {
                        physical_width: current_window_size.width,
                        physical_height: current_window_size.height,
                        scale_factor: window.scale_factor() as f32,
                    };

                    match renderer.render(
                        &objects,
                        &camera,
                        #[cfg(feature = "gui")]
                        (paint_jobs, tdelta, screen_descriptor),
                    ) {
                        Ok(_) => {}
                        // Recreate the swap_chain if lost
                        Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                    window.request_redraw();

                    device_event = DeviceEvent::Text { codepoint: ' ' };
                }
                _ => (),
            }
        });
        //logic(&mut renderer, WindowCallbackEvents::After, &window);

        Ok(())
    }
}
