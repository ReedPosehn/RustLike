# CLAUDE.md — RustLike

This file gives Claude context about the project so it can assist effectively.

## Project Overview

RustLike is a top-down roguelike prototype written in Rust using the Bevy 0.11 ECS framework. It features procedural dungeon generation, tile-based maps, smooth AABB player movement, Bevy-managed state transitions, seed-based dungeon generation, and a full automated test suite. The codebase is split across four modules in `src/`:
- `main.rs` — constants, `DungeonSeed` resource, app entry point, `setup` system
- `map.rs` — `TileKind`, `Tilemap`, map builders, `spawn_map`, `TileMarker`, tests
- `player.rs` — `Player` component, `player_movement` system
- `state.rs` — `MapState` (Bevy `States`), `World` resource, transition systems

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

### Tilemap Coordinate System
- **col 0** = leftmost column, **row 0** = bottom row (Y-up, matching Bevy world space)
- Tile `(col, row)` has its world-space centre at:
  ```
  x = (col - w/2) * TILE + TILE/2
  y = (row - h/2) * TILE + TILE/2
  ```
- Inverse (world → tile): `col = floor(px / TILE + w/2)` — **no TILE/2 offset**. Adding `- TILE/2` shifts the grid by half a tile and causes asymmetric collision gaps.

### Collision System
AABB collision with `Anchor::Center` (translation = sprite centre):
- **X axis**: test two corners on the leading face — `(face_x, py ± (HALF_H - 1))`.
- **Y axis**: test two corners on the leading face — `(px ± (HALF_W - 1), face_y)`.
- Axes resolve independently → wall sliding works automatically.
- The 1px inset on perpendicular corners prevents false positives on exact tile boundaries.
- Out-of-bounds probes (`world_to_tile` returns `None`) are treated as solid.

### State Transitions (Bevy `States`)
`MapState` derives Bevy's `States` trait — the engine owns the current state via `State<MapState>` and `NextState<MapState>`. No manual cooldown resource exists.

- **`stair_detection`** (runs in `Update`): checks if the player centre is on a stair tile; if so, calls `next_state.set(...)`.
- **`despawn_map`** (runs on `OnExit` for both states): bulk-despawns all `TileMarker` entities.
- **`on_enter_hub` / `on_enter_dungeon`** (runs on `OnEnter`): spawns the new map and teleports the player one tile above the destination stairs.
- Bevy runs `OnExit` → `OnEnter` between frames, so by the time `stair_detection` fires again the player is already at the new position — no cooldown needed.
- **Important**: `OnEnter(MapState::Hub)` does NOT fire at startup (Hub is the default state). The initial hub map is spawned directly in `setup`.

### Seed-Based Dungeon Generation
- A `u64` seed is generated at startup (or read from `FIXED_SEED` in `main.rs`).
- Printed to stdout: `Dungeon seed: 12345  (set FIXED_SEED = Some(12345) to replay)`.
- `build_dungeon(seed)` uses `rand::rngs::StdRng::seed_from_u64(seed)` — deterministic and portable.
- The seed is stored in the `DungeonSeed` resource for future save/load use.
- To replay a dungeon: copy the seed from the console, set `FIXED_SEED = Some(that_number)`, rebuild.

### Dungeon Sizing
`DUNGEON_W=33, DUNGEON_H=18` is derived from `floor(1280 / TILE) × floor(720 / TILE)`. This ensures the entire dungeon fits within the 1280×720 window at all times. If `SCALE` is changed, recalculate these values.

### ECS Components & Resources
| Name | Type | Purpose |
|---|---|---|
| `Player` | Component | Tags the player entity |
| `TileMarker` | Component | Tags all spawned tile sprites for bulk despawn |
| `World` | Resource | Holds both tilemaps and stair positions (no state field) |
| `DungeonSeed` | Resource | The `u64` seed used for this run's dungeon |

### Automated Tests (`src/map.rs`)
14 tests in a `#[cfg(test)]` module at the bottom of `map.rs`. Run with `cargo test`. Coverage:
- `tile_center` / `world_to_tile` round-trip for every tile in the map
- Near-edge interior points resolve correctly
- Out-of-bounds points return `None` / treated as solid
- Dungeon always contains floor tiles and exactly one stair
- Stair tile is non-solid and within map bounds
- Same seed produces identical layout; different seeds produce different layouts
- Hub stair position is accurate and non-solid

## Asset Pipeline

`generate_assets.py` produces all 32×32 PNGs using Pillow. Characters are centred on the canvas so the hitbox (`HALF_W = TILE/2`) aligns with the visible sprite. Re-run this script whenever you modify sprite art.

### Tile → Asset Mapping
| TileKind | File | Solid? |
|---|---|---|
| Wall | wall.png | ✅ |
| Rock | rock.png | ✅ |
| Water | water.png | ✅ |
| Stairs | gravel.png | ❌ |
| Everything else | matching name | ❌ |

## Known Issues & Caveats

- **No enemies or combat**: enemy sprite assets (goblin, orc, skeleton, spider) exist but are not yet wired into the game — Phase 4.
- **Stair detection uses player centre only**: a large fast-moving player could theoretically skip over a stair tile. Acceptable at current speed.
- **Dungeon sizing tied to window size**: `DUNGEON_W/H` must be manually recalculated if `SCALE` or window resolution changes.
- **No camera scrolling**: the dungeon is sized to fit the window rather than implementing a scrolling camera. This limits dungeon complexity.

## Roadmap (see also PROJECT_STATUS.md)

1. **Phase 4** — Enemy spawning and basic combat
2. Inventory and item pickups
3. HUD, sound, save/load