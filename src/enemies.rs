use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use crate::{SCALE, TILE, HALF_W, HALF_H, DungeonSeed};
use crate::state::{World, MapState};
use crate::player::Player;

// ─── constants ───────────────────────────────────────────────────────────────

/// World-units per second for wandering movement.
const WANDER_SPEED: f32 = 45.0;
/// World-units per second when chasing the player.
const CHASE_SPEED:  f32 = 90.0;
/// Distance at which an enemy notices the player and starts chasing.
const CHASE_RADIUS: f32 = TILE * 5.0;
/// Distance at which a chasing enemy gives up and goes back to wandering.
/// Larger than CHASE_RADIUS to prevent jitter at the boundary.
const LOSE_RADIUS:  f32 = TILE * 7.0;
/// How long (seconds) an enemy walks in one wander direction before
/// choosing a new one.
const WANDER_WALK_SECS: f32 = 1.2;
/// How long (seconds) an enemy pauses between wander steps.
const WANDER_PAUSE_SECS: f32 = 0.6;

// ─── enemy types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyKind {
    Goblin,
    Orc,
    Skeleton,
}

impl EnemyKind {
    fn asset(self) -> &'static str {
        match self {
            EnemyKind::Goblin   => "goblin.png",
            EnemyKind::Orc      => "orc.png",
            EnemyKind::Skeleton => "skeleton.png",
        }
    }
}

// ─── AI state ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum AiMode {
    /// Walking in a random direction for `remaining` seconds.
    Walking { dir: Vec2, remaining: f32 },
    /// Standing still for `remaining` seconds before picking a new direction.
    Pausing { remaining: f32 },
    /// Actively chasing the player.
    Chasing,
}

// ─── ECS components ──────────────────────────────────────────────────────────

/// Tags all enemy entities so they can be bulk-despawned on map transitions.
#[derive(Component)]
pub struct EnemyMarker;

/// Data component on each enemy entity.
#[derive(Component)]
pub struct Enemy {
    pub kind: EnemyKind,
}

/// AI state machine component — drives wander/chase behaviour.
#[derive(Component)]
pub struct EnemyAi {
    mode: AiMode,
    /// Seeded per-enemy RNG so each enemy wanders independently.
    rng:  StdRng,
}

impl EnemyAi {
    fn new(seed: u64) -> Self {
        EnemyAi {
            mode: AiMode::Pausing { remaining: 0.0 },
            rng:  StdRng::seed_from_u64(seed),
        }
    }

    /// Pick a fresh random unit direction for wandering.
    fn random_dir(&mut self) -> Vec2 {
        // Choose from the 8 cardinal + diagonal directions for tile-friendly movement.
        let dirs = [
            Vec2::new( 1.0,  0.0), Vec2::new(-1.0,  0.0),
            Vec2::new( 0.0,  1.0), Vec2::new( 0.0, -1.0),
            Vec2::new( 1.0,  1.0).normalize(), Vec2::new(-1.0,  1.0).normalize(),
            Vec2::new( 1.0, -1.0).normalize(), Vec2::new(-1.0, -1.0).normalize(),
        ];
        dirs[self.rng.gen_range(0..dirs.len())]
    }
}

// ─── spawn / despawn systems ─────────────────────────────────────────────────

/// Spawns enemies in dungeon rooms when entering the Dungeon state.
pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    world:        Res<World>,
    seed:         Res<DungeonSeed>,
) {
    // Offset the seed so enemy placement RNG is independent from map gen.
    let mut rng = StdRng::seed_from_u64(seed.0.wrapping_add(1));

    let room_centres = &world.dungeon_rooms;
    let kinds = [EnemyKind::Goblin, EnemyKind::Orc, EnemyKind::Skeleton];

    // Skip the last room — reserved for stairs.
    let spawn_rooms = if room_centres.len() > 1 {
        &room_centres[..room_centres.len() - 1]
    } else {
        return;
    };

    for room in spawn_rooms {
        let centre = room.centre;
        let count  = rng.gen_range(1..=2usize);
        for i in 0..count {
            let kind = kinds[rng.gen_range(0..kinds.len())];
            let x_offset = if i == 0 { -TILE * 0.5 } else { TILE * 0.5 };

            // Each enemy gets a unique AI seed derived from the dungeon seed
            // and its spawn index so they wander independently.
            let ai_seed = seed.0.wrapping_add(rng.gen::<u64>());

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(kind.asset()),
                    transform: Transform::from_xyz(
                        centre.x + x_offset,
                        centre.y,
                        1.0,
                    ).with_scale(Vec3::splat(SCALE)),
                    ..Default::default()
                },
                Enemy { kind },
                EnemyAi::new(ai_seed),
                EnemyMarker,
            ));
        }
    }
}

/// Despawn all enemy entities when leaving the Dungeon state.
pub fn despawn_enemies(
    query:        Query<Entity, With<EnemyMarker>>,
    mut commands: Commands,
) {
    for e in &query { commands.entity(e).despawn(); }
}

// ─── AI system ───────────────────────────────────────────────────────────────

/// Drives enemy wander/chase behaviour. Only runs in the Dungeon state.
/// Uses the same AABB leading-face collision probes as the player.
pub fn enemy_ai(
    time:       Res<Time>,
    world:      Res<World>,
    p_query:    Query<&Transform, With<Player>>,
    mut e_query: Query<(&mut Transform, &mut EnemyAi), Without<Player>>,
) {
    let map = world.current(&MapState::Dungeon);
    let dt  = time.delta_seconds();

    let player_pos = p_query
        .get_single()
        .map(|t| t.translation.truncate())
        .unwrap_or(Vec2::ZERO);

    for (mut transform, mut ai) in &mut e_query {
        let pos  = transform.translation.truncate();
        let dist = pos.distance(player_pos);

        // ── state transitions ────────────────────────────────────────────────
        match &ai.mode {
            AiMode::Chasing => {
                if dist > LOSE_RADIUS {
                    ai.mode = AiMode::Pausing { remaining: WANDER_PAUSE_SECS };
                }
            }
            _ => {
                if dist < CHASE_RADIUS {
                    ai.mode = AiMode::Chasing;
                }
            }
        }

        // ── movement ─────────────────────────────────────────────────────────
        let dir = match &mut ai.mode {
            AiMode::Chasing => {
                // Move straight toward the player.
                let to_player = player_pos - pos;
                if to_player.length() > 1.0 { to_player.normalize() } else { Vec2::ZERO }
            }

            AiMode::Walking { dir, remaining } => {
                *remaining -= dt;
                if *remaining <= 0.0 {
                    ai.mode = AiMode::Pausing { remaining: WANDER_PAUSE_SECS };
                    Vec2::ZERO
                } else {
                    *dir
                }
            }

            AiMode::Pausing { remaining } => {
                *remaining -= dt;
                if *remaining <= 0.0 {
                    let dir = ai.rng.gen_range(0..8); // pick via rng before borrowing ai.mode
                    let new_dir = {
                        let dirs = [
                            Vec2::new( 1.0,  0.0), Vec2::new(-1.0,  0.0),
                            Vec2::new( 0.0,  1.0), Vec2::new( 0.0, -1.0),
                            Vec2::new( 1.0,  1.0).normalize(),
                            Vec2::new(-1.0,  1.0).normalize(),
                            Vec2::new( 1.0, -1.0).normalize(),
                            Vec2::new(-1.0, -1.0).normalize(),
                        ];
                        dirs[dir]
                    };
                    ai.mode = AiMode::Walking {
                        dir:       new_dir,
                        remaining: WANDER_WALK_SECS,
                    };
                    new_dir
                } else {
                    Vec2::ZERO
                }
            }
        };

        if dir == Vec2::ZERO { continue; }

        let speed = match &ai.mode {
            AiMode::Chasing => CHASE_SPEED,
            _               => WANDER_SPEED,
        };

        let px = transform.translation.x;
        let py = transform.translation.y;

        // X axis — same leading-face AABB probe as player_movement.
        let dx = dir.x * speed * dt;
        if dx != 0.0 {
            let nx     = px + dx;
            let face_x = if dx > 0.0 { nx + HALF_W } else { nx - HALF_W };
            let blocked = map.solid_at(Vec2::new(face_x, py - HALF_H + 1.0))
                       || map.solid_at(Vec2::new(face_x, py + HALF_H - 1.0));
            if !blocked { transform.translation.x = nx; }
        }

        // Y axis — use updated x for correct diagonal corner handling.
        let px = transform.translation.x;
        let dy = dir.y * speed * dt;
        if dy != 0.0 {
            let ny     = py + dy;
            let face_y = if dy > 0.0 { ny + HALF_H } else { ny - HALF_H };
            let blocked = map.solid_at(Vec2::new(px - HALF_W + 1.0, face_y))
                       || map.solid_at(Vec2::new(px + HALF_W - 1.0, face_y));
            if !blocked { transform.translation.y = ny; }
        }
    }
}