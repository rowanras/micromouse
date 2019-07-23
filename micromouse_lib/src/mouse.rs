use arrayvec::ArrayVec;

use crate::control::MotionControl;
use crate::control::Target;
use crate::msgs::Msg;
use crate::msgs::MsgId;
use crate::Config;
use crate::Mouse as MouseConfig;

pub const MAX_MSGS: usize = 64;

#[derive(Clone)]
pub struct Mouse {
    pub mouse_config: MouseConfig,

    pub logged: ArrayVec<[MsgId; MAX_MSGS]>,
    pub provided: ArrayVec<[MsgId; MAX_MSGS]>,

    pub time: f32,

    pub battery: f32,

    pub left_distance: u8,
    pub front_distance: u8,
    pub right_distance: u8,

    pub left_pos: f32,
    pub right_pos: f32,

    pub linear_pos: f32,
    pub angular_pos: f32,

    pub linear_control: MotionControl,
    pub angular_control: MotionControl,

    pub linear_power: f32,
    pub angular_power: f32,

    pub left_power: f32,
    pub right_power: f32,
}

impl Mouse {
    pub fn new(config: Config) -> Mouse {
        Mouse {
            mouse_config: config.mouse,

            battery: 0.0,

            logged: ArrayVec::new(),
            provided: ArrayVec::new(),

            time: 0.0,

            left_distance: 0,
            front_distance: 0,
            right_distance: 0,

            left_pos: 0.0,
            right_pos: 0.0,

            linear_pos: 0.0,
            angular_pos: 0.0,

            linear_control: MotionControl::new(config.linear_motion),
            angular_control: MotionControl::new(config.angular_motion),

            linear_power: 0.0,
            angular_power: 0.0,

            left_power: 0.0,
            right_power: 0.0,
        }
    }

    pub fn update(&mut self, msg: Msg) {
        match msg {
            // Core
            Msg::Time(t) => self.time = t,
            Msg::Logged(m) => self.logged = m,
            Msg::Provided(m) => self.provided = m,

            // Raw in/out
            Msg::LeftPos(p) => self.left_pos = p,
            Msg::RightPos(p) => self.right_pos = p,
            Msg::LeftPower(p) => self.left_power = p,
            Msg::RightPower(p) => self.right_power = p,
            Msg::Battery(v) => self.battery = v,
            Msg::LeftDistance(d) => self.left_distance = d,
            Msg::FrontDistance(d) => self.front_distance = d,
            Msg::RightDistance(d) => self.right_distance = d,

            // Calculated
            Msg::LinearPos(p) => self.linear_pos = p,
            Msg::AngularPos(p) => self.angular_pos = p,
            Msg::LinearPower(p) => self.linear_power = p,
            Msg::AngularPower(p) => self.angular_power = p,
            Msg::LinearSet(s) => self.linear_control.position = s as f64,
            Msg::AngularSet(s) => self.linear_control.position = s as f64,
            Msg::AddLinear(v, d) => {
                self.linear_control.queue_target(Target {
                    velocity: v as f64,
                    distance: d as f64,
                });
            }
            Msg::AddAngular(v, d) => {
                self.angular_control.queue_target(Target {
                    velocity: v as f64,
                    distance: d as f64,
                });
            }

            // Config
            Msg::LinearP(p) => self.linear_control.pid.p_gain = p as f64,
            Msg::LinearI(i) => self.linear_control.pid.i_gain = i as f64,
            Msg::LinearD(d) => self.linear_control.pid.d_gain = d as f64,
            Msg::LinearAcc(a) => self.linear_control.acceleration = a as f64,
            Msg::AngularP(p) => self.angular_control.pid.p_gain = p as f64,
            Msg::AngularI(i) => self.angular_control.pid.i_gain = i as f64,
            Msg::AngularD(d) => self.angular_control.pid.d_gain = d as f64,
            Msg::AngularAcc(a) => self.angular_control.acceleration = a as f64,
        }
    }

    pub fn msg(&self, msgid: MsgId) -> Msg {
        match msgid {
            // Core
            MsgId::Time => Msg::Time(self.time),
            MsgId::Logged => Msg::Logged(self.logged.clone()),
            MsgId::Provided => Msg::Provided(self.provided.clone()),

            // Raw in/out
            MsgId::LeftPos => Msg::LeftPos(self.left_pos),
            MsgId::RightPos => Msg::RightPos(self.right_pos),
            MsgId::LeftPower => Msg::LeftPower(self.left_power),
            MsgId::RightPower => Msg::RightPower(self.right_power),
            MsgId::Battery => Msg::Battery(self.battery),
            MsgId::LeftDistance => Msg::LeftDistance(self.left_distance),
            MsgId::FrontDistance => Msg::FrontDistance(self.front_distance),
            MsgId::RightDistance => Msg::RightDistance(self.right_distance),

            // Calculated
            MsgId::LinearPos => Msg::LinearPos(self.linear_pos),
            MsgId::AngularPos => Msg::AngularPos(self.angular_pos),
            MsgId::LinearPower => Msg::LinearPower(self.linear_power),
            MsgId::AngularPower => Msg::AngularPower(self.angular_power),
            MsgId::LinearSet => {
                Msg::LinearSet(self.linear_control.position as f32)
            }
            MsgId::AngularSet => {
                Msg::AngularSet(self.linear_control.position as f32)
            }
            MsgId::AddLinear => Msg::AddLinear(0.0, 0.0),
            MsgId::AddAngular => Msg::AddAngular(0.0, 0.0),

            // Config
            MsgId::LinearP => {
                Msg::LinearP(self.linear_control.pid.p_gain as f32)
            }
            MsgId::LinearI => {
                Msg::LinearI(self.linear_control.pid.i_gain as f32)
            }
            MsgId::LinearD => {
                Msg::LinearD(self.linear_control.pid.d_gain as f32)
            }
            MsgId::LinearAcc => {
                Msg::LinearAcc(self.linear_control.acceleration as f32)
            }
            MsgId::AngularP => {
                Msg::AngularP(self.angular_control.pid.p_gain as f32)
            }
            MsgId::AngularI => {
                Msg::AngularI(self.angular_control.pid.i_gain as f32)
            }
            MsgId::AngularD => {
                Msg::AngularD(self.angular_control.pid.d_gain as f32)
            }
            MsgId::AngularAcc => {
                Msg::AngularAcc(self.angular_control.acceleration as f32)
            }
        }
    }
}
