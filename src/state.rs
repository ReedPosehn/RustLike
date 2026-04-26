use bevy::prelude::*;
use crate::{TILE, SCALE};
use crate::map::{Tilemap, TileMarker, RoomInfo, spawn_map};
use crate::player::Player;

// ─── game state ──────────────────────────────────────────────────────────────

/// Bevy-managed state for which map is active.
/// `OnEnter` / `OnExit` schedules handle all map spawning and despawning,
/// so there is no manual cooldown or state field on `World`.
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapState {
    #[default]
    Hub,
    Dungeon,
}

// ─── world resource ──────────────────────────────────────────────────────────

/// Holds both maps, their stair positions, and dungeon room data.
/// Does not store the active state — that is owned by Bevy's `State<MapState>`.
#[derive(Resource)]
pub struct World {
    pub hub:            Tilemap,
    pub hub_stairs:     Vec2,
    pub dungeon:        Tilemap,
    pub dungeon_stairs: Vec2,
    /// Centre world position of each dungeon room, in generation order.
    /// The last entry is the stair room. Used by the enemy spawner.
    pub dungeon_rooms:  Vec<RoomInfo>,
}

impl World {
    /// Returns the map for the given state.
    pub fn current(&self, state: &MapState) -> &Tilemap {
        match state {
            MapState::Hub     => &self.hub,
            MapState::Dungeon => &self.dungeon,
        }
    }

    /// Returns the stair world position for the given state.
    pub fn stairs_for(&self, state: &MapState) -> Vec2 {
        match state {
            MapState::Hub     => self.hub_stairs,
            MapState::Dungeon => self.dungeon_stairs,
        }
    }
}

// ─── transition systems ──────────────────────────────────────────────────────

/// Detects when the player steps on a stair tile and requests a state change.
/// All actual map work (despawn / spawn / teleport) is handled by the
/// `OnExit` and `OnEnter` systems below, which Bevy runs between frames.
pub fn stair_detection(
    state:    Res<State<MapState>>,
    mut next: ResMut<NextState<MapState>>,
    world:    Res<World>,
    p_query:  Query<&Transform, With<Player>>,
) {
    let Ok(transform) = p_query.get_single() else { return };
    let centre = transform.translation.truncate();

    if world.current(state.get()).stairs_at(centre) {
        let new_state = match state.get() {
            MapState::Hub     => MapState::Dungeon,
            MapState::Dungeon => MapState::Hub,
        };
        next.set(new_state);
    }
}

/// Despawn all tile sprites when leaving any map state.
/// Registered on both `OnExit(MapState::Hub)` and `OnExit(MapState::Dungeon)`.
pub fn despawn_map(
    tile_query: Query<Entity, With<TileMarker>>,
    mut commands: Commands,
) {
    for e in &tile_query { commands.entity(e).despawn(); }
}

/// Spawn the hub map and teleport the player on entering Hub state.
pub fn on_enter_hub(
    world:       Res<World>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    p_query:      Query<Entity, With<Player>>,
) {
    spawn_map(&mut commands, &asset_server, &world.hub);
    if let Ok(player_e) = p_query.get_single() {
        let dest = world.hub_stairs;
        commands.entity(player_e).insert(
            Transform::from_xyz(dest.x, dest.y + TILE, 1.0).with_scale(Vec3::splat(SCALE))
        );
    }
}

/// Spawn the dungeon map and teleport the player on entering Dungeon state.
pub fn on_enter_dungeon(
    world:        Res<World>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    p_query:      Query<Entity, With<Player>>,
) {
    spawn_map(&mut commands, &asset_server, &world.dungeon);
    if let Ok(player_e) = p_query.get_single() {
        let dest = world.dungeon_stairs;
        commands.entity(player_e).insert(
            Transform::from_xyz(dest.x, dest.y + TILE, 1.0).with_scale(Vec3::splat(SCALE))
        );
    }
}