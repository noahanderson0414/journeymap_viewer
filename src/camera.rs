use macroquad::prelude::*;

#[derive(Default)]
pub struct Camera {
    camera: Camera2D,
    zoom: f32,
}