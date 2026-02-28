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

## 📌 Current Known Issues & Warnings

- Some unused enum variants and struct fields trigger compiler warnings (`Sand`, `Gravel`, `Tile.tile_type`).
- Movement still uses a single tile test; sliding along walls isn't implemented.
- No monster or combat logic present.
- Player sprite may clip slightly due to scale/anchor mismatch; visual alignment is not perfected.

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

## 🎯 Summary
The project is at a playable prototype stage with tile‑based movement and procedural level switching. The most recent focus on pixel-perfect collision led to breakage, so we've rolled back to a known-good state. Going forward, splitting the codebase and planning feature phases will make future experimentation safer and more controlled.

Feel free to use this markdown as a living project plan; update it as features get added or priorities shift. Happy hacking! 🛠️
