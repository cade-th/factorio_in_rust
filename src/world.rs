use raylib::prelude::*;
use std::fs::File;
use std::io::{self, Read, Write};

// TODO:
// 1. Use sqlite for this somehow

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Blocks {
    GRASS,
    STONE,
    PLAYER,
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
    fn to_u8(self) -> u8 {
        self as u8
    }
}

pub struct World {
    pub data: Vec<Vec<Blocks>>,
    pub tile_size: usize,
    pub size: usize,
}

impl World {
    pub fn new() -> Self {
        World {
            data: vec![vec![Blocks::GRASS; 8]; 8],
            size: 8,
            tile_size: 64,
        }
    }

    fn entity_to_screen(entity_pos: Vector2, camera: &Camera2D) -> Vector2 {
        Vector2::new(
            (entity_pos.x - camera.target.x) * camera.zoom + camera.offset.x,
            (entity_pos.y - camera.target.y) * camera.zoom + camera.offset.y,
        )
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, texture_atlas: &Texture2D, camera: &Camera2D) {
        let _ = d.begin_mode2D(*camera);

        for i in 0..self.data.len() {
            for j in 0..self.data[0].len() {
                let tile_world_pos = Vector2::new(
                    i as f32 * self.tile_size as f32,
                    j as f32 * self.tile_size as f32,
                );

                // Convert the world position to screen position
                let tile_screen_pos = Self::entity_to_screen(tile_world_pos, camera);

                let dest_rect = Rectangle {
                    x: tile_screen_pos.x,
                    y: tile_screen_pos.y,
                    width: self.tile_size as f32 * camera.zoom,
                    height: self.tile_size as f32 * camera.zoom,
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

        // Read the tile size (4 bytes)
        let mut tile_size_bytes = [0u8; 4];
        file.read_exact(&mut tile_size_bytes)?;
        let tile_size = f32::from_le_bytes(tile_size_bytes) as usize;

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

        println!(
            "World data (size: {}, tile_size: {}) loaded from {}",
            size, tile_size, file_name
        );

        Ok(World {
            data,
            size,
            tile_size,
        })
    }
    pub fn data_to_file(&self, file_name: &str) -> io::Result<()> {
        let mut file = File::create(file_name)?;

        // Write the size of the world as a 4-byte integer
        let size_bytes = (self.size as u32).to_le_bytes();
        file.write_all(&size_bytes)?;

        // Write the tile size as a 4-byte floating-point value
        let tile_size_bytes = self.tile_size.to_le_bytes();
        file.write_all(&tile_size_bytes)?;

        // Write the world data
        for row in &self.data {
            for &block in row {
                file.write_all(&[block.to_u8()])?;
            }
        }

        println!(
            "World data (size: {}, tile_size: {}) saved to {}",
            self.size, self.tile_size, file_name
        );
        Ok(())
    }
}
