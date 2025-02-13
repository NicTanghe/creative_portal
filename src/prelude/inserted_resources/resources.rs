use bevy::ecs::system::Resource;

use crate::prelude::components::chunks::Chunks;

#[derive(Resource)]
pub struct ChunkersR {
    pub screenplay: Chunks,
}

#[derive(Resource)]
pub struct STATE {
    pub screenplay: Chunks,
}

