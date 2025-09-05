use macroquad::prelude::*;
use regex::Regex;
use std::path::Path;

pub struct Chunk {
    pub texture: Texture2D,
    pub position: Vec2,
}

impl Chunk {
    async fn load_from_file(path: &str) -> Self {
        let file_name = Path::new(path).file_name().unwrap().to_str().unwrap();

        let regex = Regex::new(r"(-?\d+),(-?\d+)\.png").unwrap();
        let captures = regex.captures(file_name).unwrap();

        let texture = load_texture(path).await.unwrap();
        texture.set_filter(FilterMode::Nearest);

        Self {
            position: vec2(
                captures[1].parse::<f32>().unwrap() * texture.size().x,
                captures[2].parse::<f32>().unwrap() * texture.size().y,
            ),
            texture,
        }
    }
}

pub struct World {
    pub chunks: Vec<Chunk>,
}

impl World {
    pub async fn load_from_directory(path: &str) -> Self {
        let entries = std::fs::read_dir(path)
            .unwrap()
            .map(|result| result.map(|entry| entry.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap();

        let mut chunks = vec![];
        for entry in entries.iter() {
            chunks.push(Chunk::load_from_file(entry.to_str().unwrap()).await);
        }
        
        Self {
            chunks,
        }
    }
    
    pub fn draw(&self) {
        for chunk in self.chunks.iter() {
            draw_texture(&chunk.texture, chunk.position.x, chunk.position.y, WHITE);
        }
    }
}
