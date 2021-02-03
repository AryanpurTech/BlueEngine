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

mod state;
use state::Vertex;

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.0, 1.0, 0.0],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [1.0, 0.0, 0.0],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.0, 0.0, 1.0],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.0, 0.5, 0.0],
    }, // E
];

const TRIANGLES: &[Vertex] = &[
    Vertex {
        position: [1.0, 1.0, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
const TRAINGLEINDICES: &[u16] = &[0, 1, 2];

fn main() {
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
    let mut state = block_on(state::State::new(&window));



    let cwd = std::env::current_dir().unwrap();
    println!("{:?}", cwd.join("shaders"));
    let vertex_shader = std::fs::read(cwd.join("shader").join("shader.vs.spv")).unwrap();
    let fragment_shader = std::fs::read(cwd.join("shader").join("shader.fs.spv")).unwrap();

    let pipe1 = state::RenderPipelineLayout {
        name: String::from("triangle 1"),
        vertex_shader: vertex_shader.clone(),
        fragment_shader: fragment_shader.clone(),
        verticies: Vec::from(VERTICES),
        indicies: Vec::from(INDICES),
    };

    let pipe2 = state::RenderPipelineLayout {
        name: String::from("triangle 1"),
        vertex_shader: vertex_shader,
        fragment_shader: fragment_shader,
        verticies: Vec::from(TRIANGLES),
        indicies: Vec::from(TRAINGLEINDICES),
    };

    state.new_pipeline(pipe1, 0..1);
    state.new_pipeline(pipe2, 0..1);

    let timer = std::time::SystemTime::now();
    let mut tick: u64 = 0;
    let mut fps: i8 = 0;

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
                    Ok(_) => {
                        let now = timer.elapsed().unwrap().as_secs();
                        if tick < now {
                            tick = now;
                            println!("FPS: {}", fps);
                            fps = 0;
                        } else {
                            fps = fps + 1;
                        }
                    }
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
