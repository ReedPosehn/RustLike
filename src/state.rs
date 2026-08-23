use bevy::prelude::*;
use crate::{TILE, SCALE};
use crate::map::{Tilemap, TileMarker, RoomInfo, spawn_map};
use crate::player::Player;

// ─── app-level state ─────────────────────────────────────────────────────────

/// Top-level application state. Gameplay only runs in `Playing`.
/// `GameOver` overlays the screen and pauses all gameplay systems.
/// `CharacterSelect` is the initial state shown on startup.
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    CharacterSelect,
    DifficultySelect,
    Playing,
    Paused,
    InventoryView,
    GameOver,
}

// ─── map state ───────────────────────────────────────────────────────────────

/// Which map is currently active.
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapState {
    #[default]
    Hub,
    Dungeon,
}

// ─── death record ────────────────────────────────────────────────────────────

/// Persists across the game-over screen so the new dungeon can spawn
/// a gravestone, and so the death screen can show gold lost.
#[derive(Resource, Default)]
pub struct LastDeathInfo {
    pub gold_lost:   u32,
    /// Which dungeon seed the player died on (shown on gravestone someday).
    pub seed:        u64,
}

/// Tags the gravestone sprite so it despawns with the dungeon on exit.
#[derive(Component)]
pub struct GravestoneMarker;

// ─── world resource ──────────────────────────────────────────────────────────

/// Holds both maps, their stair positions, and dungeon room data.
/// Does not store the active state — that is owned by Bevy's `State<MapState>`.
#[derive(Resource)]
pub struct GameWorld {
    pub hub:            Tilemap,
    pub hub_stairs:     Vec2,
    pub dungeon:        Tilemap,
    pub dungeon_stairs: Vec2,
    /// Centre world position of each dungeon room, in generation order.
    /// The last entry is the stair room. Used by the enemy spawner.
    pub dungeon_rooms:  Vec<RoomInfo>,
}

impl GameWorld {
    /// Returns the map for the given state.
    pub fn current(&self, state: &MapState) -> &Tilemap {
        match state {
            MapState::Hub     => &self.hub,
            MapState::Dungeon => &self.dungeon,
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
    world:    Res<GameWorld>,
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

/// Despawn all tile sprites and gravestones when leaving any map state.
pub fn despawn_map(
    tile_query:  Query<Entity, With<TileMarker>>,
    grave_query: Query<Entity, With<GravestoneMarker>>,
    mut commands: Commands,
) {
    for e in &tile_query  { commands.entity(e).despawn(); }
    for e in &grave_query { commands.entity(e).despawn(); }
}

/// Spawn the hub map and teleport the player on entering Hub state.
pub fn on_enter_hub(
    world:        Res<GameWorld>,
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

/// Spawn the dungeon map, teleport the player, and place a gravestone in the
/// first room if the player died on a previous run.
pub fn on_enter_dungeon(
    world:        Res<GameWorld>,
    death_info:   Res<LastDeathInfo>,
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

    // Spawn gravestone in the first room if the player has died before.
    if death_info.seed > 0 {
        if let Some(first_room) = world.dungeon_rooms.first() {
            let pos = first_room.centre;
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("gravestone.png"),
                    transform: Transform::from_xyz(pos.x, pos.y, 1.0)
                        .with_scale(Vec3::splat(SCALE)),
                    ..Default::default()
                },
                GravestoneMarker,
            ));
        }
    }
}