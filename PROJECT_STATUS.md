# RustLike Project Status

This document provides a snapshot of the current state of the roguelike prototype, along with context and suggested next phases.

## 🚧 Current Context

- **Language & Framework:** Rust using [Bevy 0.11](https://bevyengine.org/) for ECS, rendering, and input.
- **Workspace:** Four modules in `src/` — `main.rs`, `map.rs`, `player.rs`, `state.rs`. Dependencies are `bevy` and `rand`.
- **Assets:** Procedurally generated 32×32 PNG tiles and character sprites produced by `generate_assets.py` (Pillow). All characters are centred on the canvas.
- **Vision:** A top-down dungeon crawler with tile-based movement, procedural dungeon generation, Bevy-managed state transitions, and a growing gameplay loop.

## ✅ Features Implemented

1. **Project Setup** – Cargo project with Bevy, window configuration.
2. **Tilemap System** – `Tilemap` struct with `tile_center` / `world_to_tile` coordinate helpers.
3. **Procedural Dungeon Generator** – Seed-based random rooms, L-shaped corridors, and stairs. Dungeon fits within the 1280×720 window.
4. **Player Rendering** – Warrior sprite centred on a 32×32 canvas, scaled globally.
5. **AABB Movement & Collision** – WASD/arrow keys; two-corner leading-face probes; independent X/Y resolution for wall sliding.
6. **State Transitions** – Bevy `States` API; `OnEnter`/`OnExit` schedules handle map despawn/spawn and player teleport. No manual cooldown.
7. **Seed-Based Generation** – Seed printed to console at startup; set `FIXED_SEED` in `main.rs` to replay any dungeon.
8. **Automated Tests** – 14 tests in `map.rs` covering coordinate helpers, dungeon generation invariants, and hub correctness.
9. **Asset Generation Tool** – `generate_assets.py` creates all sprites and tiles with proper canvas centering.

## 📂 Source Structure

```
src/
├── main.rs     — constants, DungeonSeed resource, app entry, setup
├── map.rs      — TileKind, Tilemap, map builders, spawn_map, TileMarker, tests
├── player.rs   — Player component, AABB player_movement system
└── state.rs    — MapState (Bevy States), World resource, transition systems
```

Other files:
- `Cargo.toml` — dependencies (`bevy = "0.11"`, `rand = "0.8"`)
- `generate_assets.py` — sprite and tile generator
- `CLAUDE.md` — architecture reference for Claude sessions
- `PROJECT_STATUS.md` — this file
- `README.md` — user-facing build/run instructions and roadmap

## 🛠 Recent Work

### Session — Phase 3 complete (April 2026)

- **Bevy `States` API** — replaced manual `MapState` field in `World` with Bevy's first-class `States`. `stair_transition` split into focused systems: `stair_detection` (detects and calls `next.set()`), `despawn_map` (`OnExit`), `on_enter_hub` / `on_enter_dungeon` (`OnEnter`). `StairCooldown` removed entirely — `OnEnter`/`OnExit` run between frames so no cooldown is needed.

- **Seed-based dungeon generation** — `build_dungeon(seed: u64)` uses `StdRng::seed_from_u64`. Seed printed to console at startup; `FIXED_SEED` constant in `main.rs` pins a layout. `DungeonSeed` resource stores the seed for future save/load.

- **Dungeon fits screen** — `DUNGEON_W` 40→33, `DUNGEON_H` 30→18, derived from `floor(1280/TILE) × floor(720/TILE)`. The entire dungeon is always visible without scrolling.

- **Automated tests** — 14 tests in `#[cfg(test)]` at the bottom of `map.rs`. Covers `tile_center`/`world_to_tile` round-trips, boundary conditions, dungeon invariants (room count, stair count, stair solidity, stair bounds), determinism, and hub correctness.

### Session — Module refactor & collision rewrite (April 2026)

- **Rebuilt movement and collision from scratch** using AABB. `Anchor::Center` → translation = sprite centre. Two corner probes on the leading face; axes independent for wall sliding. Correct `world_to_tile` formula: `floor(px / TILE + w/2)` (no `TILE/2` offset).
- **Fixed sprite assets** — original art was 8×15px in the corner of a 32×32 canvas. `generate_assets.py` rewritten for properly centred, full-canvas sprites.
- **Split `src/main.rs` into modules**: `map.rs`, `player.rs`, `state.rs`, `main.rs`.
- **Added `CLAUDE.md`** with architecture notes and coordinate system documentation.

## 📌 Current Known Issues

- No enemies or combat — sprite assets exist (goblin, orc, skeleton, spider, mage, paladin, rogue) but are not wired in.
- No camera scrolling — dungeon is sized to fit the window instead.
- Dungeon dimensions must be manually recalculated if `SCALE` or window resolution changes.
- Stair detection uses player centre only — could miss at high speeds (acceptable for now).

## 🔮 Next Phases & Steps

### ✅ Phase 1 – Prototype
- ✅ Project setup, tilemap, procedural dungeon, player movement, state transitions, assets.

### ✅ Phase 2 – Movement & Collision
- ✅ AABB collision with wall sliding
- ✅ Independent X/Y axis resolution
- ✅ Sprite hitbox aligned to canvas

### ✅ Phase 3 – Architecture & Developer Experience
- ✅ Split `main.rs` into modules
- ✅ Adopt Bevy `States` API (`OnEnter`/`OnExit`)
- ✅ Seed-based dungeon generation
- ✅ Dungeon sized to fit screen
- ✅ Automated tests (14 in `map.rs`)

### ⚔️ Phase 4 – Enemies & Combat (next)
1. **Enemy component and spawning** — spawn goblins/orcs/skeletons in dungeon rooms using existing sprites.
2. **Movement AI** — random walk or player-chase behaviour.
3. **Melee combat** — health component, attack-when-adjacent, damage.
4. **Death and despawn** — remove enemies on death, track kill count.
5. **Multiple player classes** — mage, paladin, rogue sprites already present.

### 🎮 Phase 5 – Gameplay Mechanics
1. Inventory and item pickup.
2. Procedural level features: keys/locks, themed rooms.
3. Dungeon depth and increasing difficulty.

### ✨ Phase 6 – Polish & UI
1. HUD: health bar, inventory, message log.
2. Sound effects and music.
3. Save/load system (seed already stored in `DungeonSeed` resource).
4. Walk cycle, idle, and attack animations.
5. Permadeath and roguelike meta-progression.
6. Camera scrolling for larger dungeons.

## 🎯 Summary

The project has a solid, well-tested foundation. Phase 3 is complete: the codebase is modular, state transitions use Bevy's native API, dungeons are reproducible via seed, and the dungeon always fits on screen. Phase 4 (enemies and combat) is the next milestone — all the sprite assets are already in place.