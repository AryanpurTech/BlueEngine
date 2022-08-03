/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::header::{Camera, Engine, Object, Renderer, WindowDescriptor};
#[cfg(feature = "gui")]
use imgui::FontSource;
use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

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
        let event_loop = EventLoop::new();

        // bind the loop to window
        let window = new_window.build(&event_loop).unwrap();

        // The renderer init on current window
        let mut renderer =
            futures::executor::block_on(Renderer::new(&window, settings.power_preference))?;

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
    pub fn update_loop<
        #[cfg(feature = "gui")] T: 'static
            + FnMut(
                &mut Renderer,
                &Window,
                &mut Vec<Object>,
                (&winit::event::DeviceEvent, &WinitInputHelper),
                &mut Camera,
                &imgui::Ui,
            ),
        #[cfg(not(feature = "gui"))] F: 'static
            + FnMut(
                &mut Renderer,
                &Window,
                &mut Vec<Object>,
                (&winit::event::DeviceEvent, &WinitInputHelper),
                &mut Camera,
            ),
    >(
        self,
        #[cfg(feature = "gui")] mut update_function: T,
        #[cfg(not(feature = "gui"))] mut update_function: F,
    ) -> anyhow::Result<()> {
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
        let mut imgui = imgui::Context::create();
        #[cfg(feature = "gui")]
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        #[cfg(feature = "gui")]
        platform.attach_window(
            imgui.io_mut(),
            &window,
            imgui_winit_support::HiDpiMode::Default,
        );
        #[cfg(feature = "gui")]
        imgui.set_ini_filename(None);

        #[cfg(feature = "gui")]
        let hidpi_factor = window.scale_factor();

        #[cfg(feature = "gui")]
        imgui_redesign(&mut imgui, hidpi_factor);

        #[cfg(feature = "gui")]
        let mut imgui_renderer = imgui_wgpu::Renderer::new(
            &mut imgui,
            &renderer.device,
            &renderer.queue,
            imgui_wgpu::RendererConfig {
                texture_format: renderer.surface.get_supported_formats(&renderer.adapter)[0],
                ..Default::default()
            },
        );

        #[cfg(feature = "gui")]
        let mut last_frame = std::time::Instant::now();

        // The main loop
        event_loop.run(move |events, _, control_flow| {
            // updates the data on what events happened before the frame start
            input.update(&events);

            #[cfg(feature = "gui")]
            {
                let now = std::time::Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }

            #[cfg(feature = "gui")]
            platform.handle_event(imgui.io_mut(), &window, &events);

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

                    #[cfg(feature = "gui")]
                    platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("Failed to prepare frame");
                    #[cfg(feature = "gui")]
                    let ui = imgui.frame();

                    #[cfg(feature = "gui")]
                    update_function(
                        &mut renderer,
                        &window,
                        &mut objects,
                        (&device_event, &input),
                        &mut camera,
                        &ui,
                    );
                    #[cfg(not(feature = "gui"))]
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
                    let ren = renderer.render(&objects, &camera, &mut imgui_renderer, ui);
                    #[cfg(not(feature = "gui"))]
                    let ren = renderer.render(&objects, &camera);

                    match ren {
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

#[cfg(feature = "gui")]
fn imgui_redesign(imgui: &mut imgui::Context, hidpi_factor: f64) {
    let font_size = (13.0 * hidpi_factor) as f32;

    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("./utils/JetBrainsMono-Medium.ttf"),
        size_pixels: 20f32,
        config: Some(imgui::FontConfig {
            name: Some("JetBrainsMono".to_string()),
            ..Default::default()
        }),
    }]);

    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(imgui::FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()
        }),
    }]);

    imgui.set_renderer_name(Some("Blue Engine".to_string()));

    let mut style = imgui.style_mut();

    // Theme by https://github.com/ocornut/imgui/issues/707#issuecomment-917151020
    // Colors
    style.colors[imgui::sys::ImGuiCol_Text as usize] = [1f32, 1f32, 1f32, 1f32];
    style.colors[imgui::sys::ImGuiCol_TextDisabled as usize] = [0.5f32, 0.5f32, 0.5f32, 1f32];
    style.colors[imgui::sys::ImGuiCol_WindowBg as usize] = [0.1f32, 0.1f32, 0.1f32, 1f32];
    style.colors[imgui::sys::ImGuiCol_PopupBg as usize] = [0.19f32, 0.19f32, 0.19f32, 0.92f32];
    style.colors[imgui::sys::ImGuiCol_Border as usize] = [0.19f32, 0.19f32, 0.19f32, 0.29f32];
    style.colors[imgui::sys::ImGuiCol_BorderShadow as usize] = [0.00f32, 0.00f32, 0.00f32, 0.24f32];
    style.colors[imgui::sys::ImGuiCol_FrameBg as usize] = [0.05f32, 0.05f32, 0.05f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_FrameBgHovered as usize] =
        [0.19f32, 0.19f32, 0.19f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_FrameBgActive as usize] =
        [0.20f32, 0.22f32, 0.23f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_TitleBg as usize] = [0.00f32, 0.00f32, 0.00f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_TitleBgActive as usize] =
        [0.06f32, 0.06f32, 0.06f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_TitleBgCollapsed as usize] =
        [0.00f32, 0.00f32, 0.00f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_MenuBarBg as usize] = [0.14f32, 0.14f32, 0.14f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_ScrollbarBg as usize] = [0.05f32, 0.05f32, 0.05f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_ScrollbarGrab as usize] =
        [0.34f32, 0.34f32, 0.34f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_ScrollbarGrabHovered as usize] =
        [0.40f32, 0.40f32, 0.40f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_ScrollbarGrabActive as usize] =
        [0.56f32, 0.56f32, 0.56f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_CheckMark as usize] = [0.33f32, 0.67f32, 0.86f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_SliderGrab as usize] = [0.34f32, 0.34f32, 0.34f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_SliderGrabActive as usize] =
        [0.56f32, 0.56f32, 0.56f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_Button as usize] = [0.05f32, 0.05f32, 0.05f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_ButtonHovered as usize] =
        [0.19f32, 0.19f32, 0.19f32, 0.54f32];
    style.colors[imgui::sys::ImGuiCol_ButtonActive as usize] = [0.20f32, 0.22f32, 0.23f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_Header as usize] = [0.00f32, 0.00f32, 0.00f32, 0.52f32];
    style.colors[imgui::sys::ImGuiCol_HeaderHovered as usize] =
        [0.00f32, 0.00f32, 0.00f32, 0.36f32];
    style.colors[imgui::sys::ImGuiCol_HeaderActive as usize] = [0.20f32, 0.22f32, 0.23f32, 0.33f32];
    style.colors[imgui::sys::ImGuiCol_Separator as usize] = [0.28f32, 0.28f32, 0.28f32, 0.29f32];
    style.colors[imgui::sys::ImGuiCol_SeparatorHovered as usize] =
        [0.44f32, 0.44f32, 0.44f32, 0.29f32];
    style.colors[imgui::sys::ImGuiCol_SeparatorActive as usize] =
        [0.40f32, 0.44f32, 0.47f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_ResizeGrip as usize] = [0.28f32, 0.28f32, 0.28f32, 0.29f32];
    style.colors[imgui::sys::ImGuiCol_ResizeGripHovered as usize] =
        [0.44f32, 0.44f32, 0.44f32, 0.29f32];
    style.colors[imgui::sys::ImGuiCol_ResizeGripActive as usize] =
        [0.40f32, 0.44f32, 0.47f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_Tab as usize] = [0.00f32, 0.00f32, 0.00f32, 0.52f32];
    style.colors[imgui::sys::ImGuiCol_TabHovered as usize] = [0.14f32, 0.14f32, 0.14f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_TabActive as usize] = [0.20f32, 0.20f32, 0.20f32, 0.36f32];
    style.colors[imgui::sys::ImGuiCol_TabUnfocused as usize] = [0.00f32, 0.00f32, 0.00f32, 0.52f32];
    style.colors[imgui::sys::ImGuiCol_TabUnfocusedActive as usize] =
        [0.14f32, 0.14f32, 0.14f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_PlotLines as usize] = [1.00f32, 0.00f32, 0.00f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_PlotLinesHovered as usize] =
        [1.00f32, 0.00f32, 0.00f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_PlotHistogram as usize] =
        [1.00f32, 0.00f32, 0.00f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_PlotHistogramHovered as usize] =
        [1.00f32, 0.00f32, 0.00f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_TableHeaderBg as usize] =
        [0.00f32, 0.00f32, 0.00f32, 0.52f32];
    style.colors[imgui::sys::ImGuiCol_TableBorderStrong as usize] =
        [0.00f32, 0.00f32, 0.00f32, 0.52f32];
    style.colors[imgui::sys::ImGuiCol_TableBorderLight as usize] =
        [0.28f32, 0.28f32, 0.28f32, 0.29f32];
    style.colors[imgui::sys::ImGuiCol_TableRowBg as usize] = [0.00f32, 0.00f32, 0.00f32, 0.00f32];
    style.colors[imgui::sys::ImGuiCol_TableRowBgAlt as usize] =
        [1.00f32, 1.00f32, 1.00f32, 0.06f32];
    style.colors[imgui::sys::ImGuiCol_TextSelectedBg as usize] =
        [0.20f32, 0.22f32, 0.23f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_DragDropTarget as usize] =
        [0.33f32, 0.67f32, 0.86f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_NavHighlight as usize] = [1.00f32, 0.00f32, 0.00f32, 1.00f32];
    style.colors[imgui::sys::ImGuiCol_NavWindowingHighlight as usize] =
        [1.00f32, 0.00f32, 0.00f32, 0.70f32];
    style.colors[imgui::sys::ImGuiCol_NavWindowingDimBg as usize] =
        [1.00f32, 0.00f32, 0.00f32, 0.20f32];
    style.colors[imgui::sys::ImGuiCol_ModalWindowDimBg as usize] =
        [1.00f32, 0.00f32, 0.00f32, 0.35f32];

    // Configs
    style.window_padding = [8f32, 8f32];
    style.frame_padding = [5f32, 2f32];
    style.cell_padding = [6f32, 6f32];
    style.item_spacing = [6f32, 6f32];
    style.item_inner_spacing = [6f32, 6f32];
    style.touch_extra_padding = [0f32, 0f32];
    style.indent_spacing = 25f32;
    style.scrollbar_size = 15f32;
    style.grab_min_size = 10f32;
    style.window_border_size = 1f32;
    style.child_border_size = 1f32;
    style.popup_border_size = 1f32;
    style.frame_border_size = 1f32;
    style.tab_border_size = 1f32;
    style.window_rounding = 7f32;
    style.child_rounding = 4f32;
    style.frame_rounding = 3f32;
    style.popup_rounding = 4f32;
    style.scrollbar_rounding = 9f32;
    style.grab_rounding = 3f32;
    style.log_slider_deadzone = 4f32;
    style.tab_rounding = 4f32;
}
