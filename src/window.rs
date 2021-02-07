/*
 * Blue Engine is a graphics backend made by the Mystic Blue team.
 *
 * It provides API and backend details for the projects within the
 * Mystic Blue team. The license is same as the one on the root.
*/

use crate::render::Renderer;

#[allow(unreachable_code)]
use winit::{
    event::{Event, WindowEvent, *},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct Window {
    window: winit::window::Window,
    event_loop: EventLoop<()>,
}

impl Window {
    pub fn new(
        width: f64,
        height: f64,
        title: &'static str,
        decorations: bool,
        resizable: bool,
    ) -> Result<Self, ()> {
        // Dimentions of the window, as width and height
        // and then are set as a logical size that the window can accept
        let dimention = winit::dpi::LogicalSize {
            width: width,   // Which sets the width of the window
            height: height, // And sets the height of the window
        };

        // Here the size is finally made according to the dimentions we set earlier
        let size = winit::dpi::Size::Logical(dimention);

        // And we will create a new window and set all the options we stored
        let new_window = WindowBuilder::new()
            .with_inner_size(size) // sets the width and height of window
            .with_title(String::from(title)) // sets title of the window
            .with_decorations(decorations) // sets if the window should have borders
            .with_resizable(resizable); // sets the window to be resizable

        // will create the main event loop of the window.
        // and will contain all the callbacks and button press
        // also will allow graphics API

        let event_loop = EventLoop::new();
        // bind the loop to window
        let window = new_window.build(&event_loop).unwrap();

        Ok(Self {
            window: window,
            event_loop: event_loop
        })
    }
    pub fn game_loop(&mut self, event_loop: EventLoop<()>, window: winit::window::Window, renderer: Renderer) {
        let timer = std::time::SystemTime::now();
        let mut tick: u64 = 0;
        let mut fps: i32 = 0;
        // Let's start defining the loop, shall we?
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    if !renderer.input(event) {
                        match event {
                            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                            WindowEvent::KeyboardInput { input, .. } => match input {
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                } => *control_flow = ControlFlow::Exit,
                                _ => {}
                            },
                            WindowEvent::Resized(physical_renderer) => {
                                renderer.resize(*physical_renderer);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                // new_inner_renderer is &&mut so we have to dereference it twice
                                renderer.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }

                Event::MainEventsCleared => {
                    //window.request_redraw();
                    renderer.update();
                    match renderer.render() {
                        Ok(_) => {}
                        // Recreate the swap_chain if lost
                        Err(wgpu::SwapChainError::Lost) => renderer.resize(renderer.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                _ => (),
            }
            let now = timer.elapsed().unwrap().as_secs();
            if tick < now {
                tick = now;
                println!("FPS: {}", fps);
                fps = 0;
            } else {
                fps = fps + 1;
            }
        });
    }
}
