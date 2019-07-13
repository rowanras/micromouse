use std::borrow::Cow;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Instant;

use glium::backend::Facade;
use glium::glutin;
use glium::glutin::Event;
use glium::glutin::WindowEvent;
use glium::texture::ClientFormat;
use glium::texture::RawImage2d;
use glium::Display;
use glium::Rect;
use glium::Surface;
use glium::Texture2d;

use imgui::im_str;
use imgui::Condition;
use imgui::Context;
use imgui::FontConfig;
use imgui::FontGlyphRanges;
use imgui::FontSource;
use imgui::ImGuiSelectableFlags;
use imgui::ImStr;
use imgui::ImString;
use imgui::TextureId;
use imgui::Textures;
use imgui::Ui;

use imgui_glium_renderer::GliumRenderer;
use imgui_winit_support::HiDpiMode;
use imgui_winit_support::WinitPlatform;

use crate::MouseState;

struct Plot {
    pub function: fn(&MouseState) -> f32,
    pub showing: bool,
}

impl std::fmt::Debug for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!( f, "Plot {{ showing: {:?} }}", self.showing)
    }
}

#[derive(Debug)]
struct GuiState {
    pub plots: HashMap<String, Plot>,
    pub plot_range: f32,
    pub history_time: f32,
    pub history_active: bool,
}

pub fn start(
    mouse_state: Arc<Mutex<Vec<MouseState>>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let title = "Micromouse";
        let mut events_loop = glutin::EventsLoop::new();
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let builder = glutin::WindowBuilder::new()
            .with_title(title.to_owned())
            .with_dimensions(glutin::dpi::LogicalSize::new(1024f64, 768f64));
        let display = Display::new(builder, context, &events_loop)
            .expect("Failed to initialize display");

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

        let mut renderer = GliumRenderer::init(&mut imgui, &display)
            .expect("Failed to initialize renderer");

        let gl_window = display.gl_window();
        let window = gl_window.window();
        let mut last_frame = Instant::now();
        let mut run = true;

        let mut guistate = GuiState {
            plots: HashMap::new(),
            plot_range: 10.0,
            history_time: 0.0,
            history_active: false,
        };

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
            run_ui(
                &mut run,
                &mut ui,
                renderer.textures(),
                &mut guistate,
                &*mouse_state,
            );

            let mut target = display.draw();
            target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
            platform.prepare_render(&ui, &window);
            let draw_data = ui.render();
            renderer
                .render(&mut target, draw_data)
                .expect("Rendering failed");
            target.finish().expect("Failed to swap buffers");
        }
    })
}

fn view_readonly_float(
    ui: &mut Ui,
    gui_state: &mut GuiState,
    mouse_state: &MouseState,
    label: &ImStr,
    value: fn(&MouseState) -> f32,
) {
    ui.push_id(label);
    let popup_id = ImString::new(format!("Popup_{}", label));

    ui.columns(2, im_str!("state"), false);
    ui.text(label);
    ui.same_line(0.0);
    ui.text(&format!("{}", value(mouse_state)));

    ui.next_column();

    let key: &str = label.as_ref();
    if let Some(mut plot) = gui_state.plots.get_mut(key) {
        ui.checkbox(im_str!("plot"), &mut plot.showing);
        ui.next_column();
    } else {
        ui.next_column();
        gui_state.plots.insert(
            key.to_string(),
            Plot {
                function: value,
                showing: false,
            },
        );
    }

    ui.pop_id();
}

fn view_state(ui: &mut Ui, gui_state: &mut GuiState, mouse_state: &MouseState) {

    ui.columns(1, im_str!("headers"), false);
    ui.text("Raw from the Mouse");
    ui.next_column();

    view_readonly_float(ui, gui_state, mouse_state, im_str!("Time"), |m| {
        m.time as f32
    });
    view_readonly_float(
        ui,
        gui_state,
        mouse_state,
        im_str!("Left Encoder"),
        |m| m.left as f32,
    );
    view_readonly_float(
        ui,
        gui_state,
        mouse_state,
        im_str!("Right Encoder"),
        |m| m.right as f32,
    );

    ui.columns(1, im_str!("headers"), false);
    ui.text("Calculated from raw");
    ui.next_column();

    view_readonly_float(
        ui,
        gui_state,
        mouse_state,
        im_str!("X Position"),
        |m| m.x as f32,
    );
    view_readonly_float(
        ui,
        gui_state,
        mouse_state,
        im_str!("Y Position"),
        |m| m.y as f32,
    );
    view_readonly_float(
        ui,
        gui_state,
        mouse_state,
        im_str!("Direction"),
        |m| m.dir as f32,
    );
}

fn run_ui(
    run: &mut bool,
    ui: &mut Ui,
    textures: &mut Textures<Rc<Texture2d>>,
    gui_state: &mut GuiState,
    mouse_state: &Vec<MouseState>,
) {

    let mouse_state: Vec<&MouseState> = if gui_state.history_active {
        mouse_state.iter().filter(|m| (m.time as f32) < gui_state.history_time).collect()
    } else {
        mouse_state.iter().collect()
    };

    ui.window(im_str!("Debug"))
        .size([400.0, 300.0], Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("{:#?}", gui_state));
        });

    ui.window(im_str!("State"))
        .size([400.0, 300.0], Condition::FirstUseEver)
        .build(|| {
            ui.checkbox(im_str!("Show history"), &mut gui_state.history_active);
            ui.input_float(im_str!("Historic time"), &mut gui_state.history_time).build();

            if let Some(state) = mouse_state.last() {
                view_state(ui, gui_state, state);
            } else {
                ui.text("No state yet :(");
            }
        });

    let count_plots = gui_state.plots.iter().filter(|(l, p)| p.showing).count();
    if count_plots > 0 {
        ui.window(im_str!("Plots"))
            .size(
                [800.0, (count_plots as f32) * 200.0],
                Condition::FirstUseEver,
            )
            .build(|| {
                let [width, height] = ui.get_window_size();
                let mut range = &mut gui_state.plot_range;

                ui.input_float(im_str!("plot range"), range).build();

                for (label, mut plot) in
                    gui_state.plots.iter_mut().filter(|(l, p)| p.showing)
                {
                    let last = mouse_state
                        .last()
                        .map(|m| m.time)
                        .unwrap_or(0.0);

                    let points: Vec<_> = mouse_state
                        .iter()
                        .filter(|m| m.time as f32 > last as f32 - *range)
                        .map(|m| (plot.function)(m) as f32)
                        .collect();

                    ui.push_id(label);
                    ui.text(label);
                    ui.plot_lines(im_str!(""), &points)
                        .graph_size([width - 50.0, 150.0])
                        .scale_max(200.0)
                        .scale_min(0.0)
                        .build();
                    ui.pop_id();
                }
            });
    }
}
