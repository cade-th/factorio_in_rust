use raylib::prelude::*;
use std::fs::File;
use std::io::{self, Read};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Blocks {
    GRASS = 0,
    STONE = 1,
    PLAYER = 2,
}

impl Blocks {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Blocks::GRASS),
            1 => Some(Blocks::STONE),
            2 => Some(Blocks::PLAYER),
            _ => None,
        }
    }
}

pub struct World {
    pub data: Vec<Vec<Blocks>>,
    pub size: usize,
}

impl World {
    pub fn new() -> io::Result<World> {
        Self::from_file("data.cade")
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, texture_atlas: &Texture2D) {
        let tile_size = 64.0;

        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                let dest_rect = Rectangle {
                    x: i as f32 * tile_size as f32,
                    y: j as f32 * tile_size as f32,
                    width: tile_size - 1.0,
                    height: tile_size - 1.0,
                };

                let texture_section = match self.data[i][j] {
                    Blocks::STONE => Rectangle {
                        x: 0.0,
                        y: 32.0,
                        width: 32.0,
                        height: 32.0,
                    },
                    Blocks::GRASS => Rectangle {
                        x: 32.0,
                        y: 32.0,
                        width: 32.0,
                        height: 32.0,
                    },
                    _ => Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: 32.0,
                        height: 32.0,
                    },
                };

                d.draw_texture_pro(
                    texture_atlas,
                    texture_section,
                    dest_rect,
                    Vector2::new(0.0, 0.0),
                    0.0,
                    Color::WHITE,
                );
            }
        }
    }

    pub fn from_file(file_name: &str) -> io::Result<Self> {
        let mut file = File::open(file_name)?;

        // Read the world size (4 bytes)
        let mut size_bytes = [0u8; 4];
        file.read_exact(&mut size_bytes)?;
        let size = u32::from_le_bytes(size_bytes) as usize;

        // Calculate the expected number of data bytes
        let expected_data_length = size * size;

        // Read the remaining file data
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Validate the file size
        if buffer.len() != expected_data_length {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "File size does not match expected world dimensions",
            ));
        }

        // Initialize the world data dynamically
        let mut data = vec![vec![Blocks::GRASS; size]; size];
        let mut index = 0;

        for i in 0..size {
            for j in 0..size {
                if let Some(block) = Blocks::from_u8(buffer[index]) {
                    data[i][j] = block;
                } else {
                    println!("Invalid block type in file at position {}", index);
                }
                index += 1;
            }
        }

        // TODO: Get the tile size from the level maker

        println!("World data (size: {}) loaded from {}", size, file_name);
        Ok(World { data, size })
    }
}
