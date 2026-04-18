use bevy::prelude::*;
use crate::{SPEED, HALF_W, HALF_H};
use crate::state::{World, MapState};

/// Tags the player entity.
#[derive(Component)]
pub struct Player;

/// AABB collision — Anchor::Center means translation = sprite centre.
/// Player hitbox: centre ± (HALF_W, HALF_H).
/// X and Y axes resolve independently so the player slides along walls.
/// Two corners are tested on each leading face; the 1px inset prevents
/// false positives when a corner lands exactly on a tile boundary.
pub fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    time:     Res<Time>,
    world:    Res<World>,
    state:    Res<State<MapState>>,
    mut q:    Query<&mut Transform, With<Player>>,
) {
    let mut dir = Vec2::ZERO;
    if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up)    { dir.y += 1.0; }
    if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down)  { dir.y -= 1.0; }
    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left)  { dir.x -= 1.0; }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) { dir.x += 1.0; }
    if dir == Vec2::ZERO { return; }

    let map = world.current(state.get());
    let dt  = time.delta_seconds();

    for mut t in &mut q {
        let px = t.translation.x;
        let py = t.translation.y;

        // X axis
        let dx = dir.x * SPEED * dt;
        if dx != 0.0 {
            let nx     = px + dx;
            let face_x = if dx > 0.0 { nx + HALF_W } else { nx - HALF_W };
            let blocked = map.solid_at(Vec2::new(face_x, py - HALF_H + 1.0))
                       || map.solid_at(Vec2::new(face_x, py + HALF_H - 1.0));
            if !blocked { t.translation.x = nx; }
        }

        // Y axis — use the updated x so diagonal corners are handled correctly
        let px = t.translation.x;
        let dy = dir.y * SPEED * dt;
        if dy != 0.0 {
            let ny     = py + dy;
            let face_y = if dy > 0.0 { ny + HALF_H } else { ny - HALF_H };
            let blocked = map.solid_at(Vec2::new(px - HALF_W + 1.0, face_y))
                       || map.solid_at(Vec2::new(px + HALF_W - 1.0, face_y));
            if !blocked { t.translation.y = ny; }
        }
    }
}