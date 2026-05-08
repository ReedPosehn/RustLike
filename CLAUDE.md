# CLAUDE.md — RustLike

This file gives Claude context about the project so it can assist effectively.

## Project Overview

RustLike is a top-down roguelike prototype written in Rust using the Bevy 0.11 ECS framework. It features procedural seeded dungeons, AABB movement with wall sliding, Bevy-managed state transitions, real-time enemy AI with wander/chase behaviour, melee combat with health and damage, and a HUD with gradient health bars. The codebase is split across six modules in `src/`:
- `main.rs` — constants, `DungeonSeed` resource, app entry point, `setup` system
- `map.rs` — `TileKind`, `Tilemap`, `RoomInfo`, map builders, `spawn_map`, `TileMarker`, tests
- `player.rs` — `Player`, `spawn_player`, `player_movement`
- `state.rs` — `MapState` (Bevy `States`), `GameWorld` resource, transition systems
- `enemies.rs` — `EnemyKind`, `Enemy`, `EnemyAi`, spawn/despawn/AI systems
- `combat.rs` — `Health`, `Dead`, `Facing`, `DamageEvent`, `SplatEvent`, combat systems
- `hud.rs` — `BarAssets`, player health bar (gradient), enemy bars, damage splats

## Build & Run

```bash
# Install deps and run
cargo run

# Run automated tests
cargo test

# Regenerate all sprite and tile assets (requires Pillow)
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

### Key Constants (`src/combat.rs`)
| Constant | Value | Purpose |
|---|---|---|
| `CONTACT_DAMAGE_INTERVAL` | 0.8s | Seconds between enemy contact damage ticks |
| `PLAYER_MELEE_DAMAGE` | 25 | Damage per melee strike (F key) |
| `ENEMY_CONTACT_DAMAGE` | 10 | Damage per enemy contact tick |

### Tilemap Coordinate System
- **col 0** = leftmost column, **row 0** = bottom row (Y-up, matching Bevy world space)
- Tile `(col, row)` has its world-space centre at:
  ```
  x = (col - w/2) * TILE + TILE/2
  y = (row - h/2) * TILE + TILE/2
  ```
- Inverse (world → tile): `col = floor(px / TILE + w/2)` — **no TILE/2 offset**. Adding `- TILE/2` shifts the grid by half a tile and causes asymmetric collision gaps.

### Collision System
AABB collision with `Anchor::Center` (translation = sprite centre). Used by both player and enemies:
- **X axis**: test two corners on the leading face — `(face_x, py ± (HALF_H - 1))`
- **Y axis**: test two corners on the leading face — `(px ± (HALF_W - 1), face_y)`
- Axes resolve independently → wall sliding works automatically
- Out-of-bounds probes treated as solid

### Combat System (`src/combat.rs`)
All damage flows through `DamageEvent { target, amount, source }`. Add new `DamageSource` variants (`Ranged`, `Magic`, `AreaOfEffect`, etc.) without touching existing systems.

- `Facing(Vec2)` — updated each frame from last movement direction; used for melee targeting
- `player_melee_attack` — `F` key; hits enemies within one tile ahead in facing direction
- `enemy_contact_damage` — fires `DamageEvent` when enemy and player AABBs overlap, gated by `ContactDamageTimer` (0.8s interval)
- `apply_damage` — drains `Health`, inserts `Dead`, fires `SplatEvent` with hit position
- `despawn_dead_enemies` — removes dead enemy entities

### Enemy HP by kind
| EnemyKind | Max HP |
|---|---|
| Goblin | 30 |
| Orc | 60 |
| Skeleton | 45 |

### HUD (`src/hud.rs`)
- **Player health bar** — world-space UI nodes inside a rounded `ImageBundle` container (`ui_panel.png`). White 2px border around a dark-red background. Fill is an `ImageBundle` with a gradient texture built in memory via `Assets<Image>::add()` (no async loading). Gradient swaps: green → yellow → red as HP drops.
- **Enemy health bars** — world-space sprites (`EnemyBarFor(Entity)` component) that follow each enemy. Always green; shrink to show remaining HP. Tagged `EnemyMarker` for automatic despawn.
- **Damage splats** — `Text2dBundle` spawned at hit position, rises 28px/s and fades over 0.9s.
- `BarAssets` resource — holds gradient `Handle<Image>` for all three fill states.

### State Transitions (Bevy `States`)
`MapState` derives Bevy's `States` trait. No manual cooldown.
- `stair_detection` (`Update`): detects player on stair, calls `next_state.set(...)`
- `despawn_map` / `despawn_enemies` (`OnExit`): bulk-despawn by marker component
- `on_enter_hub` / `on_enter_dungeon` (`OnEnter`): spawn map, teleport player
- `spawn_enemies` (`OnEnter(Dungeon)`): 1–2 enemies per room, last room skipped (stairs)
- `OnEnter(Hub)` does NOT fire at startup — initial hub is spawned in `setup`

### Seed-Based Generation
- Seed printed to stdout at startup: `Dungeon seed: 12345  (set FIXED_SEED = Some(12345) to replay)`
- `build_dungeon(seed)` uses `StdRng::seed_from_u64(seed)`
- Enemy placement uses `seed + 1`; each enemy's AI uses a further unique offset
- `DungeonSeed` resource stores the seed for future save/load

### `GameWorld` Resource
Renamed from `World` to avoid clash with `bevy::prelude::World` (Bevy's ECS world type). Contains both tilemaps, stair positions, and `dungeon_rooms: Vec<RoomInfo>`.

### Automated Tests (`src/map.rs`)
14 tests in `#[cfg(test)]`. Run with `cargo test`. Covers coordinate round-trips, dungeon invariants, determinism, hub correctness.

## Asset Pipeline

`generate_assets.py` (requires Pillow) produces all 32×32 PNGs. Re-run after any sprite changes.

### Generated assets
| File | Purpose |
|---|---|
| `warrior/mage/paladin/rogue.png` | Player class sprites (centred on canvas) |
| `goblin/orc/skeleton/spider.png` | Enemy sprites |
| `grass/dirt/stone/wood/water/sand/gravel/rock/wall/door.png` | Tile textures (hand-crafted patterns) |
| `ui_panel.png` | Dark semi-transparent rounded HUD container |
| `ui_bar.png` | White rounded rect (used by world-space player bar background) |

### Tile → Solid mapping
| TileKind | Solid? |
|---|---|
| Wall, Rock, Water | ✅ |
| Everything else | ❌ |

## Known Issues & Caveats

- **No player death screen yet** — `Dead` marker is added to the player but no game-over state/UI exists
- **No ranged/magic attacks** — `DamageSource` has `Ranged`/`Magic` variants, systems not yet added
- **No inventory or items** — Phase 5
- **Enemies don't pathfind** — move directly toward player, can get stuck on corners
- **Enemies don't separate** — multiple enemies can overlap
- **Dungeon sizing tied to window** — `DUNGEON_W/H` must be recalculated if `SCALE` or resolution changes
- **No camera scrolling** — dungeon sized to window instead

## Roadmap (see also PROJECT_STATUS.md)

1. **Next** — Player death / game-over screen, ranged/magic attacks
2. Multiple player classes, inventory, HUD expansion
3. Sound, save/load, animations, permadeath