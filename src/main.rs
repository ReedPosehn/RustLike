use bevy::prelude::*;
use rand::Rng;

mod character_select;
mod combat;
mod difficulty;
mod enemies;
mod game_over;
mod hud;
mod inventory;
mod map;
mod pause;
mod player;
mod projectile;
mod state;

use character_select::{
    cleanup_character_select, handle_class_input, setup_character_select, PlayerClass,
    SelectedClass,
};
use combat::{
    apply_damage, despawn_dead_enemies, enemy_contact_damage, player_melee_attack, update_facing,
    DamageEvent, SplatEvent,
};
use difficulty::{
    cleanup_difficulty_select, handle_difficulty_input, setup_difficulty_select, Difficulty,
    SelectedDifficultyIndex,
};
use enemies::{despawn_enemies, enemy_ai, spawn_enemies};
use game_over::{check_player_death, cleanup_game_over, handle_respawn_input, setup_game_over};
use hud::{
    handle_splat_events, setup_hud, sync_enemy_bars, update_damage_splats, update_gold_display,
    update_health_bar,
};
use inventory::{
    cleanup_inventory_view, collect_gold, collect_potions, despawn_pickups, handle_inventory_input,
    setup_inventory_view, toggle_inventory_on_key, use_health_potion,
};
use map::{build_dungeon, build_hub, spawn_map};
use pause::{
    cleanup_pause_menu, handle_pause_input, handle_pause_options_input, setup_pause_menu,
    toggle_pause_on_escape,
};
use player::{player_movement, spawn_player};
use projectile::{despawn_projectiles, fire_ranged_attack, update_projectiles};
use state::{
    despawn_map, on_enter_dungeon, on_enter_hub, stair_detection, AppState, GameWorld,
    LastDeathInfo, MapState,
};

// ─── constants ───────────────────────────────────────────────────────────────

pub const TILE_PX: f32 = 32.0;
pub const SCALE: f32 = 1.2;
pub const TILE: f32 = TILE_PX * SCALE; // 38.4

pub const HALF_W: f32 = TILE / 2.0;
pub const HALF_H: f32 = TILE / 2.0;

/// Base movement speed used by enemies. Player speed is driven by `PlayerClass::speed()`.
pub const ENEMY_SPEED: f32 = 150.0;

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

    let (hub, hub_stairs) = build_hub();
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
        .insert_resource(GameWorld {
            hub,
            hub_stairs,
            dungeon,
            dungeon_stairs,
            dungeon_rooms,
        })
        .insert_resource(DungeonSeed(seed))
        .insert_resource(LastDeathInfo::default())
        .insert_resource(PlayerClass::default())
        .insert_resource(SelectedClass::default())
        .insert_resource(Difficulty::default())
        .insert_resource(SelectedDifficultyIndex::default())
        .add_event::<DamageEvent>()
        .add_event::<SplatEvent>()
        // States
        .add_state::<AppState>()
        .add_state::<MapState>()
        // ── Character select ────────────────────────────────────────────────
        .add_systems(OnEnter(AppState::CharacterSelect), setup_character_select)
        .add_systems(OnExit(AppState::CharacterSelect), cleanup_character_select)
        .add_systems(
            Update,
            handle_class_input.run_if(in_state(AppState::CharacterSelect)),
        )
        // ── Difficulty select ───────────────────────────────────────────────
        .add_systems(OnEnter(AppState::DifficultySelect), setup_difficulty_select)
        .add_systems(
            OnExit(AppState::DifficultySelect),
            cleanup_difficulty_select,
        )
        .add_systems(
            Update,
            handle_difficulty_input.run_if(in_state(AppState::DifficultySelect)),
        )
        // ── Map lifecycle ───────────────────────────────────────────────────
        .add_systems(OnExit(MapState::Hub), despawn_map)
        .add_systems(OnExit(MapState::Dungeon), despawn_map)
        .add_systems(OnEnter(MapState::Hub), on_enter_hub)
        .add_systems(
            OnEnter(MapState::Dungeon),
            (on_enter_dungeon, spawn_enemies),
        )
        // ── Enemy lifecycle ─────────────────────────────────────────────────
        .add_systems(OnExit(MapState::Dungeon), despawn_enemies)
        .add_systems(OnExit(MapState::Dungeon), despawn_projectiles)
        .add_systems(OnExit(MapState::Dungeon), despawn_pickups)
        // ── Game over lifecycle ─────────────────────────────────────────────
        .add_systems(OnEnter(AppState::GameOver), setup_game_over)
        .add_systems(OnExit(AppState::GameOver), cleanup_game_over)
        // ── Pause lifecycle ─────────────────────────────────────────────────
        .add_systems(OnEnter(AppState::Paused), setup_pause_menu)
        .add_systems(OnExit(AppState::Paused), cleanup_pause_menu)
        .add_systems(
            Update,
            toggle_pause_on_escape.run_if(in_state(AppState::Playing)),
        )
        .add_systems(
            Update,
            handle_pause_input.run_if(in_state(AppState::Paused)),
        )
        .add_systems(
            Update,
            handle_pause_options_input.run_if(in_state(AppState::Paused)),
        )
        // ── Inventory view lifecycle ────────────────────────────────────────
        .add_systems(OnEnter(AppState::InventoryView), setup_inventory_view)
        .add_systems(OnExit(AppState::InventoryView), cleanup_inventory_view)
        .add_systems(
            Update,
            toggle_inventory_on_key.run_if(in_state(AppState::Playing)),
        )
        .add_systems(
            Update,
            handle_inventory_input.run_if(in_state(AppState::InventoryView)),
        )
        // ── Player spawn (after class is chosen) ───────────────────────────
        .add_systems(OnEnter(AppState::Playing), spawn_player)
        // ── Startup (camera + HUD only; no player yet) ──────────────────────
        .add_systems(Startup, (setup_camera, setup_hud, setup_hub_map))
        // ── Update — only while playing ─────────────────────────────────────
        .add_systems(
            Update,
            (
                player_movement,
                stair_detection,
                update_facing,
                apply_damage,
                check_player_death,
                update_health_bar,
                update_gold_display,
                handle_splat_events,
                update_damage_splats,
                sync_enemy_bars,
                use_health_potion,
            )
                .run_if(in_state(AppState::Playing)),
        )
        // ── Update — dungeon only (and only while actually playing, not on
        //    the game-over screen — dying doesn't change MapState, only
        //    AppState, so both conditions are required) ───────────────────────
        .add_systems(
            Update,
            (
                enemy_ai,
                player_melee_attack,
                enemy_contact_damage,
                despawn_dead_enemies,
                collect_gold,
                collect_potions,
                fire_ranged_attack,
                update_projectiles,
            )
                .run_if(in_state(MapState::Dungeon).and_then(in_state(AppState::Playing))),
        )
        // ── Update — game over ───────────────────────────────────────────────
        .add_systems(
            Update,
            handle_respawn_input.run_if(in_state(AppState::GameOver)),
        )
        .run();
}

/// Spawn the camera and nothing else — player spawns after class selection.
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Spawn the initial hub map at startup so it's visible behind the class
/// select screen.
fn setup_hub_map(mut commands: Commands, asset_server: Res<AssetServer>, world: Res<GameWorld>) {
    spawn_map(&mut commands, &asset_server, &world.hub);
}
