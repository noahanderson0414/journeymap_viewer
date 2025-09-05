pub struct Options {
    pub minimum_zoom: f32,
    pub maximum_zoom: f32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            minimum_zoom: 0.0001,
            maximum_zoom: 1.,
        }
    }
}
