# RustLike

A top-down roguelike prototype built in Rust using the Bevy engine.
Features procedural seeded dungeons, AABB movement with wall sliding, real-time enemy AI, melee combat, and a gradient HUD.

## Getting Started

```bash
# Clone
git clone https://github.com/ReedPosehn/RustLike.git
cd RustLike

# Generate assets (requires Pillow)
pip install pillow
python generate_assets.py

# Build and run
cargo run

# Run tests
cargo test
```

To replay a specific dungeon: copy the seed printed to the console at startup and set `FIXED_SEED = Some(your_seed)` in `src/main.rs`.

## Controls

| Key | Action |
|---|---|
| WASD / Arrow keys | Move |
| F | Melee attack (hits tile ahead in movement direction) |
| Walk onto stairs | Enter / exit dungeon |

## Source Layout

```
src/
├── main.rs      # constants, DungeonSeed resource, app entry, setup
├── map.rs       # TileKind, Tilemap, map builders, tile spawning, tests
├── player.rs    # Player component, AABB movement & collision
├── state.rs     # MapState (Bevy States), GameWorld resource, transitions
├── enemies.rs   # EnemyKind, Enemy, EnemyAi, spawn/despawn/AI systems
├── combat.rs    # Health, DamageEvent, Facing, contact damage, melee
└── hud.rs       # Gradient health bar, enemy bars, damage splats
```

## Features

- Procedural dungeon generation — random rooms, L-shaped corridors, stairs
- Seed-based generation — reproducible layouts, seed printed at startup
- AABB player movement with wall sliding
- Hub ↔ dungeon transitions via Bevy `OnEnter`/`OnExit` state schedules
- Dungeon always fits the 1280×720 window
- Enemies (goblin, orc, skeleton) with wander/chase AI and wall collision
- Melee combat — press F to attack in the direction you're moving
- Contact damage — enemies hurt the player on touch (0.8s cooldown)
- Gradient health bar (green → yellow → red) with rounded panel and white border
- Enemy health bars that shrink as enemies take damage
- Floating damage splat numbers at hit positions
- 14 automated tests covering coordinate math and dungeon generation

## Roadmap

### ✅ Phase 1 – Prototype
### ✅ Phase 2 – Movement & Collision
### ✅ Phase 3 – Architecture & Developer Experience
- Modular codebase, Bevy `States` API, seed-based generation, automated tests

### ⚔️ Phase 4 – Enemies & Combat (in progress)
- ✅ Enemy spawning, wander/chase AI, wall collision
- ✅ Health system + extensible DamageEvent
- ✅ Player melee (F key), enemy contact damage
- ✅ Gradient HUD, enemy health bars, damage splats
- ⬜ Player death + game-over screen
- ⬜ Ranged / magic attacks
- ⬜ Multiple player classes

### 🎮 Phase 5 – Gameplay Mechanics
- Inventory and item pickup, keys/locks, dungeon depth

### ✨ Phase 6 – Polish & UI
- Sound, save/load, animations, permadeath, camera scrolling

## Project Status

See [`PROJECT_STATUS.md`](PROJECT_STATUS.md) for detailed current state, recent work, and known issues.

## License

MIT — see [`LICENSE`](LICENSE).