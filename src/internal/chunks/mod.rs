use bevy_reflect::{FromReflect, Reflect};

use crate::plugins::{
    game_world::resources::{GameWorld, GameWorldMeta},
    static_mesh::components::Vertex,
};

use super::{
    direction::Direction,
    pos::{ChunkPos, GlobalVoxelPos, VoxelPos},
    voxel::{voxels_to_vertex::append_vertex, Voxel},
    voxels_generator::generate_voxels,
};
use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Clone, Default, Reflect, FromReflect)]
pub struct ChunkPointer {
    #[reflect(ignore)]
    chunk: Arc<Mutex<Chunk>>,
    pos: ChunkPos,
}

#[derive(Default)]
pub struct Chunk {
    voxels: Vec<Voxel>,
    need_redraw: bool,
    neighbors: [Option<ChunkPointer>; Direction::COUNT],
}

impl Chunk {
    pub const SIZE: usize = 16;
    pub const SIZE_I64: i64 = Self::SIZE as i64;
    pub const VOLUME: usize = Self::SIZE * Self::SIZE * Self::SIZE;
    pub const VOLUME_I64: i64 = Self::VOLUME as i64;
    pub const SIZES: VoxelPos = VoxelPos::from_scalar(Self::SIZE);

    pub fn generate(world_meta: GameWorldMeta, pos: ChunkPos) -> Self {
        Self {
            voxels: generate_voxels(world_meta.seed, pos * Self::SIZE as i64, Self::SIZES),
            need_redraw: true,
            neighbors: Direction::iter_map(|_| None),
        }
    }

    pub fn is_need_redraw(&self) -> bool {
        self.need_redraw
    }

    pub fn set_need_redraw(&mut self, need_redraw: bool) {
        self.need_redraw = need_redraw;
    }

    /// Updates the neighbors of this chunk.
    ///
    /// **WARNING**: This function only update **THIS** chunk, you also need to add this chunk to each neighbor.
    pub fn update_neighbors(&mut self, world: &GameWorld, pos: ChunkPos) {
        Direction::iter_map(|dir| {
            let neighbor_pos: ChunkPos = pos + dir;
            let neighbor_chunk = world.get_chunk(neighbor_pos);
            self.set_neighbor(dir, neighbor_chunk);
        });
    }

    pub fn set_neighbor(&mut self, dir: Direction, chunk: Option<ChunkPointer>) {
        self.neighbors[dir as usize] = chunk;
    }

    /// Returns the voxel at the given relative to chunk position.
    ///
    /// note: If the position is out of bounds this function will try to get the voxel from the neighbor chunk.
    /// If the neighbor chunk is not loaded, this function will return `None`.
    pub fn get_voxel(&self, pos: GlobalVoxelPos) -> Option<Voxel> {
        if pos.x >= Self::SIZE_I64 {
            return self
                .get_neighbor(Direction::X)?
                .lock()
                .get_voxel(pos - GlobalVoxelPos::new(Self::SIZE_I64, 0, 0));
        }

        if pos.y >= Self::SIZE_I64 {
            return self
                .get_neighbor(Direction::Y)?
                .lock()
                .get_voxel(pos - GlobalVoxelPos::new(0, Self::SIZE_I64, 0));
        }

        if pos.z >= Self::SIZE_I64 {
            return self
                .get_neighbor(Direction::Z)?
                .lock()
                .get_voxel(pos - GlobalVoxelPos::new(0, 0, Self::SIZE_I64));
        }

        if pos.x < 0 {
            return self
                .get_neighbor(Direction::X_NEG)?
                .lock()
                .get_voxel(pos + GlobalVoxelPos::new(Self::SIZE_I64, 0, 0));
        }

        if pos.y < 0 {
            return self
                .get_neighbor(Direction::Y_NEG)?
                .lock()
                .get_voxel(pos + GlobalVoxelPos::new(0, Self::SIZE_I64, 0));
        }

        if pos.z < 0 {
            return self
                .get_neighbor(Direction::Z_NEG)?
                .lock()
                .get_voxel(pos + GlobalVoxelPos::new(0, 0, Self::SIZE_I64));
        }

        Some(
            self.voxels[VoxelPos::new(pos.x as usize, pos.y as usize, pos.z as usize)
                .to_index(Self::SIZE)],
        )
    }

    pub fn get_neighbor(&self, dir: Direction) -> Option<ChunkPointer> {
        self.neighbors[dir as usize].clone()
    }

    /// Set the voxel at the given position.
    ///
    /// **WARNING**: If the position is out of bounds (one of the coordinates is greater than `OVERLAP_SIZE`), this function will panic.
    pub fn set_voxel(&mut self, pos: VoxelPos, voxel: Voxel) {
        if pos.x >= Self::SIZE || pos.y >= Self::SIZE || pos.z >= Self::SIZE {
            panic!("Voxel position out of bounds: {:?}", pos);
        }
        self.voxels[pos.to_index(Self::SIZE)] = voxel;
    }

    pub fn generate_vertices(&mut self) -> Vec<Vertex> {
        let mut vertices: Vec<Vertex> = Vec::new();
        for x in 0..Self::SIZE {
            for y in 0..Self::SIZE {
                for z in 0..Self::SIZE {
                    append_vertex((x, y, z).into(), self, &mut vertices);
                }
            }
        }

        vertices
    }

    pub fn iter_neighbors(&self) -> impl Iterator<Item = (Direction, Option<ChunkPointer>)> {
        self.neighbors
            .clone()
            .into_iter()
            .enumerate()
            .map(|(dir, neighbor)| (dir.try_into().unwrap(), neighbor))
    }
}

impl ChunkPointer {
    pub fn new(chunk: Chunk, pos: ChunkPos) -> Self {
        Self {
            chunk: Arc::new(Mutex::new(chunk)),
            pos,
        }
    }

    pub fn lock(&self) -> MutexGuard<Chunk> {
        self.chunk.lock().unwrap()
    }

    pub fn get_pos(&self) -> ChunkPos {
        self.pos
    }
}

impl Debug for ChunkPointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkPointer").finish()
    }
}
