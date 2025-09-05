use std::time::Instant;

use macroquad::prelude::*;

use crate::options::Options;

fn lerp<T>(a: T, b: T, progress: f32) -> T
where
    T: std::ops::Add<T, Output = T>,
    T: std::ops::Mul<f32, Output = T>,
{
    return a * (1.0 - progress) + b * progress;
}

pub struct Camera {
    pub camera: Camera2D,
    pub target: Vec2,
    pub zoom: f32,
    pub last_left_click: Option<Instant>,
    pub last_middle_click: Option<Instant>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            camera: Camera2D {
                zoom: vec2(
                    1.0 / 100.0,
                    screen_width() / screen_height() * (1.0 / 100.0),
                ),
                ..Default::default()
            },
            target: Default::default(),
            zoom: 1.0 / 100.0,
            last_left_click: Default::default(),
            last_middle_click: Default::default(),
        }
    }
}

impl Camera {
    pub fn update(&mut self, options: &Options, pointer_requested: bool) {
        if pointer_requested {
            return;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let now = Instant::now();
            if let Some(last_left_click) = self.last_left_click
                && now.duration_since(last_left_click).as_secs_f32() < 0.25
            {
                self.target = Default::default();
            }

            self.last_left_click = Some(now);
        }

        if is_mouse_button_down(MouseButton::Left) {
            self.target += mouse_delta_position() / self.zoom;
        }

        if is_mouse_button_pressed(MouseButton::Middle) {
            let now = Instant::now();
            if let Some(last_middle_click) = self.last_middle_click
                && now.duration_since(last_middle_click).as_secs_f32() < 0.25
            {
                self.zoom = 1.0 / 100.0;
            }

            self.last_middle_click = Some(now);
        }

        let mut scroll = mouse_wheel().1;
        if scroll != 0.0 {
            scroll /= scroll.abs();
            self.zoom *= (scroll + 1.0) / 2.0 + 0.5;
        }

        self.zoom = self.zoom.clamp(options.minimum_zoom, options.maximum_zoom);

        let delta = get_frame_time();
        self.camera.target = lerp(self.camera.target, self.target, delta / 0.1);
        self.camera.zoom = lerp(
            self.camera.zoom,
            vec2(self.zoom, screen_width() / screen_height() * self.zoom),
            delta / 0.05,
        );
    }
}
