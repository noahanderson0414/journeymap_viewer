use macroquad::prelude::*;

pub struct Camera {
    pub camera: Camera2D,
    pub target: Vec2,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            camera: Default::default(),
            target: Default::default(),
            zoom: 1.0,
        }
    }
}

impl Camera {
    pub fn update(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            self.target += mouse_delta_position() / self.zoom;
        }

        let mut scroll = mouse_wheel().1;
        scroll = if scroll > 0.0 { 1.0 } else { -1.0 };
        self.zoom *= (scroll + 1.0) / 2.0 + 0.5;

        self.camera.target = self.camera.target * 0.9 + self.target * 0.1;
        self.camera.zoom = self.camera.zoom * 0.75 + vec2(self.zoom, screen_width() / screen_height() * self.zoom) * 0.25;
    }
}
