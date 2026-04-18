use bevy::prelude::*;
use rand::Rng;

mod map;
mod player;
mod state;

use map::{build_hub, build_dungeon, spawn_map};
use player::{Player, player_movement};
use state::{
    MapState, World,
    stair_detection, despawn_map, on_enter_hub, on_enter_dungeon,
};

// ─── constants ───────────────────────────────────────────────────────────────

pub const TILE_PX: f32 = 32.0;  // source PNG size in pixels
pub const SCALE:   f32 = 1.2;   // global sprite scale
pub const TILE:    f32 = TILE_PX * SCALE; // world-space tile size (38.4)

/// Player AABB half-extents. Exactly half a tile so the hitbox matches the
/// sprite edges. The 1px corner insets in collision probes prevent
/// false positives when a corner lands exactly on a tile boundary.
pub const HALF_W: f32 = TILE / 2.0;
pub const HALF_H: f32 = TILE / 2.0;

pub const SPEED: f32 = 150.0; // player movement speed in world units per second

// Dungeon dimensions chosen so the map fits exactly within the 1280×720 window
// at SCALE=1.2 (TILE=38.4px): floor(1280/38.4)=33 cols, floor(720/38.4)=18 rows.
// This means the entire dungeon is always visible without scrolling.
pub const DUNGEON_W: usize = 33;
pub const DUNGEON_H: usize = 18;

// ─── seed ────────────────────────────────────────────────────────────────────

/// Stores the seed used to generate the current dungeon.
/// Check the console at startup to see the seed, then set `FIXED_SEED`
/// to reproduce any run exactly.
#[derive(Resource)]
pub struct DungeonSeed(pub u64);

/// Set to `Some(seed)` to force a specific dungeon layout, or `None` to
/// generate a fresh random dungeon every run.
const FIXED_SEED: Option<u64> = None;

// ─── main ────────────────────────────────────────────────────────────────────

fn main() {
    let seed = FIXED_SEED.unwrap_or_else(|| rand::thread_rng().gen());
    println!("Dungeon seed: {seed}  (set FIXED_SEED = Some({seed}) to replay)");

    let (hub,     hub_stairs)     = build_hub();
    let (dungeon, dungeon_stairs) = build_dungeon(seed);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RustLike".into(),
                resolution: (1280.0, 720.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(World { hub, hub_stairs, dungeon, dungeon_stairs })
        .insert_resource(DungeonSeed(seed))
        .add_state::<MapState>()
        // Despawn the old map on leaving either state
        .add_systems(OnExit(MapState::Hub),      despawn_map)
        .add_systems(OnExit(MapState::Dungeon),  despawn_map)
        // Spawn the new map and teleport the player on entering either state.
        // OnEnter(Hub) is NOT called at startup — the initial hub is spawned
        // by `setup` so the player entity exists before any teleport.
        .add_systems(OnEnter(MapState::Hub),     on_enter_hub)
        .add_systems(OnEnter(MapState::Dungeon), on_enter_dungeon)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, stair_detection))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, world: Res<World>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn the initial hub map directly — OnEnter(Hub) does not fire at
    // startup because Hub is the default state, not a transition into it.
    spawn_map(&mut commands, &asset_server, &world.hub);

    // Spawn the player at hub centre tile (10, 7), away from the stair tile.
    let start = world.hub.tile_center(10, 7);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("warrior.png"),
            transform: Transform::from_xyz(start.x, start.y, 1.0)
                .with_scale(Vec3::splat(SCALE)),
            ..Default::default()
        },
        Player,
    ));
}