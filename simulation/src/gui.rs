use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;
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

use arrayvec::ArrayVec;

use micromouse_lib::mouse::Mouse;
use micromouse_lib::msgs::Msg as MouseMsg;
use micromouse_lib::msgs::MsgId as MouseMsgId;

use crate::SimulatorState;

macro_rules! ui_f32 {
    ($m:ident, $f:expr, $s:expr, $l:expr, $u:expr, $c:expr) => {
        $c.view_float(
            $u,
            $s,
            $l,
            ImString::new(stringify!($m)).as_ref(),
            $f,
            None,
            Some(MouseMsgId::$m),
        );
    }
}

macro_rules! ui_f32c {
    ($m:ident, $f:expr, $s:expr, $l:expr, $u:expr, $c:expr) => {
        $c.view_float(
            $u,
            $s,
            $l,
            ImString::new(stringify!($m)).as_ref(),
            $f,
            None,
            None
        );
    }
}

macro_rules! ui_f32m {
    ($m:ident, $f:expr, $s:expr, $l:expr, $u:expr, $c:expr) => {
        $c.view_float(
            $u,
            $s,
            $l,
            ImString::new(stringify!($m)).as_ref(),
            $f,
            Some(MouseMsg::$m),
            Some(MouseMsgId::$m),
        );
    }
}

#[derive(Debug)]
pub enum GuiMsg {
    Mouse(MouseMsg)
}

struct Plot {
    pub function: fn(&Mouse, Option<&Mouse>) -> f32,
    pub showing: bool,
    pub range: [f32; 2],
}

impl std::fmt::Debug for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Plot {{ showing: {:?} }}", self.showing)
    }
}

#[derive(Debug)]
struct State {
    pub plots: HashMap<String, Plot>,
    pub plot_range: f32,
    pub history_time: f32,
    pub history_active: bool,
    pub editables: HashMap<String, f32>,
    pub logged: HashSet<MouseMsgId>,
}

impl State {
    pub fn new() -> State {
        State {
            plots: HashMap::new(),
            plot_range: 10.0,
            history_time: 0.0,
            history_active: false,
            editables: HashMap::new(),
            logged: HashSet::new(),
        }
    }
}

struct Gui<Msg: 'static + Send> {
    pub tx: Sender<Msg>,
    pub on_gui_msg: fn(GuiMsg) -> Msg,
    pub state: State,
}

impl<Msg: 'static + Send> Gui<Msg> {
    fn view_float(
        &mut self,
        ui: &mut Ui,
        mouse: &Mouse,
        last_mouse: Option<&Mouse>,
        label: &ImStr,
        value: fn(&Mouse, Option<&Mouse>) -> f32,
        on_edit: Option<fn(f32) -> MouseMsg>,
        id: Option<MouseMsgId>,
    ) {
        ui.push_id(label);
        let popup_id = ImString::new(format!("Popup_{}", label));

        ui.columns(2, im_str!("state"), false);
        ui.text(&format!("{}: {}", label, value(mouse, last_mouse)));

        ui.next_column();

        let key: &str = label.as_ref();
        if let Some(mut plot) = self.state.plots.get_mut(key) {
            ui.checkbox(im_str!("plot"), &mut plot.showing);
        } else {
            self.state.plots.insert(
                key.to_string(),
                Plot {
                    function: value,
                    showing: false,
                    range: [200.0, 0.0]
                },
            );
        }

        ui.same_line(0.0);

        if let Some(id) = id {
            let mut log = self.state.logged.remove(&id);
            let update_log = ui.checkbox(im_str!("log"), &mut log);
            if log {
                self.state.logged.insert(id);
            }
            if update_log {
                let log_msg_ids = self.state.logged.iter().cloned().collect();
                self.tx.send((self.on_gui_msg)(GuiMsg::Mouse(MouseMsg::Logged(log_msg_ids))));
            }
        }

        if let Some(on_edit) = on_edit {
            ui.same_line(0.0);

            if let Some(mut editable) = self.state.editables.get_mut(key) {
                {
                    let _w = ui.push_item_width(-40.0);
                    ui.input_float(im_str!(""), editable).build();
                }
                ui.same_line(0.0);
                {
                    let _w = ui.push_item_width(-1.0);
                    if ui.small_button(im_str!("Set")) {
                        self.tx.send((self.on_gui_msg)(GuiMsg::Mouse(on_edit(*editable as f32))));
                    }
                }
            } else {
                self.state.editables.insert(
                    key.to_string(),
                    value(mouse, last_mouse),
                );
            }
        }

        ui.next_column();

        ui.pop_id();
    }

    fn view_state(&mut self, ui: &mut Ui, m: &Mouse, lm: Option<&Mouse>) {

        ui_f32!(Time, |m, _| m.time, m, lm, ui, self);
        ui_f32c!(DeltaTime, |m, lm| lm.map(|lm| m.time - lm.time).unwrap_or(0.0), m, lm, ui, self);

        ui_f32m!(LeftPos, |m, _| m.left_pos, m, lm, ui, self);
        ui_f32m!(RightPos, |m, _| m.right_pos, m, lm, ui, self);
        ui_f32c!(LeftVelocity, |m, lm| {
            lm.map(|lm| (m.left_pos - lm.left_pos) / (m.time - lm.time)).unwrap_or(0.0)
        }, m, lm, ui, self);
        ui_f32c!(RightVelocity, |m, lm| {
            lm.map(|lm| (m.right_pos - lm.right_pos) / (m.time - lm.time)).unwrap_or(0.0)
        }, m, lm, ui, self);
        ui_f32m!(LeftPower, |m, _| m.left_power, m, lm, ui, self);
        ui_f32m!(RightPower, |m, _| m.right_power, m, lm, ui, self);
        ui_f32m!(Battery, |m, _| m.battery, m, lm, ui, self);

        // Calculated
        ui_f32m!(LinearPos, |m, _| m.linear_pos, m, lm, ui, self);
        ui_f32m!(AngularPos, |m, _| m.angular_pos, m, lm, ui, self);
        ui_f32c!(LinearVelocity, |m, lm| {
            lm.map(|lm| (m.linear_pos - lm.linear_pos) / (m.time - lm.time)).unwrap_or(0.0)
        }, m, lm, ui, self);
        ui_f32c!(AngularVelocity, |m, lm| {
            lm.map(|lm| (m.angular_pos - lm.angular_pos) / (m.time - lm.time)).unwrap_or(0.0)
        }, m, lm, ui, self);
        ui_f32m!(LinearPower, |m, _| m.linear_power, m, lm, ui, self);
        ui_f32m!(AngularPower, |m, _| m.angular_power, m, lm, ui, self);
        ui_f32m!(LinearSet, |m, _| m.linear_control.position as f32, m, lm, ui, self);
        ui_f32m!(AngularSet, |m, _| m.angular_control.position as f32, m, lm, ui, self);

        // Config
        ui_f32m!(LinearP, |m, _| m.linear_control.pid.p_gain as f32, m, lm, ui, self);
        ui_f32m!(LinearI, |m, _| m.linear_control.pid.i_gain as f32, m, lm, ui, self);
        ui_f32m!(LinearD, |m, _| m.linear_control.pid.d_gain as f32, m, lm, ui, self);
        ui_f32m!(LinearAcc, |m, _| m.linear_control.acceleration as f32, m, lm, ui, self);
        ui_f32m!(AngularP, |m, _| m.angular_control.pid.p_gain as f32, m, lm, ui, self);
        ui_f32m!(AngularI, |m, _| m.angular_control.pid.i_gain as f32, m, lm, ui, self);
        ui_f32m!(AngularD, |m, _| m.angular_control.pid.d_gain as f32, m, lm, ui, self);
        ui_f32m!(AngularAcc, |m, _| m.angular_control.acceleration as f32, m, lm, ui, self);
    }

    fn run_ui(&mut self, ui: &mut Ui, simulator_state: &SimulatorState) {
        let mice: Vec<&Mouse> = if self.state.history_active {
            simulator_state.mice
                .iter()
                .filter(|m| (m.time as f32) < self.state.history_time)
                .collect()
        } else {
            simulator_state.mice.iter().collect()
        };

        ui.window(im_str!("Debug"))
            .size([400.0, 300.0], Condition::FirstUseEver)
            .build(|| {
                ui.text(format!("{:#?}", self.state));
            });

        ui.window(im_str!("State"))
            .size([400.0, 800.0], Condition::FirstUseEver)
            .position([400.0, 0.0], Condition::FirstUseEver)
            .build(|| {
                ui.checkbox(
                    im_str!("Show history"),
                    &mut self.state.history_active,
                );
                ui.input_float(
                    im_str!("Historic time"),
                    &mut self.state.history_time,
                )
                .build();

                if let Some(mouse) = mice.last() {
                    self.view_state(ui, mouse, mice.get((mice.len() as i32 - 2).max(0) as usize).map(|&m| m));
                } else {
                    ui.text("No state yet :(");
                }
            });

        let count_plots =
            self.state.plots.iter().filter(|(l, p)| p.showing).count();
        if count_plots > 0 {
            ui.window(im_str!("Plots"))
                .size(
                    [800.0, ((count_plots as f32) * 220.0) + 20.0],
                    Condition::FirstUseEver,
                )
                .build(|| {
                    let [width, height] = ui.get_window_size();
                    let mut range = &mut self.state.plot_range;

                    ui.input_float(im_str!("plot range"), range).build();

                    for (label, mut plot) in
                        self.state.plots.iter_mut().filter(|(l, p)| p.showing)
                    {
                        let last =
                            mice.last().map(|m| m.time).unwrap_or(0.0);

                        let points: Vec<_> = mice
                            .iter()
                            .filter(|m| m.time as f32 > last as f32 - *range).collect();

                        let points: Vec<_> = points
                            .first()
                            .iter()
                            .map(|s| (plot.function)(s, None))
                            .chain(points.windows(2)
                                .map(|s| (plot.function)(s[1], Some(s[0])))
                            )
                            .collect();

                        ui.push_id(label);
                        ui.text(label);

                        {
                            let _w = ui.push_item_width(-50.0);
                            ui.same_line(0.0);
                            ui.input_float2(im_str!("y range"), &mut plot.range).build();
                        }

                        ui.plot_lines(im_str!(""), &points)
                            .graph_size([width - 10.0, 150.0])
                            .scale_max(plot.range[0])
                            .scale_min(plot.range[1])
                            .build();
                        ui.pop_id();
                    }
                });
        }

        ui.window(im_str!("Simulator"))
            .size([400.0, 300.0], Condition::FirstUseEver)
            .build(|| {
                ui.label_text(im_str!("Uart buffer"), ImString::new(simulator_state.uart_buffer_len.to_string()).as_ref());
            });
    }
}

pub fn start<Msg: 'static + Send>(
    simulator_state: Arc<Mutex<SimulatorState>>,
    tx: Sender<Msg>,
    on_gui_msg: fn(GuiMsg) -> Msg,
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

        let mut gui = Gui { tx, on_gui_msg, state: State::new() };

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

            let simulator_state = simulator_state.lock().unwrap();
            gui.run_ui(&mut ui, &simulator_state);

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
