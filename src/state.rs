use bevy::prelude::*;
use crate::{TILE, SCALE};
use crate::map::{Tilemap, TileMarker, spawn_map};
use crate::player::Player;

// ─── game state ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapState { Hub, Dungeon }

/// Central game resource holding both maps and the current state.
#[derive(Resource)]
pub struct World {
    pub state:          MapState,
    pub hub:            Tilemap,
    pub hub_stairs:     Vec2,
    pub dungeon:        Tilemap,
    pub dungeon_stairs: Vec2,
}

impl World {
    /// Returns a reference to whichever map is currently active.
    pub fn current(&self) -> &Tilemap {
        match self.state {
            MapState::Hub     => &self.hub,
            MapState::Dungeon => &self.dungeon,
        }
    }
}

// ─── stair transition ────────────────────────────────────────────────────────

/// Counts down each frame after a stair transition; the transition cannot
/// re-fire until it reaches zero. 60 frames ≈ 1 s at 60 fps — long enough
/// to outlast the deferred `commands.insert` that repositions the player.
#[derive(Resource)]
pub struct StairCooldown(pub u32);
pub const COOLDOWN_FRAMES: u32 = 60;

pub fn stair_transition(
    mut world:    ResMut<World>,
    mut cooldown: ResMut<StairCooldown>,
    p_query:      Query<(Entity, &Transform), With<Player>>,
    tile_query:   Query<Entity, With<TileMarker>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if cooldown.0 > 0 { cooldown.0 -= 1; return; }

    let Ok((player_e, transform)) = p_query.get_single() else { return };
    let centre = Vec2::new(transform.translation.x, transform.translation.y);
    if !world.current().stairs_at(centre) { return; }

    world.state = match world.state {
        MapState::Hub     => MapState::Dungeon,
        MapState::Dungeon => MapState::Hub,
    };

    for e in &tile_query { commands.entity(e).despawn(); }
    spawn_map(&mut commands, &asset_server, world.current());

    // Teleport to one tile above the destination stairs — always inside the
    // room interior and guaranteed to be open ground.
    let dest = match world.state {
        MapState::Hub     => world.hub_stairs,
        MapState::Dungeon => world.dungeon_stairs,
    };
    commands.entity(player_e).insert(
        Transform::from_xyz(dest.x, dest.y + TILE, 1.0).with_scale(Vec3::splat(SCALE))
    );

    cooldown.0 = COOLDOWN_FRAMES;
}