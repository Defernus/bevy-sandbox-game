use super::*;
use crate::{
    internal::{color::Color, pos::ChunkPos},
    plugins::world_generator::resources::{
        GenCaveInp, GenVoxelInp, LandscapeHeightInp, WorldGenerator,
    },
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TundraBiome;

impl TundraBiome {
    pub const ID: BiomeID = "tundra";

    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl Biome for TundraBiome {
    fn get_id(&self) -> BiomeID {
        Self::ID
    }

    fn get_generate_voxel_inp(&self, _gen: &WorldGenerator, _pos: ChunkPos) -> GenVoxelInp {
        GenVoxelInp {
            cave_inp: GenCaveInp {
                cave_factor: 1.3,
                cave_offset: 0.3,
                cave_strength: 100.0,
            },
            bumps_factor: 0.1,
            first_layer_color: Color::rgb_u8(255, 255, 255).into(),
            second_layer_color: Color::rgb_u8(65, 40, 22).into(),
            rest_layers_color: Color::rgb_u8(100, 100, 100).into(),
        }
    }

    fn get_landscape_height_inp(
        &self,
        _gen: &WorldGenerator,
        _pos: ChunkPos,
    ) -> LandscapeHeightInp {
        LandscapeHeightInp { height: 3.0 }
    }

    fn check_pos(&self, _gen: &WorldGenerator, _pos: ChunkPos, inp: BiomeCheckInput) -> bool {
        inp.temperature < 0.0
    }

    fn spawn_objects(
        &self,
        _chunk_pos: ChunkPos,
        _commands: &mut Commands,
        _gen: &WorldGenerator,
    ) -> usize {
        0
    }
}
