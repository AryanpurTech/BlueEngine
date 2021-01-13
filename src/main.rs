/*
 * Blue Engine is a graphics backend made by the Mystic Blue team.
 *
 * It provides API and backend details for the projects within the
 * Mystic Blue team. The license is same as the one on the root.
*/

#[allow(unreachable_code)]
use futures::executor::block_on;
use winit::{
    event::{Event, WindowEvent, *},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
extern crate env_logger;

mod shader;
mod state;

fn main() {
    let cwd = std::env::current_dir().unwrap();
    println!("{:?}", cwd.join("shaders"));
    let vertex_shader = std::fs::read(cwd.join("shaders").join("shader.vs.spv")).unwrap();
    let fragment_shader = std::fs::read(cwd.join("shaders").join("shader.fs.spv")).unwrap();

    env_logger::init();
    let width: f64 = 800.0;
    let height: f64 = 600.0;
    let title = "Blue Engine";
    let decorations = true;
    let resizable = true;

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

    // State class for wgpu
    let mut state = block_on(state::State::new(&window, vertex_shader.as_slice(), fragment_shader.as_slice()));

    // Let's start defining the loop, shall we?
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
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
                        WindowEvent::Resized(physical_state) => {
                            state.resize(*physical_state);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_state is &&mut so we have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(_) => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => (),
        }
    });
}
