use bevy::prelude::*;
use crate::{TILE, HALF_W, HALF_H};
use crate::player::Player;
use crate::enemies::Enemy;
use crate::inventory::spawn_gold_pickup;

// ─── constants ───────────────────────────────────────────────────────────────

/// Seconds between contact damage ticks from enemies touching the player.
const CONTACT_DAMAGE_INTERVAL: f32 = 0.8;
/// Damage dealt per enemy contact tick.
const ENEMY_CONTACT_DAMAGE: i32 = 10;

// ─── health ──────────────────────────────────────────────────────────────────

/// Health component — attached to the player and all enemies.
#[derive(Component, Debug)]
pub struct Health {
    pub current: i32,
    pub max:     i32,
}

impl Health {
    pub fn new(max: i32) -> Self { Health { current: max, max } }
    pub fn is_dead(&self) -> bool { self.current <= 0 }
}

/// Marker added to an entity the frame it reaches 0 HP.
/// Systems downstream (death FX, game-over, despawn) query for this.
#[derive(Component)]
pub struct Dead;

// ─── damage event ─────────────────────────────────────────────────────────────

/// All damage in the game flows through this event.
/// Any system — melee, ranged, magic, AoE, traps — fires a `DamageEvent`
/// rather than modifying `Health` directly. This keeps combat logic decoupled
/// and makes it easy to add new damage mechanics.
#[derive(Event, Debug)]
pub struct DamageEvent {
    /// The entity receiving damage.
    pub target: Entity,
    pub amount: i32,
    #[allow(dead_code)] // kept for future damage-type filtering and logging
    pub source: DamageSource,
}

/// How the damage was dealt. Extend this enum to add new mechanics.
/// Variants beyond `Melee` and `Contact` are stubs for future attack types.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum DamageSource {
    Melee,
    Ranged,
    Magic,
    Contact,
    AreaOfEffect,
}

// ─── facing ───────────────────────────────────────────────────────────────────

/// Tracks the direction the player last moved — used for melee targeting.
#[derive(Component, Default)]
pub struct Facing(pub Vec2);

// ─── contact damage timer ─────────────────────────────────────────────────────

/// Per-player timer that limits how often contact damage can fire.
#[derive(Component)]
pub struct ContactDamageTimer(pub Timer);

impl Default for ContactDamageTimer {
    fn default() -> Self {
        ContactDamageTimer(Timer::from_seconds(CONTACT_DAMAGE_INTERVAL, TimerMode::Repeating))
    }
}

/// Fired by `apply_damage` when damage lands successfully.
/// `hud.rs` listens for this to spawn floating damage numbers.
#[derive(Event)]
pub struct SplatEvent {
    pub position: Vec2,
    pub amount:   i32,
}

// ─── systems ──────────────────────────────────────────────────────────────────

/// Reads all `DamageEvent`s and applies them to `Health` components.
/// Marks entities with 0 HP as `Dead` and fires a `SplatEvent` for visual feedback.
pub fn apply_damage(
    mut events:    EventReader<DamageEvent>,
    mut q_health:  Query<(&mut Health, &GlobalTransform)>,
    mut splats:    EventWriter<SplatEvent>,
    mut commands:  Commands,
) {
    for ev in events.iter() {
        if let Ok((mut health, gtransform)) = q_health.get_mut(ev.target) {
            if health.is_dead() { continue; }
            health.current = (health.current - ev.amount).max(0);

            // Fire a splat at the target's world position.
            let pos = gtransform.translation().truncate();
            splats.send(SplatEvent { position: pos, amount: ev.amount });

            if health.is_dead() {
                commands.entity(ev.target).insert(Dead);
            }
        }
    }
}

/// Despawns any entity tagged `Dead` on the enemy side, along with its
/// associated health bar sprites (which are separate entities tracked via
/// `EnemyBarFor` — they don't get `Dead` themselves, so without this they'd
/// be left behind, frozen at the enemy's last position). Also drops a gold
/// pickup at the enemy's position before it's removed.
/// Player death is handled separately (game-over screen).
pub fn despawn_dead_enemies(
    dead_query:   Query<(Entity, &Transform, &Enemy), (With<Dead>, With<Enemy>)>,
    bar_query:    Query<(Entity, &crate::hud::EnemyBarFor)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for (dead_entity, transform, enemy) in &dead_query {
        spawn_gold_pickup(
            &mut commands,
            &asset_server,
            transform.translation.truncate(),
            enemy.kind.gold_drop(),
        );

        commands.entity(dead_entity).despawn();
        for (bar_entity, bar_for) in &bar_query {
            if bar_for.0 == dead_entity {
                commands.entity(bar_entity).despawn();
            }
        }
    }
}

/// Updates the player's `Facing` component from the last movement direction.
pub fn update_facing(
    keyboard: Res<Input<KeyCode>>,
    mut q:    Query<&mut Facing, With<Player>>,
) {
    let mut dir = Vec2::ZERO;
    if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up)    { dir.y += 1.0; }
    if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down)  { dir.y -= 1.0; }
    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left)  { dir.x -= 1.0; }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) { dir.x += 1.0; }

    if dir == Vec2::ZERO { return; }
    let dir = dir.normalize();
    for mut facing in &mut q {
        facing.0 = dir;
    }
}

/// Player presses F — hits the tile directly ahead in the facing direction.
/// Any enemy whose centre falls within that tile receives melee damage.
pub fn player_melee_attack(
    keyboard:   Res<Input<KeyCode>>,
    p_query:    Query<(&Transform, &Facing, &crate::character_select::PlayerClass), With<Player>>,
    e_query:    Query<(Entity, &Transform), (With<Enemy>, Without<Dead>)>,
    mut events: EventWriter<DamageEvent>,
) {
    if !keyboard.just_pressed(KeyCode::F) { return; }

    let Ok((p_transform, facing, class)) = p_query.get_single() else { return };
    if facing.0 == Vec2::ZERO { return; }

    let player_pos    = p_transform.translation.truncate();
    let target_centre = player_pos + facing.0 * TILE;

    for (entity, e_transform) in &e_query {
        let enemy_pos = e_transform.translation.truncate();
        if enemy_pos.distance(target_centre) < TILE {
            events.send(DamageEvent {
                target: entity,
                amount: class.melee_damage(),
                source: DamageSource::Melee,
            });
        }
    }
}

/// Enemies deal contact damage to the player when their AABBs overlap.
/// Fires at most once per `CONTACT_DAMAGE_INTERVAL` seconds to avoid
/// instant kills on contact.
pub fn enemy_contact_damage(
    time:       Res<Time>,
    e_query:    Query<&Transform, (With<Enemy>, Without<Dead>)>,
    mut p_query: Query<(Entity, &Transform, &mut ContactDamageTimer), With<Player>>,
    mut events: EventWriter<DamageEvent>,
) {
    let Ok((player_entity, p_transform, mut timer)) = p_query.get_single_mut() else { return };

    timer.0.tick(time.delta());
    if !timer.0.just_finished() { return; }

    let pp = p_transform.translation.truncate();

    for e_transform in &e_query {
        let ep = e_transform.translation.truncate();
        // Simple AABB overlap check.
        let overlap_x = (pp.x - ep.x).abs() < HALF_W * 2.0;
        let overlap_y = (pp.y - ep.y).abs() < HALF_H * 2.0;
        if overlap_x && overlap_y {
            events.send(DamageEvent {
                target: player_entity,
                amount: ENEMY_CONTACT_DAMAGE,
                source: DamageSource::Contact,
            });
            // Only one contact damage tick per interval even if multiple enemies overlap.
            break;
        }
    }
}