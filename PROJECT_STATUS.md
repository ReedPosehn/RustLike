# RustLike Project Status

This document provides a snapshot of the current state of the roguelike prototype, along with context and suggested next phases.

## 🚧 Current Context

- **Language & Framework:** Rust using [Bevy 0.11](https://bevyengine.org/) for ECS, rendering, and input.
- **Workspace:** Single `src/main.rs` containing all logic; dependencies are `bevy` and `rand`.
- **Assets:** Procedurally generated 32×32 PNG tiles produced by a Python script (`generate_assets.py`).
- **Vision:** A simple top‑down dungeon crawler with tile-based movement, procedural dungeon generation, and player sprite.

## ✅ Features Implemented

1. **Project Setup** – Cargo project with Bevy, window configuration.
2. **Tilemap System** – `Tilemap` struct supporting hub and dungeon layouts.
3. **Procedural Dungeon Generator** – random rooms and corridors with stairs to hub.
4. **Player Rendering** – warrior sprite positioned bottom‑center, scaled globally.
5. **Basic Movement** – WASD/arrow keys move player; collision prevents entry into solid tiles.
6. **State Transitions** – stepping on stairs swaps between hub and dungeon with teleport+nudge.
7. **Asset Generation Tool** – Python script to create placeholder art and inspect sprite bounds.

## 📂 Source Structure

- `src/main.rs` – game logic, systems, types.
- `Cargo.toml` – dependencies.
- `generate_assets.py` – tile and sprite generator.

## 🛠 Recent Work

Over numerous iterations, collision logic was progressively refined:

- Started with simple tile lookup based on player position.
- Added AABB-based corner sampling and helper functions.
- Introduced a child/parent entity to offset the sprite and collider.
- Attempted pixel-accurate hitbox using sprite bounding-box constants.
- Hit compile-time const evaluation issues, and collision became broken.
- Ultimately **rolled back to the original simple tile-based collision** to restore playability.

The current build uses the stable movement code that allowed dungeon navigation before experimentation.

## 🔧 Bug Fixes Applied (March 2026)

The following bugs were identified and fixed in `src/main.rs`:

1. **Stair transition loops infinitely** – `update_on_stairs` would re-trigger on the very next frame before the player moved off the stair tile, bouncing them back and forth between states. Fixed by adding a `TransitionCooldown` resource that skips the stair check for one frame after each transition.

2. **Wrong tile coordinate in `update_on_stairs`** – Used a bare `as usize` cast (truncation toward zero) instead of `.floor()` when converting world coordinates to tile indices, causing stairs near the lower/left edges to never be detected. Fixed to match the `.floor()` logic already used in `player_movement`.

3. **Player sprite shrinks after state transition** – `Transform::from_xyz(x, y, 1.0)` was used to teleport the player, which replaced the transform without preserving `SCALE`. After a transition the player rendered at 1× size. Fixed by chaining `.with_scale(Vec3::splat(SCALE))`.

4. **Stale orphaned doc-comment** – A `/// return true if an AABB…` doc-comment left over from the removed AABB collision helper was deleted to reduce noise.


## 📌 Current Known Issues & Warnings

- Some unused enum variants and struct fields trigger compiler warnings (`Sand`, `Gravel`, `Tile.tile_type`).
- Movement still uses a single tile test; sliding along walls isn't implemented.
- No monster or combat logic present.
- Player sprite may clip slightly due to scale/anchor mismatch; visual alignment is not perfected.
- ~~Stair transition loops / player bounces~~ ✅ Fixed
- ~~Player shrinks after state transition~~ ✅ Fixed

## 🔮 Next Phases & Steps

### Phase 1 – Foundation & Clean-up
1. **Refactor code into modules** (`player.rs`, `map.rs`, `systems.rs`, etc.) for maintainability.
2. **Address compiler warnings** and remove dead code (unused tile types, etc.).
3. **Add comments and documentation** for public functions and types.
4. **Expand asset pipeline**: load actual art, possibly integrate texture atlases.

### Phase 2 – Improve Movement & Collision
1. **Revisit hitbox strategy**: store a `Collider` component with configurable size/offset.
2. **Implement sliding movement** so players slide along walls instead of stopping completely.
3. **Support diagonal movement and consistent speed**.
4. **Consider continuous collision detection** for smoother behavior.

### Phase 3 – Gameplay Mechanics
1. **Entities**: enemies, items, doors, traps.
2. **Combat system**: turn-based or real-time attacks, health, damage.
3. **Inventory & pickup logic**.
4. **Procedural level features**: keys/locks, rooms with themes.

### Phase 4 – Polish & UI
1. **HUD** with health, inventory, messages.
2. **Sound effects / music** integration.
3. **Save/load system**.
4. **Better art and animations** (walk cycle, idle, attack).
5. **Optional: roguelike features** like permadeath, procedural monsters, dungeon depth.

### Utility Tasks
- Add automated testing for map generation and movement logic.
- Create a `README.md` with build/run instructions and development notes (this document can serve as a starting point).
- Set up version control commits for milestones or use Git tags.

### Phase 5 – Architecture & Developer Experience
1. **Split `main.rs` into modules** (`player.rs`, `map.rs`, `systems.rs`, `state.rs`) — the file is now long enough that navigation is painful.
2. **Introduce Bevy `States`** — replace the manual `GameState` enum in `GameData` with Bevy's first-class `States` API so systems can be scheduled per-state, removing the need for runtime `match game_data.state` guards everywhere.
3. **Replace `TransitionCooldown` with Bevy `OnExit`/`OnEnter` schedules** — once `States` is adopted, the cooldown hack can be removed; tile despawn/spawn naturally belongs in `OnExit<GameState::Hub>` and `OnEnter<GameState::Dungeon>`.
4. **Add automated tests** for dungeon generation (room count, connectivity, stair placement) and tile-coordinate conversion helpers.
5. **Seed-based dungeon generation** — store the RNG seed so dungeons are reproducible for debugging and future save/load support.

### Phase 6 – Enemies & Combat (Next Gameplay Milestone)
1. **Enemy component and spawning** — spawn goblins/orcs/skeletons in dungeon rooms using the existing sprite assets already present in `assets/`.
2. **Turn-based or real-time movement AI** — simple random-walk or player-chase behaviour.
3. **Melee combat** — attack when adjacent; health component on player and enemies; basic damage numbers.
4. **Death and respawn** — despawn enemies on death; track kill count in HUD.
5. **Multiple character classes** — the mage, paladin, and rogue sprites are already in assets; let the player choose at startup.


## 🎯 Summary
The project is at a playable prototype stage with tile‑based movement and procedural level switching. The most recent focus on pixel-perfect collision led to breakage, so we've rolled back to a known-good state. Going forward, splitting the codebase and planning feature phases will make future experimentation safer and more controlled.