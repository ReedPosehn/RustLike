use bevy::prelude::*;
use crate::{HALF_W, HALF_H};
use crate::state::{GameWorld, MapState};
use crate::combat::{Facing, Health, ContactDamageTimer};
use crate::inventory::{Gold, Inventory};
use crate::character_select::PlayerClass;

/// Tags the player entity.
#[derive(Component)]
pub struct Player;

/// Spawns the player using stats from the chosen `PlayerClass`.
/// Called by `OnEnter(AppState::Playing)`.
/// Guards against re-entry: if a player entity already exists (e.g. returning
/// from GameOver) we skip spawning to avoid duplicate entities.
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    world:        Res<GameWorld>,
    class:        Res<PlayerClass>,
    existing:     Query<(), With<Player>>,
) {
    // Already exists — respawn path restores the existing entity directly.
    if !existing.is_empty() { return; }
    use crate::SCALE;
    let start = world.hub.tile_center(10, 7);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(class.sprite()),
            transform: Transform::from_xyz(start.x, start.y, 1.0)
                .with_scale(Vec3::splat(SCALE)),
            ..Default::default()
        },
        Player,
        Health::new(class.max_hp()),
        Facing::default(),
        ContactDamageTimer::default(),
        Gold::default(),
        Inventory::default(),
        *class, // store chosen class on the entity for stat lookups
    ));
}

/// AABB movement — reads speed from the player's `PlayerClass` component.
pub fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    time:     Res<Time>,
    world:    Res<GameWorld>,
    state:    Res<State<MapState>>,
    mut q:    Query<(&mut Transform, &PlayerClass), With<Player>>,
) {
    let mut dir = Vec2::ZERO;
    if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up)    { dir.y += 1.0; }
    if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down)  { dir.y -= 1.0; }
    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left)  { dir.x -= 1.0; }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) { dir.x += 1.0; }
    if dir == Vec2::ZERO { return; }

    let map = world.current(state.get());
    let dt  = time.delta_seconds();

    for (mut t, class) in &mut q {
        let speed = class.speed();
        let px = t.translation.x;
        let py = t.translation.y;

        let dx = dir.x * speed * dt;
        if dx != 0.0 {
            let nx     = px + dx;
            let face_x = if dx > 0.0 { nx + HALF_W } else { nx - HALF_W };
            let blocked = map.solid_at(Vec2::new(face_x, py - HALF_H + 1.0))
                       || map.solid_at(Vec2::new(face_x, py + HALF_H - 1.0));
            if !blocked { t.translation.x = nx; }
        }

        let px = t.translation.x;
        let dy = dir.y * speed * dt;
        if dy != 0.0 {
            let ny     = py + dy;
            let face_y = if dy > 0.0 { ny + HALF_H } else { ny - HALF_H };
            let blocked = map.solid_at(Vec2::new(px - HALF_W + 1.0, face_y))
                       || map.solid_at(Vec2::new(px + HALF_W - 1.0, face_y));
            if !blocked { t.translation.y = ny; }
        }
    }
}