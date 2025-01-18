use raylib::prelude::*;
use std::fs::File;
use std::io::{self, Read};

const SIZE: usize = 8;

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
    pub data: [[Blocks; SIZE]; SIZE],
    pub tile_size: f32,
}

impl World {
    pub fn new() -> io::Result<World> {
        Self::from_file("data.cade")
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, texture_atlas: &Texture2D) {
        // Render the world (tiles)
        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                let dest_rect = Rectangle {
                    x: i as f32 * self.tile_size as f32,
                    y: j as f32 * self.tile_size as f32,
                    width: self.tile_size - 1.0, // Scale the tile with zoom
                    height: self.tile_size - 1.0,
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

    // Load world data from a file
    pub fn from_file(file_name: &str) -> io::Result<Self> {
        let mut file = File::open(file_name)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        if buffer.len() != SIZE * SIZE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "File size does not match expected world dimensions",
            ));
        }

        let mut data = [[Blocks::STONE; SIZE]; SIZE];
        let mut index = 0;

        for i in 0..SIZE {
            for j in 0..SIZE {
                if let Some(block) = Blocks::from_u8(buffer[index]) {
                    data[i][j] = block;
                } else {
                    println!("Invalid block type in file at position {}", index);
                }
                index += 1;
            }
        }

        println!("World data loaded from {}", file_name);
        Ok(World {
            data,
            // Assuming tile_size is calculated based on `SIZE`:
            tile_size: SIZE as f32 * SIZE as f32,
        })
    }
}
