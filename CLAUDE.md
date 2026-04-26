# CLAUDE.md — RustLike

This file gives Claude context about the project so it can assist effectively.

## Project Overview

RustLike is a top-down roguelike prototype written in Rust using the Bevy 0.11 ECS framework. It features procedural seeded dungeons, AABB movement with wall sliding, Bevy-managed state transitions, enemy AI with wander/chase behaviour, and a full automated test suite. The codebase is split across five modules in `src/`:
- `main.rs` — constants, `DungeonSeed` resource, app entry point, `setup` system
- `map.rs` — `TileKind`, `Tilemap`, `RoomInfo`, map builders, `spawn_map`, `TileMarker`, tests
- `player.rs` — `Player` component, `player_movement` system
- `state.rs` — `MapState` (Bevy `States`), `World` resource, transition systems
- `enemies.rs` — `EnemyKind`, `Enemy`, `EnemyAi`, spawn/despawn/AI systems

## Build & Run

```bash
# Install deps and run
cargo run

# Run automated tests
cargo test

# Regenerate all sprite assets (requires Pillow)
pip install pillow
python generate_assets.py
```

Assets are 32×32 PNG files in `assets/`. The game window is 1280×720.

## Architecture

### Key Constants (`src/main.rs`)
| Constant | Value | Purpose |
|---|---|---|
| `TILE_PX` | 32.0 | Source PNG size in pixels |
| `SCALE` | 1.2 | Global sprite scale factor |
| `TILE` | 38.4 | World-space tile size (`TILE_PX * SCALE`) |
| `HALF_W / HALF_H` | `TILE / 2.0` | Player AABB half-extents |
| `SPEED` | 150.0 | Player movement speed (world units/sec) |
| `DUNGEON_W/H` | 33 × 18 | Dungeon dimensions — fits exactly in 1280×720 at TILE=38.4 |
| `FIXED_SEED` | `Option<u64>` | Set to `Some(n)` to pin a dungeon layout, `None` for random |

### Key Constants (`src/enemies.rs`)
| Constant | Value | Purpose |
|---|---|---|
| `WANDER_SPEED` | 45.0 | Enemy speed while wandering |
| `CHASE_SPEED` | 90.0 | Enemy speed while chasing player |
| `CHASE_RADIUS` | `TILE * 5.0` | Distance at which enemy starts chasing |
| `LOSE_RADIUS` | `TILE * 7.0` | Distance at which chasing enemy gives up |
| `WANDER_WALK_SECS` | 1.2 | Seconds per wander step |
| `WANDER_PAUSE_SECS` | 0.6 | Seconds paused between wander steps |

### Tilemap Coordinate System
- **col 0** = leftmost column, **row 0** = bottom row (Y-up, matching Bevy world space)
- Tile `(col, row)` has its world-space centre at:
  ```
  x = (col - w/2) * TILE + TILE/2
  y = (row - h/2) * TILE + TILE/2
  ```
- Inverse (world → tile): `col = floor(px / TILE + w/2)` — **no TILE/2 offset**. Adding `- TILE/2` shifts the grid by half a tile and causes asymmetric collision gaps.

### Collision System
AABB collision with `Anchor::Center` (translation = sprite centre). Used by both the player and enemies:
- **X axis**: test two corners on the leading face — `(face_x, py ± (HALF_H - 1))`.
- **Y axis**: test two corners on the leading face — `(px ± (HALF_W - 1), face_y)`.
- Axes resolve independently → wall sliding works automatically.
- The 1px inset on perpendicular corners prevents false positives on exact tile boundaries.
- Out-of-bounds probes (`world_to_tile` returns `None`) are treated as solid.

### Enemy AI (`src/enemies.rs`)
Each enemy has an `EnemyAi` component with three modes:
- `Pausing { remaining }` — standing still for a short time
- `Walking { dir, remaining }` — moving in a random direction for ~1.2s
- `Chasing` — moving directly toward the player at chase speed

**Transitions:**
- Any mode → `Chasing` when `dist_to_player < CHASE_RADIUS`
- `Chasing` → `Pausing` when `dist_to_player > LOSE_RADIUS`
- The two-radius hysteresis prevents jitter at the boundary

**Determinism:** Each enemy gets a unique AI seed derived from the dungeon seed so wander patterns are reproducible. `enemy_ai` runs only in `MapState::Dungeon`.

### State Transitions (Bevy `States`)
`MapState` derives Bevy's `States` trait. No manual cooldown resource.

- **`stair_detection`** (`Update`): detects player on stair tile, calls `next_state.set(...)`
- **`despawn_map`** (`OnExit` both states): bulk-despawns `TileMarker` entities
- **`despawn_enemies`** (`OnExit(Dungeon)`): bulk-despawns `EnemyMarker` entities
- **`on_enter_hub`** (`OnEnter(Hub)`): spawns hub map, teleports player
- **`on_enter_dungeon`** (`OnEnter(Dungeon)`): spawns dungeon map, teleports player
- **`spawn_enemies`** (`OnEnter(Dungeon)`): spawns enemies in all rooms except the last
- `OnEnter(Hub)` does NOT fire at startup — initial hub is spawned directly in `setup`

### Seed-Based Generation
- Dungeon seed: `FIXED_SEED` in `main.rs`, or random `u64` generated at startup
- Printed to stdout: `Dungeon seed: 12345  (set FIXED_SEED = Some(12345) to replay)`
- `build_dungeon(seed)` uses `StdRng::seed_from_u64(seed)`
- Enemy placement uses `seed + 1`; each enemy's wander AI uses a further unique offset
- Seed stored in `DungeonSeed` resource for future save/load

### Dungeon Sizing
`DUNGEON_W=33, DUNGEON_H=18` derived from `floor(1280/TILE) × floor(720/TILE)`. Recalculate if `SCALE` or window resolution changes.

### ECS Components & Resources
| Name | Type | Purpose |
|---|---|---|
| `Player` | Component | Tags the player entity |
| `TileMarker` | Component | Tags all tile sprites for bulk despawn |
| `EnemyMarker` | Component | Tags all enemy entities for bulk despawn |
| `Enemy` | Component | Stores `EnemyKind` on each enemy |
| `EnemyAi` | Component | Wander/chase state machine + per-enemy RNG |
| `World` | Resource | Both tilemaps, stair positions, room centres |
| `DungeonSeed` | Resource | The `u64` seed used for this run |

### Automated Tests (`src/map.rs`)
14 tests in `#[cfg(test)]` at the bottom of `map.rs`. Run with `cargo test`.

## Asset Pipeline

`generate_assets.py` produces all 32×32 PNGs. Characters are centred on the canvas so the hitbox (`HALF_W = TILE/2`) aligns with the visible sprite.

### Tile → Asset Mapping
| TileKind | File | Solid? |
|---|---|---|
| Wall | wall.png | ✅ |
| Rock | rock.png | ✅ |
| Water | water.png | ✅ |
| Stairs | gravel.png | ❌ |
| Everything else | matching name | ❌ |

## Known Issues & Caveats

- **No combat yet**: enemies chase but don't deal or receive damage — Phase 4 continues.
- **No player health**: health component not yet added.
- **Enemies don't pathfind**: they move directly toward the player and can get stuck on corners. A simple unstick nudge or proper pathfinding (A*) would help.
- **Enemies don't respect each other**: no separation — multiple enemies can overlap.
- **Stair detection uses player centre only**: could miss at high speeds (acceptable for now).
- **Dungeon sizing tied to window**: `DUNGEON_W/H` must be recalculated if `SCALE` changes.

## Roadmap (see also PROJECT_STATUS.md)
7. **Next** — Health components, melee combat, death/despawn
8. Multiple player classes (sprites ready)
9. Inventory and item pickups
10. HUD, sound, save/load