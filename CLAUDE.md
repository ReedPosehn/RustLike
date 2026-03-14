# CLAUDE.md — RustLike

This file gives Claude context about the project so it can assist effectively.

## Project Overview

RustLike is a top-down roguelike prototype written in Rust using the Bevy 0.11 ECS framework. It features procedural dungeon generation, tile-based maps, smooth AABB player movement, and hub ↔ dungeon state transitions. The codebase lives in a single `src/main.rs` file; splitting into modules is a planned next step.

## Build & Run

```bash
# Install deps and run
cargo run

# Regenerate all sprite assets (requires Pillow)
pip install pillow
python generate_assets.py
```

Assets are 32×32 PNG files in `assets/`. The game window is 1280×720.

## Architecture

### Key Constants (`src/main.rs`)
| Constant | Value | Purpose |
|---|---|---
| `TILE_PX` | 32.0 | Source PNG size in pixels |
| `SCALE` | 1.2 | Global sprite scale factor |
| `TILE` | 38.4 | World-space tile size (`TILE_PX * SCALE`) |
| `HALF_W / HALF_H` | `TILE / 2.0` | Player AABB half-extents |
| `SPEED` | 150.0 | Player movement speed (world units/sec) |
| `DUNGEON_W/H` | 40 × 30 | Dungeon map dimensions in tiles |
| `COOLDOWN_FRAMES` | 60 | Frames before stair transition can re-fire |

### Tilemap Coordinate System
- **col 0** = leftmost column, **row 0** = bottom row (Y-up, matching Bevy world space)
- Tile `(col, row)` has its world-space centre at:
  ```
  x = (col - w/2) * TILE + TILE/2
  y = (row - h/2) * TILE + TILE/2
  ```
- Inverse (world → tile): `col = floor(px / TILE + w/2)` — **no TILE/2 offset**. This is the correct formula; adding a `- TILE/2` shifts the grid by half a tile and causes asymmetric collision gaps.

### Collision System
AABB collision with `Anchor::Center` (translation = sprite centre):
- **X axis**: test two corners on the leading face — `(face_x, py ± (HALF_H - 1))`.  
- **Y axis**: test two corners on the leading face — `(px ± (HALF_W - 1), face_y)`.  
- Axes resolve independently → wall sliding works automatically.
- The 1px inset on perpendicular corners prevents false positives on exact tile boundaries.
- Out-of-bounds probes (`world_to_tile` returns `None`) are treated as solid.

### State Transitions
- `StairCooldown(u32)` counts down each frame after a transition. Set to `COOLDOWN_FRAMES` (60) on fire; blocks re-triggering until zero.
- On transition: old tile entities are despawned, new map is spawned, player is teleported to `dest_stairs + (0, TILE)` — one tile above the destination stair, always inside the room interior.
- `commands.entity(...).insert(Transform)` is deferred (applies next frame), which is why the cooldown must be long enough to outlast it.

### ECS Components & Resources
| Name | Type | Purpose |
|---|---|---|
| `Player` | Component | Tags the player entity |
| `TileMarker` | Component | Tags all spawned tile sprites for bulk despawn |
| `World` | Resource | Holds both tilemaps, stair positions, and current `MapState` |
| `StairCooldown` | Resource | Frame countdown preventing stair re-trigger |

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

- **Single file**: all game logic is in `src/main.rs`. Splitting into modules (`map.rs`, `player.rs`, `systems.rs`) is the next planned refactor.
- **Bevy `States` not used**: `MapState` is a plain enum inside the `World` resource. Migrating to Bevy's first-class `States` API would allow per-state system scheduling and cleaner `OnEnter`/`OnExit` hooks.
- **No enemies or combat**: the enemy sprite assets (goblin, orc, skeleton, spider) exist but are not yet wired into the game.
- **Stair detection uses player centre only**: a large fast-moving player could theoretically skip over a stair tile. Acceptable at current speed.
- **Dungeon regenerates on every run**: no seed persistence yet.

## Roadmap (see also PROJECT_STATUS.md)

1. Split `main.rs` into modules
2. Adopt Bevy `States` API
3. Enemy spawning and basic combat
4. Inventory and item pickups
5. HUD, sound, save/load