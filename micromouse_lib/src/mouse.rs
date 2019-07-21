
use arrayvec::ArrayVec;

use crate::Config;
use crate::Mouse as MouseConfig;
use crate::control::MotionControl;
use crate::msgs::Msg;
use crate::msgs::MsgId;

pub struct Mouse {
    mouse_config: MouseConfig,

    logged: ArrayVec<[MsgId; 8]>,
    provided: ArrayVec<[MsgId; 8]>,

    time: f32,

    left_encoder_pos: f32,
    right_encoder_pos: f32,

    linear_position: f32,
    angular_position: f32,

    linear_control: MotionControl,
    angular_control: MotionControl,

    linear_power: f32,
    angular_power: f32,

    left_motor_power: f32,
    right_motor_power: f32,
}

impl Mouse {
    pub fn new(config: Config) -> Mouse {
        Mouse {
            mouse_config: config.mouse,

            logged: ArrayVec::new(),
            provided: ArrayVec::new(),

            time: 0.0,

            left_encoder_pos: 0.0,
            right_encoder_pos: 0.0,

            linear_position: 0.0,
            angular_position: 0.0,

            linear_control: MotionControl::new(config.linear_motion),
            angular_control: MotionControl::new(config.angular_motion),

            linear_power: 0.0,
            angular_power: 0.0,

            left_motor_power: 0.0,
            right_motor_power: 0.0,
        }
    }

    pub fn update(&mut self, msg: Msg) {
        match msg{
            // Core
            Msg::Time(t) => self.time = t,
            Msg::Logged(m) => self.logged = m,
            Msg::Provided(m) => self.provided = m,

            // Raw in/out
            Msg::LeftPos(p) => self.left_encoder_pos = p,
            Msg::RightPos(p) => self.right_encoder_pos = p,
            Msg::LeftPower(p) => self.left_motor_power = p,
            Msg::RightPower(p) => self.right_motor_power = p,

            // Calculated
            Msg::LinearPos(p) => self.linear_position = p,
            Msg::AngularPos(p) => self.angular_position = p,
            Msg::LinearSet(s) => self.linear_
            Msg::AngularSet(s) => {},
            Msg::AddLinear(v, d) => {},
            Msg::AddAngular(v, d) => {},

            // Config
            Msg::LinearP(p) => {},
            Msg::LinearI(i) => {},
            Msg::LinearD(d) => {},
            Msg::LinearAcc(a) => {},
            Msg::AngularP(p) => {},
            Msg::AngularI(i) => {},
            Msg::AngularD(d) => {},
            Msg::AngularAcc(a) => {},
        }
    }
}

