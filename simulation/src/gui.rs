
use std::thread;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

use glium::glutin;
use glium::glutin::Event;
use glium::glutin::WindowEvent;
use glium::Display;
use glium::Surface;

use imgui::Ui;
use imgui::Context;
use imgui::FontConfig;
use imgui::FontGlyphRanges;
use imgui::FontSource;
use imgui::Condition;
use imgui::ImStr;
use imgui::ImString;
use imgui::im_str;

use imgui_glium_renderer::GliumRenderer;
use imgui_winit_support::HiDpiMode;
use imgui_winit_support::WinitPlatform;

use crate::MouseState;

struct GuiState {

}

pub fn start(mouse_state: Arc<Mutex<Vec<MouseState>>>) {
    thread::spawn(move || {
        let title = "Micromouse";
        let mut events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let builder = glutin::WindowBuilder::new()
            .with_title(title.to_owned())
            .with_dimensions(glutin::dpi::LogicalSize::new(1024f64, 768f64));
        let display =
            Display::new(builder, context, &events_loop).expect("Failed to initialize display");

        let mut imgui = Context::create();
        imgui.set_ini_filename(None);

        let mut platform = WinitPlatform::init(&mut imgui);
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);
        }

        let hidpi_factor = platform.hidpi_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.fonts().add_font(&[
            FontSource::TtfData {
                data: include_bytes!("FiraSans-Regular.ttf"),
                size_pixels: font_size,
                config: Some(FontConfig {
                    rasterizer_multiply: 1.00,
                    ..FontConfig::default()
                }),
            },
            FontSource::DefaultFontData {
                config: Some(FontConfig {
                    size_pixels: font_size,
                    ..FontConfig::default()
                }),
            },
        ]);

        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        let mut renderer =
            GliumRenderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

        let gl_window = display.gl_window();
        let window = gl_window.window();
        let mut last_frame = Instant::now();
        let mut run = true;

        while run {
            events_loop.poll_events(|event| {
                platform.handle_event(imgui.io_mut(), &window, &event);

                if let Event::WindowEvent { event, .. } = event {
                    if let WindowEvent::CloseRequested = event {
                        run = false;
                    }
                }
            });

            let io = imgui.io_mut();
            platform
                .prepare_frame(io, &window)
                .expect("Failed to start frame");
            last_frame = io.update_delta_time(last_frame);
            let mut ui = imgui.frame();

            let mouse_state = mouse_state.lock().unwrap();
            run_ui(&*mouse_state, &mut run, &mut ui);

            let mut target = display.draw();
            target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
            platform.prepare_render(&ui, &window);
            let draw_data = ui.render();
            renderer
                .render(&mut target, draw_data)
                .expect("Rendering failed");
            target.finish().expect("Failed to swap buffers");
        }
    });
}

fn view_readonly(ui: &mut Ui, label: &ImStr, value: &ImStr) {
    ui.text(label);
    ui.same_line(0.0);
    ui.text(value);
}

fn view_state(ui: &mut Ui, mouse_state: &MouseState) {
    view_readonly(ui, im_str!("Left Encoder"), ImString::new(mouse_state.left.to_string()).as_ref());
    view_readonly(ui, im_str!("Right Encoder"), ImString::new(mouse_state.right.to_string()).as_ref());
    view_readonly(ui, im_str!("X Position"), ImString::new(mouse_state.x.to_string()).as_ref());
    view_readonly(ui, im_str!("Y Position"), ImString::new(mouse_state.y.to_string()).as_ref());
    view_readonly(ui, im_str!("Direction"), ImString::new(mouse_state.dir.to_string()).as_ref());
}

fn run_ui(mouse_state: &Vec<MouseState>, run: &mut bool, ui: &mut Ui) {
    ui.window(im_str!("State"))
        .size([400.0, 300.0], Condition::FirstUseEver)
        .build( || {
            if let Some(state) = mouse_state.last() {
                view_state(ui, state);
            } else {
                ui.text("No state yet :(");
            }
        });
}
