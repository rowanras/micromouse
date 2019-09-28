use core::f32::consts::FRAC_PI_2;
use core::f32::consts::FRAC_PI_4;

use micromath::F32Ext;

use arrayvec::ArrayVec;
use arrayvec::Array;

use pid_control::Controller;
use pid_control::PIDController;
use pid_control::DerivativeMode;

use crate::config;

const HALF_SQRT_2: f32 = 0.707106781;

fn psign(x: f32) -> f32 {
    if x > 0.0 {
        1.0
    } else {
        -1.0
    }
}

fn nsign(x: f32) -> f32 {
    if x > 0.0 {
        -1.0
    } else {
        1.0
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Segment {
    Line(f32, f32),
    Arc45(f32),
    Arc90(f32),
}

impl Segment {
    pub fn total_distance(&self) -> f32 {
        match self {
            &Segment::Line(d) => d,
            &Segment::Arc45(r) => FRAC_PI_4 * r,
            &Segment::Arc90(r) => FRAC_PI_2 * r,
        }
    }

    pub fn distance_along(&self, x: f32, y: f32) -> f32 {
        match self {
            &Segment::Line(_) => x,
            &Segment::Arc45(r) => x.atan2(r - y) * r,
            &Segment::Arc90(r) => x.atan2(r - y) * r,
        }
    }

    pub fn distance_from(&self, x: f32, y: f32) -> f32 {
        match self {
            &Segment::Line(_d) => y,
            &Segment::Arc45(r) => r.abs() - (x * x + (r - y) * (r - y)).sqrt(),
            &Segment::Arc90(r) => r.abs() - (x * x + (r - y) * (r - y)).sqrt(),
        }
    }

    pub fn offset_coords(&self, x: f32, y: f32) -> (f32, f32) {
        match self {
            &Segment::Line(d) => (x - d, y),
            &Segment::Arc45(r) => (
                -HALF_SQRT_2 * ((nsign(r)) * x + (r - y)),
                -HALF_SQRT_2 * ((psign(r)) * x + (r - y)) + r,
            ),
            &Segment::Arc90(r) => (r - y, r - x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Segment;
    use core::f32::consts::PI;
    use micromath::F32Ext;

    const MAX_DELTA: f32 = 0.000001;

    #[test]
    fn segment_line_total_distance() {
        let line = Segment::Line(10.0);
        assert_eq!(line.total_distance(), 10.0);
    }
    #[test]
    fn segment_line_distance_along() {
        let line = Segment::Line(10.0);
        assert_eq!(line.distance_along(7.0, 2.0), 7.0);
    }
    #[test]
    fn segment_line_distance_from() {
        let line = Segment::Line(10.0);
        assert_eq!(line.distance_from(7.0, 2.0), 2.0);
    }
    #[test]
    fn segment_line_offset_coords() {
        let line = Segment::Line(10.0);
        assert_eq!(line.offset_coords(11.0, 2.0), (1.0, 2.0));
    }

    #[test]
    fn segment_arc45_total() {
        let arc = Segment::Arc45(10.0);
        assert_eq!(arc.total_distance(), 10.0 * PI * 2.0 / 8.0);
    }

    #[test]
    fn segment_arc45_distance_along() {
        let arc = Segment::Arc45(10.0);
        assert_close(
            arc.distance_along(7.77817459305, 2.22182540695),
            arc.total_distance(),
        );
    }

    #[test]
    fn segment_arc45_distance_from() {
        let arc = Segment::Arc45(10.0);
        assert_close(arc.distance_from(7.77817459305, 2.22182540695), -1.0);
    }

    #[test]
    fn segment_arc_45_offset_coords() {
        let arc = Segment::Arc45(10.0);
        assert_close2(arc.offset_coords(7.77817459305, 2.22182540695), (0.0, -1.0));
    }

    #[test]
    fn segment_arc90_total() {
        let arc = Segment::Arc90(10.0);
        assert_close(arc.total_distance(), 10.0 * PI * 2.0 / 4.0);
    }

    #[test]
    fn segment_arc90_distance_along() {
        let arc = Segment::Arc90(10.0);
        assert_close(arc.distance_from(7.77817459305, 2.22182540695), -1.0);
    }

    #[test]
    fn segment_arc90_offset_coords() {
        let arc = Segment::Arc90(10.0);
        assert_close2(arc.offset_coords(11.0, 10.0), (0.0, -1.0));
    }

    fn assert_close2(left: (f32, f32), right: (f32, f32)) {
        let delta0 = (left.0 - right.0).abs();
        let delta1 = (left.1 - right.1).abs();
        assert!(
            delta0 <= MAX_DELTA && delta1 <= MAX_DELTA,
            "\nleft: {:?}\nright: {:?}\ndelta: {:?}\n",
            left,
            right,
            (delta0, delta1),
        );
    }

    fn assert_close(left: f32, right: f32) {
        let delta = (left - right).abs();
        assert!(
            delta <= MAX_DELTA,
            "\nleft: {}\nright: {}\ndelta: {}\n",
            left,
            right,
            delta
        );
    }
}

pub const PATH_BUF_LEN: usize = 64;

pub struct PathConfig {
    p: f32,
    i: f32,
    d: f32,
}

#[derive(Clone, Debug)]
pub struct Path {
    pub pid: PIDController,
    pub segment_buffer: ArrayVec<[Segment; PATH_BUF_LEN]>,
    pub left: f32,
    pub right: f32,
    pub x: f32,
    pub y: f32,
    pub dir: f32,
    pub time: f32,
}

impl Path {
    pub fn new(config: config::PathPath, now: f32, left: f32, right: f32) -> Path {
        let mut pid = PIDController::new(config.p as f64, config.i as f64, config.d as f64);
        pid.d_mode = DerivativeMode::OnError;
        Path {
            pid,
            segment_buffer: ArrayVec::new(),
            left,
            right,
            x: 0.0,
            y: 0.0,
            dir: 0.0,
            time: now,
        }
    }

    pub fn add_segments(&mut self, segments: &[Segment]) -> Result<usize, usize> {
        for (i, segment) in segments.iter().enumerate() {
            if self.segment_buffer.try_push(*segment).is_err() {
                return Err(i);
            }
        }

        Ok(PATH_BUF_LEN - self.segment_buffer.len())
    }

    pub fn update(&mut self, mouse: &config::Mouse, now: f32, left: f32, right: f32) -> (f32, u8) {
        let delta_time = now - self.time;

        let delta_left = left - self.left;
        let delta_right = right - self.right;

        let delta_linear = (delta_left + delta_right) / 2.0;
        let delta_angular = mouse.mm_to_rads((delta_left - delta_right) / 2.0);

        let  mid_dir = self.dir + delta_angular / 2.0;

        self.x += delta_linear * mid_dir.cos();
        self.y += delta_linear * mid_dir.sin();
        self.dir += delta_angular;

        if let Some(segment) = self.segment_buffer.first() {
            if segment.distance_along(self.x, self.y) >= segment.total_distance() {
                let (x, y) = segment.offset_coords(self.x, self.y);
                self.x = x;
                self.y = y;
                self.segment_buffer.pop();
            }
        }

        if let Some(segment) = self.segment_buffer.first() {
            let offset = segment.distance_from(self.x, self.y);
            self.pid.update(offset as f64, delta_time as f64) as f32
        } else {
            0.0
        }
    }
}
