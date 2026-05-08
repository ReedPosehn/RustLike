use bevy::prelude::*;
use rand::Rng;

mod map;
mod player;
mod state;
mod enemies;
mod combat;
mod hud;

use map::{build_hub, build_dungeon, spawn_map};
use player::{spawn_player, player_movement};
use state::{
    MapState, GameWorld,
    stair_detection, despawn_map, on_enter_hub, on_enter_dungeon,
};
use enemies::{spawn_enemies, despawn_enemies, enemy_ai};
use combat::{
    DamageEvent, SplatEvent,
    apply_damage, despawn_dead_enemies,
    update_facing, player_melee_attack,
    enemy_contact_damage,
};
use hud::{setup_hud, update_health_bar, sync_enemy_bars, handle_splat_events, update_damage_splats};

// ─── constants ───────────────────────────────────────────────────────────────

pub const TILE_PX: f32 = 32.0;
pub const SCALE:   f32 = 1.2;
pub const TILE:    f32 = TILE_PX * SCALE; // 38.4

pub const HALF_W: f32 = TILE / 2.0;
pub const HALF_H: f32 = TILE / 2.0;

pub const SPEED: f32 = 150.0;

// Dungeon sized to fit exactly in 1280×720 at TILE=38.4.
pub const DUNGEON_W: usize = 33;
pub const DUNGEON_H: usize = 18;

// ─── seed ────────────────────────────────────────────────────────────────────

#[derive(Resource)]
pub struct DungeonSeed(pub u64);

const FIXED_SEED: Option<u64> = None;

// ─── main ────────────────────────────────────────────────────────────────────

fn main() {
    let seed = FIXED_SEED.unwrap_or_else(|| rand::thread_rng().gen());
    println!("Dungeon seed: {seed}  (set FIXED_SEED = Some({seed}) to replay)");

    let (hub, hub_stairs)                       = build_hub();
    let (dungeon, dungeon_stairs, dungeon_rooms) = build_dungeon(seed);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RustLike".into(),
                resolution: (1280.0, 720.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(GameWorld { hub, hub_stairs, dungeon, dungeon_stairs, dungeon_rooms })
        .insert_resource(DungeonSeed(seed))
        .add_event::<DamageEvent>()
        .add_event::<SplatEvent>()
        .add_state::<MapState>()
        // Map lifecycle
        .add_systems(OnExit(MapState::Hub),      despawn_map)
        .add_systems(OnExit(MapState::Dungeon),  despawn_map)
        .add_systems(OnEnter(MapState::Hub),     on_enter_hub)
        .add_systems(OnEnter(MapState::Dungeon), (on_enter_dungeon, spawn_enemies))
        // Enemy lifecycle
        .add_systems(OnExit(MapState::Dungeon),  despawn_enemies)
        // Startup
        .add_systems(Startup, (setup, setup_hud))
        // Update — always active
        .add_systems(Update, (
            player_movement,
            stair_detection,
            update_facing,
            apply_damage,
            update_health_bar,
            handle_splat_events,
            update_damage_splats,
            sync_enemy_bars,
        ))
        // Update — dungeon only
        .add_systems(Update, (
            enemy_ai,
            player_melee_attack,
            enemy_contact_damage,
            despawn_dead_enemies,
        ).run_if(in_state(MapState::Dungeon)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, world: Res<GameWorld>) {
    commands.spawn(Camera2dBundle::default());
    spawn_map(&mut commands, &asset_server, &world.hub);

    let start = world.hub.tile_center(10, 7);
    spawn_player(&mut commands, &asset_server, start);
}