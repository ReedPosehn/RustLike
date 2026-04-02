use bevy::prelude::*;

mod map;
mod player;
mod state;

use map::{build_hub, build_dungeon, spawn_map};
use player::{Player, player_movement};
use state::{World, StairCooldown, stair_transition};

// ─── constants ───────────────────────────────────────────────────────────────

pub const TILE_PX: f32 = 32.0;  // source PNG size in pixels
pub const SCALE:   f32 = 1.2;   // global sprite scale
pub const TILE:    f32 = TILE_PX * SCALE; // world-space tile size (38.4)

/// Player AABB half-extents. Exactly half a tile so the hitbox matches the
/// sprite edges. The 1px corner insets in collision probes prevent
/// false positives on exact tile boundaries.
pub const HALF_W: f32 = TILE / 2.0;
pub const HALF_H: f32 = TILE / 2.0;

pub const SPEED: f32 = 150.0; // player movement speed in world units per second

pub const DUNGEON_W: usize = 40;
pub const DUNGEON_H: usize = 30;

// ─── main ────────────────────────────────────────────────────────────────────

fn main() {
    let (hub,     hub_stairs)     = build_hub();
    let (dungeon, dungeon_stairs) = build_dungeon();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RustLike".into(),
                resolution: (1280.0, 720.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(World {
            state: state::MapState::Hub,
            hub,
            hub_stairs,
            dungeon,
            dungeon_stairs,
        })
        .insert_resource(StairCooldown(0))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, stair_transition))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, world: Res<World>) {
    commands.spawn(Camera2dBundle::default());
    spawn_map(&mut commands, &asset_server, world.current());

    // Spawn the player at the hub centre tile (10, 7), away from the stair tile.
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