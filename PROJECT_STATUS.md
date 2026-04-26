# RustLike Project Status

This document provides a snapshot of the current state of the roguelike prototype, along with context and suggested next phases.

## 🚧 Current Context

- **Language & Framework:** Rust using [Bevy 0.11](https://bevyengine.org/) for ECS, rendering, and input.
- **Workspace:** Five modules in `src/` — `main.rs`, `map.rs`, `player.rs`, `state.rs`, `enemies.rs`. Dependencies are `bevy` and `rand`.
- **Assets:** Procedurally generated 32×32 PNG sprites produced by `generate_assets.py` (Pillow). All characters centred on canvas.
- **Vision:** A top-down dungeon crawler with smooth movement, procedural seeded dungeons, real-time enemy AI, and a growing gameplay loop.

## ✅ Features Implemented

1. **Project Setup** – Cargo project with Bevy, window configuration.
2. **Tilemap System** – `Tilemap` struct with `tile_center` / `world_to_tile` coordinate helpers.
3. **Procedural Dungeon Generator** – Seed-based random rooms, L-shaped corridors, stairs. Dungeon fits within 1280×720.
4. **Player Rendering** – Warrior sprite centred on a 32×32 canvas, scaled globally.
5. **AABB Movement & Collision** – WASD/arrow keys; two-corner leading-face probes; independent X/Y axes for wall sliding.
6. **State Transitions** – Bevy `States` API; `OnEnter`/`OnExit` schedules for map despawn/spawn and player teleport.
7. **Seed-Based Generation** – Seed printed to console; `FIXED_SEED` in `main.rs` replays any dungeon.
8. **Enemy Spawning** – Goblins, orcs, and skeletons placed in dungeon rooms (1–2 per room, last room reserved for stairs).
9. **Enemy AI** – Wander/chase state machine. Enemies patrol randomly and chase the player within 5 tiles; give up beyond 7 tiles. Full wall collision.
10. **Automated Tests** – 14 tests in `map.rs` covering coordinate helpers and dungeon generation invariants.
11. **Asset Generation Tool** – `generate_assets.py` creates all sprites with proper canvas centering.

## 📂 Source Structure

```
src/
├── main.rs      — constants, DungeonSeed resource, app entry, setup
├── map.rs       — TileKind, Tilemap, RoomInfo, map builders, spawn_map, TileMarker, tests
├── player.rs    — Player component, AABB player_movement system
├── state.rs     — MapState (Bevy States), World resource, transition systems
└── enemies.rs   — EnemyKind, Enemy, EnemyAi, spawn/despawn/AI systems
```

Other files:
- `Cargo.toml` — dependencies (`bevy = "0.11"`, `rand = "0.8"`)
- `generate_assets.py` — sprite and tile generator
- `CLAUDE.md` — architecture reference for Claude sessions
- `PROJECT_STATUS.md` — this file
- `README.md` — user-facing build/run instructions and roadmap

## 🛠 Recent Work

### Session — Enemy spawning & AI (April 2026)

- **Enemy spawning** — `enemies.rs` added. `EnemyKind` (Goblin, Orc, Skeleton), `Enemy` component, `EnemyMarker` for bulk despawn. `spawn_enemies` runs on `OnEnter(Dungeon)`, `despawn_enemies` on `OnExit(Dungeon)`. 1–2 enemies per room, last room skipped (stairs).

- **`build_dungeon` updated** — now returns `(Tilemap, Vec2, Vec<RoomInfo>)`. `RoomInfo` exposes room centre positions to the enemy spawner. All 14 tests updated for the new signature.

- **`World` resource updated** — gained `dungeon_rooms: Vec<RoomInfo>`.

- **Wander/chase AI** — `EnemyAi` component with `Pausing`, `Walking`, `Chasing` modes. Transitions at `CHASE_RADIUS` (5 tiles) and `LOSE_RADIUS` (7 tiles). Enemies use the same AABB collision probes as the player. Each enemy has a unique seeded RNG for independent wander patterns. `enemy_ai` system runs only in `MapState::Dungeon`.

### Session — Phase 3 complete (April 2026)

- Bevy `States` API replacing manual state management.
- Seed-based dungeon generation with `StdRng`.
- Dungeon sized to fit screen (`DUNGEON_W=33, DUNGEON_H=18`).
- 14 automated tests in `map.rs`.

### Session — Module refactor & collision rewrite (April 2026)

- AABB movement and collision rebuilt from scratch.
- Sprite assets rewritten for proper canvas centering.
- `main.rs` split into four modules.
- `CLAUDE.md` added.

## 📌 Current Known Issues

- No combat — enemies chase but don't deal or receive damage yet.
- No player health component yet.
- Enemies don't pathfind — can get stuck on corners when chasing.
- Enemies don't separate from each other — can overlap.
- Stair detection uses player centre only (acceptable at current speed).
- Dungeon dimensions must be recalculated if `SCALE` changes.

## 🔮 Next Phases & Steps

### ✅ Phase 1 – Prototype
- ✅ Project setup, tilemap, dungeon, player movement, state transitions, assets.

### ✅ Phase 2 – Movement & Collision
- ✅ AABB collision with wall sliding, independent axes, sprite hitbox aligned.

### ✅ Phase 3 – Architecture & Developer Experience
- ✅ Modular codebase, Bevy `States`, seed-based generation, screen-fit dungeon, 14 tests.

### ⚔️ Phase 4 – Enemies & Combat (in progress)
- ✅ Enemy spawning (goblin, orc, skeleton)
- ✅ Wander/chase AI with wall collision
- ⬜ Health component on player and enemies
- ⬜ Melee combat — attack when adjacent, damage numbers
- ⬜ Death and despawn, kill counter
- ⬜ Multiple player classes (mage, paladin, rogue sprites ready)

### 🎮 Phase 5 – Gameplay Mechanics
1. Inventory and item pickup.
2. Procedural level features: keys/locks, themed rooms.
3. Dungeon depth and increasing difficulty.

### ✨ Phase 6 – Polish & UI
1. HUD: health bar, inventory, message log.
2. Sound effects and music.
3. Save/load system (`DungeonSeed` resource already in place).
4. Walk cycle, idle, and attack animations.
5. Permadeath and roguelike meta-progression.
6. Camera scrolling for larger dungeons.

## 🎯 Summary

The project has a solid foundation with working enemies. Phase 3 is complete and Phase 4 is underway — enemies spawn and chase the player with wall-aware AI. The immediate next step is adding health components and melee combat to complete the core gameplay loop.