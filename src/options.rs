pub struct Options {
    pub game_path: String,
    pub minimum_zoom: f32,
    pub maximum_zoom: f32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            game_path: Default::default(),
            minimum_zoom: 0.0001,
            maximum_zoom: 1.,
        }
    }
}
