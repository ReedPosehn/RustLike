use bevy::prelude::*;
use crate::{TILE, HALF_W, SCALE};
use crate::player::Player;
use crate::enemies::Enemy;
use crate::combat::{Facing, Dead, DamageEvent, DamageSource};
use crate::state::{GameWorld, MapState};
use crate::character_select::{PlayerClass, AttackKind};

// ─── constants ───────────────────────────────────────────────────────────────

const PROJECTILE_SPEED:         f32 = 340.0;
const PROJECTILE_LIFETIME_SECS: f32 = 1.1;
/// Distance at which a projectile is considered to have hit an enemy.
const HIT_RADIUS: f32 = TILE * 0.4;

// ─── components ──────────────────────────────────────────────────────────────

/// A travelling ranged/magic attack fired by the player.
#[derive(Component)]
pub struct Projectile {
    direction: Vec2,
    damage:    i32,
    source:    DamageSource,
}

/// Despawns the projectile once its timer finishes, even if it never hits
/// anything — prevents projectiles from flying forever in open areas.
#[derive(Component)]
pub struct ProjectileLifetime(Timer);

// ─── fire ────────────────────────────────────────────────────────────────────

/// Space key — fires a ranged or magic projectile in the player's facing
/// direction. Sprite and `DamageSource` depend on the player's class.
pub fn fire_ranged_attack(
    keyboard:     Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    p_query:      Query<(&Transform, &Facing, &PlayerClass), With<Player>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) { return; }

    let Ok((transform, facing, class)) = p_query.get_single() else { return };
    if facing.0 == Vec2::ZERO { return; }

    let (sprite, source) = match class.attack_kind() {
        AttackKind::Magic  => ("magic_bolt.png", DamageSource::Magic),
        AttackKind::Ranged => ("arrow.png",      DamageSource::Ranged),
    };

    // Spawn just ahead of the player so the projectile doesn't immediately
    // overlap and despawn at point-blank range.
    let spawn_pos = transform.translation.truncate() + facing.0 * (HALF_W + 6.0);
    let angle     = facing.0.y.atan2(facing.0.x);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(sprite),
            transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 1.5)
                .with_scale(Vec3::splat(SCALE * 0.6))
                .with_rotation(Quat::from_rotation_z(angle)),
            ..Default::default()
        },
        Projectile {
            direction: facing.0,
            damage:    class.ranged_damage(),
            source,
        },
        ProjectileLifetime(Timer::from_seconds(PROJECTILE_LIFETIME_SECS, TimerMode::Once)),
    ));
}

// ─── update ──────────────────────────────────────────────────────────────────

/// Moves all projectiles, despawning them on wall hit, enemy hit, or
/// lifetime expiry. Combined into one system (rather than separate
/// movement/collision systems) so each projectile is despawned at most
/// once per frame.
pub fn update_projectiles(
    time:         Res<Time>,
    world:        Res<GameWorld>,
    state:        Res<State<MapState>>,
    mut commands: Commands,
    mut events:   EventWriter<DamageEvent>,
    mut p_query:  Query<(Entity, &mut Transform, &Projectile, &mut ProjectileLifetime)>,
    e_query:      Query<(Entity, &Transform), (With<Enemy>, Without<Dead>, Without<Projectile>)>,
) {
    let map = world.current(state.get());
    let dt  = time.delta_seconds();

    for (entity, mut transform, proj, mut lifetime) in &mut p_query {
        lifetime.0.tick(time.delta());
        if lifetime.0.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        let delta = proj.direction * PROJECTILE_SPEED * dt;
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
        let pos = transform.translation.truncate();

        // Wall collision
        if map.solid_at(pos) {
            commands.entity(entity).despawn();
            continue;
        }

        // Enemy collision — filtered by `Enemy` (not `EnemyMarker`) so this
        // never accidentally targets a health-bar sprite.
        let mut hit = false;
        for (e_entity, e_transform) in &e_query {
            if pos.distance(e_transform.translation.truncate()) < HIT_RADIUS {
                events.send(DamageEvent { target: e_entity, amount: proj.damage, source: proj.source });
                hit = true;
                break;
            }
        }
        if hit {
            commands.entity(entity).despawn();
        }
    }
}

// ─── cleanup ─────────────────────────────────────────────────────────────────

/// Despawns any in-flight projectiles when leaving the dungeon — covers both
/// walking onto the stairs and dying mid-flight (death stops `update_projectiles`
/// from running, since it requires `AppState::Playing`, so without this the
/// projectile would otherwise sit frozen on screen forever).
pub fn despawn_projectiles(
    query:        Query<Entity, With<Projectile>>,
    mut commands: Commands,
) {
    for e in &query { commands.entity(e).despawn(); }
}